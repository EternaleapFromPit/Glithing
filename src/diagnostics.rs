use std::collections::{HashMap, HashSet};

use crate::ast::*;
use crate::codegen::sanitize_ident;
use crate::tir::{self, TypedProgram};

pub(crate) struct CompatibilityAnalyzer<'a> {
    source: &'a str,
    known_types: HashSet<String>,
    symbols: HashSet<String>,
    interface_implementations: HashMap<String, String>,
    emitted: HashSet<String>,
    diagnostics: Vec<String>,
}

impl<'a> CompatibilityAnalyzer<'a> {
    pub(crate) fn analyze(
        source: &'a str,
        _program: &Program,
        typed: &TypedProgram,
        emit_llvm: bool,
    ) -> Vec<String> {
        let interface_names = typed
            .types
            .iter()
            .filter(|ty| matches!(ty.kind, TypeKind::Interface))
            .map(|ty| ty.name.clone())
            .collect::<HashSet<_>>();
        let mut interface_candidates = HashMap::<String, Vec<String>>::new();
        for ty in &typed.types {
            if !matches!(ty.kind, TypeKind::Class) {
                continue;
            }
            for base in &ty.bases {
                let base = base_type_name(base).to_string();
                if interface_names.contains(&base) {
                    interface_candidates
                        .entry(base)
                        .or_default()
                        .push(ty.name.clone());
                }
            }
        }
        let interface_implementations = interface_candidates
            .into_iter()
            .filter_map(|(interface, mut implementations)| {
                implementations.sort();
                implementations.dedup();
                (implementations.len() == 1).then(|| (interface, implementations.remove(0)))
            })
            .collect();
        let mut analyzer = Self {
            source,
            known_types: typed.types.iter().map(|ty| ty.name.clone()).collect(),
            symbols: typed
                .functions
                .iter()
                .map(|function| function.symbol.clone())
                .chain(typed.types.iter().flat_map(|ty| {
                    ty.constructors
                        .iter()
                        .chain(&ty.methods)
                        .map(|function| function.symbol.clone())
                }))
                .collect(),
            interface_implementations,
            emitted: HashSet::new(),
            diagnostics: Vec::new(),
        };
        analyzer.detect_native_blocks();
        analyzer.detect_unexecutable_endpoints(typed);
        for function in &typed.functions {
            analyzer.visit_stmts(&function.body, emit_llvm);
        }
        for ty in &typed.types {
            for function in ty.constructors.iter().chain(&ty.methods) {
                analyzer.visit_stmts(&function.body, emit_llvm);
            }
        }
        analyzer.diagnostics
    }

    fn detect_unexecutable_endpoints(&mut self, typed: &TypedProgram) {
        let types = typed
            .types
            .iter()
            .map(|ty| (ty.name.as_str(), ty))
            .collect::<HashMap<_, _>>();
        for endpoint in &typed.endpoint_handlers {
            let mut blockers = Vec::new();
            let mut proposals = Vec::new();
            let return_supported = match &endpoint.response_type {
                tir::IrType::String => true,
                tir::IrType::Class(name) | tir::IrType::Struct(name) => {
                    types.get(name.as_str()).is_some_and(|ty| {
                        !matches!(ty.kind, TypeKind::Interface)
                            && ty.fields.iter().all(|field| {
                                matches!(
                                    field.ty,
                                    tir::IrType::Bool
                                        | tir::IrType::Byte
                                        | tir::IrType::Short
                                        | tir::IrType::Int
                                        | tir::IrType::Long
                                        | tir::IrType::UInt
                                        | tir::IrType::Double
                                        | tir::IrType::Decimal
                                        | tir::IrType::String
                                )
                            })
                    })
                }
                _ => false,
            };
            if !return_supported {
                blockers.push(format!(
                    "return type {:?} with response {:?}",
                    endpoint.return_type, endpoint.response_type
                ));
                proposals.push(
                    "add Task<T>/ActionResult<T> LLVM unwrapping and JSON serialization, or temporarily return `string` containing the serialized response"
                        .to_string(),
                );
            }
            for param in &endpoint.params {
                let supported = match (&param.source, &param.ty) {
                    (
                        tir::EndpointParameterSource::Route,
                        tir::IrType::Int | tir::IrType::Long | tir::IrType::String,
                    )
                    | (
                        tir::EndpointParameterSource::Query,
                        tir::IrType::Bool
                        | tir::IrType::Int
                        | tir::IrType::Long
                        | tir::IrType::String,
                    )
                    | (tir::EndpointParameterSource::Body, tir::IrType::String) => true,
                    (
                        tir::EndpointParameterSource::Body,
                        tir::IrType::Class(name) | tir::IrType::Struct(name),
                    ) => types.get(name.as_str()).is_some_and(|ty| {
                        !matches!(ty.kind, TypeKind::Interface)
                            && (ty.constructors.is_empty()
                                || ty
                                    .constructors
                                    .iter()
                                    .any(|constructor| constructor.params.len() == 1))
                            && ty.fields.iter().all(|field| {
                                matches!(
                                    field.ty,
                                    tir::IrType::Bool
                                        | tir::IrType::Byte
                                        | tir::IrType::Short
                                        | tir::IrType::Int
                                        | tir::IrType::Long
                                        | tir::IrType::UInt
                                        | tir::IrType::Double
                                        | tir::IrType::Decimal
                                        | tir::IrType::String
                                )
                            })
                    }),
                    _ => false,
                };
                if !supported {
                    blockers.push(format!(
                        "parameter '{}' uses {:?} {:?}",
                        param.name, param.source, param.ty
                    ));
                    proposals.push(format!(
                        "rewrite `{}` temporarily as `[FromBody] string body` and deserialize it explicitly, or implement a JSON binder for {:?}",
                        param.name, param.ty
                    ));
                }
            }
            for dependency in &endpoint.constructor_params {
                let supported = diagnostic_dependency_supported(
                    dependency,
                    &types,
                    &self.interface_implementations,
                    &mut HashSet::new(),
                );
                if !supported {
                    blockers.push(format!("constructor dependency {dependency:?}"));
                    proposals.push(format!(
                        "register a concrete owned implementation for {dependency:?} and add interface vtable/drop metadata; a temporary source rewrite can inject the concrete class directly"
                    ));
                }
            }
            if blockers.is_empty() {
                continue;
            }
            let route = format!("{} {}", endpoint.http_method, endpoint.path);
            self.emit(
                "GL3101",
                endpoint
                    .controller
                    .as_deref()
                    .unwrap_or(endpoint.function.as_str()),
                format!(
                    "endpoint {route} is discoverable but currently returns 501: {}",
                    blockers.join(", ")
                ),
                proposals.join("; "),
            );
        }
    }

    fn detect_native_blocks(&mut self) {
        for (index, line) in self.source.lines().enumerate() {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("native \"") {
                continue;
            }
            let col = line.len() - trimmed.len() + 1;
            let key = format!("GL2004:{}:{col}", index + 1);
            if self.emitted.insert(key) {
                self.diagnostics.push(render_diagnostic(
                    "GL2004",
                    index + 1,
                    col,
                    line,
                    "native block allocations cannot be proven leak-free",
                    "rewrite allocation calls to `glitch_calloc`, `glitch_realloc`, and `glitch_free`, then declare borrowed/owned transfer behavior in the package wrapper",
                ));
            }
        }
    }

    fn visit_stmts(&mut self, stmts: &[tir::TypedStmt], emit_llvm: bool) {
        for stmt in stmts {
            match &stmt.kind {
                tir::TypedStmtKind::Let { expr, .. }
                | tir::TypedStmtKind::Assign { expr, .. }
                | tir::TypedStmtKind::Print(expr)
                | tir::TypedStmtKind::Expr(expr)
                | tir::TypedStmtKind::Throw(expr) => self.visit_expr(expr, emit_llvm),
                tir::TypedStmtKind::AssignTarget { target, expr } => {
                    self.visit_expr(target, emit_llvm);
                    self.visit_expr(expr, emit_llvm);
                }
                tir::TypedStmtKind::Block(body) | tir::TypedStmtKind::While { body, .. } => {
                    self.visit_stmts(body, emit_llvm)
                }
                tir::TypedStmtKind::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    self.visit_expr(condition, emit_llvm);
                    self.visit_stmts(then_body, emit_llvm);
                    self.visit_stmts(else_body, emit_llvm);
                }
                tir::TypedStmtKind::Try {
                    try_body,
                    catch_body,
                    finally_body,
                    ..
                } => {
                    self.visit_stmts(try_body, emit_llvm);
                    self.visit_stmts(catch_body, emit_llvm);
                    self.visit_stmts(finally_body, emit_llvm);
                }
                tir::TypedStmtKind::Switch {
                    expr,
                    cases,
                    default,
                } => {
                    self.visit_expr(expr, emit_llvm);
                    for case in cases {
                        self.visit_expr(&case.value, emit_llvm);
                        self.visit_stmts(&case.body, emit_llvm);
                    }
                    self.visit_stmts(default, emit_llvm);
                }
                tir::TypedStmtKind::For {
                    init,
                    condition,
                    increment,
                    body,
                } => {
                    if let Some(init) = init {
                        self.visit_stmts(std::slice::from_ref(init), emit_llvm);
                    }
                    if let Some(condition) = condition {
                        self.visit_expr(condition, emit_llvm);
                    }
                    self.visit_stmts(body, emit_llvm);
                    if let Some(increment) = increment {
                        self.visit_stmts(std::slice::from_ref(increment), emit_llvm);
                    }
                }
                tir::TypedStmtKind::ForEach {
                    collection, body, ..
                } => {
                    self.visit_expr(collection, emit_llvm);
                    self.visit_stmts(body, emit_llvm);
                }
                tir::TypedStmtKind::Return(Some(expr)) => self.visit_expr(expr, emit_llvm),
                tir::TypedStmtKind::Return(None)
                | tir::TypedStmtKind::Break
                | tir::TypedStmtKind::Continue => {}
            }
        }
    }

    fn visit_expr(&mut self, expr: &tir::TypedExpr, emit_llvm: bool) {
        match &expr.kind {
            tir::TypedExprKind::Call(call) => {
                match &call.kind {
                    tir::TypedCallKind::Function { name, symbol } => {
                        if name == "sizeof" {
                            for arg in &call.args {
                                self.visit_expr(arg, emit_llvm);
                            }
                            return;
                        }
                        if !self.symbols.contains(symbol) && !is_llvm_runtime_function(symbol) {
                            self.emit(
                                "GL3002",
                                &format!("{name}("),
                                format!("function '{name}' has no linked GL or LLVM implementation"),
                                format!(
                                    "add a package function with the same signature, for example `{}`",
                                    framework_stub(name, &expr.ty)
                                ),
                            );
                        }
                    }
                    tir::TypedCallKind::Method {
                        target,
                        name,
                        symbol,
                        resolution,
                    } => {
                        if matches!(resolution, tir::CallResolution::Unknown)
                            || (matches!(resolution, tir::CallResolution::InstanceMethod)
                                && !self.symbols.contains(symbol))
                        {
                            if let tir::IrType::Interface(interface_name) = &target.ty {
                                if self
                                    .interface_implementations
                                    .get(interface_name)
                                    .is_some_and(|implementation| {
                                        let prefix = format!(
                                            "{}_{}",
                                            sanitize_ident(implementation),
                                            sanitize_ident(name)
                                        );
                                        self.symbols.iter().any(|symbol| {
                                            symbol == &prefix
                                                || symbol.starts_with(&format!("{prefix}__"))
                                        })
                                })
                                {
                                    self.visit_expr(target, emit_llvm);
                                    for arg in &call.args {
                                        self.visit_expr(arg, emit_llvm);
                                    }
                                    return;
                                }
                            }
                            self.emit(
                                "GL3001",
                                &format!(".{name}"),
                                format!(
                                    "member '{name}' on {:?} has no linked implementation; LLVM emits a typed default",
                                    target.ty
                                ),
                                format!(
                                    "implement this member in a `.gl` package, for example `{}`",
                                    framework_stub(name, &expr.ty)
                                ),
                            );
                        }
                        self.visit_expr(target, emit_llvm);
                    }
                }
                for arg in &call.args {
                    self.visit_expr(arg, emit_llvm);
                }
            }
            tir::TypedExprKind::Field { target, name } => {
                if matches!(target.ty, tir::IrType::Unknown(_)) {
                    self.emit(
                        "GL3003",
                        &format!(".{name}"),
                        format!(
                            "static or opaque member '{name}' on {:?} has no linked implementation",
                            target.ty
                        ),
                        format!(
                            "define the containing framework type and `{name}` member in a `.gl` package; until then LLVM uses {}",
                            typed_default_description(&expr.ty)
                        ),
                    );
                }
                self.visit_expr(target, emit_llvm);
            }
            tir::TypedExprKind::IsPattern { expr, .. } => self.visit_expr(expr, emit_llvm),
            tir::TypedExprKind::NewObject {
                type_name,
                args,
                fields,
                ..
            } => {
                if type_name == "Rc_int" || type_name.starts_with("Rc_") {
                    for arg in args {
                        self.visit_expr(arg, emit_llvm);
                    }
                    for field in fields {
                        self.visit_expr(&field.expr, emit_llvm);
                    }
                    return;
                }
                if type_name != "Exception"
                    && type_name != "System.Exception"
                    && !self.known_types.contains(type_name)
                {
                    self.emit(
                        "GL3004",
                        &format!("new {type_name}"),
                        format!("type '{type_name}' has no linked LLVM layout"),
                        format!(
                            "add `class {type_name} {{ ... }}` to the corresponding `.gl` package; until then construction produces an opaque null handle"
                        ),
                    );
                }
                for arg in args {
                    self.visit_expr(arg, emit_llvm);
                }
                for field in fields {
                    self.visit_expr(&field.expr, emit_llvm);
                }
            }
            tir::TypedExprKind::ArrayLiteral(values) => {
                for value in values {
                    self.visit_expr(value, emit_llvm);
                }
            }
            tir::TypedExprKind::NewArray { values, .. } => {
                for value in values {
                    self.visit_expr(value, emit_llvm);
                }
            }
            tir::TypedExprKind::Index { target, index } => {
                self.visit_expr(target, emit_llvm);
                self.visit_expr(index, emit_llvm);
            }
            tir::TypedExprKind::Await(inner) => {
                if !matches!(inner.ty, tir::IrType::Task(_)) {
                    self.emit(
                        "GL3006",
                        "await ",
                        format!(
                            "awaited expression has unresolved task type {:?}; compatibility mode evaluates it synchronously",
                            inner.ty
                        ),
                            "implement the called method with a `Task<T>` return type in a `.gl` package to enable owned async state-machine lowering".to_string(),
                    );
                }
                self.visit_expr(inner, emit_llvm);
            }
            tir::TypedExprKind::Unary { expr: inner, .. } => self.visit_expr(inner, emit_llvm),
            tir::TypedExprKind::Lambda { body, .. } => {
                if !emit_llvm {
                    self.emit(
                        "GL3005",
                        "=>",
                        "lambda has no executable LLVM closure or expression-tree lowering; compatibility mode emits an opaque delegate".to_string(),
                        "for framework configuration, add the receiving API to a `.gl` package; for executable code, rewrite the lambda as a named function until closure lowering is available".to_string(),
                    );
                }
                self.visit_expr(body, emit_llvm);
            }
            tir::TypedExprKind::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.visit_expr(condition, emit_llvm);
                self.visit_expr(when_true, emit_llvm);
                self.visit_expr(when_false, emit_llvm);
            }
            tir::TypedExprKind::Binary { left, right, .. } => {
                self.visit_expr(left, emit_llvm);
                self.visit_expr(right, emit_llvm);
            }
            _ => {}
        }
    }

    fn emit(&mut self, code: &str, needle: &str, message: String, help: String) {
        let key = format!("{code}:{message}");
        if !self.emitted.insert(key) {
            return;
        }
        let (line, col, snippet) = locate_source(self.source, needle);
        self.diagnostics
            .push(render_diagnostic(code, line, col, snippet, &message, &help));
    }
}

fn diagnostic_dependency_supported(
    dependency: &tir::IrType,
    types: &HashMap<&str, &tir::TypedType>,
    interface_implementations: &HashMap<String, String>,
    visiting: &mut HashSet<String>,
) -> bool {
    let type_name = match dependency {
        tir::IrType::Class(name) | tir::IrType::Struct(name) => name.clone(),
        tir::IrType::Interface(name) => {
            let Some(implementation) = interface_implementations.get(name) else {
                return false;
            };
            implementation.clone()
        }
        _ => return false,
    };
    if !visiting.insert(type_name.clone()) {
        return false;
    }
    let Some(ty) = types.get(type_name.as_str()) else {
        visiting.remove(&type_name);
        return false;
    };
    if matches!(ty.kind, TypeKind::Interface) {
        visiting.remove(&type_name);
        return false;
    }
    let supported = ty.constructors.first().is_none_or(|constructor| {
        constructor.params.iter().skip(1).all(|param| {
            diagnostic_dependency_supported(&param.ty, types, interface_implementations, visiting)
        })
    });
    visiting.remove(&type_name);
    supported
}

fn base_type_name(name: &str) -> &str {
    name.split('<')
        .next()
        .unwrap_or(name)
        .rsplit('.')
        .next()
        .unwrap_or(name)
}

fn is_llvm_runtime_function(symbol: &str) -> bool {
    matches!(
        symbol,
        "GlitchRestHost_Run"
            | "GlitchMiddlewareHandlers_Apply"
            | "glitch_register_attribute_routes"
            | "GlitchEndpointHandlers_Contains"
            | "GlitchEndpointHandlers_Invoke"
            | "Ok"
            | "Created"
            | "CreatedAtAction"
            | "Accepted"
            | "NoContent"
            | "NotFound"
            | "BadRequest"
    )
}

pub(crate) fn locate_source<'a>(source: &'a str, needle: &str) -> (usize, usize, &'a str) {
    for (index, line) in source.lines().enumerate() {
        if let Some(col) = line.find(needle) {
            return (index + 1, col + 1, line);
        }
    }
    (1, 1, source.lines().next().unwrap_or(""))
}

pub(crate) fn render_diagnostic(
    code: &str,
    line: usize,
    col: usize,
    snippet: &str,
    message: &str,
    help: &str,
) -> String {
    format!(
        "warning {code} at {line}:{col}: {message}\n  {line} | {}\n  help: {help}",
        snippet.trim_end()
    )
}

pub(crate) fn referenced_class<'a>(
    ty: &'a TypeSyntax,
    classes: &HashSet<String>,
) -> Option<&'a String> {
    match ty {
        TypeSyntax::Named(name) if classes.contains(name) => Some(name),
        TypeSyntax::Nullable(inner) => referenced_class(inner, classes),
        _ => None,
    }
}

pub(crate) fn has_type_path(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut HashSet<String>,
) -> bool {
    if current == target {
        return true;
    }
    if !visited.insert(current.to_string()) {
        return false;
    }
    graph.get(current).is_some_and(|next| {
        next.iter()
            .any(|name| has_type_path(graph, name, target, visited))
    })
}

#[allow(dead_code)]
pub(crate) fn type_syntax_display(ty: &TypeSyntax) -> String {
    match ty {
        TypeSyntax::Named(name) => name.clone(),
        TypeSyntax::GenericNamed { name, args } => {
            let arg_strs = args
                .iter()
                .map(type_syntax_display)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}<{}>", name, arg_strs)
        }
        TypeSyntax::Nullable(inner) => format!("{}?", type_syntax_display(inner)),
        TypeSyntax::List(inner) => format!("List<{}>", type_syntax_display(inner)),
        TypeSyntax::Dictionary(key, value) => format!(
            "Dictionary<{}, {}>",
            type_syntax_display(key),
            type_syntax_display(value)
        ),
        TypeSyntax::Task(inner) => format!("Task<{}>", type_syntax_display(inner)),
        TypeSyntax::IEnumerable(inner) => format!("IEnumerable<{}>", type_syntax_display(inner)),
        other => format!("{other:?}"),
    }
}

fn framework_stub(name: &str, return_type: &tir::IrType) -> String {
    format!(
        "{} {name}(/* parameters */) {{ return {}; }}",
        ir_type_display(return_type),
        typed_default_description(return_type)
    )
}

fn ir_type_display(ty: &tir::IrType) -> String {
    match ty {
        tir::IrType::Void => "void".to_string(),
        tir::IrType::Bool => "bool".to_string(),
        tir::IrType::Int => "int".to_string(),
        tir::IrType::Long => "long".to_string(),
        tir::IrType::Double => "double".to_string(),
        tir::IrType::String => "string".to_string(),
        tir::IrType::Class(name)
        | tir::IrType::Struct(name)
        | tir::IrType::Interface(name)
        | tir::IrType::Unknown(name) => name.clone(),
        other => format!("{other:?}"),
    }
}

fn typed_default_description(ty: &tir::IrType) -> &'static str {
    match ty {
        tir::IrType::Void => "no value",
        tir::IrType::Bool => "false",
        tir::IrType::Byte
        | tir::IrType::Short
        | tir::IrType::Int
        | tir::IrType::Long
        | tir::IrType::UInt
        | tir::IrType::Double
        | tir::IrType::Decimal => "0",
        tir::IrType::String => "\"\"",
        _ => "null",
    }
}
