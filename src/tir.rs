#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::*;
#[path = "tir/generics.rs"]
mod generics;
#[path = "tir/startup.rs"]
mod startup;
#[path = "tir/stmt_lowering.rs"]
mod stmt_lowering;
#[path = "tir/summary.rs"]
mod summary;
#[path = "tir/endpoints.rs"]
mod endpoints;

use self::endpoints::*;
use self::generics::*;
use self::startup::*;
use self::stmt_lowering::*;
use self::summary::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ownership {
    Copy,
    Owned,
    Borrowed,
    Shared,
    View,
    Moved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DropKind {
    None,
    Free,
    DropStruct,
    DropClass,
    DropDelegate,
    DropCollection,
    DropTask,
    DropException,
    ViewOnly,
    BorrowOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum IrType {
    Bool,
    Byte,
    Short,
    Int,
    Long,
    UInt,
    Double,
    Decimal,
    String,
    Void,
    Array(Box<IrType>),
    Ref(Box<IrType>),
    Struct(String),
    Class(String),
    Interface(String),
    List(Box<IrType>),
    Dictionary(Box<IrType>, Box<IrType>),
    Enumerable(Box<IrType>),
    Thread,
    Task(Box<IrType>),
    Nullable(Box<IrType>),
    Function {
        params: Vec<IrType>,
        return_type: Box<IrType>,
    },
    Exception,
    Weak(Box<IrType>),
    Unknown(String),
}

#[derive(Debug, Clone)]
pub(crate) struct TypedProgram {
    pub(crate) functions: Vec<TypedFunction>,
    pub(crate) types: Vec<TypedType>,
    pub(crate) generic_instantiations: Vec<GenericInstantiation>,
    pub(crate) endpoint_handlers: Vec<EndpointHandlerBinding>,
    pub(crate) startup: Option<TypedStartup>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedStartup {
    pub(crate) symbol: String,
    pub(crate) accepts_args: bool,
    pub(crate) returns_int: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedType {
    pub(crate) package_id: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) name: String,
    pub(crate) namespace: Vec<String>,
    pub(crate) generic_params: Vec<String>,
    pub(crate) symbol_id: usize,
    pub(crate) is_extension: bool,
    pub(crate) kind: TypeKind,
    pub(crate) bases: Vec<String>,
    pub(crate) fields: Vec<TypedBinding>,
    pub(crate) constructors: Vec<TypedFunction>,
    pub(crate) methods: Vec<TypedFunction>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedFunction {
    pub(crate) package_id: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) name: String,
    pub(crate) symbol: String,
    pub(crate) is_async: bool,
    pub(crate) generic_params: Vec<String>,
    pub(crate) is_extern: bool,
    pub(crate) required_params: usize,
    pub(crate) return_type: IrType,
    pub(crate) return_ownership: Ownership,
    pub(crate) params: Vec<TypedBinding>,
    pub(crate) locals: Vec<TypedBinding>,
    pub(crate) body: Vec<TypedStmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedBinding {
    pub(crate) name: String,
    pub(crate) ty: IrType,
    pub(crate) ownership: Ownership,
}

impl TypedBinding {
    pub(crate) fn drop_kind(&self) -> DropKind {
        drop_kind_for_type(&self.ty, &self.ownership)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TypedStmt {
    pub(crate) kind: TypedStmtKind,
}

#[derive(Debug, Clone)]
pub(crate) enum TypedStmtKind {
    Let {
        binding: TypedBinding,
        expr: TypedExpr,
    },
    Assign {
        name: String,
        expr: TypedExpr,
    },
    AssignTarget {
        target: TypedExpr,
        expr: TypedExpr,
    },
    Block(Vec<TypedStmt>),
    If {
        condition: TypedExpr,
        then_body: Vec<TypedStmt>,
        else_body: Vec<TypedStmt>,
    },
    Try {
        try_body: Vec<TypedStmt>,
        catch_name: Option<String>,
        catch_body: Vec<TypedStmt>,
        finally_body: Vec<TypedStmt>,
    },
    Switch {
        expr: TypedExpr,
        cases: Vec<TypedSwitchCase>,
        default: Vec<TypedStmt>,
    },
    While {
        condition: TypedExpr,
        body: Vec<TypedStmt>,
    },
    For {
        init: Option<Box<TypedStmt>>,
        condition: Option<TypedExpr>,
        increment: Option<Box<TypedStmt>>,
        body: Vec<TypedStmt>,
    },
    ForEach {
        item: TypedBinding,
        collection: TypedExpr,
        body: Vec<TypedStmt>,
    },
    Print(TypedExpr),
    Expr(TypedExpr),
    Return(Option<TypedExpr>),
    Throw(TypedExpr),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedSwitchCase {
    pub(crate) value: TypedExpr,
    pub(crate) body: Vec<TypedStmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedExpr {
    pub(crate) kind: TypedExprKind,
    pub(crate) ty: IrType,
    pub(crate) ownership: Ownership,
    pub(crate) drop_kind: DropKind,
}

#[derive(Debug, Clone)]
pub(crate) enum TypedExprKind {
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
    String(String),
    Var(String),
    FunctionSymbol(String),
    Move(String),
    NullableSome(Box<TypedExpr>),
    ArrayLiteral(Vec<TypedExpr>),
    NewArray {
        element_type: IrType,
        length: Option<Box<TypedExpr>>,
        values: Vec<TypedExpr>,
    },
    Index {
        target: Box<TypedExpr>,
        index: Box<TypedExpr>,
    },
    Field {
        target: Box<TypedExpr>,
        name: String,
    },
    IsPattern {
        expr: Box<TypedExpr>,
        binding: Option<TypedBinding>,
    },
    Throw(Box<TypedExpr>),
    Assign {
        target: Box<TypedExpr>,
        value: Box<TypedExpr>,
    },
    Call(TypedCall),
    NewObject {
        type_name: String,
        constructor: Option<String>,
        args: Vec<TypedExpr>,
        fields: Vec<TypedFieldInit>,
    },
    NewCollection(IrType),
    NewThread(String),
    Borrow {
        mutable: bool,
        name: String,
    },
    Await(Box<TypedExpr>),
    Lambda {
        params: Vec<String>,
        body: Box<TypedExpr>,
    },
    Conditional {
        condition: Box<TypedExpr>,
        when_true: Box<TypedExpr>,
        when_false: Box<TypedExpr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<TypedExpr>,
    },
    IncDec {
        target: Box<TypedExpr>,
        delta: i32,
        prefix: bool,
    },
    Binary {
        left: Box<TypedExpr>,
        op: BinaryOp,
        right: Box<TypedExpr>,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct TypedFieldInit {
    pub(crate) name: String,
    pub(crate) expr: TypedExpr,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedCall {
    pub(crate) kind: TypedCallKind,
    pub(crate) args: Vec<TypedExpr>,
    pub(crate) generic_args: Vec<IrType>,
}

#[derive(Debug, Clone)]
pub(crate) enum TypedCallKind {
    Function {
        name: String,
        symbol: String,
    },
    Method {
        target: Box<TypedExpr>,
        name: String,
        symbol: String,
        resolution: CallResolution,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallResolution {
    StaticFunction,
    InstanceMethod,
    CollectionMethod,
    TaskMethod,
    WeakMethod,
    EndpointHandlerRegistration {
        http_method: String,
        path: String,
        handler: String,
    },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenericInstantiation {
    pub(crate) name: String,
    pub(crate) args: Vec<IrType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EndpointHandlerBinding {
    pub(crate) http_method: String,
    pub(crate) path: String,
    pub(crate) function: String,
    pub(crate) return_type: IrType,
    pub(crate) response_type: IrType,
    pub(crate) ownership: Ownership,
    pub(crate) controller: Option<String>,
    pub(crate) constructor: Option<String>,
    pub(crate) constructor_params: Vec<IrType>,
    pub(crate) params: Vec<EndpointParameterBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EndpointParameterSource {
    Route,
    Query,
    Body,
    Form,
    Header,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EndpointParameterBinding {
    pub(crate) name: String,
    pub(crate) ty: IrType,
    pub(crate) source: EndpointParameterSource,
}

#[derive(Debug, Clone)]
struct FunctionSignature {
    package_id: Option<String>,
    visibility: Visibility,
    generic_params: Vec<String>,
    params: Vec<IrType>,
    param_ownerships: Vec<Ownership>,
    required_params: usize,
    params_element_type: Option<IrType>,
    return_type: IrType,
    return_ownership: Ownership,
    symbol: String,
    is_static: bool,
}

#[derive(Debug, Clone)]
struct FieldSignature {
    package_id: Option<String>,
    visibility: Visibility,
    ty: IrType,
    ownership: Ownership,
}

#[derive(Debug, Clone)]
struct DelegateSignature {
    full_name: String,
    generic_params: Vec<String>,
    params: Vec<TypeSyntax>,
    return_type: TypeSyntax,
}

#[derive(Default)]
struct TypeEnv {
    kinds: HashMap<String, TypeKind>,
    type_packages: HashMap<String, Option<String>>,
    type_visibilities: HashMap<String, Visibility>,
    bases: HashMap<String, Vec<String>>,
    type_generic_params: HashMap<String, Vec<String>>,
    static_fields: HashMap<(String, String), Expr>,
    enum_values: HashMap<(String, String), i64>,
    delegates: HashMap<String, DelegateSignature>,
    functions: HashMap<String, Vec<FunctionSignature>>,
    methods: HashMap<(String, String), Vec<FunctionSignature>>,
    extension_methods: HashMap<(String, String), Vec<FunctionSignature>>,
    constructors: HashMap<String, Vec<FunctionSignature>>,
    fields: HashMap<(String, String), FieldSignature>,
    field_order: HashMap<String, Vec<(String, FieldSignature)>>,
}

impl TypedProgram {
    pub(crate) fn lower(program: &Program, startup_object: Option<&str>) -> Result<Self, String> {
        let mut env = TypeEnv::default();
        for enum_def in &program.enums {
            env.kinds.insert(enum_def.name.clone(), TypeKind::Enum);
            env.type_packages
                .insert(enum_def.name.clone(), enum_def.package_id.clone());
            env.type_visibilities
                .insert(enum_def.name.clone(), enum_def.visibility);
            for (index, variant) in enum_def.variants.iter().enumerate() {
                env.enum_values.insert(
                    (enum_def.name.clone(), variant.name.clone()),
                    variant.value.unwrap_or(index as i64),
                );
            }
        }
        for ty in &program.types {
            env.kinds.insert(ty.name.clone(), ty.kind);
            env.type_packages
                .insert(ty.name.clone(), ty.package_id.clone());
            env.type_visibilities
                .insert(ty.name.clone(), ty.visibility);
            env.bases.insert(ty.name.clone(), ty.bases.clone());
            env.type_generic_params.insert(
                ty.name.clone(),
                ty.generic_params
                    .iter()
                    .map(|param| param.name.clone())
                    .collect(),
            );
        }
        populate_delegate_signatures(program, &mut env);
        populate_function_signatures(program, &mut env);
        populate_method_signatures(program, &mut env);
        populate_constructor_signatures(program, &mut env);
        for ty in &program.types {
            for field in &ty.fields {
                if field.is_static {
                    if let Some(initializer) = &field.initializer {
                        env.static_fields
                            .insert((ty.name.clone(), field.name.clone()), initializer.clone());
                    }
                    continue;
                }
                let field_type = type_syntax_to_ir(&field.ty, &env);
                let field_signature = FieldSignature {
                    package_id: ty.package_id.clone(),
                    visibility: field.visibility,
                    ty: field_type.clone(),
                    ownership: ownership_for_declared_type_syntax(&field.ty, &env),
                };
                env.fields
                    .insert((ty.name.clone(), field.name.clone()), field_signature.clone());
                env.field_order
                    .entry(ty.name.clone())
                    .or_default()
                    .push((field.name.clone(), field_signature));
            }
        }

        let types = program
            .types
            .iter()
            .enumerate()
            .map(|(index, ty)| lower_type(ty, index, &env))
            .collect::<Result<Vec<_>, _>>()?;
        let functions = program
            .functions
            .iter()
            .map(|function| lower_function(function, &env, &[], None, None))
            .collect::<Result<Vec<_>, _>>()?;

        let mut generic_instantiations = Vec::new();
        for function in &functions {
            collect_instantiation(&function.return_type, &mut generic_instantiations);
            for param in &function.params {
                collect_instantiation(&param.ty, &mut generic_instantiations);
            }
            for local in &function.locals {
                collect_instantiation(&local.ty, &mut generic_instantiations);
            }
        }
        for ty in &types {
            for field in &ty.fields {
                collect_instantiation(&field.ty, &mut generic_instantiations);
            }
            for constructor in &ty.constructors {
                for param in &constructor.params {
                    collect_instantiation(&param.ty, &mut generic_instantiations);
                }
            }
            for method in &ty.methods {
                collect_instantiation(&method.return_type, &mut generic_instantiations);
            }
        }
        collect_generic_call_instantiations(
            &functions,
            &types,
            &mut generic_instantiations,
        );
        let endpoint_handlers = collect_endpoint_handlers(program, &env)?;

        let startup = resolve_typed_startup(program, &types, &functions, startup_object)?;
        let typed = Self {
            functions,
            types,
            generic_instantiations,
            endpoint_handlers,
            startup,
        };
        typed.validate_visibility(&env)?;
        typed.validate_async_lowering()?;
        Ok(typed)
    }

    pub(crate) fn check_ownership(program: &Program) -> Result<(), String> {
        let env = TypeEnv::from_program(program);
        for function in &program.functions {
            OwnershipChecker::check_function(function, &env, Vec::new())?;
        }
        for ty in &program.types {
            for constructor in &ty.constructors {
                let mut params = vec![TypedBinding {
                    name: "this".to_string(),
                    ty: collection_this_type(ty),
                    ownership: Ownership::Borrowed,
                }];
                for param in &constructor.params {
                    let ty = type_syntax_to_ir(&param.ty, &env);
                    params.push(TypedBinding {
                        name: param.name.clone(),
                        ty,
                        ownership: ownership_for_declared_type_syntax(&param.ty, &env),
                    });
                }
                OwnershipChecker::check_body(
                    &format!("constructor {}", ty.name),
                    &constructor.body,
                    &env,
                    params,
                    &IrType::Void,
                    Ownership::Copy,
                )?;
            }
            for method in &ty.methods {
                OwnershipChecker::check_function(
                    method,
                    &env,
                    vec![TypedBinding {
                        name: "this".to_string(),
                        ty: collection_this_type(ty),
                        ownership: Ownership::Borrowed,
                    }],
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn ownership_summary(&self) -> String {
        let mut out = String::new();
        for function in &self.functions {
            out.push_str(&format!(
                "function {} returns {:?} {:?}\n",
                function.name, function.return_ownership, function.return_type
            ));
            if !function.generic_params.is_empty() {
                out.push_str(&format!(
                    "  generic params {:?}\n",
                    function.generic_params
                ));
            }
            for param in &function.params {
                out.push_str(&format!(
                    "  param {}: {:?} {:?} drop={:?}\n",
                    param.name,
                    param.ownership,
                    param.ty,
                    param.drop_kind()
                ));
            }
            for local in &function.locals {
                out.push_str(&format!(
                    "  local {}: {:?} {:?} drop={:?}\n",
                    local.name,
                    local.ownership,
                    local.ty,
                    local.drop_kind()
                ));
            }
            out.push_str(&format!("  typed body stmts={}\n", function.body.len()));
            summarize_typed_stmts(&function.body, "  ", &mut out);
        }
        for ty in &self.types {
            out.push_str(&format!("type {} {:?}\n", ty.name, ty.kind));
            for field in &ty.fields {
                out.push_str(&format!(
                    "  field {}: {:?} {:?} drop={:?}\n",
                    field.name,
                    field.ownership,
                    field.ty,
                    field.drop_kind()
                ));
            }
            for constructor in &ty.constructors {
                out.push_str(&format!(
                    "  constructor {} params={}\n",
                    constructor.symbol,
                    constructor.params.len().saturating_sub(1)
                ));
            }
            for method in &ty.methods {
                out.push_str(&format!(
                    "  method {} returns {:?} {:?}\n",
                    method.name, method.return_ownership, method.return_type
                ));
            }
        }
        for instantiation in &self.generic_instantiations {
            out.push_str(&format!(
                "generic {}<{:?}>\n",
                instantiation.name, instantiation.args
            ));
        }
        for endpoint in &self.endpoint_handlers {
            out.push_str(&format!(
                "endpoint {} {} -> {} returns {:?} {:?}\n",
                endpoint.http_method,
                endpoint.path,
                endpoint.function,
                endpoint.ownership,
                endpoint.return_type
            ));
        }
        out
    }

    fn validate_visibility(&self, env: &TypeEnv) -> Result<(), String> {
        for function in &self.functions {
            validate_typed_function_visibility(function, env)?;
        }
        for ty in &self.types {
            for field in &ty.fields {
                ensure_type_accessible(
                    env,
                    &field.ty,
                    ty.package_id.as_deref(),
                    &format!("type '{}.{}'", ty.name, field.name),
                )?;
            }
            for base in &ty.bases {
                if let Some((package_id, visibility)) =
                    env.lookup_type_visibility(base, ty.package_id.as_deref())
                {
                    if !visibility_allows_access(
                        visibility,
                        package_id.as_deref(),
                        ty.package_id.as_deref(),
                    ) {
                        return Err(format!(
                            "type '{}' is not public in package '{}' and cannot be used from '{}'",
                            base,
                            package_id.as_deref().unwrap_or("<root>"),
                            ty.package_id.as_deref().unwrap_or("<root>")
                        ));
                    }
                }
            }
            for constructor in &ty.constructors {
                validate_typed_function_visibility(constructor, env)?;
            }
            for method in &ty.methods {
                validate_typed_function_visibility(method, env)?;
            }
        }
        Ok(())
    }

    pub(crate) fn validate_async_lowering(&self) -> Result<(), String> {
        for function in &self.functions {
            validate_async_function(function)?;
        }
        for ty in &self.types {
            for constructor in &ty.constructors {
                validate_async_function(constructor)?;
            }
            for method in &ty.methods {
                validate_async_function(method)?;
            }
        }
        Ok(())
    }
}

fn validate_typed_function_visibility(
    function: &TypedFunction,
    env: &TypeEnv,
) -> Result<(), String> {
    let caller_package = function.package_id.as_deref();
    if caller_package.is_some() {
        return Ok(());
    }
    ensure_type_accessible(
        env,
        &function.return_type,
        caller_package,
        &format!("return type of '{}'", function.name),
    )?;
    for param in &function.params {
        ensure_type_accessible(
            env,
            &param.ty,
            caller_package,
            &format!("parameter '{}' of '{}'", param.name, function.name),
        )?;
    }
    for local in &function.locals {
        ensure_type_accessible(
            env,
            &local.ty,
            caller_package,
            &format!("local '{}' of '{}'", local.name, function.name),
        )?;
    }
    for stmt in &function.body {
        validate_typed_stmt_visibility(stmt, env, caller_package, &function.name)?;
    }
    Ok(())
}

fn validate_typed_stmt_visibility(
    stmt: &TypedStmt,
    env: &TypeEnv,
    caller_package: Option<&str>,
    function_name: &str,
) -> Result<(), String> {
    match &stmt.kind {
        TypedStmtKind::Let { binding, expr } => {
            ensure_type_accessible(
                env,
                &binding.ty,
                caller_package,
                &format!("binding '{}' in '{}'", binding.name, function_name),
            )?;
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Throw(expr) => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedStmtKind::AssignTarget { target, expr } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedStmtKind::Block(body) => {
            for stmt in body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::If { condition, then_body, else_body } => {
            validate_typed_expr_visibility(condition, env, caller_package, function_name)?;
            for stmt in then_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
            for stmt in else_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::Try { try_body, catch_body, finally_body, .. } => {
            for stmt in try_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
            for stmt in catch_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
            for stmt in finally_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::Switch { expr, cases, default } => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
            for case in cases {
                validate_typed_expr_visibility(&case.value, env, caller_package, function_name)?;
                for stmt in &case.body {
                    validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
                }
            }
            for stmt in default {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::While { condition, body } => {
            validate_typed_expr_visibility(condition, env, caller_package, function_name)?;
            for stmt in body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::For { init, condition, increment, body } => {
            if let Some(init) = init {
                validate_typed_stmt_visibility(init, env, caller_package, function_name)?;
            }
            if let Some(condition) = condition {
                validate_typed_expr_visibility(condition, env, caller_package, function_name)?;
            }
            if let Some(increment) = increment {
                validate_typed_stmt_visibility(increment, env, caller_package, function_name)?;
            }
            for stmt in body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::ForEach { item, collection, body } => {
            ensure_type_accessible(
                env,
                &item.ty,
                caller_package,
                &format!("foreach item '{}' in '{}'", item.name, function_name),
            )?;
            validate_typed_expr_visibility(collection, env, caller_package, function_name)?;
            for stmt in body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::Return(Some(expr)) => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
    }
    Ok(())
}

fn validate_typed_expr_visibility(
    expr: &TypedExpr,
    env: &TypeEnv,
    caller_package: Option<&str>,
    function_name: &str,
) -> Result<(), String> {
    ensure_type_accessible(env, &expr.ty, caller_package, &format!("expression in '{}'", function_name))?;
    match &expr.kind {
        TypedExprKind::Field { target, name } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
            if let Some(type_name) = object_type_name(&target.ty) {
                if let Some(field) = env.resolve_field(type_name, name) {
                    if !visibility_allows_access(field.visibility, field.package_id.as_deref(), caller_package) {
                        return Err(format!(
                            "field '{}.{}' is not public in package '{}' and cannot be accessed from '{}'",
                            type_name,
                            name,
                            field.package_id.as_deref().unwrap_or("<root>"),
                            caller_package.unwrap_or("<root>")
                        ));
                    }
                }
            }
        }
        TypedExprKind::Call(call) => {
            if let TypedCallKind::Method { target, symbol, .. } = &call.kind {
                validate_typed_expr_visibility(target, env, caller_package, function_name)?;
                if let Some(signature) = env.lookup_signature_by_symbol(symbol) {
                    if !visibility_allows_access(signature.visibility, signature.package_id.as_deref(), caller_package) {
                        return Err(format!(
                            "method '{}' is not public in package '{}' and cannot be called from '{}'",
                            symbol,
                            signature.package_id.as_deref().unwrap_or("<root>"),
                            caller_package.unwrap_or("<root>")
                        ));
                    }
                }
            } else if let TypedCallKind::Function { symbol, .. } = &call.kind {
                if let Some(signature) = env.lookup_signature_by_symbol(symbol) {
                    if !visibility_allows_access(signature.visibility, signature.package_id.as_deref(), caller_package) {
                        return Err(format!(
                            "function '{}' is not public in package '{}' and cannot be called from '{}'",
                            symbol,
                            signature.package_id.as_deref().unwrap_or("<root>"),
                            caller_package.unwrap_or("<root>")
                        ));
                    }
                }
            }
            for arg in &call.args {
                validate_typed_expr_visibility(arg, env, caller_package, function_name)?;
            }
        }
        TypedExprKind::NewObject { type_name, constructor, args, fields } => {
            if let Some((package_id, visibility)) =
                env.lookup_type_visibility(type_name, caller_package)
            {
                if !visibility_allows_access(visibility, package_id.as_deref(), caller_package) {
                    return Err(format!(
                        "type '{}' is not public in package '{}' and cannot be constructed from '{}'",
                        type_name,
                        package_id.as_deref().unwrap_or("<root>"),
                        caller_package.unwrap_or("<root>")
                    ));
                }
            }
            if let Some(symbol) = constructor {
                if let Some(signature) = env.lookup_signature_by_symbol(symbol) {
                    if !visibility_allows_access(signature.visibility, signature.package_id.as_deref(), caller_package) {
                        return Err(format!(
                            "constructor '{}' is not public in package '{}' and cannot be called from '{}'",
                            symbol,
                            signature.package_id.as_deref().unwrap_or("<root>"),
                            caller_package.unwrap_or("<root>")
                        ));
                    }
                }
            }
            for arg in args {
                validate_typed_expr_visibility(arg, env, caller_package, function_name)?;
            }
            for field in fields {
                validate_typed_expr_visibility(&field.expr, env, caller_package, function_name)?;
            }
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                validate_typed_expr_visibility(value, env, caller_package, function_name)?;
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                validate_typed_expr_visibility(length, env, caller_package, function_name)?;
            }
            for value in values {
                validate_typed_expr_visibility(value, env, caller_package, function_name)?;
            }
        }
        TypedExprKind::Index { target, index } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
            validate_typed_expr_visibility(index, env, caller_package, function_name)?;
        }
        TypedExprKind::IsPattern { expr, binding } => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
            if let Some(binding) = binding {
                ensure_type_accessible(env, &binding.ty, caller_package, &format!("pattern binding '{}' in '{}'", binding.name, function_name))?;
            }
        }
        TypedExprKind::Throw(expr)
        | TypedExprKind::Await(expr)
        | TypedExprKind::NullableSome(expr)
        | TypedExprKind::Unary { expr, .. } => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedExprKind::Assign { target, value } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
            validate_typed_expr_visibility(value, env, caller_package, function_name)?;
        }
        TypedExprKind::Conditional { condition, when_true, when_false } => {
            validate_typed_expr_visibility(condition, env, caller_package, function_name)?;
            validate_typed_expr_visibility(when_true, env, caller_package, function_name)?;
            validate_typed_expr_visibility(when_false, env, caller_package, function_name)?;
        }
        TypedExprKind::IncDec { target, .. } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
        }
        TypedExprKind::Binary { left, right, .. } => {
            validate_typed_expr_visibility(left, env, caller_package, function_name)?;
            validate_typed_expr_visibility(right, env, caller_package, function_name)?;
        }
        TypedExprKind::Lambda { body, .. } => {
            validate_typed_expr_visibility(body, env, caller_package, function_name)?;
        }
        TypedExprKind::FunctionSymbol(symbol) => {
            if let Some(signature) = env.lookup_signature_by_symbol(symbol) {
                if !visibility_allows_access(signature.visibility, signature.package_id.as_deref(), caller_package) {
                    return Err(format!(
                        "function '{}' is not public in package '{}' and cannot be referenced from '{}'",
                        symbol,
                        signature.package_id.as_deref().unwrap_or("<root>"),
                        caller_package.unwrap_or("<root>")
                    ));
                }
            }
        }
        TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. }
        | TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_) => {}
    }
    Ok(())
}

fn ensure_type_accessible(
    env: &TypeEnv,
    ty: &IrType,
    caller_package: Option<&str>,
    context: &str,
) -> Result<(), String> {
    match ty {
        IrType::Array(inner)
        | IrType::Ref(inner)
        | IrType::Nullable(inner)
        | IrType::Weak(inner)
        | IrType::Task(inner)
        | IrType::Enumerable(inner)
        | IrType::List(inner) => ensure_type_accessible(env, inner, caller_package, context),
        IrType::Dictionary(key, value) => {
            ensure_type_accessible(env, key, caller_package, context)?;
            ensure_type_accessible(env, value, caller_package, context)
        }
        IrType::Function { params, return_type } => {
            for param in params {
                ensure_type_accessible(env, param, caller_package, context)?;
            }
            ensure_type_accessible(env, return_type, caller_package, context)
        }
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) | IrType::Unknown(name) => {
            if let Some((package_id, visibility)) =
                env.lookup_type_visibility(name, caller_package)
            {
                if !visibility_allows_access(visibility, package_id.as_deref(), caller_package) {
                    return Err(format!(
                        "{context} uses non-public type '{}' from package '{}'",
                        name,
                        package_id.as_deref().unwrap_or("<root>")
                    ));
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn visibility_allows_access(
    visibility: Visibility,
    declaration_package: Option<&str>,
    caller_package: Option<&str>,
) -> bool {
    visibility == Visibility::Public || declaration_package == caller_package
}

fn object_type_name(ty: &IrType) -> Option<&str> {
    match ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => Some(name),
        IrType::Ref(inner) | IrType::Nullable(inner) => object_type_name(inner),
        _ => None,
    }
}

fn validate_async_function(function: &TypedFunction) -> Result<(), String> {
    if !function.is_async {
        return Ok(());
    }
    let IrType::Task(_) = &function.return_type else {
        return Err(format!(
            "async lowering: function '{}' must return Task, Task<T>, ValueTask, or ValueTask<T> in the current gate",
            function.name
        ));
    };
    for param in &function.params {
        if !async_state_capture_supported(param) {
            return Err(format!(
                "async lowering: parameter '{}' in '{}' cannot be captured into the async state safely yet; rewrite it to a copy/shared/class value or construct the owned value inside the async body",
                param.name, function.name
            ));
        }
    }
    let mut scope = HashMap::new();
    for param in &function.params {
        scope.insert(param.name.clone(), param.clone());
    }
    validate_async_stmts(&function.name, &function.body, &HashSet::new(), &mut scope)
}

fn async_state_capture_supported(binding: &TypedBinding) -> bool {
    match binding.ownership {
        Ownership::Copy => true,
        Ownership::Borrowed => {
            binding.name == "this"
                && matches!(binding.ty, IrType::Class(_) | IrType::Interface(_))
        }
        Ownership::Shared => matches!(
            binding.ty,
            IrType::Class(_)
                | IrType::Interface(_)
                | IrType::Function { .. }
                | IrType::Nullable(_)
                | IrType::Unknown(_)
        ),
        Ownership::Owned => matches!(binding.ty, IrType::String | IrType::Exception)
            || matches!(&binding.ty, IrType::Struct(name) if name == "CancellationToken" || name.ends_with(".CancellationToken")),
        Ownership::View | Ownership::Moved => false,
    }
}

fn validate_async_stmts(
    function_name: &str,
    stmts: &[TypedStmt],
    outer_after_uses: &HashSet<String>,
    scope: &mut HashMap<String, TypedBinding>,
) -> Result<(), String> {
    let mut suffix_uses = vec![HashSet::<String>::new(); stmts.len() + 1];
    suffix_uses[stmts.len()] = outer_after_uses.clone();
    for index in (0..stmts.len()).rev() {
        let mut uses = suffix_uses[index + 1].clone();
        collect_used_bindings_stmt(&stmts[index], &mut uses);
        suffix_uses[index] = uses;
    }

    for (index, stmt) in stmts.iter().enumerate() {
        let after_uses = &suffix_uses[index + 1];
        validate_async_stmt(function_name, stmt, after_uses, scope)?;
        if let TypedStmtKind::Let { binding, .. } = &stmt.kind {
            scope.insert(binding.name.clone(), binding.clone());
        }
    }
    Ok(())
}

fn validate_async_stmt(
    function_name: &str,
    stmt: &TypedStmt,
    after_uses: &HashSet<String>,
    scope: &HashMap<String, TypedBinding>,
) -> Result<(), String> {
    match &stmt.kind {
        TypedStmtKind::Let { expr, .. }
        | TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Return(Some(expr))
        | TypedStmtKind::Throw(expr) => validate_async_expr(function_name, expr, after_uses, scope),
        TypedStmtKind::AssignTarget { target, expr } => {
            validate_async_expr(function_name, target, after_uses, scope)?;
            validate_async_expr(function_name, expr, after_uses, scope)
        }
        TypedStmtKind::Block(body) => {
            let mut nested_scope = scope.clone();
            validate_async_stmts(function_name, body, after_uses, &mut nested_scope)
        }
        TypedStmtKind::If {
            condition,
            then_body,
            else_body,
        } => {
            let mut condition_after = after_uses.clone();
            collect_used_bindings_stmts(then_body, &mut condition_after);
            collect_used_bindings_stmts(else_body, &mut condition_after);
            validate_async_expr(function_name, condition, &condition_after, scope)?;

            let mut then_scope = scope.clone();
            validate_async_stmts(function_name, then_body, after_uses, &mut then_scope)?;
            let mut else_scope = scope.clone();
            validate_async_stmts(function_name, else_body, after_uses, &mut else_scope)
        }
        TypedStmtKind::Try {
            try_body,
            catch_name,
            catch_body,
            finally_body,
            ..
        } => {
            let mut try_after = after_uses.clone();
            collect_used_bindings_stmts(catch_body, &mut try_after);
            collect_used_bindings_stmts(finally_body, &mut try_after);
            let mut try_scope = scope.clone();
            validate_async_stmts(function_name, try_body, &try_after, &mut try_scope)?;

            let mut catch_after = after_uses.clone();
            collect_used_bindings_stmts(finally_body, &mut catch_after);
            let mut catch_scope = scope.clone();
            if let Some(name) = catch_name {
                catch_scope.insert(
                    name.clone(),
                    TypedBinding {
                        name: name.clone(),
                        ty: IrType::Exception,
                        ownership: Ownership::Owned,
                    },
                );
            }
            validate_async_stmts(function_name, catch_body, &catch_after, &mut catch_scope)?;

            let mut finally_scope = scope.clone();
            validate_async_stmts(function_name, finally_body, after_uses, &mut finally_scope)
        }
        TypedStmtKind::Switch { expr, cases, default } => {
            let mut expr_after = after_uses.clone();
            for case in cases {
                collect_used_bindings_expr(&case.value, &mut expr_after);
                collect_used_bindings_stmts(&case.body, &mut expr_after);
            }
            collect_used_bindings_stmts(default, &mut expr_after);
            validate_async_expr(function_name, expr, &expr_after, scope)?;
            for case in cases {
                let mut case_after = after_uses.clone();
                collect_used_bindings_stmts(default, &mut case_after);
                validate_async_expr(function_name, &case.value, &case_after, scope)?;
                let mut case_scope = scope.clone();
                validate_async_stmts(function_name, &case.body, after_uses, &mut case_scope)?;
            }
            let mut default_scope = scope.clone();
            validate_async_stmts(function_name, default, after_uses, &mut default_scope)
        }
        TypedStmtKind::While { condition, body } => {
            let mut loop_after = after_uses.clone();
            collect_used_bindings_expr(condition, &mut loop_after);
            collect_used_bindings_stmts(body, &mut loop_after);
            validate_async_expr(function_name, condition, &loop_after, scope)?;
            let mut body_scope = scope.clone();
            validate_async_stmts(function_name, body, &loop_after, &mut body_scope)
        }
        TypedStmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            let mut loop_after = after_uses.clone();
            if let Some(init) = init.as_deref() {
                collect_used_bindings_stmt(init, &mut loop_after);
            }
            if let Some(condition) = condition {
                collect_used_bindings_expr(condition, &mut loop_after);
            }
            if let Some(increment) = increment.as_deref() {
                collect_used_bindings_stmt(increment, &mut loop_after);
            }
            collect_used_bindings_stmts(body, &mut loop_after);
            if let Some(init) = init.as_deref() {
                validate_async_stmt(function_name, init, &loop_after, scope)?;
            }
            if let Some(condition) = condition {
                validate_async_expr(function_name, condition, &loop_after, scope)?;
            }
            let mut body_scope = scope.clone();
            validate_async_stmts(function_name, body, &loop_after, &mut body_scope)?;
            if let Some(increment) = increment.as_deref() {
                validate_async_stmt(function_name, increment, &loop_after, scope)?;
            }
            Ok(())
        }
        TypedStmtKind::ForEach {
            item,
            collection,
            body,
        } => {
            let mut loop_after = after_uses.clone();
            collect_used_bindings_expr(collection, &mut loop_after);
            collect_used_bindings_stmts(body, &mut loop_after);
            validate_async_expr(function_name, collection, &loop_after, scope)?;
            let mut body_scope = scope.clone();
            body_scope.insert(item.name.clone(), item.clone());
            validate_async_stmts(function_name, body, &loop_after, &mut body_scope)
        }
        TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => Ok(()),
    }
}

fn validate_async_expr(
    function_name: &str,
    expr: &TypedExpr,
    after_uses: &HashSet<String>,
    scope: &HashMap<String, TypedBinding>,
) -> Result<(), String> {
    match &expr.kind {
        TypedExprKind::Await(inner) => {
            for (name, binding) in scope {
                if !after_uses.contains(name) {
                    continue;
                }
                if matches!(binding.ownership, Ownership::Borrowed | Ownership::View)
                    && !(name == "this"
                        && matches!(binding.ty, IrType::Class(_) | IrType::Interface(_)))
                {
                    let rewrite = if matches!(binding.ownership, Ownership::View) {
                        "materialize the view into an owned collection or shorten the view scope before await"
                    } else {
                        "copy/move the value into an owned local or shorten the borrow before await"
                    };
                    return Err(format!(
                        "async lowering: '{}' stays {:?} across await in '{}'; {rewrite}",
                        name, binding.ownership, function_name
                    ));
                }
            }
            validate_async_expr(function_name, inner, after_uses, scope)
        }
        TypedExprKind::NullableSome(value)
        | TypedExprKind::Throw(value)
        | TypedExprKind::Unary { expr: value, .. } => {
            validate_async_expr(function_name, value, after_uses, scope)
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                validate_async_expr(function_name, value, after_uses, scope)?;
            }
            Ok(())
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                validate_async_expr(function_name, length, after_uses, scope)?;
            }
            for value in values {
                validate_async_expr(function_name, value, after_uses, scope)?;
            }
            Ok(())
        }
        TypedExprKind::Index { target, index } => {
            validate_async_expr(function_name, target, after_uses, scope)?;
            validate_async_expr(function_name, index, after_uses, scope)
        }
        TypedExprKind::Field { target, .. } => {
            validate_async_expr(function_name, target, after_uses, scope)
        }
        TypedExprKind::IsPattern { expr, .. } => {
            validate_async_expr(function_name, expr, after_uses, scope)
        }
        TypedExprKind::Assign { target, value } => {
            validate_async_expr(function_name, target, after_uses, scope)?;
            validate_async_expr(function_name, value, after_uses, scope)
        }
        TypedExprKind::Call(call) => {
            if let TypedCallKind::Method { target, .. } = &call.kind {
                validate_async_expr(function_name, target, after_uses, scope)?;
            }
            for arg in &call.args {
                validate_async_expr(function_name, arg, after_uses, scope)?;
            }
            Ok(())
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                validate_async_expr(function_name, arg, after_uses, scope)?;
            }
            for field in fields {
                validate_async_expr(function_name, &field.expr, after_uses, scope)?;
            }
            Ok(())
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            validate_async_expr(function_name, condition, after_uses, scope)?;
            validate_async_expr(function_name, when_true, after_uses, scope)?;
            validate_async_expr(function_name, when_false, after_uses, scope)
        }
        TypedExprKind::Binary { left, right, .. } => {
            validate_async_expr(function_name, left, after_uses, scope)?;
            validate_async_expr(function_name, right, after_uses, scope)
        }
        TypedExprKind::IncDec { target, .. } => {
            validate_async_expr(function_name, target, after_uses, scope)
        }
        TypedExprKind::Lambda { body, .. } => {
            validate_async_expr(function_name, body, after_uses, scope)
        }
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. } => Ok(()),
    }
}

fn stmt_contains_await(stmt: &TypedStmt) -> bool {
    match &stmt.kind {
        TypedStmtKind::Let { expr, .. }
        | TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Return(Some(expr))
        | TypedStmtKind::Throw(expr) => expr_contains_await(expr),
        TypedStmtKind::AssignTarget { target, expr } => {
            expr_contains_await(target) || expr_contains_await(expr)
        }
        TypedStmtKind::Block(body) => body.iter().any(stmt_contains_await),
        TypedStmtKind::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_contains_await(condition)
                || then_body.iter().any(stmt_contains_await)
                || else_body.iter().any(stmt_contains_await)
        }
        TypedStmtKind::Try {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            try_body.iter().any(stmt_contains_await)
                || catch_body.iter().any(stmt_contains_await)
                || finally_body.iter().any(stmt_contains_await)
        }
        TypedStmtKind::Switch {
            expr,
            cases,
            default,
        } => {
            expr_contains_await(expr)
                || cases.iter().any(|case| {
                    expr_contains_await(&case.value) || case.body.iter().any(stmt_contains_await)
                })
                || default.iter().any(stmt_contains_await)
        }
        TypedStmtKind::While { condition, body } => {
            expr_contains_await(condition) || body.iter().any(stmt_contains_await)
        }
        TypedStmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            init.as_deref().is_some_and(stmt_contains_await)
                || condition.as_ref().is_some_and(expr_contains_await)
                || increment.as_deref().is_some_and(stmt_contains_await)
                || body.iter().any(stmt_contains_await)
        }
        TypedStmtKind::ForEach {
            collection, body, ..
        } => expr_contains_await(collection) || body.iter().any(stmt_contains_await),
        TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => false,
    }
}

fn expr_contains_await(expr: &TypedExpr) -> bool {
    match &expr.kind {
        TypedExprKind::Await(_) => true,
        TypedExprKind::NullableSome(value)
        | TypedExprKind::Throw(value)
        | TypedExprKind::Unary { expr: value, .. } => expr_contains_await(value),
        TypedExprKind::ArrayLiteral(values) => values.iter().any(expr_contains_await),
        TypedExprKind::NewArray { length, values, .. } => {
            length.as_deref().is_some_and(expr_contains_await)
                || values.iter().any(expr_contains_await)
        }
        TypedExprKind::Index { target, index } => {
            expr_contains_await(target) || expr_contains_await(index)
        }
        TypedExprKind::Field { target, .. } | TypedExprKind::IsPattern { expr: target, .. } => {
            expr_contains_await(target)
        }
        TypedExprKind::Assign { target, value } => {
            expr_contains_await(target) || expr_contains_await(value)
        }
        TypedExprKind::Call(call) => {
            let target_await = match &call.kind {
                TypedCallKind::Method { target, .. } => expr_contains_await(target),
                TypedCallKind::Function { .. } => false,
            };
            target_await || call.args.iter().any(expr_contains_await)
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            args.iter().any(expr_contains_await)
                || fields.iter().any(|field| expr_contains_await(&field.expr))
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            expr_contains_await(condition)
                || expr_contains_await(when_true)
                || expr_contains_await(when_false)
        }
        TypedExprKind::Binary { left, right, .. } => {
            expr_contains_await(left) || expr_contains_await(right)
        }
        TypedExprKind::IncDec { target, .. } => expr_contains_await(target),
        TypedExprKind::Lambda { body, .. } => expr_contains_await(body),
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. } => false,
    }
}

fn collect_used_bindings_stmts(stmts: &[TypedStmt], out: &mut HashSet<String>) {
    for stmt in stmts {
        collect_used_bindings_stmt(stmt, out);
    }
}

fn collect_used_bindings_stmt(stmt: &TypedStmt, out: &mut HashSet<String>) {
    match &stmt.kind {
        TypedStmtKind::Let { expr, .. }
        | TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Return(Some(expr))
        | TypedStmtKind::Throw(expr) => collect_used_bindings_expr(expr, out),
        TypedStmtKind::AssignTarget { target, expr } => {
            collect_used_bindings_expr(target, out);
            collect_used_bindings_expr(expr, out);
        }
        TypedStmtKind::Block(body) => collect_used_bindings_stmts(body, out),
        TypedStmtKind::If {
            condition,
            then_body,
            else_body,
        } => {
            collect_used_bindings_expr(condition, out);
            collect_used_bindings_stmts(then_body, out);
            collect_used_bindings_stmts(else_body, out);
        }
        TypedStmtKind::Try {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            collect_used_bindings_stmts(try_body, out);
            collect_used_bindings_stmts(catch_body, out);
            collect_used_bindings_stmts(finally_body, out);
        }
        TypedStmtKind::Switch {
            expr,
            cases,
            default,
        } => {
            collect_used_bindings_expr(expr, out);
            for case in cases {
                collect_used_bindings_expr(&case.value, out);
                collect_used_bindings_stmts(&case.body, out);
            }
            collect_used_bindings_stmts(default, out);
        }
        TypedStmtKind::While { condition, body } => {
            collect_used_bindings_expr(condition, out);
            collect_used_bindings_stmts(body, out);
        }
        TypedStmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            if let Some(init) = init.as_deref() {
                collect_used_bindings_stmt(init, out);
            }
            if let Some(condition) = condition {
                collect_used_bindings_expr(condition, out);
            }
            if let Some(increment) = increment.as_deref() {
                collect_used_bindings_stmt(increment, out);
            }
            collect_used_bindings_stmts(body, out);
        }
        TypedStmtKind::ForEach {
            collection, body, ..
        } => {
            collect_used_bindings_expr(collection, out);
            collect_used_bindings_stmts(body, out);
        }
        TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
    }
}

fn collect_used_bindings_expr(expr: &TypedExpr, out: &mut HashSet<String>) {
    match &expr.kind {
        TypedExprKind::Var(name) | TypedExprKind::Move(name) | TypedExprKind::Borrow { name, .. } => {
            out.insert(name.clone());
        }
        TypedExprKind::NullableSome(value)
        | TypedExprKind::Throw(value)
        | TypedExprKind::Unary { expr: value, .. }
        | TypedExprKind::Await(value) => collect_used_bindings_expr(value, out),
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                collect_used_bindings_expr(value, out);
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_used_bindings_expr(length, out);
            }
            for value in values {
                collect_used_bindings_expr(value, out);
            }
        }
        TypedExprKind::Index { target, index } => {
            collect_used_bindings_expr(target, out);
            collect_used_bindings_expr(index, out);
        }
        TypedExprKind::Field { target, .. } | TypedExprKind::IsPattern { expr: target, .. } => {
            collect_used_bindings_expr(target, out)
        }
        TypedExprKind::Assign { target, value } => {
            collect_used_bindings_expr(target, out);
            collect_used_bindings_expr(value, out);
        }
        TypedExprKind::Call(call) => {
            if let TypedCallKind::Method { target, .. } = &call.kind {
                collect_used_bindings_expr(target, out);
            }
            for arg in &call.args {
                collect_used_bindings_expr(arg, out);
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_used_bindings_expr(arg, out);
            }
            for field in fields {
                collect_used_bindings_expr(&field.expr, out);
            }
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_used_bindings_expr(condition, out);
            collect_used_bindings_expr(when_true, out);
            collect_used_bindings_expr(when_false, out);
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_used_bindings_expr(left, out);
            collect_used_bindings_expr(right, out);
        }
        TypedExprKind::IncDec { target, .. } => collect_used_bindings_expr(target, out),
        TypedExprKind::Lambda { body, .. } => collect_used_bindings_expr(body, out),
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::FunctionSymbol(_) => {}
    }
}

fn populate_function_signatures(program: &Program, env: &mut TypeEnv) {
    let mut counts = HashMap::<String, usize>::new();
    for function in &program.functions {
        *counts.entry(function.name.clone()).or_default() += 1;
    }
    for function in &program.functions {
        let overloaded = counts.get(&function.name).copied().unwrap_or_default() > 1;
        let signature = function_signature(function, env, overloaded);
        env.functions
            .entry(function.name.clone())
            .or_default()
            .push(signature);
    }
}

fn populate_delegate_signatures(program: &Program, env: &mut TypeEnv) {
    for delegate in &program.delegates {
        let signature = DelegateSignature {
            full_name: qualified_delegate_name(&delegate.namespace, &delegate.name),
            generic_params: delegate
                .generic_params
                .iter()
                .map(|param| param.name.clone())
                .collect(),
            params: delegate.params.iter().map(|param| param.ty.clone()).collect(),
            return_type: delegate.return_type.clone(),
        };
        env.delegates
            .entry(signature.full_name.clone())
            .or_insert_with(|| signature.clone());
        env.delegates
            .entry(delegate.name.clone())
            .or_insert(signature);
    }
}

fn populate_method_signatures(program: &Program, env: &mut TypeEnv) {
    for (type_index, ty) in program.types.iter().enumerate() {
        for method in &ty.methods {
            let overloaded = ty
                .methods
                .iter()
                .filter(|candidate| candidate.name == method.name)
                .count()
                > 1;
            let params = method
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let param_ownerships = method
                .params
                .iter()
                .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
                .collect::<Vec<_>>();
            let suffix = params
                .iter()
                .map(ir_symbol_suffix)
                .collect::<Vec<_>>()
                .join("_");
            let base = format!(
                "{}_{}__g{}",
                qualified_type_symbol_name(
                    &ty.namespace,
                    &ty.name,
                    ty.generic_params.len(),
                    type_index,
                ),
                sanitize_ir_symbol(&method.name),
                method.generic_params.len()
            );
            // Preserve the C# source method name for lookup, but give each overload
            // a stable lowered symbol that later IR/codegen stages can call directly.
            let symbol = if overloaded {
                if suffix.is_empty() {
                    format!("{base}__overload")
                } else {
                    format!("{base}__{suffix}")
                }
            } else {
                base
            };
            let return_type = type_syntax_to_ir(&method.return_type, env);
            let return_ownership = ownership_for_declared_type_syntax(&method.return_type, env);
            let params_element_type = params_element_type(&method.params, env);
            let required_params = method.params.iter().take_while(|param| param.default.is_none()).count();
            let key = (ty.name.clone(), method.name.clone());
            let signature = FunctionSignature {
                package_id: method.package_id.clone().or_else(|| ty.package_id.clone()),
                visibility: method.visibility,
                generic_params: method
                    .generic_params
                    .iter()
                    .map(|param| param.name.clone())
                    .collect(),
                params,
                param_ownerships,
                required_params,
                params_element_type,
                return_type,
                return_ownership,
                symbol,
                is_static: method.is_static,
            };
            if method.is_extension || ty.is_extension {
                env.extension_methods.entry(key).or_default().push(signature);
            } else {
                env.methods.entry(key).or_default().push(signature);
            }
        }
    }
}

fn populate_constructor_signatures(program: &Program, env: &mut TypeEnv) {
    for (type_index, ty) in program.types.iter().enumerate() {
        let overloaded = ty.constructors.len() > 1;
        for constructor in &ty.constructors {
            let params = constructor
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let param_ownerships = constructor
                .params
                .iter()
                .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
                .collect::<Vec<_>>();
            let suffix = params
                .iter()
                .map(ir_symbol_suffix)
                .collect::<Vec<_>>()
                .join("_");
            let base = format!(
                "{}_ctor",
                qualified_type_symbol_name(
                    &ty.namespace,
                    &ty.name,
                    ty.generic_params.len(),
                    type_index,
                )
            );
            let symbol = if overloaded {
                if suffix.is_empty() {
                    format!("{base}__empty")
                } else {
                    format!("{base}__{suffix}")
                }
            } else {
                base
            };
            let params_element_type = params_element_type(&constructor.params, env);
            let required_params = constructor.params.iter().take_while(|param| param.default.is_none()).count();
            env.constructors
                .entry(ty.name.clone())
                .or_default()
                .push(FunctionSignature {
                    package_id: ty.package_id.clone(),
                    visibility: constructor.visibility,
                    generic_params: ty
                        .generic_params
                        .iter()
                        .map(|param| param.name.clone())
                        .collect(),
                    params,
                    param_ownerships,
                    required_params,
                    params_element_type,
                    return_type: IrType::Void,
                    return_ownership: Ownership::Copy,
                    symbol,
                    is_static: false,
                });
        }
    }
}

impl TypeEnv {
    fn from_program(program: &Program) -> Self {
        let mut env = TypeEnv::default();
        for enum_def in &program.enums {
            env.kinds.insert(enum_def.name.clone(), TypeKind::Enum);
            env.type_packages
                .insert(enum_def.name.clone(), enum_def.package_id.clone());
            env.type_visibilities
                .insert(enum_def.name.clone(), enum_def.visibility);
            if let Some(package_id) = &enum_def.package_id {
                let key = package_type_key(package_id, &enum_def.name);
                env.kinds.insert(key.clone(), TypeKind::Enum);
                env.type_packages.insert(key.clone(), enum_def.package_id.clone());
                env.type_visibilities.insert(key, enum_def.visibility);
            }
        }
        for ty in &program.types {
            env.kinds.insert(ty.name.clone(), ty.kind);
            env.type_packages
                .insert(ty.name.clone(), ty.package_id.clone());
            env.type_visibilities
                .insert(ty.name.clone(), ty.visibility);
            if let Some(package_id) = &ty.package_id {
                let key = package_type_key(package_id, &ty.name);
                env.kinds.insert(key.clone(), ty.kind);
                env.type_packages.insert(key.clone(), ty.package_id.clone());
                env.type_visibilities.insert(key, ty.visibility);
            }
            env.bases.insert(ty.name.clone(), ty.bases.clone());
        }
        populate_delegate_signatures(program, &mut env);
        populate_function_signatures(program, &mut env);
        populate_method_signatures(program, &mut env);
        populate_constructor_signatures(program, &mut env);
        for ty in &program.types {
            for field in &ty.fields {
                let field_type = type_syntax_to_ir(&field.ty, &env);
                let field_signature = FieldSignature {
                    package_id: ty.package_id.clone(),
                    visibility: field.visibility,
                    ty: field_type.clone(),
                    ownership: ownership_for_declared_type_syntax(&field.ty, &env),
                };
                env.fields
                    .insert((ty.name.clone(), field.name.clone()), field_signature.clone());
                env.field_order
                    .entry(ty.name.clone())
                    .or_default()
                    .push((field.name.clone(), field_signature));
            }
        }
        env
    }

    fn single_function(&self, name: &str) -> Option<&FunctionSignature> {
        let signatures = self.functions.get(name)?;
        if signatures.len() == 1 {
            signatures.first()
        } else {
            None
        }
    }

    fn lookup_type_visibility(
        &self,
        type_name: &str,
        caller_package: Option<&str>,
    ) -> Option<(Option<String>, Visibility)> {
        let package_candidate = |candidate: &str| {
            caller_package.map(|package_id| package_type_key(package_id, candidate))
        };
        for candidate in [
            type_name.to_string(),
            base_type_name(type_name).to_string(),
            type_name.rsplit('.').next().unwrap_or(type_name).to_string(),
        ] {
            if let Some(package_key) = package_candidate(&candidate) {
                if let Some(visibility) = self.type_visibilities.get(&package_key) {
                    let package_id = self
                        .type_packages
                        .get(&package_key)
                        .cloned()
                        .unwrap_or(None);
                    return Some((package_id, *visibility));
                }
            }
            if let Some(visibility) = self.type_visibilities.get(&candidate) {
                let package_id = self
                    .type_packages
                    .get(&candidate)
                    .cloned()
                    .unwrap_or(None);
                return Some((package_id, *visibility));
            }
        }
        None
    }

    fn lookup_signature_by_symbol(&self, symbol: &str) -> Option<&FunctionSignature> {
        self.functions
            .values()
            .chain(self.methods.values())
            .chain(self.extension_methods.values())
            .chain(self.constructors.values())
            .flat_map(|signatures| signatures.iter())
            .find(|signature| signature.symbol == symbol)
    }

    fn resolve_function(&self, name: &str, arg_types: &[IrType]) -> Option<&FunctionSignature> {
        let signatures = self.functions.get(name)?;
        resolve_signature(signatures, arg_types)
    }

    fn resolve_function_call(
        &self,
        name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<&FunctionSignature>, String> {
        let Some(signatures) = self.functions.get(name) else {
            return Ok(None);
        };
        resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("call to '{name}'"),
        )
    }

    fn resolve_method(
        &self,
        type_name: &str,
        method_name: &str,
        arg_types: &[IrType],
    ) -> Option<FunctionSignature> {
        if let Some(signatures) = self
            .methods
            .get(&(type_name.to_string(), method_name.to_string()))
        {
            if let Some(sig) = resolve_signature(signatures, arg_types) {
                return Some(sig.clone());
            }
        }
        if let Some(signature) =
            self.resolve_specialized_method_signature(type_name, method_name, arg_types)
        {
            return Some(signature);
        }
        self.bases.get(type_name)?.iter().find_map(|base| {
            let simple = base.rsplit('.').next().unwrap_or(base);
            self.resolve_method(simple, method_name, arg_types)
        })
    }

    fn resolve_method_call(
        &self,
        type_name: &str,
        method_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        let resolve = |args: &[TypedExpr]| -> Result<Option<FunctionSignature>, String> {
            if let Some(signatures) = self
                .methods
                .get(&(type_name.to_string(), method_name.to_string()))
            {
                if let Some(sig) = resolve_call_signature(
                    signatures,
                    args,
                    |expected, arg| ir_conversion_rank(expected, arg, self),
                    || format!("call to '{}.{}'", type_name, method_name),
                )? {
                    return Ok(Some(sig.clone()));
                }
            }
            Ok(None)
        };
        if let Some(sig) = resolve(args)? {
            return Ok(Some(sig));
        }
        if let Some(signature) =
            self.resolve_specialized_method_call_signature(type_name, method_name, args)?
        {
            return Ok(Some(signature));
        }
        if method_name.ends_with("Async") && args.len() > 1 {
            if let Some(sig) = resolve(&args[..args.len() - 1])? {
                return Ok(Some(sig));
            }
        }
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                if let Some(sig) = self.resolve_method_call(simple, method_name, args)? {
                    return Ok(Some(sig));
                }
            }
        }
        Ok(None)
    }

    fn resolve_extension_method_call(
        &self,
        type_name: &str,
        method_name: &str,
        receiver: &TypedExpr,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        let Some(signatures) = self
            .extension_methods
            .get(&(type_name.to_string(), method_name.to_string()))
        else {
            return Ok(None);
        };
        let mut combined = Vec::with_capacity(args.len() + 1);
        combined.push(receiver.clone());
        combined.extend(args.iter().cloned());
        Ok(resolve_call_signature(
            signatures,
            &combined,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("extension call to '{}.{}'", type_name, method_name),
        )?
        .cloned())
    }

    fn resolve_constructor(
        &self,
        type_name: &str,
        arg_types: &[IrType],
    ) -> Option<FunctionSignature> {
        if let Some(signatures) = self.constructors.get(type_name) {
            if let Some(signature) = resolve_signature(signatures, arg_types) {
                return Some(signature.clone());
            }
        }
        self.resolve_specialized_constructor_signature(type_name, arg_types)
    }

    fn resolve_constructor_call(
        &self,
        type_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        if let Some(signatures) = self.constructors.get(type_name) {
            if let Some(signature) = resolve_call_signature(
                signatures,
                args,
                |expected, arg| ir_conversion_rank(expected, arg, self),
                || format!("constructor call '{type_name}'"),
            )? {
                return Ok(Some(signature.clone()));
            }
        }
        self.resolve_specialized_constructor_call_signature(type_name, args)
    }

    fn resolve_field(&self, type_name: &str, field_name: &str) -> Option<FieldSignature> {
        if let Some(ty) = self
            .fields
            .get(&(type_name.to_string(), field_name.to_string()))
        {
            return Some(ty.clone());
        }
        if let Some(field) = self.resolve_specialized_field(type_name, field_name) {
            return Some(field);
        }
        self.bases.get(type_name)?.iter().find_map(|base| {
            let simple = base.rsplit('.').next().unwrap_or(base);
            self.resolve_field(simple, field_name)
        })
    }

    fn all_fields(&self, type_name: &str) -> Vec<(String, IrType)> {
        self.all_field_infos(type_name)
            .into_iter()
            .map(|(name, field)| (name, field.ty))
            .collect()
    }

    fn all_field_infos(&self, type_name: &str) -> Vec<(String, FieldSignature)> {
        let mut fields = Vec::new();
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                fields.extend(self.all_field_infos(simple));
            }
        }
        if let Some(own_fields) = self.field_order.get(type_name) {
            for (name, field) in own_fields {
                if let Some(existing) = fields.iter_mut().find(|(field, _)| field == name) {
                    *existing = (name.clone(), field.clone());
                } else {
                    fields.push((name.clone(), field.clone()));
                }
            }
        }
        if fields.is_empty() {
            if let Some(specialized_fields) = self.resolve_specialized_all_field_infos(type_name) {
                return specialized_fields;
            }
        }
        fields
    }

    fn resolve_specialized_field(&self, type_name: &str, field_name: &str) -> Option<FieldSignature> {
        let (base_name, subst) = self.generic_owner_subst(type_name)?;
        let field = self.fields.get(&(base_name.clone(), field_name.to_string()))?;
        Some(substitute_field_signature(field, &subst))
    }

    fn resolve_specialized_all_field_infos(
        &self,
        type_name: &str,
    ) -> Option<Vec<(String, FieldSignature)>> {
        let (base_name, subst) = self.generic_owner_subst(type_name)?;
        let own_fields = self.field_order.get(&base_name)?;
        let mut fields = Vec::new();
        if let Some(bases) = self.bases.get(&base_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                fields.extend(self.all_field_infos(simple));
            }
        }
        for (name, field) in own_fields {
            let field = substitute_field_signature(field, &subst);
            if let Some(existing) = fields.iter_mut().find(|(field_name, _)| field_name == name) {
                *existing = (name.clone(), field);
            } else {
                fields.push((name.clone(), field));
            }
        }
        Some(fields)
    }

    fn resolve_specialized_method_signature(
        &self,
        type_name: &str,
        method_name: &str,
        arg_types: &[IrType],
    ) -> Option<FunctionSignature> {
        let (base_name, subst) = self.generic_owner_subst(type_name)?;
        let signatures = self.methods.get(&(base_name.clone(), method_name.to_string()))?;
        let signature = resolve_signature(signatures, arg_types)?;
        Some(substitute_function_signature(signature, &subst))
    }

    fn resolve_specialized_method_call_signature(
        &self,
        type_name: &str,
        method_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        let Some((base_name, subst)) = self.generic_owner_subst(type_name) else {
            return Ok(None);
        };
        let Some(signatures) = self.methods.get(&(base_name.clone(), method_name.to_string())) else {
            return Ok(None);
        };
        let signature = resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("call to '{}.{}'", type_name, method_name),
        )?;
        Ok(signature.map(|sig| substitute_function_signature(sig, &subst)))
    }

    fn resolve_specialized_constructor_signature(
        &self,
        type_name: &str,
        arg_types: &[IrType],
    ) -> Option<FunctionSignature> {
        let (base_name, subst) = self.generic_owner_subst(type_name)?;
        let signatures = self.constructors.get(&base_name)?;
        let signature = resolve_signature(signatures, arg_types)?;
        Some(substitute_function_signature(signature, &subst))
    }

    fn resolve_specialized_constructor_call_signature(
        &self,
        type_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        let Some((base_name, subst)) = self.generic_owner_subst(type_name) else {
            return Ok(None);
        };
        let Some(signatures) = self.constructors.get(&base_name) else {
            return Ok(None);
        };
        let signature = resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("constructor call '{type_name}'"),
        )?;
        Ok(signature.map(|sig| substitute_function_signature(sig, &subst)))
    }

    fn generic_owner_subst(&self, type_name: &str) -> Option<(String, HashMap<String, IrType>)> {
        let (base, args) = split_monomorphized_type(type_name)?;
        let base_name = base_type_name(base).to_string();
        let generic_params = self.type_generic_params.get(&base_name)?;
        if generic_params.len() != args.len() || generic_params.is_empty() {
            return None;
        }
        let args = args
            .into_iter()
            .map(|arg| parse_monomorphized_ir_type(&arg, self))
            .collect::<Option<Vec<_>>>()?;
        let mut subst = HashMap::new();
        for (name, arg) in generic_params.iter().cloned().zip(args) {
            subst.insert(name, arg);
        }
        Some((base_name, subst))
    }
}

fn resolve_signature<'a>(
    signatures: &'a [FunctionSignature],
    arg_types: &[IrType],
) -> Option<&'a FunctionSignature> {
    let args = arg_types
        .iter()
        .map(|ty| TypedExpr {
            kind: TypedExprKind::Var("<type-only>".to_string()),
            ty: ty.clone(),
            ownership: ownership_for_type(ty),
            drop_kind: drop_kind_for_type(ty, &ownership_for_type(ty)),
        })
        .collect::<Vec<_>>();
    resolve_call_signature(
        signatures,
        &args,
        |expected, arg| {
            if ir_arg_matches(expected, &arg.ty) {
                Some(0)
            } else {
                None
            }
        },
        || "type-only overload lookup".to_string(),
    )
    .ok()
    .flatten()
}

fn ir_arg_matches(expected: &IrType, actual: &IrType) -> bool {
    expected == actual || matches!((expected, actual), (IrType::Int, IrType::Long))
}

fn resolve_call_signature<'a, F, C>(
    signatures: &'a [FunctionSignature],
    args: &[TypedExpr],
    rank: F,
    context: C,
) -> Result<Option<&'a FunctionSignature>, String>
where
    F: Fn(&IrType, &TypedExpr) -> Option<u16>,
    C: Fn() -> String,
{
    let mut applicable = Vec::<(&FunctionSignature, Vec<u16>)>::new();
    let mut seen = HashSet::<String>::new();
    for signature in signatures {
        let key = format!(
            "{:?}|{:?}|{:?}|{:?}|{}",
            signature.params,
            signature.params_element_type,
            signature.return_type,
            signature.return_ownership,
            signature.is_static
        );
        if !seen.insert(key) {
            continue;
        }
        if args.len() >= signature.required_params && args.len() <= signature.params.len() {
            if let Some(ranks) = signature
                .params
                .iter()
                .take(args.len())
                .zip(args.iter())
                .map(|(expected, arg)| rank(expected, arg))
                .collect::<Option<Vec<_>>>()
            {
                applicable.push((signature, ranks));
                continue;
            }
        }
        let Some(params_element_type) = &signature.params_element_type else {
            continue;
        };
        let fixed_len = signature.params.len().saturating_sub(1);
        if args.len() < fixed_len {
            continue;
        }
        let mut ranks = Vec::new();
        let mut applicable_signature = true;
        for (expected, arg) in signature
            .params
            .iter()
            .take(fixed_len)
            .zip(args.iter().take(fixed_len))
        {
            let Some(rank_value) = rank(expected, arg) else {
                applicable_signature = false;
                break;
            };
            ranks.push(rank_value);
        }
        if !applicable_signature {
            continue;
        }
        for arg in args.iter().skip(fixed_len) {
            let Some(rank_value) = rank(params_element_type, arg) else {
                applicable_signature = false;
                break;
            };
            ranks.push(rank_value);
        }
        if applicable_signature {
            applicable.push((signature, ranks));
        }
    }

    if applicable.is_empty() {
        return Ok(None);
    }

    let Some((best, best_ranks)) = applicable.iter().find(|(_, ranks)| {
        applicable.iter().all(|(_, other)| {
            ranks == other
                || ranks
                    .iter()
                    .zip(other.iter())
                    .all(|(candidate, other)| candidate <= other)
        })
    }) else {
        let best_generic_arity = applicable
            .iter()
            .map(|(signature, _)| signature.generic_params.len())
            .min();
        if let Some(best_generic_arity) = best_generic_arity {
            let least_generic = applicable
                .iter()
                .filter(|(signature, _)| signature.generic_params.len() == best_generic_arity)
                .collect::<Vec<_>>();
            if least_generic.len() == 1 {
                return Ok(Some(least_generic[0].0));
            }
        }
        let candidates = applicable
            .iter()
            .map(|(signature, ranks)| {
                format!(
                    "{:?} -> {:?} ranks={:?}",
                    signature.params, signature.return_type, ranks
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        return Err(format!(
            "ambiguous overload resolution for {}: {}",
            context(),
            candidates
        ));
    };

    let tied = applicable
        .iter()
        .filter(|(_, ranks)| {
            ranks == best_ranks
                || (ranks
                    .iter()
                    .zip(best_ranks.iter())
                    .all(|(candidate, best)| candidate <= best)
                    && best_ranks
                        .iter()
                        .zip(ranks.iter())
                        .all(|(best, candidate)| best <= candidate))
                })
        .count();
    if tied > 1 {
        let tied_candidates = applicable
            .iter()
            .filter(|(_, ranks)| ranks == best_ranks)
            .collect::<Vec<_>>();
        let best_specificity = tied_candidates
            .iter()
            .map(|(signature, _)| signature_specificity(signature))
            .min();
        if let Some(best_specificity) = best_specificity {
            let specialized = tied_candidates
                .into_iter()
                .filter(|(signature, _)| signature_specificity(signature) == best_specificity)
                .collect::<Vec<_>>();
            if specialized.len() == 1 {
                return Ok(Some(specialized[0].0));
            }
            let best_generic_arity = specialized
                .iter()
                .map(|(signature, _)| signature.generic_params.len())
                .min();
            if let Some(best_generic_arity) = best_generic_arity {
                let least_generic = specialized
                    .into_iter()
                    .filter(|(signature, _)| signature.generic_params.len() == best_generic_arity)
                    .collect::<Vec<_>>();
                if least_generic.len() == 1 {
                    return Ok(Some(least_generic[0].0));
                }
            }
        }
        let candidates = applicable
            .iter()
            .map(|(signature, ranks)| {
                format!(
                    "{:?} -> {:?} ranks={:?}",
                    signature.params, signature.return_type, ranks
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        return Err(format!(
            "ambiguous overload resolution for {}: {}",
            context(),
            candidates
        ));
    }

    Ok(Some(*best))
}

fn signature_specificity(signature: &FunctionSignature) -> u16 {
    signature
        .params
        .iter()
        .map(ir_type_specificity)
        .sum()
}

fn ir_type_specificity(ty: &IrType) -> u16 {
    match ty {
        IrType::Enumerable(_) => 0,
        IrType::List(_) => 1,
        IrType::Array(_) => 0,
        IrType::Dictionary(_, _) => 0,
        IrType::Ref(inner)
        | IrType::Task(inner)
        | IrType::Weak(inner)
        | IrType::Nullable(inner) => ir_type_specificity(inner),
        IrType::Function {
            params,
            return_type,
        } => params.iter().map(ir_type_specificity).sum::<u16>() + ir_type_specificity(return_type),
        _ => 0,
    }
}

#[derive(Debug, Clone)]
struct CheckedVar {
    ty: IrType,
    ownership: Ownership,
    scope_depth: usize,
    view_source: Option<String>,
}

#[derive(Debug, Clone)]
struct OwnershipSnapshot {
    scopes: Vec<Vec<String>>,
    vars: HashMap<String, CheckedVar>,
    exits_function: bool,
    exits_loop: bool,
}

fn package_type_key(package_id: &str, type_name: &str) -> String {
    format!("{package_id}::{type_name}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopExitKind {
    Break,
    Continue,
}

#[derive(Debug, Clone)]
struct LoopExitSnapshot {
    kind: LoopExitKind,
    snapshot: OwnershipSnapshot,
}

#[derive(Debug, Clone)]
struct OwnershipState {
    scopes: Vec<Vec<String>>,
    vars: HashMap<String, CheckedVar>,
    exits_function: bool,
    exits_loop: bool,
    loop_exit_snapshots: Vec<LoopExitSnapshot>,
}

impl OwnershipState {
    fn new(params: Vec<TypedBinding>) -> Self {
        let mut state = Self {
            scopes: vec![Vec::new()],
            vars: HashMap::new(),
            exits_function: false,
            exits_loop: false,
            loop_exit_snapshots: Vec::new(),
        };
        for param in params {
            state.declare(param.name, param.ty, param.ownership, None);
        }
        state
    }

    fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for name in scope {
                self.vars.remove(&name);
            }
        }
    }

    fn depth(&self) -> usize {
        self.scopes.len().saturating_sub(1)
    }

    fn declare(
        &mut self,
        name: String,
        ty: IrType,
        ownership: Ownership,
        view_source: Option<String>,
    ) {
        self.scopes.last_mut().unwrap().push(name.clone());
        self.vars.insert(
            name,
            CheckedVar {
                ty,
                ownership,
                scope_depth: self.depth(),
                view_source,
            },
        );
    }

    fn assign(
        &mut self,
        name: &str,
        ty: IrType,
        ownership: Ownership,
        view_source: Option<String>,
    ) -> Result<(), String> {
        let Some(existing) = self.vars.get_mut(name) else {
            return Ok(());
        };
        existing.ty = ty;
        existing.ownership = ownership;
        existing.view_source = view_source;
        Ok(())
    }

    fn get(&self, name: &str) -> Option<&CheckedVar> {
        self.vars.get(name)
    }

    fn snapshot(&self) -> OwnershipSnapshot {
        OwnershipSnapshot {
            scopes: self.scopes.clone(),
            vars: self.vars.clone(),
            exits_function: self.exits_function,
            exits_loop: self.exits_loop,
        }
    }

    fn from_snapshot(snapshot: &OwnershipSnapshot) -> Self {
        Self {
            scopes: snapshot.scopes.clone(),
            vars: snapshot.vars.clone(),
            exits_function: snapshot.exits_function,
            exits_loop: snapshot.exits_loop,
            loop_exit_snapshots: Vec::new(),
        }
    }
}

fn merge_checked_vars(left: &CheckedVar, right: &CheckedVar) -> CheckedVar {
    CheckedVar {
        ty: if left.ty == right.ty {
            left.ty.clone()
        } else if matches!(left.ty, IrType::Unknown(_)) {
            right.ty.clone()
        } else {
            left.ty.clone()
        },
        ownership: merge_ownership(&left.ownership, &right.ownership),
        scope_depth: left.scope_depth.min(right.scope_depth),
        view_source: if left.view_source == right.view_source {
            left.view_source.clone()
        } else if matches!(
            (&left.view_source, &right.view_source),
            (Some(_), Some(_))
        ) {
            left.view_source.clone().or(right.view_source.clone())
        } else {
            None
        },
    }
}

fn merge_ownership(left: &Ownership, right: &Ownership) -> Ownership {
    fn rank(value: &Ownership) -> u8 {
        match value {
            Ownership::Copy => 0,
            Ownership::Owned => 1,
            Ownership::Shared => 2,
            Ownership::View => 3,
            Ownership::Borrowed => 4,
            Ownership::Moved => 5,
        }
    }

    match rank(left).max(rank(right)) {
        0 => Ownership::Copy,
        1 => Ownership::Owned,
        2 => Ownership::Shared,
        3 => Ownership::View,
        4 => Ownership::Borrowed,
        _ => Ownership::Moved,
    }
}

#[derive(Debug, Clone)]
struct CheckedExpr {
    ty: IrType,
    ownership: Ownership,
    source_var: Option<String>,
    is_move: bool,
}

struct OwnershipChecker;

impl OwnershipChecker {
    fn check_function(
        function: &Function,
        env: &TypeEnv,
        implicit_params: Vec<TypedBinding>,
    ) -> Result<(), String> {
        let mut params = implicit_params;
        for param in &function.params {
            let ty = type_syntax_to_ir(&param.ty, env);
            params.push(TypedBinding {
                name: param.name.clone(),
                ownership: ownership_for_declared_type_syntax(&param.ty, env),
                ty,
            });
        }
        Self::check_body(
            &format!("function {}", function.name),
            &function.body,
            env,
            params,
            &type_syntax_to_ir(&function.return_type, env),
            ownership_for_declared_type_syntax(&function.return_type, env),
        )
    }

    fn check_body(
        context: &str,
        body: &[Stmt],
        env: &TypeEnv,
        params: Vec<TypedBinding>,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<(), String> {
        let mut state = OwnershipState::new(params);
        Self::check_stmts(context, body, env, &mut state, return_type, return_ownership)
    }

    fn check_stmts(
        context: &str,
        stmts: &[Stmt],
        env: &TypeEnv,
        state: &mut OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<(), String> {
        for stmt in stmts {
            Self::check_stmt(context, stmt, env, state, return_type, return_ownership.clone())?;
            if state.exits_function || state.exits_loop {
                break;
            }
        }
        Ok(())
    }

    fn check_block_snapshot(
        context: &str,
        stmts: &[Stmt],
        env: &TypeEnv,
        state: &OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<OwnershipState, String> {
        let mut branch_state = state.clone();
        Self::check_stmts(
            context,
            stmts,
            env,
            &mut branch_state,
            return_type,
            return_ownership,
        )?;
        Ok(branch_state)
    }

    fn apply_finally_to_state(
        context: &str,
        finally_body: &[Stmt],
        env: &TypeEnv,
        state: &OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<OwnershipState, String> {
        let mut final_state = state.clone();
        let exited_function = final_state.exits_function;
        let exited_loop = final_state.exits_loop;
        let existing_snapshots = final_state.loop_exit_snapshots.clone();
        final_state.push_scope();
        Self::check_stmts(
            context,
            finally_body,
            env,
            &mut final_state,
            return_type,
            return_ownership.clone(),
        )?;
        final_state.pop_scope();
        final_state.exits_function |= exited_function;
        final_state.exits_loop |= exited_loop;

        let mut propagated_snapshots = Vec::new();
        for snapshot in existing_snapshots {
            let mut snapshot_state = OwnershipState::from_snapshot(&snapshot.snapshot);
            snapshot_state.push_scope();
            Self::check_stmts(
                context,
                finally_body,
                env,
                &mut snapshot_state,
                return_type,
                return_ownership.clone(),
            )?;
            snapshot_state.pop_scope();
            propagated_snapshots.push(LoopExitSnapshot {
                kind: snapshot.kind,
                snapshot: snapshot_state.snapshot(),
            });
        }
        final_state.loop_exit_snapshots.extend(propagated_snapshots);
        Ok(final_state)
    }

    fn merge_branch_states(base: &OwnershipState, branches: &[OwnershipState]) -> OwnershipState {
        let mut merged = base.clone();
        let live_branches: Vec<&OwnershipState> = branches
            .iter()
            .filter(|branch| !branch.exits_function && !branch.exits_loop)
            .collect();
        let active_branches = if live_branches.is_empty() {
            branches.iter().collect::<Vec<_>>()
        } else {
            live_branches
        };
        for (name, base_var) in base.vars.iter() {
            let mut current = base_var.clone();
            for branch in &active_branches {
                if let Some(branch_var) = branch.vars.get(name) {
                    current = merge_checked_vars(&current, branch_var);
                }
            }
            merged.vars.insert(name.clone(), current);
        }
        merged.loop_exit_snapshots = branches
            .iter()
            .flat_map(|branch| branch.loop_exit_snapshots.iter().cloned())
            .collect();
        merged
    }

    fn merge_loop_exit_states(base: &OwnershipState, branches: &[OwnershipState]) -> OwnershipState {
        let mut merged = base.clone();
        for (name, base_var) in base.vars.iter() {
            let mut current = base_var.clone();
            for branch in branches {
                if let Some(branch_var) = branch.vars.get(name) {
                    current = merge_checked_vars(&current, branch_var);
                }
            }
            merged.vars.insert(name.clone(), current);
        }
        merged.loop_exit_snapshots = branches
            .iter()
            .flat_map(|branch| branch.loop_exit_snapshots.iter().cloned())
            .collect();
        merged
    }

    fn check_stmt(
        context: &str,
        stmt: &Stmt,
        env: &TypeEnv,
        state: &mut OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<(), String> {
        match stmt {
            Stmt::Let {
                name,
                declared_type,
                expr,
                ..
            } => {
                let expr = Self::check_expr(expr, env, state)?;
                let ty = declared_type
                    .as_ref()
                    .map(|ty| type_syntax_to_ir(ty, env))
                    .unwrap_or_else(|| expr.ty.clone());
                let expr = Self::coerce_for_target(&ty, expr);
                let target_ownership = declared_type
                    .as_ref()
                    .map(|declared| ownership_for_declared_type_syntax(declared, env))
                    .unwrap_or_else(|| ownership_for_type(&ty));
                Self::check_assignment_safety(context, env, &ty, target_ownership, &expr)?;
                Self::check_view_source_lifetime(context, state, name, &expr)?;
                let ownership = declared_type
                    .as_ref()
                    .map(|declared| ownership_for_declared_type_syntax(declared, env))
                    .unwrap_or_else(|| expr.ownership.clone());
                state.declare(name.clone(), ty, ownership, expr.source_var);
            }
            Stmt::Assign { name, expr } => {
                let expr = Self::check_expr(expr, env, state)?;
                let target_ty = state
                    .get(name)
                    .map(|var| var.ty.clone())
                    .unwrap_or_else(|| expr.ty.clone());
                let target_ownership = state
                    .get(name)
                    .map(|var| var.ownership.clone())
                    .unwrap_or_else(|| ownership_for_type(&target_ty));
                let expr = Self::coerce_for_target(&target_ty, expr);
                Self::check_assignment_safety(context, env, &target_ty, target_ownership, &expr)?;
                Self::check_view_source_lifetime(context, state, name, &expr)?;
                state.assign(name, target_ty, expr.ownership, expr.source_var)?;
            }
            Stmt::AssignTarget { target, expr } => {
                let (target_ty, target_ownership) = Self::target_type(target, env, state)
                    .unwrap_or_else(|| {
                        (
                            IrType::Unknown(format!("target:{target:?}")),
                            Ownership::Shared,
                        )
                    });
                let expr = Self::check_expr(expr, env, state)?;
                let expr = Self::coerce_for_target(&target_ty, expr);
                Self::check_assignment_safety(context, env, &target_ty, target_ownership, &expr)?;
            }
            Stmt::Block(body) => {
                state.push_scope();
                Self::check_stmts(context, body, env, state, return_type, return_ownership.clone())?;
                state.pop_scope();
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                Self::check_expr(condition, env, state)?;
                let base = state.clone();
                let then_state = Self::check_block_snapshot(
                    context,
                    then_body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                let else_state = Self::check_block_snapshot(
                    context,
                    else_body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                *state = Self::merge_branch_states(&base, &[then_state, else_state]);
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                let base = state.clone();
                let try_state = Self::check_block_snapshot(
                    context,
                    try_body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                let try_state = Self::apply_finally_to_state(
                    context,
                    finally_body,
                    env,
                    &try_state,
                    return_type,
                    return_ownership.clone(),
                )?;
                let mut branches = vec![try_state];
                if let Some(catch) = catch {
                    let mut catch_state = base.clone();
                    catch_state.push_scope();
                    if let Some(name) = &catch.name {
                        catch_state.declare(
                            name.clone(),
                            IrType::Exception,
                            Ownership::Borrowed,
                            None,
                        );
                    }
                    Self::check_stmts(
                        context,
                        &catch.body,
                        env,
                        &mut catch_state,
                        return_type,
                        return_ownership.clone(),
                    )?;
                    catch_state.pop_scope();
                    let catch_state = Self::apply_finally_to_state(
                        context,
                        finally_body,
                        env,
                        &catch_state,
                        return_type,
                        return_ownership.clone(),
                    )?;
                    branches.push(catch_state);
                }
                *state = Self::merge_branch_states(&base, &branches);
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                Self::check_expr(expr, env, state)?;
                let base = state.clone();
                let mut branches = Vec::new();
                for case in cases {
                    Self::check_expr(&case.value, env, state)?;
                    branches.push(Self::check_block_snapshot(
                        context,
                        &case.body,
                        env,
                        &base,
                        return_type,
                        return_ownership.clone(),
                    )?);
                }
                branches.push(Self::check_block_snapshot(
                    context,
                    default,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?);
                *state = Self::merge_branch_states(&base, &branches);
            }
            Stmt::While { condition, body } => {
                Self::check_expr(condition, env, state)?;
                let base = state.clone();
                let body_state = Self::check_block_snapshot(
                    context,
                    body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                let mut loop_branches = vec![body_state.clone()];
                loop_branches.extend(
                    body_state
                        .loop_exit_snapshots
                        .iter()
                        .map(|snapshot| OwnershipState::from_snapshot(&snapshot.snapshot)),
                );
                *state = Self::merge_loop_exit_states(&base, &loop_branches);
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                state.push_scope();
                if let Some(init) = init {
                    Self::check_stmt(context, init, env, state, return_type, return_ownership.clone())?;
                }
                if let Some(condition) = condition {
                    Self::check_expr(condition, env, state)?;
                }
                let base = state.clone();
                let body_state = Self::check_block_snapshot(
                    context,
                    body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                let mut loop_branches = Vec::new();
                if !body_state.exits_function && !body_state.exits_loop {
                    let mut steady_state = body_state.clone();
                    if let Some(increment) = increment {
                        Self::check_stmt(
                            context,
                            increment,
                            env,
                            &mut steady_state,
                            return_type,
                            return_ownership.clone(),
                        )?;
                    }
                    loop_branches.push(steady_state);
                }
                for snapshot in &body_state.loop_exit_snapshots {
                    let mut branch_state = OwnershipState::from_snapshot(&snapshot.snapshot);
                    match snapshot.kind {
                        LoopExitKind::Break => loop_branches.push(branch_state),
                        LoopExitKind::Continue => {
                            branch_state.exits_loop = false;
                            if let Some(increment) = increment {
                                Self::check_stmt(
                                    context,
                                    increment,
                                    env,
                                    &mut branch_state,
                                    return_type,
                                    return_ownership.clone(),
                                )?;
                            }
                            loop_branches.push(branch_state);
                        }
                    }
                }
                if loop_branches.is_empty() {
                    loop_branches.push(base.clone());
                }
                *state = Self::merge_loop_exit_states(&base, &loop_branches);
                state.pop_scope();
            }
            Stmt::ForEach {
                item_type,
                item_name,
                collection,
                body,
            } => {
                Self::check_expr(collection, env, state)?;
                let item_ty = type_syntax_to_ir(item_type, env);
                state.push_scope();
                state.declare(
                    item_name.clone(),
                    item_ty.clone(),
                    ownership_for_declared_type_syntax(item_type, env),
                    None,
                );
                let base = state.clone();
                let body_state = Self::check_block_snapshot(
                    context,
                    body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                let mut loop_branches = vec![body_state.clone()];
                loop_branches.extend(
                    body_state
                        .loop_exit_snapshots
                        .iter()
                        .map(|snapshot| OwnershipState::from_snapshot(&snapshot.snapshot)),
                );
                *state = Self::merge_loop_exit_states(&base, &loop_branches);
                state.pop_scope();
            }
            Stmt::Print(expr) | Stmt::Expr(expr) | Stmt::Throw(expr) => {
                Self::check_expr(expr, env, state)?;
            }
            Stmt::Return(Some(expr)) => {
                let expr = Self::check_expr(expr, env, state)?;
                Self::check_assignment_safety(context, env, return_type, return_ownership.clone(), &expr)?;
                if matches!(expr.ownership, Ownership::View | Ownership::Borrowed)
                    && !matches!(return_ownership, Ownership::Borrowed | Ownership::View)
                {
                    if matches!(return_type, IrType::String) && expr.source_var.as_deref() == Some("this")
                    {
                        return Ok(());
                    }
                    return Err(format!(
                        "ownership checker: {context}: cannot return {:?} value into {:?} result without owning clone",
                        expr.ownership, return_ownership
                    ));
                }
                if matches!(expr.ownership, Ownership::View | Ownership::Borrowed) {
                    if let Some(source) = &expr.source_var {
                        let Some(source_var) = state.get(source) else {
                            return Ok(());
                        };
                        if source_var.scope_depth > 0 {
                            return Err(format!(
                                "ownership checker: {context}: returned value from '{source}' would outlive its source"
                            ));
                        }
                    }
                }
                state.exits_function = true;
            }
            Stmt::Return(None) => {
                state.exits_function = true;
            }
            Stmt::Break | Stmt::Continue => {
                let kind = match stmt {
                    Stmt::Break => LoopExitKind::Break,
                    Stmt::Continue => LoopExitKind::Continue,
                    _ => unreachable!(),
                };
                state.loop_exit_snapshots.push(LoopExitSnapshot {
                    kind,
                    snapshot: state.snapshot(),
                });
                state.exits_loop = true;
            }
        }
        Ok(())
    }

    fn check_expr(
        expr: &Expr,
        env: &TypeEnv,
        state: &OwnershipState,
    ) -> Result<CheckedExpr, String> {
        let checked = match expr {
            Expr::Int(_) => checked_temp(IrType::Long),
            Expr::Float(_) => checked_temp(IrType::Double),
            Expr::Bool(_) => checked_temp(IrType::Bool),
            Expr::Null => checked_temp(IrType::Unknown("null".to_string())),
            Expr::String(_) => CheckedExpr {
                ty: IrType::String,
                ownership: Ownership::Owned,
                source_var: None,
                is_move: false,
            },
            Expr::Var(name) => {
                if let Some(var) = state.get(name) {
                    CheckedExpr {
                        ty: var.ty.clone(),
                        ownership: var.ownership.clone(),
                        source_var: Some(name.clone()),
                        is_move: false,
                    }
                } else if let Some(signature) = env.single_function(name) {
                    CheckedExpr {
                        ty: IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        },
                        ownership: Ownership::Copy,
                        source_var: None,
                        is_move: false,
                    }
                } else {
                    CheckedExpr {
                        ty: IrType::Unknown(name.clone()),
                        ownership: Ownership::Shared,
                        source_var: Some(name.clone()),
                        is_move: false,
                    }
                }
            }
            Expr::Move(name) => {
                let Some(var) = state.get(name) else {
                    return Ok(CheckedExpr {
                        ty: IrType::Unknown(name.clone()),
                        ownership: Ownership::Moved,
                        source_var: Some(name.clone()),
                        is_move: true,
                    });
                };
                CheckedExpr {
                    ty: var.ty.clone(),
                    ownership: Ownership::Moved,
                    source_var: Some(name.clone()),
                    is_move: true,
                }
            }
            Expr::ArrayLiteral(values) => {
                for value in values {
                    Self::check_expr(value, env, state)?;
                }
                CheckedExpr {
                    ty: IrType::Array(Box::new(IrType::Long)),
                    ownership: Ownership::Owned,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::NewArray {
                element_type,
                length,
                values,
            } => {
                if let Some(length) = length {
                    Self::check_expr(length, env, state)?;
                }
                for value in values {
                    Self::check_expr(value, env, state)?;
                }
                CheckedExpr {
                    ty: IrType::Array(Box::new(type_syntax_to_ir(element_type, env))),
                    ownership: Ownership::Owned,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::Index { target, index } => {
                let target = Self::check_expr(target, env, state)?;
                Self::check_expr(index, env, state)?;
                let ty = match target.ty {
                    IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => {
                        *inner
                    }
                    IrType::Dictionary(_, value) => *value,
                    _ => IrType::Unknown("index".to_string()),
                };
                let ownership = if ty == IrType::String {
                    Ownership::Borrowed
                } else {
                    ownership_for_type(&ty)
                };
                CheckedExpr {
                    ty,
                    ownership,
                    source_var: target.source_var,
                    is_move: false,
                }
            }
            Expr::Field { target, name } => {
                let target = Self::check_expr(target, env, state)?;
                let field_info = match (&target.ty, name.as_str()) {
                    (IrType::Class(type_name), field) | (IrType::Struct(type_name), field) => {
                        env.resolve_field(type_name, field)
                    }
                    (IrType::Unknown(type_name), field) => env.resolve_field(type_name, field),
                    (IrType::List(_), "Count")
                    | (IrType::Dictionary(_, _), "Count")
                    | (IrType::Array(_), "Length" | "Count") => Some(FieldSignature {
                        package_id: None,
                        visibility: Visibility::Public,
                        ty: IrType::Int,
                        ownership: Ownership::Copy,
                    }),
                    (IrType::Interface(type_name), "Current")
                        if base_type_name(type_name) == "IEnumerator" =>
                    {
                        split_monomorphized_type(type_name)
                            .and_then(|(_, args)| args.first().cloned())
                            .and_then(|arg| parse_monomorphized_ir_type(&arg, env))
                            .map(|ty| FieldSignature {
                                package_id: None,
                                visibility: Visibility::Public,
                                ownership: ownership_for_type(&ty),
                                ty,
                            })
                    }
                    (IrType::Task(inner), "Result") => Some(FieldSignature {
                        package_id: None,
                        visibility: Visibility::Public,
                        ty: inner.as_ref().clone(),
                        ownership: ownership_for_type(inner),
                    }),
                    (IrType::Task(_), "IsCompleted") | (IrType::Task(_), "IsCompletedSuccessfully") => Some(FieldSignature {
                        package_id: None,
                        visibility: Visibility::Public,
                        ty: IrType::Bool,
                        ownership: Ownership::Copy,
                    }),
                    (IrType::Exception, "Message") => Some(FieldSignature {
                        package_id: None,
                        visibility: Visibility::Public,
                        ty: IrType::String,
                        ownership: Ownership::Borrowed,
                    }),
                    _ => None,
                };
                let (ty, ownership) = field_info
                    .map(|field| (field.ty, field.ownership))
                    .unwrap_or_else(|| {
                        let ty = IrType::Unknown(name.clone());
                        let ownership = ownership_for_type(&ty);
                        (ty, ownership)
                    });
                CheckedExpr {
                    ty,
                    ownership,
                    source_var: target.source_var,
                    is_move: false,
                }
            }
            Expr::IsPattern { expr, .. } => {
                Self::check_expr(expr, env, state)?;
                checked_temp(IrType::Bool)
            }
            Expr::Throw(expr) => {
                let inner = Self::check_expr(expr, env, state)?;
                CheckedExpr {
                    ty: inner.ty,
                    ownership: inner.ownership,
                    source_var: inner.source_var,
                    is_move: false,
                }
            }
            Expr::Assign { target, value } => {
                let target = Self::check_expr(target, env, state)?;
                let value = Self::check_expr(value, env, state)?;
                CheckedExpr {
                    ty: value.ty,
                    ownership: value.ownership,
                    source_var: target.source_var,
                    is_move: false,
                }
            }
            Expr::MethodCall { target, name, args } => {
                let target = Self::check_expr(target, env, state)?;
                let args = args
                    .iter()
                    .map(|arg| Self::check_expr(arg, env, state))
                    .collect::<Result<Vec<_>, _>>()?;
                let ty = match (&target.ty, name.as_str()) {
                    (IrType::Class(type_name), "MapGet" | "MapPost")
                        if type_name == "WebApplication" =>
                    {
                        IrType::Void
                    }
                    (IrType::Task(inner), "GetResult") => inner.as_ref().clone(),
                    (IrType::Task(inner), "GetAwaiter") => IrType::Task(inner.clone()),
                    (IrType::Task(_), "IsCompletedSuccessfully") => IrType::Bool,
                    (_, "Contains") | (_, "ContainsKey") | (_, "Remove") | (_, "TryGetValue") => IrType::Bool,
                    (_, "Add") | (_, "Clear") | (_, "Wait") => IrType::Void,
                    (IrType::Unknown(target) | IrType::Class(target), "Run")
                        if target == "Task" =>
                    {
                        args.first()
                            .map(|arg| IrType::Task(Box::new(arg.ty.clone())))
                            .unwrap_or_else(|| IrType::Task(Box::new(IrType::Void)))
                    }
                    _ => IrType::Unknown(name.clone()),
                };
                let ownership = if target.ty == IrType::Exception && matches!(name.as_str(), "Message")
                {
                    Ownership::Borrowed
                } else if let IrType::Class(type_name)
                | IrType::Struct(type_name)
                | IrType::Interface(type_name) = &target.ty
                {
                    env.resolve_method(type_name, name, &args.iter().map(|arg| arg.ty.clone()).collect::<Vec<_>>())
                        .map(|signature| signature.return_ownership.clone())
                        .unwrap_or_else(|| ownership_for_type(&ty))
                } else {
                    ownership_for_type(&ty)
                };
                CheckedExpr {
                    ownership,
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::FunctionCall { name, args } => {
                if name == "sizeof" {
                    return Ok(CheckedExpr {
                        ownership: Ownership::Copy,
                        ty: IrType::Int,
                        source_var: None,
                        is_move: false,
                    });
                }
            let checked_args = args
                .iter()
                .map(|arg| Self::check_expr(arg, env, state))
                .collect::<Result<Vec<_>, _>>()?;
            let signature = env
                .resolve_function(
                name,
                checked_args
                    .iter()
                    .map(|arg| arg.ty.clone())
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
                .cloned()
                .or_else(|| {
                    current_enclosing_type_from_state(state).and_then(|current_type| {
                        env.resolve_method(
                            &current_type,
                            name,
                            &checked_args
                                .iter()
                                .map(|arg| arg.ty.clone())
                                .collect::<Vec<_>>(),
                        )
                    })
                })
            ;
            let (ty, ownership) = signature
                .map(|signature| {
                    (
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                        )
                    })
                    .unwrap_or_else(|| (IrType::Unknown(name.clone()), Ownership::Shared));
                CheckedExpr {
                    ownership,
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::NewObject {
                type_name,
                args,
                fields,
            } => {
                for arg in args {
                    Self::check_expr(arg, env, state)?;
                }
                for field in fields {
                    let expr = Self::check_expr(&field.expr, env, state)?;
                    let target_info = env
                        .resolve_field(type_name, &field.name)
                        .unwrap_or_else(|| FieldSignature {
                            package_id: None,
                            visibility: Visibility::Public,
                            ty: expr.ty.clone(),
                            ownership: ownership_for_type(&expr.ty),
                        });
                    let expr = Self::coerce_for_target(&target_info.ty, expr);
                    Self::check_assignment_safety(
                        &format!("object initializer {type_name}.{}", field.name),
                        env,
                        &target_info.ty,
                        target_info.ownership,
                        &expr,
                    )?;
                }
                let ty = match env.kinds.get(type_name) {
                    Some(TypeKind::Class) => IrType::Class(type_name.clone()),
                    _ if type_name == "Exception" || type_name == "System.Exception" => {
                        IrType::Exception
                    }
                    _ => IrType::Struct(type_name.clone()),
                };
                CheckedExpr {
                    ownership: ownership_for_type(&ty),
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::NewCollection(ty) => CheckedExpr {
                ty: type_syntax_to_ir(ty, env),
                ownership: Ownership::Owned,
                source_var: None,
                is_move: false,
            },
            Expr::NewThread(_) => CheckedExpr {
                ty: IrType::Thread,
                ownership: Ownership::Owned,
                source_var: None,
                is_move: false,
            },
            Expr::Borrow { name, .. } => {
                let ty = state
                    .get(name)
                    .map(|var| var.ty.clone())
                    .unwrap_or_else(|| IrType::Unknown(name.clone()));
                CheckedExpr {
                    ty: IrType::Ref(Box::new(ty)),
                    ownership: Ownership::Borrowed,
                    source_var: Some(name.clone()),
                    is_move: false,
                }
            }
            Expr::Await(inner) => {
                let inner = Self::check_expr(inner, env, state)?;
                let ty = match inner.ty {
                    IrType::Task(result) => *result,
                    other => IrType::Unknown(format!("await {other:?}")),
                };
                CheckedExpr {
                    ownership: ownership_for_type(&ty),
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::Unary { op, expr } => {
                let expr = Self::check_expr(expr, env, state)?;
                let ty = match op {
                    UnaryOp::Not => IrType::Bool,
                    UnaryOp::Neg => expr.ty,
                };
                CheckedExpr {
                    ty,
                    ownership: Ownership::Copy,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::IncDec { name, .. } => {
                let ty = if let Some(var) = state.get(name) {
                    var.ty.clone()
                } else if let Some(this_var) = state.get("this") {
                    let owner = match &this_var.ty {
                        IrType::Class(name) | IrType::Struct(name) => Some(name.as_str()),
                        IrType::Ref(inner) => match inner.as_ref() {
                            IrType::Struct(name) | IrType::Class(name) => Some(name.as_str()),
                            _ => None,
                        },
                        _ => None,
                    };
                    let field_ty = owner.and_then(|o| env.resolve_field(o, name));
                    field_ty
                        .map(|field| field.ty)
                        .unwrap_or_else(|| IrType::Unknown(name.clone()))
                } else {
                    IrType::Unknown(name.clone())
                };
                CheckedExpr {
                    ty,
                    ownership: Ownership::Copy,
                    source_var: Some(name.clone()),
                    is_move: false,
                }
            }
            Expr::Lambda { body, .. } => {
                Self::check_expr(body, env, state)?;
                CheckedExpr {
                    ty: IrType::Unknown("lambda".to_string()),
                    ownership: Ownership::Shared,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                Self::check_expr(condition, env, state)?;
                let when_true = Self::check_expr(when_true, env, state)?;
                let when_false = Self::check_expr(when_false, env, state)?;
                let ty = if when_true.ty == IrType::Unknown("null".to_string()) {
                    when_false.ty
                } else {
                    when_true.ty
                };
                CheckedExpr {
                    ownership: ownership_for_type(&ty),
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::Binary { left, op, right } => {
                let left = Self::check_expr(left, env, state)?;
                Self::check_expr(right, env, state)?;
                let ty = if op.is_comparison() {
                    IrType::Bool
                } else {
                    left.ty
                };
                checked_temp(ty)
            }
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                Self::check_expr(expr, env, state)?
            }
        };
        Ok(checked)
    }

    fn target_type(
        target: &Expr,
        env: &TypeEnv,
        state: &OwnershipState,
    ) -> Option<(IrType, Ownership)> {
        match target {
            Expr::Var(name) => state.get(name).map(|var| (var.ty.clone(), var.ownership.clone())),
            Expr::Index { target, .. } => {
                let target = Self::check_expr(target, env, state).ok()?;
                match target.ty {
                    IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => {
                        let element = inner.as_ref().clone();
                        Some((element.clone(), ownership_for_type(&element)))
                    }
                    IrType::Dictionary(_, value) => {
                        let value = value.as_ref().clone();
                        Some((value.clone(), ownership_for_type(&value)))
                    }
                    IrType::String => Some((IrType::Byte, Ownership::Copy)),
                    _ => None,
                }
            }
            Expr::Field { target, name } => {
                let target = Self::check_expr(target, env, state).ok()?;
                match target.ty {
                    IrType::Class(type_name) | IrType::Struct(type_name) | IrType::Interface(type_name) => env
                        .resolve_field(&type_name, name)
                        .or_else(|| env.resolve_field(base_type_name(&type_name), name))
                        .map(|field| (field.ty, field.ownership)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn check_assignment_safety(
        context: &str,
        env: &TypeEnv,
        target_ty: &IrType,
        target_ownership: Ownership,
        expr: &CheckedExpr,
    ) -> Result<(), String> {
        if matches!(expr.ownership, Ownership::View | Ownership::Borrowed)
            && !matches!(target_ownership, Ownership::Borrowed | Ownership::View)
            && !matches!(
                target_ty,
                IrType::Enumerable(_)
                    | IrType::Ref(_)
                    | IrType::String
                    | IrType::Class(_)
                    | IrType::Interface(_)
            )
        {
            return Err(format!(
                "ownership checker: {context}: cannot store {:?} value into owned target {:?}",
                expr.ownership, target_ty
            ));
        }
        if matches!(target_ty, IrType::Class(_) | IrType::Interface(_))
            && expr.source_var.is_some()
            && !expr.is_move
            && !allows_shared_reference_flow(env, target_ty)
        {
            return Err(format!(
                "ownership checker: {context}: assigning class/interface value requires explicit move"
            ));
        }
        Ok(())
    }

    fn coerce_for_target(target_ty: &IrType, mut expr: CheckedExpr) -> CheckedExpr {
        match target_ty {
            IrType::Enumerable(_) if expr.source_var.is_some() => {
                expr.ty = target_ty.clone();
                expr.ownership = Ownership::View;
                expr.is_move = false;
                expr
            }
            IrType::Ref(_) if expr.source_var.is_some() => {
                expr.ty = target_ty.clone();
                expr.ownership = Ownership::Borrowed;
                expr.is_move = false;
                expr
            }
            _ => expr,
        }
    }

    fn check_view_source_lifetime(
        context: &str,
        state: &OwnershipState,
        target_name: &str,
        expr: &CheckedExpr,
    ) -> Result<(), String> {
        if !matches!(expr.ownership, Ownership::View | Ownership::Borrowed) {
            return Ok(());
        }
        let Some(source) = &expr.source_var else {
            return Ok(());
        };
        let Some(source_var) = state.get(source) else {
            return Ok(());
        };
        let target_depth = state
            .get(target_name)
            .map(|target| target.scope_depth)
            .unwrap_or_else(|| state.depth());
        if source_var.scope_depth > target_depth {
            return Err(format!(
                "ownership checker: {context}: '{target_name}' would outlive borrowed/view source '{source}'"
            ));
        }
        Ok(())
    }
}

fn checked_temp(ty: IrType) -> CheckedExpr {
    CheckedExpr {
        ownership: ownership_for_type(&ty),
        ty,
        source_var: None,
        is_move: false,
    }
}

fn function_signature(function: &Function, env: &TypeEnv, overloaded: bool) -> FunctionSignature {
    let params = function
        .params
        .iter()
        .map(|param| type_syntax_to_ir(&param.ty, env))
        .collect::<Vec<_>>();
    let param_ownerships = function
        .params
        .iter()
        .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
        .collect::<Vec<_>>();
    let params_element_type = params_element_type(&function.params, env);
    let required_params = function
        .params
        .iter()
        .take_while(|param| param.default.is_none())
        .count();
    let base_symbol = if function.generic_params.is_empty() {
        qualified_function_symbol_name(&function.namespace, &function.name)
    } else {
        format!(
            "{}__g{}",
            qualified_function_symbol_name(&function.namespace, &function.name),
            function.generic_params.len()
        )
    };
    FunctionSignature {
        package_id: function.package_id.clone(),
        visibility: function.visibility,
        generic_params: function
            .generic_params
            .iter()
            .map(|param| param.name.clone())
            .collect(),
        symbol: if overloaded {
            overloaded_function_symbol(&base_symbol, &params)
        } else {
            base_symbol
        },
        params,
        param_ownerships,
        required_params,
        params_element_type,
        return_type: type_syntax_to_ir(&function.return_type, env),
        return_ownership: ownership_for_declared_type_syntax(&function.return_type, env),
        is_static: false,
    }
}

fn params_element_type(params: &[Param], env: &TypeEnv) -> Option<IrType> {
    let last = params.last()?;
    if !last.is_params {
        return None;
    }
    match type_syntax_to_ir(&last.ty, env) {
        IrType::Array(inner) => Some(*inner),
        other => Some(other),
    }
}

fn overloaded_function_symbol(name: &str, params: &[IrType]) -> String {
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    if suffix.is_empty() {
        format!("{name}__overload")
    } else {
        format!("{name}__{suffix}")
    }
}

fn qualified_function_symbol_name(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        sanitize_ir_symbol(name)
    } else {
        format!(
            "{}_{}",
            sanitize_ir_symbol(&namespace.join("_")),
            sanitize_ir_symbol(name)
        )
    }
}

fn qualified_type_symbol_name(
    namespace: &[String],
    name: &str,
    generic_arity: usize,
    type_id: usize,
) -> String {
    if namespace.is_empty() {
        format!("{}__g{}__t{}", sanitize_ir_symbol(name), generic_arity, type_id)
    } else {
        format!(
            "{}_{}__g{}__t{}",
            sanitize_ir_symbol(&namespace.join("_")),
            sanitize_ir_symbol(name),
            generic_arity,
            type_id
        )
    }
}

fn type_method_symbol(ty: &TypeDef, type_id: usize, method: &Function, env: &TypeEnv) -> String {
    let overloaded = ty
        .methods
        .iter()
        .filter(|candidate| candidate.name == method.name)
        .count()
        > 1;
    let params = method
        .params
        .iter()
        .map(|param| type_syntax_to_ir(&param.ty, env))
        .collect::<Vec<_>>();
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    let base = format!(
        "{}_{}__g{}",
        qualified_type_symbol_name(
            &ty.namespace,
            &ty.name,
            ty.generic_params.len(),
            type_id,
        ),
        sanitize_ir_symbol(&method.name),
        method.generic_params.len()
    );
    let base = if method.is_extension {
        format!("{base}__ext")
    } else {
        base
    };
    if overloaded {
        if suffix.is_empty() {
            format!("{base}__overload")
        } else {
            format!("{base}__{suffix}")
        }
    } else {
        base
    }
}

fn type_constructor_symbol(
    ty: &TypeDef,
    type_id: usize,
    constructor: &Constructor,
    env: &TypeEnv,
) -> String {
    let overloaded = ty.constructors.len() > 1;
    let params = constructor
        .params
        .iter()
        .map(|param| type_syntax_to_ir(&param.ty, env))
        .collect::<Vec<_>>();
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    let base = format!(
        "{}_ctor",
        qualified_type_symbol_name(
            &ty.namespace,
            &ty.name,
            ty.generic_params.len(),
            type_id,
        )
    );
    if overloaded {
        if suffix.is_empty() {
            format!("{base}__empty")
        } else {
            format!("{base}__{suffix}")
        }
    } else {
        base
    }
}

pub(crate) fn ir_symbol_suffix(ty: &IrType) -> String {
    match ty {
        IrType::Bool => "bool".to_string(),
        IrType::Byte => "byte".to_string(),
        IrType::Short => "short".to_string(),
        IrType::Int => "int".to_string(),
        IrType::Long => "long".to_string(),
        IrType::UInt => "uint".to_string(),
        IrType::Double => "double".to_string(),
        IrType::Decimal => "decimal".to_string(),
        IrType::String => "string".to_string(),
        IrType::Void => "void".to_string(),
        IrType::Class(name)
        | IrType::Struct(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => sanitize_ir_symbol(name),
        IrType::Weak(inner) => format!("weak_{}", ir_symbol_suffix(inner)),
        IrType::Array(inner) => format!("array_{}", ir_symbol_suffix(inner)),
        IrType::Ref(inner) => format!("ref_{}", ir_symbol_suffix(inner)),
        IrType::List(inner) => format!("list_{}", ir_symbol_suffix(inner)),
        IrType::Dictionary(key, value) => {
            format!("dict_{}_{}", ir_symbol_suffix(key), ir_symbol_suffix(value))
        }
        IrType::Enumerable(inner) => format!("ienumerable_{}", ir_symbol_suffix(inner)),
        IrType::Nullable(inner) => format!("nullable_{}", ir_symbol_suffix(inner)),
        IrType::Thread => "thread".to_string(),
        IrType::Task(inner) => format!("task_{}", ir_symbol_suffix(inner)),
        IrType::Function { params, return_type } => {
            let mut parts = params.iter().map(ir_symbol_suffix).collect::<Vec<_>>();
            if !matches!(return_type.as_ref(), IrType::Void) {
                parts.push(format!("ret_{}", ir_symbol_suffix(return_type)));
            }
            if parts.is_empty() {
                "fn".to_string()
            } else {
                format!("fn_{}", parts.join("_"))
            }
        }
        IrType::Exception => "exception".to_string(),
    }
}

fn render_monomorphized_ir_type(ty: &IrType) -> String {
    match ty {
        IrType::Bool => "bool".to_string(),
        IrType::Byte => "byte".to_string(),
        IrType::Short => "short".to_string(),
        IrType::Int => "int".to_string(),
        IrType::Long => "long".to_string(),
        IrType::UInt => "uint".to_string(),
        IrType::Double => "double".to_string(),
        IrType::Decimal => "decimal".to_string(),
        IrType::String => "string".to_string(),
        IrType::Void => "void".to_string(),
        IrType::Array(inner) => format!("{}[]", render_monomorphized_ir_type(inner)),
        IrType::Ref(inner) => format!("ref {}", render_monomorphized_ir_type(inner)),
        IrType::Struct(name) | IrType::Class(name) | IrType::Interface(name) | IrType::Unknown(name) => {
            name.clone()
        }
        IrType::List(inner) => format!("List<{}>", render_monomorphized_ir_type(inner)),
        IrType::Dictionary(key, value) => format!(
            "Dictionary<{},{}>",
            render_monomorphized_ir_type(key),
            render_monomorphized_ir_type(value)
        ),
        IrType::Enumerable(inner) => {
            format!("IEnumerable<{}>", render_monomorphized_ir_type(inner))
        }
        IrType::Thread => "Thread".to_string(),
        IrType::Task(inner) => format!("Task<{}>", render_monomorphized_ir_type(inner)),
        IrType::Nullable(inner) => format!("Nullable<{}>", render_monomorphized_ir_type(inner)),
        IrType::Function { params, return_type } => {
            let mut all = params
                .iter()
                .map(render_monomorphized_ir_type)
                .collect::<Vec<_>>();
            all.push(render_monomorphized_ir_type(return_type));
            format!("Func<{}>", all.join(","))
        }
        IrType::Exception => "Exception".to_string(),
        IrType::Weak(inner) => format!("Weak<{}>", render_monomorphized_ir_type(inner)),
    }
}

fn monomorphized_type_name(name: &str, args: &[IrType]) -> String {
    format!(
        "{}<{}>",
        name,
        args.iter()
            .map(render_monomorphized_ir_type)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn rc_runtime_type_name(inner: &IrType) -> String {
    let rendered = render_monomorphized_ir_type(inner);
    let mut suffix = String::new();
    for ch in rendered.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            suffix.push(ch);
        } else {
            suffix.push('_');
        }
    }
    format!("Rc_{suffix}")
}

fn is_weak_reference_type_name(name: &str) -> bool {
    name.starts_with("Weak_")
        || name.starts_with("System_WeakReference_")
        || name.starts_with("WeakReference_")
        || name.starts_with("Weak<")
        || name.starts_with("WeakReference<")
        || name.starts_with("System.WeakReference<")
}

fn base_type_name(name: &str) -> &str {
    name.split('<')
        .next()
        .unwrap_or(name)
        .rsplit('.')
        .next()
        .unwrap_or(name)
}

fn split_monomorphized_type(text: &str) -> Option<(&str, Vec<String>)> {
    let mut depth = 0usize;
    let mut open = None;
    let mut close = None;
    for (index, ch) in text.char_indices() {
        match ch {
            '<' => {
                if depth == 0 {
                    open = Some(index);
                }
                depth += 1;
            }
            '>' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    close = Some(index);
                    break;
                }
            }
            _ => {}
        }
    }
    let open = open?;
    let close = close?;
    let base = &text[..open];
    let args = &text[open + 1..close];
    let mut parsed_args = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    for ch in args.chars() {
        match ch {
            '<' => {
                depth += 1;
                current.push(ch);
            }
            '>' => {
                depth = depth.saturating_sub(1);
                current.push(ch);
            }
            ',' if depth == 0 => {
                parsed_args.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        parsed_args.push(current.trim().to_string());
    }
    Some((base, parsed_args))
}

fn parse_monomorphized_ir_type(text: &str, env: &TypeEnv) -> Option<IrType> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    match text {
        "bool" => Some(IrType::Bool),
        "byte" => Some(IrType::Byte),
        "short" => Some(IrType::Short),
        "int" => Some(IrType::Int),
        "long" => Some(IrType::Long),
        "uint" => Some(IrType::UInt),
        "double" => Some(IrType::Double),
        "decimal" => Some(IrType::Decimal),
        "string" => Some(IrType::String),
        "void" => Some(IrType::Void),
        "Exception" | "System.Exception" => Some(IrType::Exception),
        _ => {
            if let Some(inner) = text.strip_suffix("[]") {
                return Some(IrType::Array(Box::new(parse_monomorphized_ir_type(inner, env)?)));
            }
            if let Some((base, args)) = split_monomorphized_type(text) {
                let base_name = base_type_name(base);
                return match base_name {
                    "List" => Some(IrType::List(Box::new(parse_monomorphized_ir_type(
                        args.first()?,
                        env,
                    )?))),
                    "Dictionary" => Some(IrType::Dictionary(
                        Box::new(parse_monomorphized_ir_type(args.first()?, env)?),
                        Box::new(parse_monomorphized_ir_type(args.get(1)?, env)?),
                    )),
                    "IEnumerable" | "ICollection" => Some(IrType::Enumerable(Box::new(
                        parse_monomorphized_ir_type(args.first()?, env)?,
                    ))),
                    "Task" | "ValueTask" => Some(IrType::Task(Box::new(
                        parse_monomorphized_ir_type(args.first()?, env)?,
                    ))),
                    "Weak" | "WeakReference" => Some(IrType::Weak(Box::new(
                        parse_monomorphized_ir_type(args.first()?, env)?,
                    ))),
                    "Nullable" => Some(IrType::Nullable(Box::new(
                        parse_monomorphized_ir_type(args.first()?, env)?,
                    ))),
                    _ => {
                        let kind = env.kinds.get(base_name).copied()?;
                        Some(match kind {
                            TypeKind::Class => IrType::Class(text.to_string()),
                            TypeKind::Interface => IrType::Interface(text.to_string()),
                            TypeKind::Enum => IrType::Int,
                            TypeKind::Struct | TypeKind::RefStruct => {
                                IrType::Struct(text.to_string())
                            }
                        })
                    }
                };
            }
            match env.kinds.get(base_type_name(text)).copied() {
                Some(TypeKind::Class) => Some(IrType::Class(text.to_string())),
                Some(TypeKind::Interface) => Some(IrType::Interface(text.to_string())),
                Some(TypeKind::Enum) => Some(IrType::Int),
                Some(TypeKind::Struct | TypeKind::RefStruct) => {
                    Some(IrType::Struct(text.to_string()))
                }
                None => Some(IrType::Unknown(text.to_string())),
            }
        }
    }
}

fn sanitize_ir_symbol(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

fn qualified_delegate_name(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", namespace.join("."), name)
    }
}

fn collect_instantiation(ty: &IrType, output: &mut Vec<GenericInstantiation>) {
    let instantiation = match ty {
        IrType::List(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "List".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Dictionary(key, value) => {
            collect_instantiation(key, output);
            collect_instantiation(value, output);
            Some(GenericInstantiation {
                name: "Dictionary".to_string(),
                args: vec![key.as_ref().clone(), value.as_ref().clone()],
            })
        }
        IrType::Enumerable(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "IEnumerable".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Task(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "Task".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Array(inner) | IrType::Ref(inner) | IrType::Weak(inner) => {
            collect_instantiation(inner, output);
            None
        }
        IrType::Function {
            params,
            return_type,
        } => {
            for param in params {
                collect_instantiation(param, output);
            }
            collect_instantiation(return_type, output);
            None
        }
        _ => None,
    };
    if let Some(instantiation) = instantiation {
        if !output.contains(&instantiation) {
            output.push(instantiation);
        }
    }
}

fn collect_generic_call_instantiations(
    functions: &[TypedFunction],
    types: &[TypedType],
    output: &mut Vec<GenericInstantiation>,
) {
    let mut generic_symbols = HashSet::new();
    for function in functions {
        if !function.generic_params.is_empty() {
            generic_symbols.insert(function.symbol.clone());
        }
    }
    for ty in types {
        for constructor in &ty.constructors {
            if !constructor.generic_params.is_empty() {
                generic_symbols.insert(constructor.symbol.clone());
            }
        }
        for method in &ty.methods {
            if !method.generic_params.is_empty() {
                generic_symbols.insert(method.symbol.clone());
            }
        }
    }

    for function in functions {
        collect_generic_call_instantiations_function(
            function,
            &generic_symbols,
            output,
        );
    }
    for ty in types {
        for constructor in &ty.constructors {
            collect_generic_call_instantiations_function(
                constructor,
                &generic_symbols,
                output,
            );
        }
        for method in &ty.methods {
            collect_generic_call_instantiations_function(
                method,
                &generic_symbols,
                output,
            );
        }
    }
}

fn collect_generic_call_instantiations_function(
    function: &TypedFunction,
    generic_symbols: &HashSet<String>,
    output: &mut Vec<GenericInstantiation>,
) {
    collect_generic_call_instantiations_stmts(
        &function.body,
        generic_symbols,
        output,
    );
}

fn collect_generic_call_instantiations_stmts(
    stmts: &[TypedStmt],
    generic_symbols: &HashSet<String>,
    output: &mut Vec<GenericInstantiation>,
) {
    for stmt in stmts {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                collect_instantiation(&binding.ty, output);
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Assign { expr, .. }
            | TypedStmtKind::Print(expr)
            | TypedStmtKind::Expr(expr)
            | TypedStmtKind::Return(Some(expr))
            | TypedStmtKind::Throw(expr) => {
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::AssignTarget { target, expr } => {
                collect_generic_call_instantiations_expr(
                    target,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Block(body)
            | TypedStmtKind::While { body, .. }
            | TypedStmtKind::For { body, .. }
            | TypedStmtKind::ForEach { body, .. } => {
                collect_generic_call_instantiations_stmts(
                    body,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_generic_call_instantiations_expr(
                    condition,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    then_body,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    else_body,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                collect_generic_call_instantiations_stmts(
                    try_body,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    catch_body,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    finally_body,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Switch { expr, cases, default } => {
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    output,
                );
                for case in cases {
                    collect_generic_call_instantiations_expr(
                        &case.value,
                        generic_symbols,
                        output,
                    );
                    collect_generic_call_instantiations_stmts(
                        &case.body,
                        generic_symbols,
                        output,
                    );
                }
                collect_generic_call_instantiations_stmts(
                    default,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
        }
    }
}

fn collect_generic_call_instantiations_expr(
    expr: &TypedExpr,
    generic_symbols: &HashSet<String>,
    output: &mut Vec<GenericInstantiation>,
) {
    match &expr.kind {
        TypedExprKind::NullableSome(value) => {
            collect_generic_call_instantiations_expr(value, generic_symbols, output);
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                collect_generic_call_instantiations_expr(value, generic_symbols, output);
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_generic_call_instantiations_expr(length, generic_symbols, output);
            }
            for value in values {
                collect_generic_call_instantiations_expr(value, generic_symbols, output);
            }
        }
        TypedExprKind::Index { target, index } => {
            collect_generic_call_instantiations_expr(target, generic_symbols, output);
            collect_generic_call_instantiations_expr(index, generic_symbols, output);
        }
        TypedExprKind::Field { target, .. } => {
            collect_generic_call_instantiations_expr(target, generic_symbols, output);
        }
        TypedExprKind::IsPattern { expr, .. } => {
            collect_generic_call_instantiations_expr(expr, generic_symbols, output);
        }
        TypedExprKind::Throw(expr) | TypedExprKind::Await(expr) => {
            collect_generic_call_instantiations_expr(expr, generic_symbols, output);
        }
        TypedExprKind::Assign { target, value } => {
            collect_generic_call_instantiations_expr(target, generic_symbols, output);
            collect_generic_call_instantiations_expr(value, generic_symbols, output);
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_generic_call_instantiations_expr(condition, generic_symbols, output);
            collect_generic_call_instantiations_expr(when_true, generic_symbols, output);
            collect_generic_call_instantiations_expr(when_false, generic_symbols, output);
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_generic_call_instantiations_expr(left, generic_symbols, output);
            collect_generic_call_instantiations_expr(right, generic_symbols, output);
        }
        TypedExprKind::Unary { expr: inner, .. }
        | TypedExprKind::IncDec { target: inner, .. } => {
            collect_generic_call_instantiations_expr(inner, generic_symbols, output);
        }
        TypedExprKind::Lambda { body, .. } => {
            collect_generic_call_instantiations_expr(body, generic_symbols, output);
        }
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Function { symbol, .. } => {
                    if generic_symbols.contains(symbol) && !call.generic_args.is_empty() {
                        let instantiation = GenericInstantiation {
                            name: symbol.clone(),
                            args: call.generic_args.clone(),
                        };
                        if !output.contains(&instantiation) {
                            output.push(instantiation);
                        }
                    }
                }
                TypedCallKind::Method { target, symbol, resolution, .. } => {
                    collect_generic_call_instantiations_expr(target, generic_symbols, output);
                    if generic_symbols.contains(symbol) && !call.generic_args.is_empty() {
                        let instantiation = GenericInstantiation {
                            name: symbol.clone(),
                            args: call.generic_args.clone(),
                        };
                        if !output.contains(&instantiation) {
                            output.push(instantiation);
                        }
                    }
                    let _ = resolution;
                }
            }
            for arg in &call.args {
                collect_generic_call_instantiations_expr(arg, generic_symbols, output);
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_generic_call_instantiations_expr(arg, generic_symbols, output);
            }
            for field in fields {
                collect_generic_call_instantiations_expr(&field.expr, generic_symbols, output);
            }
        }
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. } => {}
    }
}

fn lower_type(ty: &TypeDef, symbol_id: usize, env: &TypeEnv) -> Result<TypedType, String> {
    let this_type = collection_this_type(ty);
    let fields = env
        .all_field_infos(&ty.name)
        .into_iter()
        .map(|(name, field)| TypedBinding {
            name,
            ownership: field.ownership,
            ty: field.ty,
        })
        .collect::<Vec<_>>();
    let this_binding = TypedBinding {
        name: "this".to_string(),
        ty: this_type,
        ownership: Ownership::Borrowed,
    };
    let constructors = ty
        .constructors
        .iter()
        .map(|constructor| {
            let function = Function {
                package_id: ty.package_id.clone(),
                visibility: constructor.visibility,
                namespace: constructor.namespace.clone(),
                attributes: constructor.attributes.clone(),
                is_async: false,
                is_extern: false,
                is_static: false,
                is_extension: false,
                name: ty.name.clone(),
                generic_params: Vec::new(),
                params: constructor.params.clone(),
                return_type: TypeSyntax::Void,
                body: constructor.body.clone(),
            };
            lower_function(
                &function,
                env,
                std::slice::from_ref(&this_binding),
                Some(ty.name.clone()),
                Some(type_constructor_symbol(ty, symbol_id, constructor, env)),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let methods = ty
        .methods
        .iter()
        .map(|method| {
            let implicit_params: &[TypedBinding] = if method.is_extension || method.is_static {
                &[]
            } else {
                std::slice::from_ref(&this_binding)
            };
            lower_function(
                method,
                env,
                implicit_params,
                Some(ty.name.clone()),
                Some(type_method_symbol(ty, symbol_id, method, env)),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TypedType {
        package_id: ty.package_id.clone(),
        visibility: ty.visibility,
        name: ty.name.clone(),
        namespace: ty.namespace.clone(),
        generic_params: ty
            .generic_params
            .iter()
            .map(|param| param.name.clone())
            .collect(),
        symbol_id,
        is_extension: ty.is_extension,
        kind: ty.kind,
        bases: ty.bases.clone(),
        fields,
        constructors,
        methods,
    })
}

fn lower_function(
    function: &Function,
    env: &TypeEnv,
    implicit_params: &[TypedBinding],
    current_type: Option<String>,
    symbol_override: Option<String>,
) -> Result<TypedFunction, String> {
    let return_type = type_syntax_to_ir(&function.return_type, env);
    let mut scopes = vec![HashMap::new()];
    let mut params = Vec::new();
    for param in implicit_params {
        scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), param.clone());
        params.push(param.clone());
    }
    if let Some(current_type) = &current_type {
        scopes
            .last_mut()
            .unwrap()
            .insert(
                "__glitching_current_type".to_string(),
                TypedBinding {
                    name: "__glitching_current_type".to_string(),
                    ty: IrType::Class(current_type.clone()),
                    ownership: Ownership::Shared,
                },
            );
    }
    for param in &function.params {
        let ty = type_syntax_to_ir(&param.ty, env);
        let binding = TypedBinding {
            name: param.name.clone(),
            ownership: ownership_for_declared_type_syntax(&param.ty, env),
            ty,
        };
        scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), binding.clone());
        params.push(binding);
    }
    let mut locals = Vec::new();
    lower_stmts(&function.body, env, &mut scopes, &mut locals)?;
    let mut typed_scopes = vec![HashMap::new()];
    for param in &params {
        typed_scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), param.clone());
    }
    if let Some(current_type) = current_type {
        typed_scopes
            .last_mut()
            .unwrap()
            .insert(
                "__glitching_current_type".to_string(),
                TypedBinding {
                    name: "__glitching_current_type".to_string(),
                    ty: IrType::Class(current_type),
                    ownership: Ownership::Shared,
                },
            );
    }
    let body = lower_typed_stmts(&function.body, env, &mut typed_scopes)?;
    Ok(TypedFunction {
        package_id: function.package_id.clone(),
        visibility: function.visibility,
        name: function.name.clone(),
        symbol: symbol_override.unwrap_or_else(|| {
            env.resolve_function(
                &function.name,
                &function
                    .params
                    .iter()
                    .map(|param| type_syntax_to_ir(&param.ty, env))
                    .collect::<Vec<_>>(),
            )
            .map(|signature| signature.symbol.clone())
            .unwrap_or_else(|| function.name.clone())
        }),
        is_async: function.is_async,
        generic_params: function
            .generic_params
            .iter()
            .map(|param| param.name.clone())
            .collect(),
        is_extern: function.is_extern,
        required_params: function
            .params
            .iter()
            .take_while(|param| param.default.is_none())
            .count(),
        return_ownership: ownership_for_declared_type_syntax(&function.return_type, env),
        return_type,
        params,
        locals,
        body,
    })
}

fn collection_this_type(ty: &TypeDef) -> IrType {
    match ty.name.as_str() {
        "List" | "System.Collections.Generic.List" => {
            let item = ty
                .generic_params
                .first()
                .map(|param| IrType::Unknown(param.name.clone()))
                .unwrap_or_else(|| IrType::Unknown("T".to_string()));
            IrType::List(Box::new(item))
        }
        "Dictionary" | "System.Collections.Generic.Dictionary" => {
            let key = ty
                .generic_params
                .first()
                .map(|param| IrType::Unknown(param.name.clone()))
                .unwrap_or_else(|| IrType::Unknown("TKey".to_string()));
            let value = ty
                .generic_params
                .get(1)
                .map(|param| IrType::Unknown(param.name.clone()))
                .unwrap_or_else(|| IrType::Unknown("TValue".to_string()));
            IrType::Dictionary(Box::new(key), Box::new(value))
        }
        "IEnumerable" | "System.Collections.Generic.IEnumerable" => {
            let item = ty
                .generic_params
                .first()
                .map(|param| IrType::Unknown(param.name.clone()))
                .unwrap_or_else(|| IrType::Unknown("T".to_string()));
            IrType::Enumerable(Box::new(item))
        }
        _ => match ty.kind {
            TypeKind::Class => {
                if ty.generic_params.is_empty() {
                    IrType::Class(ty.name.clone())
                } else {
                    IrType::Class(monomorphized_type_name(
                        &ty.name,
                        &ty.generic_params
                            .iter()
                            .map(|param| IrType::Unknown(param.name.clone()))
                            .collect::<Vec<_>>(),
                    ))
                }
            }
            TypeKind::Interface => {
                if ty.generic_params.is_empty() {
                    IrType::Interface(ty.name.clone())
                } else {
                    IrType::Interface(monomorphized_type_name(
                        &ty.name,
                        &ty.generic_params
                            .iter()
                            .map(|param| IrType::Unknown(param.name.clone()))
                            .collect::<Vec<_>>(),
                    ))
                }
            }
            _ => {
                let name = if ty.generic_params.is_empty() {
                    ty.name.clone()
                } else {
                    monomorphized_type_name(
                        &ty.name,
                        &ty.generic_params
                            .iter()
                            .map(|param| IrType::Unknown(param.name.clone()))
                            .collect::<Vec<_>>(),
                    )
                };
                IrType::Ref(Box::new(IrType::Struct(name)))
            }
        },
    }
}

fn find_expected_types(candidates: &[FunctionSignature], args: &[Expr]) -> Vec<Option<IrType>> {
    let mut expected = vec![None; args.len()];
    for sig in candidates {
        if let Some(element_ty) = &sig.params_element_type {
            let fixed_len = sig.params.len().saturating_sub(1);
            if args.len() >= fixed_len {
                for (i, param_ty) in sig.params.iter().take(fixed_len).enumerate() {
                    if expected[i].is_none() {
                        expected[i] = Some(param_ty.clone());
                    }
                }
                for i in fixed_len..args.len() {
                    if expected[i].is_none() {
                        expected[i] = Some(element_ty.clone());
                    }
                }
            }
        } else if sig.params.len() == args.len() {
            for (i, param_ty) in sig.params.iter().enumerate() {
                if expected[i].is_none() {
                    expected[i] = Some(param_ty.clone());
                }
            }
        }
    }
    expected
}

fn lower_lambda(
    params: &[String],
    body: &Expr,
    expected_type: Option<&IrType>,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedExpr, String> {
    let mut captures = Vec::new();
    collect_lambda_captures(body, params, &mut captures);
    captures.sort();
    captures.dedup();
    for capture in &captures {
        if let Some(binding) = lookup(scopes, capture) {
            if matches!(binding.ownership, Ownership::Borrowed | Ownership::View) {
                return Err(format!(
                    "ownership checker: lambda capture '{capture}' may outlive borrowed/view source; move the value into the closure or use an owned copy"
                ));
            }
        }
    }
    let param_types: Vec<IrType> = if let Some(IrType::Function {
        params: expected_params,
        ..
    }) = expected_type
    {
        expected_params.iter().cloned().collect()
    } else {
        vec![IrType::Unknown("lambda_param".to_string()); params.len()]
    };
    let mut new_scopes = scopes.to_vec();
    let mut lambda_scope = HashMap::new();
    for (name, ty) in params.iter().zip(param_types.iter()) {
        lambda_scope.insert(
            name.clone(),
            TypedBinding {
                name: name.clone(),
                ty: ty.clone(),
                ownership: ownership_for_type(ty),
            },
        );
    }
    new_scopes.push(lambda_scope);
    let typed_body = lower_typed_expr(&body, env, &new_scopes)?;
    let return_type = typed_body.ty.clone();
    let lambda_ty = IrType::Function {
        params: param_types,
        return_type: Box::new(return_type),
    };
    Ok(typed_expr_with_ownership(
        TypedExprKind::Lambda {
            params: params.to_vec(),
            body: Box::new(typed_body),
        },
        lambda_ty,
        Ownership::Shared,
    ))
}

fn collect_lambda_captures(expr: &Expr, params: &[String], out: &mut Vec<String>) {
    match expr {
        Expr::Var(name) | Expr::Move(name) | Expr::Borrow { name, .. } => {
            if !params.contains(name) {
                out.push(name.clone());
            }
        }
        Expr::ArrayLiteral(values) => {
            for value in values {
                collect_lambda_captures(value, params, out);
            }
        }
        Expr::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_lambda_captures(length, params, out);
            }
            for value in values {
                collect_lambda_captures(value, params, out);
            }
        }
        Expr::Index { target, index } => {
            collect_lambda_captures(target, params, out);
            collect_lambda_captures(index, params, out);
        }
        Expr::Field { target, .. } => collect_lambda_captures(target, params, out),
        Expr::IsPattern { expr, .. } | Expr::Await(expr) | Expr::Unary { expr, .. } => {
            collect_lambda_captures(expr, params, out)
        }
        Expr::Throw(expr) => collect_lambda_captures(expr, params, out),
        Expr::Assign { target, value } => {
            collect_lambda_captures(target, params, out);
            collect_lambda_captures(value, params, out);
        }
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_lambda_captures(condition, params, out);
            collect_lambda_captures(when_true, params, out);
            collect_lambda_captures(when_false, params, out);
        }
        Expr::Binary { left, right, .. } => {
            collect_lambda_captures(left, params, out);
            collect_lambda_captures(right, params, out);
        }
        Expr::MethodCall { target, args, .. } => {
            collect_lambda_captures(target, params, out);
            for arg in args {
                collect_lambda_captures(arg, params, out);
            }
        }
        Expr::IncDec { name, .. } => {
            if !params.contains(name) {
                out.push(name.clone());
            }
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_lambda_captures(arg, params, out);
            }
        }
        Expr::NewObject { args, fields, .. } => {
            for arg in args {
                collect_lambda_captures(arg, params, out);
            }
            for field in fields {
                collect_lambda_captures(&field.expr, params, out);
            }
        }
        Expr::Lambda { params: inner_params, body } => {
            let mut merged = params.to_vec();
            merged.extend(inner_params.iter().cloned());
            collect_lambda_captures(body, &merged, out);
        }
        Expr::Int(_)
        | Expr::Float(_)
        | Expr::Bool(_)
        | Expr::Null
        | Expr::String(_)
        | Expr::NewCollection(_)
        | Expr::NewThread(_)
        => {}
        Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
            collect_lambda_captures(expr, params, out);
        }
    }
}

fn lower_call_args(
    args: &[Expr],
    expected_types: &[Option<IrType>],
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<Vec<TypedExpr>, String> {
    let mut lowered = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        let expected = expected_types.get(i).and_then(|x| x.as_ref());
        let typed_arg = match arg {
            Expr::Lambda { params, body } => lower_lambda(params, body, expected, env, scopes)?,
            Expr::NamedArg { name: _, expr } => {
                let inner = match expr.as_ref() {
                    Expr::Lambda { params, body } => {
                        lower_lambda(params, body, expected, env, scopes)?
                    }
                    _ => lower_typed_expr(expr, env, scopes)?,
                };
                inner
            }
            _ => lower_typed_expr(arg, env, scopes)?,
        };
        lowered.push(typed_arg);
    }
    Ok(lowered)
}

fn pack_params_args(
    signature: &FunctionSignature,
    args: Vec<TypedExpr>,
) -> Vec<TypedExpr> {
    let Some(element_ty) = &signature.params_element_type else {
        return args;
    };
    let fixed_len = signature.params.len().saturating_sub(1);
    if args.len() == signature.params.len()
        && signature
            .params
            .last()
            .is_some_and(|expected| ir_arg_matches(expected, args.last().map(|arg| &arg.ty).unwrap()))
    {
        return args;
    }
    if args.len() < fixed_len {
        return args;
    }
    let mut packed = args[..fixed_len].to_vec();
    let tail = args[fixed_len..].to_vec();
    let array_ty = IrType::Array(Box::new(element_ty.clone()));
    packed.push(typed_expr_with_ownership(
        TypedExprKind::NewArray {
            element_type: element_ty.clone(),
            length: None,
            values: tail,
        },
        array_ty,
        Ownership::Owned,
    ));
    packed
}

fn lower_typed_expr(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedExpr, String> {
    lower_typed_expr_with_expected(expr, env, scopes, None)
}

fn lower_typed_expr_with_expected(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
    expected: Option<&IrType>,
) -> Result<TypedExpr, String> {
    let typed = match expr {
        Expr::Int(value) => typed_expr(TypedExprKind::Int(*value), IrType::Long),
        Expr::Float(value) => typed_expr(TypedExprKind::Float(*value), IrType::Double),
        Expr::Bool(value) => typed_expr(TypedExprKind::Bool(*value), IrType::Bool),
        Expr::Null => typed_expr(TypedExprKind::Null, IrType::Unknown("null".to_string())),
        Expr::String(value) => typed_expr_with_ownership(
            TypedExprKind::String(value.clone()),
            IrType::String,
            Ownership::Owned,
        ),
        Expr::Var(name) => {
            if let Some(binding) = lookup(scopes, name) {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    binding.ty,
                    binding.ownership,
                )
            } else if let Some((this_type, field_type, field_ownership)) =
                implicit_field(env, scopes, name)
            {
                let this_expr = typed_expr_with_ownership(
                    TypedExprKind::Var("this".to_string()),
                    this_type,
                    Ownership::Borrowed,
                );
                typed_expr_with_ownership(
                    TypedExprKind::Field {
                        target: Box::new(this_expr),
                        name: name.clone(),
                    },
                    field_type.clone(),
                    field_ownership,
                )
            } else if let Some(current_type) = current_enclosing_type(scopes) {
                if let Some(signature) = env.resolve_method_call(&current_type, name, &[])?
                {
                    if signature.is_static {
                        return Ok(typed_expr_with_ownership(
                            TypedExprKind::Call(TypedCall {
                                kind: TypedCallKind::Function {
                                    name: name.clone(),
                                    symbol: signature.symbol.clone(),
                                },
                                args: Vec::new(),
                                generic_args: Vec::new(),
                            }),
                            signature.return_type.clone(),
                            signature.return_ownership.clone(),
                        ));
                    }
                }
                if let Some(signature) = env.resolve_method(&current_type, &format!("get_{name}"), &[]) {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Call(TypedCall {
                            kind: TypedCallKind::Function {
                                name: format!("get_{name}"),
                                symbol: signature.symbol.clone(),
                            },
                            args: Vec::new(),
                            generic_args: Vec::new(),
                        }),
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                    ));
                }
                if let Some(initializer) =
                    env.static_fields.get(&(current_type.clone(), name.clone()))
                {
                    return lower_typed_expr_with_expected(initializer, env, scopes, expected);
                }
                if let Some(kind) = env.kinds.get(name) {
                    let ty = match kind {
                        TypeKind::Class => IrType::Class(name.clone()),
                        TypeKind::Interface => IrType::Interface(name.clone()),
                        TypeKind::Enum => IrType::Int,
                        _ => IrType::Struct(name.clone()),
                    };
                    typed_expr_with_ownership(TypedExprKind::Var(name.clone()), ty, Ownership::Shared)
                } else if name == "Exception" || name == "System.Exception" {
                    typed_expr_with_ownership(
                        TypedExprKind::Var(name.clone()),
                        IrType::Exception,
                        Ownership::Shared,
                    )
                } else if let Some(signature) = env.single_function(name) {
                    typed_expr_with_ownership(
                        TypedExprKind::FunctionSymbol(name.clone()),
                        IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        },
                        Ownership::Copy,
                    )
                } else {
                    typed_expr_with_ownership(
                        TypedExprKind::Var(name.clone()),
                        IrType::Unknown(name.clone()),
                        Ownership::Shared,
                    )
                }
            } else if let Some(kind) = env.kinds.get(name) {
                let ty = match kind {
                    TypeKind::Class => IrType::Class(name.clone()),
                    TypeKind::Interface => IrType::Interface(name.clone()),
                    TypeKind::Enum => IrType::Int,
                    _ => IrType::Struct(name.clone()),
                };
                typed_expr_with_ownership(TypedExprKind::Var(name.clone()), ty, Ownership::Shared)
            } else if name == "Exception" || name == "System.Exception" {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    IrType::Exception,
                    Ownership::Shared,
                )
            } else if let Some(signature) = env.single_function(name) {
                typed_expr_with_ownership(
                    TypedExprKind::FunctionSymbol(name.clone()),
                    IrType::Function {
                        params: signature.params.clone(),
                        return_type: Box::new(signature.return_type.clone()),
                    },
                    Ownership::Copy,
                )
            } else {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    IrType::Unknown(name.clone()),
                    Ownership::Shared,
                )
            }
        }
        Expr::Move(name) => {
            let ty = lookup(scopes, name)
                .map(|binding| binding.ty)
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            typed_expr_with_ownership(TypedExprKind::Move(name.clone()), ty, Ownership::Moved)
        }
        Expr::ArrayLiteral(values) => {
            let values = values
                .iter()
                .map(|value| lower_typed_expr(value, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            typed_expr_with_ownership(
                TypedExprKind::ArrayLiteral(values),
                IrType::Array(Box::new(IrType::Long)),
                Ownership::Owned,
            )
        }
        Expr::NewArray {
            element_type,
            length,
            values,
        } => {
            let element_type = type_syntax_to_ir(element_type, env);
            let length = length
                .as_ref()
                .map(|length| lower_typed_expr(length, env, scopes))
                .transpose()?
                .map(Box::new);
            let values = values
                .iter()
                .map(|value| lower_typed_expr(value, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            typed_expr_with_ownership(
                TypedExprKind::NewArray {
                    element_type: element_type.clone(),
                    length,
                    values,
                },
                IrType::Array(Box::new(element_type)),
                Ownership::Owned,
            )
        }
        Expr::Index { target, index } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let index = lower_typed_expr(index, env, scopes)?;
            let ty = match &target.ty {
                IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => {
                    inner.as_ref().clone()
                }
                IrType::Dictionary(_, value) => value.as_ref().clone(),
                IrType::String => IrType::Byte,
                _ => IrType::Unknown("index".to_string()),
            };
            let ownership = if ty == IrType::String {
                Ownership::Borrowed
            } else {
                ownership_for_type(&ty)
            };
            typed_expr_with_ownership(
                TypedExprKind::Index {
                    target: Box::new(target),
                    index: Box::new(index),
                },
                ty,
                ownership,
            )
        }
        Expr::Field { target, name } => {
            let target = lower_typed_expr(target, env, scopes)?;
            if let TypedExprKind::Var(enum_name) = &target.kind {
                if let Some(value) = env.enum_values.get(&(enum_name.clone(), name.clone())) {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Int(*value),
                        IrType::Int,
                        Ownership::Copy,
                    ));
                }
            }
            if let IrType::Interface(type_name) = &target.ty {
                if base_type_name(type_name) == "IEnumerator" && name == "Current" {
                    if let Some(item_ty) = split_monomorphized_type(type_name)
                        .and_then(|(_, args)| args.first().cloned())
                        .and_then(|arg| parse_monomorphized_ir_type(&arg, env))
                    {
                        return Ok(typed_expr_with_ownership(
                            TypedExprKind::Field {
                                target: Box::new(target),
                                name: name.clone(),
                            },
                            item_ty.clone(),
                            ownership_for_type(&item_ty),
                        ));
                    }
                }
            }
            if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                let getter_name = format!("get_{name}");
                if let Some(signature) = env.resolve_method(type_name, &getter_name, &[]) {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Call(TypedCall {
                            kind: TypedCallKind::Method {
                                target: Box::new(target),
                                name: getter_name,
                                symbol: signature.symbol.clone(),
                                resolution: CallResolution::InstanceMethod,
                            },
                            args: Vec::new(),
                            generic_args: Vec::new(),
                        }),
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                    ));
                }
            }
            let field_info = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), field) | (IrType::Struct(type_name), field) => {
                    env.resolve_field(type_name, field)
                }
                (IrType::Unknown(type_name), field) => env.resolve_field(type_name, field),
                (IrType::List(_), "Count")
                | (IrType::Dictionary(_, _), "Count")
                | (IrType::Array(_), "Length" | "Count") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Interface(type_name), "Current")
                    if base_type_name(type_name) == "IEnumerator" =>
                {
                    split_monomorphized_type(type_name)
                        .and_then(|(_, args)| args.first().cloned())
                        .and_then(|arg| parse_monomorphized_ir_type(&arg, env))
                        .map(|ty| FieldSignature {
                            package_id: None,
                            visibility: Visibility::Public,
                            ownership: ownership_for_type(&ty),
                            ty,
                        })
                }
                (IrType::String, "Length") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Task(inner), "Result") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: inner.as_ref().clone(),
                    ownership: ownership_for_type(inner),
                }),
                (IrType::Task(_), "IsCompleted") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Bool,
                    ownership: Ownership::Copy,
                }),
                (IrType::Exception, "Message") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::String,
                    ownership: Ownership::Borrowed,
                }),
                (IrType::Weak(inner), "Target") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: inner.as_ref().clone(),
                    ownership: Ownership::Borrowed,
                }),
                _ => None,
            };
            let (ty, ownership) = field_info
                .map(|field| (field.ty, field.ownership))
                .unwrap_or_else(|| {
                    let ty = IrType::Unknown(name.clone());
                    let ownership = ownership_for_type(&ty);
                    (ty, ownership)
                });
            typed_expr_with_ownership(
                TypedExprKind::Field {
                    target: Box::new(target),
                    name: name.clone(),
                },
                ty,
                ownership,
            )
        }
        Expr::IsPattern { expr, ty, name } => {
            let expr = lower_typed_expr(expr, env, scopes)?;
            let pattern_type = type_syntax_to_ir(ty, env);
            let binding = name.as_ref().map(|name| TypedBinding {
                name: name.clone(),
                ty: pattern_type,
                ownership: Ownership::Borrowed,
            });
            typed_expr(
                TypedExprKind::IsPattern {
                    expr: Box::new(expr),
                    binding,
                },
                IrType::Bool,
            )
        }
        Expr::MethodCall { target, name, args } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let mut candidates = Vec::new();
            if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                if let Some(sigs) = env.methods.get(&(type_name.clone(), name.clone())) {
                    candidates = sigs.clone();
                }
            }
            if candidates.is_empty() {
                if let Some(sigs) = env.functions.get(name) {
                    candidates = sigs.clone();
                }
            }
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            let (ty, _ownership, symbol, resolution) = resolve_method_call(env, &target, name, &args)?;
            let resolved_signature = candidates
                .iter()
                .find(|signature| signature.symbol == symbol)
                .cloned()
                .or_else(|| match &target.ty {
                    IrType::Class(type_name)
                    | IrType::Struct(type_name)
                    | IrType::Interface(type_name)
                    | IrType::Unknown(type_name) => env
                        .resolve_method_call(type_name, name, &args)
                        .ok()
                        .flatten(),
                    IrType::String => ["string", "String", "System.String"]
                        .iter()
                        .find_map(|string_type| env.resolve_method_call(string_type, name, &args).ok().flatten()),
                    _ => None,
                })
                .or_else(|| env.resolve_function_call(name, &args).ok().flatten().cloned());
            let generic_args = resolved_signature
                .as_ref()
                .map(|signature| {
                    infer_generic_args_from_signature_with_expected(signature, &args, expected)
                })
                .unwrap_or_default();
            let ty = resolved_signature
                .as_ref()
                .map(|signature| {
                    substitute_generic_args_in_ir_type(
                        &signature.return_type,
                        &signature.generic_params,
                        &generic_args,
                    )
                })
                .unwrap_or(ty);
            let ownership = ownership_for_type(&ty);
            let args = if let Some(signature) = resolved_signature.as_ref() {
                pack_params_args(signature, args)
            } else {
                args
            };
            typed_expr_with_ownership(
                TypedExprKind::Call(TypedCall {
                    kind: TypedCallKind::Method {
                        target: Box::new(target),
                        name: name.clone(),
                        symbol,
                        resolution,
                    },
                    args,
                    generic_args,
                }),
                ty,
                ownership,
            )
        }
        Expr::FunctionCall { name, args } => {
            if name == "sizeof" {
                let type_name = if let Some(Expr::Var(tn)) = args.first() {
                    tn.clone()
                } else {
                    "int".to_string()
                };
                return Ok(typed_expr(
                    TypedExprKind::Call(TypedCall {
                        kind: TypedCallKind::Function {
                            name: name.clone(),
                            symbol: name.clone(),
                        },
                        args: vec![typed_expr(
                            TypedExprKind::Var(type_name.clone()),
                            IrType::Unknown(type_name),
                        )],
                        generic_args: Vec::new(),
                    }),
                    IrType::Int,
                ));
            }
            let mut candidates = Vec::new();
            if let Some(sigs) = env.functions.get(name) {
                candidates = sigs.clone();
            }
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            let current_type = current_enclosing_type(scopes);
            let method_signature = current_type.as_ref().and_then(|current_type| {
                env.resolve_method_call(current_type, name, &args).ok().flatten()
            });
            let function_signature = if method_signature.is_none() {
                env.resolve_function_call(name, &args)?.cloned()
            } else {
                None
            };
            let signature = method_signature.as_ref().or(function_signature.as_ref());
            let generic_args = signature
                .map(|signature| {
                    infer_generic_args_from_signature_with_expected(signature, &args, expected)
                })
                .unwrap_or_default();
            let args = if let Some(signature) = signature {
                pack_params_args(signature, args)
            } else {
                args
            };
            let ty = signature
                .map(|signature| {
                    substitute_generic_args_in_ir_type(
                        &signature.return_type,
                        &signature.generic_params,
                        &generic_args,
                    )
                })
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            let ownership = ownership_for_type(&ty);
            typed_expr_with_ownership(
                TypedExprKind::Call(TypedCall {
                    kind: if let Some(signature) = method_signature {
                        if signature.is_static {
                            TypedCallKind::Function {
                                name: name.clone(),
                                symbol: signature.symbol.clone(),
                            }
                        } else {
                            let current_this = lookup(scopes, "this");
                            TypedCallKind::Method {
                                target: Box::new(typed_expr(
                                    TypedExprKind::Var("this".to_string()),
                                    current_this
                                        .as_ref()
                                        .map(|binding| binding.ty.clone())
                                        .unwrap_or_else(|| IrType::Unknown("this".to_string())),
                                )),
                                name: name.clone(),
                                symbol: signature.symbol.clone(),
                                resolution: CallResolution::InstanceMethod,
                            }
                        }
                    } else {
                        TypedCallKind::Function {
                            name: name.clone(),
                            symbol: function_signature
                                .as_ref()
                                .map(|signature| signature.symbol.clone())
                                .unwrap_or_else(|| name.clone()),
                        }
                    },
                    args,
                    generic_args,
                }),
                ty,
                ownership,
            )
        }
        Expr::NewObject {
            type_name,
            args,
            fields,
        } => {
            let mut candidates = Vec::new();
            if let Some(sigs) = env.constructors.get(type_name) {
                candidates = sigs.clone();
            }
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            if let Some((base, generic_args)) = split_monomorphized_type(type_name) {
                if base_type_name(base) == "Rc" {
                    let inner_ty = generic_args
                        .first()
                        .and_then(|arg| parse_monomorphized_ir_type(arg, env))
                        .unwrap_or(IrType::Unknown("T".to_string()));
                    let runtime_name = rc_runtime_type_name(&inner_ty);
                    let ty = IrType::Struct(runtime_name.clone());
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::NewObject {
                            type_name: runtime_name,
                            constructor: None,
                            args,
                            fields: Vec::new(),
                        },
                        ty.clone(),
                        ownership_for_type(&ty),
                    ));
                }
            }
            if is_weak_reference_type_name(type_name) {
                let inner_ty = if let Some(arg) = args.first() {
                    arg.ty.clone()
                } else {
                    IrType::Unknown("T".to_string())
                };
                let ty = IrType::Weak(Box::new(inner_ty));
                return Ok(typed_expr_with_ownership(
                    TypedExprKind::NewObject {
                        type_name: type_name.clone(),
                        constructor: None,
                        args,
                        fields: Vec::new(),
                    },
                    ty.clone(),
                    Ownership::Copy,
                ));
            }
            let constructor_signature = env.resolve_constructor_call(type_name, &args)?;
            let args = if let Some(signature) = constructor_signature.as_ref() {
                pack_params_args(signature, args)
            } else {
                args
            };
            let constructor = constructor_signature.map(|signature| signature.symbol.clone());
            let fields = fields
                .iter()
                .map(|field| {
                    Ok(TypedFieldInit {
                        name: field.name.clone(),
                        expr: lower_typed_expr(&field.expr, env, scopes)?,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let fields = if type_name == "Type" {
                augment_type_metadata_fields(fields, env)?
            } else {
                fields
            };
            let ty = if type_name == "__anonymous" {
                IrType::Unknown("anonymous".to_string())
            } else {
                match env.kinds.get(type_name) {
                    Some(TypeKind::Class) => IrType::Class(type_name.clone()),
                    Some(TypeKind::Interface) => IrType::Interface(type_name.clone()),
                    _ if type_name == "Exception" || type_name == "System.Exception" => {
                        IrType::Exception
                    }
                    _ => IrType::Struct(type_name.clone()),
                }
            };
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: type_name.clone(),
                    constructor,
                    args,
                    fields,
                },
                ty.clone(),
                ownership_for_type(&ty),
            )
        }
        Expr::NewCollection(ty) => {
            let ty = type_syntax_to_ir(ty, env);
            typed_expr_with_ownership(
                TypedExprKind::NewCollection(ty.clone()),
                ty,
                Ownership::Owned,
            )
        }
        Expr::NewThread(entry) => typed_expr_with_ownership(
            TypedExprKind::NewThread(entry.clone()),
            IrType::Thread,
            Ownership::Owned,
        ),
        Expr::Borrow { mutable, name } => {
            let target = lookup(scopes, name)
                .map(|binding| binding.ty)
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            typed_expr_with_ownership(
                TypedExprKind::Borrow {
                    mutable: *mutable,
                    name: name.clone(),
                },
                IrType::Ref(Box::new(target)),
                Ownership::Borrowed,
            )
        }
        Expr::Await(inner) => {
            let inner = lower_typed_expr(inner, env, scopes)?;
            let ty = match &inner.ty {
                IrType::Task(result) => result.as_ref().clone(),
                other => IrType::Unknown(format!("await {other:?}")),
            };
            typed_expr(TypedExprKind::Await(Box::new(inner)), ty)
        }
        Expr::Throw(expr) => {
            let inner = lower_typed_expr_with_expected(expr, env, scopes, expected)?;
            let ty = expected.cloned().unwrap_or_else(|| inner.ty.clone());
            typed_expr_with_ownership(
                TypedExprKind::Throw(Box::new(inner)),
                ty,
                Ownership::Copy,
            )
        }
        Expr::Assign { target, value } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let value = lower_typed_expr_with_expected(value, env, scopes, Some(&target.ty))?;
            typed_expr_with_ownership(
                TypedExprKind::Assign {
                    target: Box::new(target),
                    value: Box::new(value.clone()),
                },
                value.ty.clone(),
                value.ownership.clone(),
            )
        }
        Expr::Unary { op, expr } => {
            let expr = lower_typed_expr(expr, env, scopes)?;
            let ty = match op {
                UnaryOp::Not => IrType::Bool,
                UnaryOp::Neg => expr.ty.clone(),
            };
            typed_expr(
                TypedExprKind::Unary {
                    op: *op,
                    expr: Box::new(expr),
                },
                ty,
            )
        }
        Expr::IncDec {
            name,
            delta,
            prefix,
        } => {
            let target = if let Some((this_type, field_type, field_ownership)) =
                implicit_field(env, scopes, name)
            {
                let this_expr = typed_expr_with_ownership(
                    TypedExprKind::Var("this".to_string()),
                    this_type,
                    Ownership::Borrowed,
                );
                typed_expr_with_ownership(
                    TypedExprKind::Field {
                        target: Box::new(this_expr),
                        name: name.clone(),
                    },
                    field_type.clone(),
                    field_ownership,
                )
            } else {
                let ty = lookup(scopes, name)
                    .map(|binding| binding.ty.clone())
                    .unwrap_or_else(|| IrType::Unknown(name.clone()));
                let binding = lookup(scopes, name);
                let ownership = binding.map(|b| b.ownership).unwrap_or(Ownership::Shared);
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    ty,
                    ownership,
                )
            };
            let ty = target.ty.clone();
            typed_expr(
                TypedExprKind::IncDec {
                    target: Box::new(target),
                    delta: *delta,
                    prefix: *prefix,
                },
                ty,
            )
        }
        Expr::Lambda { params, body } => lower_lambda(params, body, expected, env, scopes)?,
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            let condition = lower_typed_expr(condition, env, scopes)?;
            let when_true = lower_typed_expr(when_true, env, scopes)?;
            let when_false = lower_typed_expr(when_false, env, scopes)?;
            let ty = if when_true.ty == IrType::Unknown("null".to_string()) {
                when_false.ty.clone()
            } else {
                when_true.ty.clone()
            };
            typed_expr_with_ownership(
                TypedExprKind::Conditional {
                    condition: Box::new(condition),
                    when_true: Box::new(when_true),
                    when_false: Box::new(when_false),
                },
                ty.clone(),
                ownership_for_type(&ty),
            )
        }
        Expr::Binary { left, op, right } => {
            let left = lower_typed_expr(left, env, scopes)?;
            let right = if *op == BinaryOp::Coalesce {
                lower_typed_expr_with_expected(right, env, scopes, Some(&left.ty))?
            } else {
                lower_typed_expr(right, env, scopes)?
            };
            let ty = if op.is_comparison() {
                IrType::Bool
            } else {
                left.ty.clone()
            };
            typed_expr(
                TypedExprKind::Binary {
                    left: Box::new(left),
                    op: *op,
                    right: Box::new(right),
                },
                ty,
            )
        }
        Expr::NamedArg { expr, .. } => lower_typed_expr(expr, env, scopes)?,
        Expr::RefArg {
            modifier,
            expr: argument,
        } => {
            if matches!(
                modifier,
                ParamModifier::Out | ParamModifier::Ref | ParamModifier::In
            ) {
                if let Expr::Var(name) = argument.as_ref() {
                    let target = lookup(scopes, name).unwrap_or(TypedBinding {
                        name: name.clone(),
                        ty: IrType::Unknown(format!("out {name}")),
                        ownership: Ownership::Shared,
                    });
                    typed_expr_with_ownership(
                        TypedExprKind::Borrow {
                            mutable: !matches!(modifier, ParamModifier::In),
                            name: name.clone(),
                        },
                        IrType::Ref(Box::new(target.ty)),
                        Ownership::Borrowed,
                    )
                } else {
                    lower_typed_expr(argument, env, scopes)?
                }
            } else {
                lower_typed_expr(argument, env, scopes)?
            }
        }
    };
    Ok(coerce_for_expected_type(typed, expected))
}

fn coerce_for_expected_type(mut typed: TypedExpr, expected: Option<&IrType>) -> TypedExpr {
    let Some(expected) = expected else {
        return typed;
    };
    match expected {
        IrType::Nullable(inner) => {
            if matches!(typed.ty, IrType::Nullable(_)) {
                return typed;
            }
            if matches!(typed.ty, IrType::Unknown(ref name) if name == "null") {
                typed.ty = expected.clone();
                typed.ownership = Ownership::Shared;
                typed.drop_kind = drop_kind_for_type(expected, &typed.ownership);
                return typed;
            }
            if is_nullable_value_type(inner) && ir_arg_matches(inner, &typed.ty) {
                let wrapped = TypedExpr {
                    kind: TypedExprKind::NullableSome(Box::new(typed)),
                    ty: expected.clone(),
                    ownership: Ownership::Shared,
                    drop_kind: drop_kind_for_type(expected, &Ownership::Shared),
                };
                return wrapped;
            }
            typed
        }
        _ => typed,
    }
}

fn lower_expr(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedBinding, String> {
    let binding = match expr {
        Expr::Int(_) => typed_temp(IrType::Long),
        Expr::Float(_) => typed_temp(IrType::Double),
        Expr::Bool(_) => typed_temp(IrType::Bool),
        Expr::Null => typed_temp(IrType::Unknown("null".to_string())),
        Expr::String(_) => TypedBinding {
            name: "<expr>".to_string(),
            ty: IrType::String,
            ownership: Ownership::Owned,
        },
        Expr::Var(name) => lookup(scopes, name)
            .or_else(|| {
                if let Some(kind) = env.kinds.get(name) {
                    let ty = match kind {
                        TypeKind::Class => IrType::Class(name.clone()),
                        TypeKind::Interface => IrType::Interface(name.clone()),
                        TypeKind::Enum => IrType::Int,
                        _ => IrType::Struct(name.clone()),
                    };
                    Some(TypedBinding {
                        name: name.clone(),
                        ty,
                        ownership: Ownership::Shared,
                    })
                } else if name == "Exception" || name == "System.Exception" {
                    Some(TypedBinding {
                        name: name.clone(),
                        ty: IrType::Exception,
                        ownership: Ownership::Shared,
                    })
                } else {
                    env.single_function(name).map(|signature| TypedBinding {
                        name: name.clone(),
                        ty: IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        },
                        ownership: Ownership::Copy,
                    })
                }
            })
            .unwrap_or(TypedBinding {
                name: name.clone(),
                ty: IrType::Unknown(name.clone()),
                ownership: Ownership::Shared,
            }),
        Expr::Move(name) => {
            let mut binding = lookup(scopes, name).unwrap_or(TypedBinding {
                name: name.clone(),
                ty: IrType::Unknown(name.clone()),
                ownership: Ownership::Moved,
            });
            binding.ownership = Ownership::Moved;
            binding
        }
        Expr::ArrayLiteral(values) => {
            for value in values {
                lower_expr(value, env, scopes)?;
            }
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Array(Box::new(IrType::Long)),
                ownership: Ownership::Owned,
            }
        }
        Expr::NewArray {
            element_type,
            length,
            values,
        } => {
            if let Some(length) = length {
                lower_expr(length, env, scopes)?;
            }
            for value in values {
                lower_expr(value, env, scopes)?;
            }
            let element = type_syntax_to_ir(element_type, env);
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Array(Box::new(element)),
                ownership: Ownership::Owned,
            }
        }
        Expr::Index { target, index } => {
            let target = lower_expr(target, env, scopes)?;
            lower_expr(index, env, scopes)?;
            let ty = match target.ty {
                IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => *inner,
                IrType::Dictionary(_, value) => *value,
                _ => IrType::Unknown("index".to_string()),
            };
            let ownership = if ty == IrType::String {
                Ownership::Borrowed
            } else {
                ownership_for_type(&ty)
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ty,
                ownership,
            }
        }
        Expr::Field { target, name } => {
            let target = lower_expr(target, env, scopes)?;
            if target.ty == IrType::Int {
                if let Some(value) = env.enum_values.get(&(target.name.clone(), name.clone())) {
                    let _ = value;
                    return Ok(TypedBinding {
                        name: "<expr>".to_string(),
                        ty: IrType::Int,
                        ownership: Ownership::Copy,
                    });
                }
            }
            let field_info = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), field) | (IrType::Struct(type_name), field) => {
                    env.resolve_field(type_name, field)
                }
                (IrType::List(_), "Count")
                | (IrType::Dictionary(_, _), "Count")
                | (IrType::Array(_), "Length" | "Count") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Interface(type_name), "Current")
                    if base_type_name(type_name) == "IEnumerator" =>
                {
                    split_monomorphized_type(type_name)
                        .and_then(|(_, args)| args.first().cloned())
                        .and_then(|arg| parse_monomorphized_ir_type(&arg, env))
                        .map(|ty| FieldSignature {
                            package_id: None,
                            visibility: Visibility::Public,
                            ownership: ownership_for_type(&ty),
                            ty,
                        })
                }
                (IrType::String, "Length") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Task(inner), "Result") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: inner.as_ref().clone(),
                    ownership: ownership_for_type(inner),
                }),
                (IrType::Task(_), "IsCompleted") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Bool,
                    ownership: Ownership::Copy,
                }),
                (IrType::Exception, "Message") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::String,
                    ownership: Ownership::Borrowed,
                }),
                (IrType::Weak(inner), "Target") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: inner.as_ref().clone(),
                    ownership: Ownership::Borrowed,
                }),
                _ => None,
            };
            let (ty, ownership) = field_info
                .map(|field| (field.ty, field.ownership))
                .unwrap_or_else(|| {
                    let ty = IrType::Unknown(name.clone());
                    let ownership = ownership_for_type(&ty);
                    (ty, ownership)
                });
            TypedBinding {
                name: "<expr>".to_string(),
                ty,
                ownership,
            }
        }
        Expr::IsPattern { expr, .. } => {
            lower_expr(expr, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Bool,
                ownership: Ownership::Copy,
            }
        }
        Expr::MethodCall { target, name, args } => {
            let target = lower_expr(target, env, scopes)?;
            let args = args
                .iter()
                .map(|arg| lower_expr(arg, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            let ty = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), "MapGet" | "MapPost")
                    if type_name == "WebApplication" =>
                {
                    IrType::Void
                }
                (IrType::Task(inner), "GetResult") => inner.as_ref().clone(),
                (IrType::Task(inner), "GetAwaiter") => IrType::Task(inner.clone()),
                (_, "Contains") | (_, "ContainsKey") | (_, "Remove") | (_, "TryGetValue") => IrType::Bool,
                (_, "Add") | (_, "Clear") | (_, "Wait") => IrType::Void,
                (IrType::Unknown(target) | IrType::Class(target), "Run") if target == "Task" => {
                    args.first()
                        .map(|arg| IrType::Task(Box::new(arg.ty.clone())))
                        .unwrap_or_else(|| IrType::Task(Box::new(IrType::Void)))
                }
                (IrType::Unknown(target) | IrType::Class(target), "ReadAllText")
                    if target == "File" || target == "System.IO.File" =>
                {
                    IrType::String
                }
                (IrType::Unknown(target) | IrType::Class(target), "WriteAllText")
                    if target == "File" || target == "System.IO.File" =>
                {
                    IrType::Void
                }
                _ => IrType::Unknown(name.clone()),
            };
            let ownership = if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                env.resolve_method(type_name, name, &args.iter().map(|arg| arg.ty.clone()).collect::<Vec<_>>())
                    .map(|signature| signature.return_ownership.clone())
                    .unwrap_or_else(|| ownership_for_type(&ty))
            } else {
                ownership_for_type(&ty)
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership,
                ty,
            }
        }
        Expr::FunctionCall { name, args } => {
            if name == "sizeof" {
                return Ok(TypedBinding {
                    name: "<expr>".to_string(),
                    ownership: Ownership::Copy,
                    ty: IrType::Int,
                });
            }
            let args = args
                .iter()
                .map(|arg| lower_expr(arg, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            let signature = env.resolve_function(
                name,
                &args.iter().map(|arg| arg.ty.clone()).collect::<Vec<_>>(),
            );
            let ty = signature
                .as_ref()
                .map(|signature| signature.return_type.clone())
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: signature
                    .as_ref()
                    .map(|signature| signature.return_ownership.clone())
                    .unwrap_or_else(|| Ownership::Shared),
                ty,
            }
        }
        Expr::NewObject {
            type_name,
            args,
            fields,
        } => {
            for arg in args {
                lower_expr(arg, env, scopes)?;
            }
            for field in fields {
                lower_expr(&field.expr, env, scopes)?;
            }
            let ty = match env.kinds.get(type_name) {
                Some(TypeKind::Class) => IrType::Class(type_name.clone()),
                _ if type_name == "Exception" || type_name == "System.Exception" => {
                    IrType::Exception
                }
                _ => IrType::Struct(type_name.clone()),
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: ownership_for_type(&ty),
                ty,
            }
        }
        Expr::NewCollection(ty) => {
            let ty = type_syntax_to_ir(ty, env);
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: Ownership::Owned,
                ty,
            }
        }
        Expr::NewThread(_) => TypedBinding {
            name: "<expr>".to_string(),
            ty: IrType::Thread,
            ownership: Ownership::Owned,
        },
        Expr::Borrow { name, .. } => {
            let target =
                lookup(scopes, name).unwrap_or_else(|| typed_temp(IrType::Unknown(name.clone())));
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Ref(Box::new(target.ty)),
                ownership: Ownership::Borrowed,
            }
        }
        Expr::Await(inner) => {
            let inner = lower_expr(inner, env, scopes)?;
            let ty = match inner.ty {
                IrType::Task(result) => *result,
                other => IrType::Unknown(format!("await {other:?}")),
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: ownership_for_type(&ty),
                ty,
            }
        }
        Expr::Throw(expr) => {
            lower_expr(expr, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Unknown("throw".to_string()),
                ownership: Ownership::Copy,
            }
        }
        Expr::Assign { target, value } => {
            lower_expr(target, env, scopes)?;
            let value = lower_expr(value, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: value.ty,
                ownership: value.ownership,
            }
        }
        Expr::Unary { expr, .. } => {
            lower_expr(expr, env, scopes)?;
            typed_temp(IrType::Bool)
        }
        Expr::Lambda { body, .. } => {
            lower_expr(body, env, scopes)?;
            typed_temp(IrType::Unknown("lambda".to_string()))
        }
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            lower_expr(condition, env, scopes)?;
            let when_true = lower_expr(when_true, env, scopes)?;
            let when_false = lower_expr(when_false, env, scopes)?;
            if when_true.ty == IrType::Unknown("null".to_string()) {
                typed_temp(when_false.ty)
            } else {
                typed_temp(when_true.ty)
            }
        }
        Expr::Binary { left, op, right } => {
            let left = lower_expr(left, env, scopes)?;
            lower_expr(right, env, scopes)?;
            let ty = if op.is_comparison() {
                IrType::Bool
            } else {
                left.ty
            };
            typed_temp(ty)
        }
        Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => lower_expr(expr, env, scopes)?,
        Expr::IncDec { name, .. } => {
            let ty = if let Some((_, field_ty, _)) = implicit_field(env, scopes, name) {
                field_ty
            } else {
                lookup(scopes, name)
                    .map(|binding| binding.ty)
                    .unwrap_or_else(|| IrType::Unknown(name.clone()))
            };
            typed_temp(ty)
        }
    };
    Ok(binding)
}

fn lookup(scopes: &[HashMap<String, TypedBinding>], name: &str) -> Option<TypedBinding> {
    scopes
        .iter()
        .rev()
        .find_map(|scope| scope.get(name).cloned())
}

fn current_enclosing_type(scopes: &[HashMap<String, TypedBinding>]) -> Option<String> {
    let binding = lookup(scopes, "__glitching_current_type")?;
    match binding.ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => Some(name),
        _ => None,
    }
}

fn current_enclosing_type_from_state(state: &OwnershipState) -> Option<String> {
    let binding = state.get("__glitching_current_type")?;
    match &binding.ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => {
            Some(name.clone())
        }
        _ => None,
    }
}

fn implicit_field(
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
    name: &str,
) -> Option<(IrType, IrType, Ownership)> {
    let this_type = lookup(scopes, "this")?.ty;
    let owner = match &this_type {
        IrType::Class(name) | IrType::Struct(name) => name.as_str(),
        IrType::Ref(inner) => match inner.as_ref() {
            IrType::Struct(name) | IrType::Class(name) => name.as_str(),
            _ => return None,
        },
        _ => return None,
    };
    let field_type = env.resolve_field(owner, name)?;
    Some((this_type, field_type.ty, field_type.ownership))
}

fn collect_condition_bindings(expr: &Expr, env: &TypeEnv) -> Vec<TypedBinding> {
    let mut bindings = Vec::new();
    collect_condition_bindings_inner(expr, env, &mut bindings);
    bindings
}

fn collect_condition_bindings_inner(expr: &Expr, env: &TypeEnv, bindings: &mut Vec<TypedBinding>) {
    match expr {
        Expr::IsPattern {
            ty,
            name: Some(name),
            ..
        } => {
            let ty = type_syntax_to_ir(ty, env);
            bindings.push(TypedBinding {
                name: name.clone(),
                ty,
                ownership: Ownership::Borrowed,
            });
        }
        Expr::Binary { left, right, .. } => {
            collect_condition_bindings_inner(left, env, bindings);
            collect_condition_bindings_inner(right, env, bindings);
        }
        Expr::MethodCall { target, args, .. } => {
            collect_condition_bindings_inner(target, env, bindings);
            for arg in args {
                collect_condition_bindings_inner(arg, env, bindings);
            }
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_condition_bindings_inner(arg, env, bindings);
            }
        }
        Expr::RefArg {
            modifier: ParamModifier::Out,
            expr,
        } => {
            if let Expr::Var(name) = expr.as_ref() {
                bindings.push(TypedBinding {
                    name: name.clone(),
                    ty: IrType::Unknown(format!("out {name}")),
                    ownership: Ownership::Shared,
                });
            }
        }
        Expr::Unary { expr, .. } => collect_condition_bindings_inner(expr, env, bindings),
        _ => {}
    }
}

fn typed_temp(ty: IrType) -> TypedBinding {
    TypedBinding {
        name: "<expr>".to_string(),
        ownership: ownership_for_type(&ty),
        ty,
    }
}

fn typed_expr(kind: TypedExprKind, ty: IrType) -> TypedExpr {
    let ownership = ownership_for_type(&ty);
    typed_expr_with_ownership(kind, ty, ownership)
}

fn typed_expr_with_ownership(kind: TypedExprKind, ty: IrType, ownership: Ownership) -> TypedExpr {
    let drop_kind = drop_kind_for_type(&ty, &ownership);
    TypedExpr {
        kind,
        ty,
        ownership,
        drop_kind,
    }
}

fn augment_type_metadata_fields(
    mut fields: Vec<TypedFieldInit>,
    env: &TypeEnv,
) -> Result<Vec<TypedFieldInit>, String> {
    let has_properties = fields.iter().any(|field| field.name == "Properties");
    if has_properties {
        return Ok(fields);
    }
    let Some(full_name) = fields.iter().find_map(|field| match &field.expr.kind {
        TypedExprKind::String(value) if field.name == "FullName" => Some(value.clone()),
        _ => None,
    }) else {
        return Ok(fields);
    };
    let type_name = full_name.rsplit('.').next().unwrap_or(&full_name);
    let properties = env.all_field_infos(type_name);
    let property_values = properties
        .into_iter()
        .map(|(name, field)| {
            let property_type = type_object_expr_from_ir_shallow(&field.ty, env);
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: "PropertyInfo".to_string(),
                    constructor: None,
                    args: Vec::new(),
                    fields: vec![
                        TypedFieldInit {
                            name: "Name".to_string(),
                            expr: typed_expr(TypedExprKind::String(name), IrType::String),
                        },
                        TypedFieldInit {
                            name: "PropertyType".to_string(),
                            expr: property_type,
                        },
                        TypedFieldInit {
                            name: "CanRead".to_string(),
                            expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                        },
                        TypedFieldInit {
                            name: "CanWrite".to_string(),
                            expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                        },
                    ],
                },
                IrType::Class("PropertyInfo".to_string()),
                Ownership::Shared,
            )
        })
        .collect::<Vec<_>>();
    fields.push(TypedFieldInit {
        name: "Properties".to_string(),
        expr: typed_expr(
            TypedExprKind::ArrayLiteral(property_values),
            IrType::Array(Box::new(IrType::Class("PropertyInfo".to_string()))),
        ),
    });
    Ok(fields)
}

fn type_object_expr_from_ir(ty: &IrType, env: &TypeEnv) -> TypedExpr {
    type_object_expr_from_ir_impl(ty, env, true)
}

fn type_object_expr_from_ir_shallow(ty: &IrType, env: &TypeEnv) -> TypedExpr {
    type_object_expr_from_ir_impl(ty, env, false)
}

fn type_object_expr_from_ir_impl(ty: &IrType, env: &TypeEnv, include_members: bool) -> TypedExpr {
    let full_name = type_object_full_name_from_ir(ty);
    let simple_name = full_name.rsplit('.').next().unwrap_or(&full_name).to_string();
    let namespace = if let Some((ns, _)) = full_name.rsplit_once('.') {
        ns.to_string()
    } else {
        String::new()
    };
    let generic_definition_name = type_object_generic_definition_name_from_ir(ty);
    let is_generic = matches!(
        ty,
        IrType::List(_)
            | IrType::Dictionary(_, _)
            | IrType::Enumerable(_)
            | IrType::Task(_)
            | IrType::Nullable(_)
            | IrType::Function { .. }
    );
    let generic_arguments = type_object_generic_arguments_from_ir(ty, env);
    let base_type = type_object_base_type_from_ir(ty, env);
    let element_type = match ty {
        IrType::Array(inner)
        | IrType::List(inner)
        | IrType::Enumerable(inner)
        | IrType::Task(inner)
        | IrType::Nullable(inner) => {
            Some(type_object_expr_from_ir_shallow(inner, env))
        }
        _ => None,
    };
    let properties = if include_members {
        type_object_properties_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    let methods = if include_members {
        type_object_methods_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    let fields = if include_members {
        type_object_fields_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    let constructors = if include_members {
        type_object_constructors_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    let interfaces = if include_members {
        type_object_interfaces_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    typed_expr_with_ownership(
        TypedExprKind::NewObject {
            type_name: "Type".to_string(),
            constructor: None,
            args: Vec::new(),
            fields: vec![
                TypedFieldInit {
                    name: "Name".to_string(),
                    expr: typed_expr(TypedExprKind::String(simple_name), IrType::String),
                },
                TypedFieldInit {
                    name: "Namespace".to_string(),
                    expr: typed_expr(TypedExprKind::String(namespace), IrType::String),
                },
                TypedFieldInit {
                    name: "FullName".to_string(),
                    expr: typed_expr(TypedExprKind::String(full_name), IrType::String),
                },
                TypedFieldInit {
                    name: "GenericTypeDefinitionName".to_string(),
                    expr: typed_expr(
                        TypedExprKind::String(generic_definition_name),
                        IrType::String,
                    ),
                },
                TypedFieldInit {
                    name: "IsGenericType".to_string(),
                    expr: typed_expr(TypedExprKind::Bool(is_generic), IrType::Bool),
                },
                TypedFieldInit {
                    name: "IsClass".to_string(),
                    expr: typed_expr(
                        TypedExprKind::Bool(matches!(ty, IrType::Class(_))),
                        IrType::Bool,
                    ),
                },
                TypedFieldInit {
                    name: "IsInterface".to_string(),
                    expr: typed_expr(
                        TypedExprKind::Bool(matches!(ty, IrType::Interface(_))),
                        IrType::Bool,
                    ),
                },
                TypedFieldInit {
                    name: "IsValueType".to_string(),
                    expr: typed_expr(
                        TypedExprKind::Bool(matches!(
                            ty,
                            IrType::Struct(_)
                                | IrType::Nullable(_)
                                | IrType::Bool
                                | IrType::Byte
                                | IrType::Short
                                | IrType::Int
                                | IrType::Long
                                | IrType::UInt
                                | IrType::Double
                                | IrType::Decimal
                        )),
                        IrType::Bool,
                    ),
                },
                TypedFieldInit {
                    name: "IsEnum".to_string(),
                    expr: typed_expr(TypedExprKind::Bool(false), IrType::Bool),
                },
                TypedFieldInit {
                    name: "IsArray".to_string(),
                    expr: typed_expr(TypedExprKind::Bool(matches!(ty, IrType::Array(_))), IrType::Bool),
                },
                TypedFieldInit {
                    name: "BaseType".to_string(),
                    expr: base_type.unwrap_or_else(|| {
                        typed_expr_with_ownership(
                            TypedExprKind::Null,
                            IrType::Unknown("null".to_string()),
                            Ownership::Copy,
                        )
                    }),
                },
                TypedFieldInit {
                    name: "ElementType".to_string(),
                    expr: element_type.unwrap_or_else(|| {
                        typed_expr_with_ownership(
                            TypedExprKind::Null,
                            IrType::Unknown("null".to_string()),
                            Ownership::Copy,
                        )
                    }),
                },
                TypedFieldInit {
                    name: "GenericArguments".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(generic_arguments),
                        IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Properties".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(properties),
                        IrType::Array(Box::new(IrType::Class("PropertyInfo".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Methods".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(methods),
                        IrType::Array(Box::new(IrType::Class("MethodInfo".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Fields".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(fields),
                        IrType::Array(Box::new(IrType::Class("FieldInfo".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Constructors".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(constructors),
                        IrType::Array(Box::new(IrType::Class("ConstructorInfo".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Interfaces".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(interfaces),
                        IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                    ),
                },
            ],
        },
        IrType::Class("Type".to_string()),
        Ownership::Shared,
    )
}

fn type_object_base_type_from_ir(ty: &IrType, env: &TypeEnv) -> Option<TypedExpr> {
    let base_name = match ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) | IrType::Unknown(name) => {
            let simple = name.rsplit('.').next().unwrap_or(name);
            env.bases.get(simple).and_then(|bases| bases.first()).cloned()
        }
        IrType::String => Some("System.Object".to_string()),
        _ => None,
    }?;
    Some(type_object_expr_from_ir_impl(&IrType::Unknown(base_name), env, false))
}

fn type_object_properties_from_ir(ty: &IrType, env: &TypeEnv, include_members: bool) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let type_name = type_object_lookup_name(ty);
    let Some(type_name) = type_name else {
        return Vec::new();
    };
    env.all_field_infos(type_name)
        .into_iter()
        .map(|(name, field)| {
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: "PropertyInfo".to_string(),
                    constructor: None,
                    args: Vec::new(),
                    fields: vec![
                        TypedFieldInit {
                            name: "Name".to_string(),
                            expr: typed_expr(TypedExprKind::String(name), IrType::String),
                        },
                        TypedFieldInit {
                            name: "PropertyType".to_string(),
                            expr: type_object_expr_from_ir_shallow(&field.ty, env),
                        },
                        TypedFieldInit {
                            name: "CanRead".to_string(),
                            expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                        },
                        TypedFieldInit {
                            name: "CanWrite".to_string(),
                            expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                        },
                    ],
                },
                IrType::Class("PropertyInfo".to_string()),
                Ownership::Shared,
            )
        })
        .collect()
}

fn type_object_methods_from_ir(ty: &IrType, env: &TypeEnv, include_members: bool) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let mut methods = builtin_methods_from_ir(ty, env);
    if let Some(type_name) = type_object_lookup_name(ty) {
        for ((owner, method_name), signatures) in &env.methods {
            if owner != type_name {
                continue;
            }
            for signature in signatures {
                let parameters = signature
                    .params
                    .iter()
                    .map(|param| {
                        type_object_expr_from_ir_shallow(param, env)
                    })
                    .collect::<Vec<_>>();
                methods.push(typed_expr_with_ownership(
                    TypedExprKind::NewObject {
                        type_name: "MethodInfo".to_string(),
                        constructor: None,
                        args: Vec::new(),
                        fields: vec![
                            TypedFieldInit {
                                name: "Name".to_string(),
                                expr: typed_expr(
                                    TypedExprKind::String(method_name.clone()),
                                    IrType::String,
                                ),
                            },
                            TypedFieldInit {
                                name: "ReturnType".to_string(),
                                expr: type_object_expr_from_ir_shallow(&signature.return_type, env),
                            },
                            TypedFieldInit {
                                name: "ParameterTypes".to_string(),
                                expr: typed_expr(
                                    TypedExprKind::ArrayLiteral(parameters),
                                    IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                                ),
                            },
                            TypedFieldInit {
                                name: "IsStatic".to_string(),
                                expr: typed_expr(TypedExprKind::Bool(signature.is_static), IrType::Bool),
                            },
                        ],
                    },
                    IrType::Class("MethodInfo".to_string()),
                    Ownership::Shared,
                ));
            }
        }
    }
    methods
}

fn type_object_fields_from_ir(ty: &IrType, env: &TypeEnv, include_members: bool) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let Some(type_name) = type_object_lookup_name(ty) else {
        return Vec::new();
    };
    env.all_field_infos(type_name)
        .into_iter()
        .map(|(name, field)| {
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: "FieldInfo".to_string(),
                    constructor: None,
                    args: Vec::new(),
                    fields: vec![
                        TypedFieldInit {
                            name: "Name".to_string(),
                            expr: typed_expr(TypedExprKind::String(name), IrType::String),
                        },
                        TypedFieldInit {
                            name: "FieldType".to_string(),
                            expr: type_object_expr_from_ir_shallow(&field.ty, env),
                        },
                    ],
                },
                IrType::Class("FieldInfo".to_string()),
                Ownership::Shared,
            )
        })
        .collect()
}

fn type_object_constructors_from_ir(
    ty: &IrType,
    env: &TypeEnv,
    include_members: bool,
) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let Some(type_name) = type_object_lookup_name(ty) else {
        return Vec::new();
    };
    env.constructors
        .get(type_name)
        .into_iter()
        .flat_map(|signatures| signatures.iter())
        .map(|signature| {
            let parameters = signature
                .params
                .iter()
                .map(|param| type_object_expr_from_ir_shallow(param, env))
                .collect::<Vec<_>>();
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: "ConstructorInfo".to_string(),
                    constructor: None,
                    args: Vec::new(),
                    fields: vec![
                        TypedFieldInit {
                            name: "Name".to_string(),
                            expr: typed_expr(
                                TypedExprKind::String(type_name.to_string()),
                                IrType::String,
                            ),
                        },
                        TypedFieldInit {
                            name: "ParameterTypes".to_string(),
                            expr: typed_expr(
                                TypedExprKind::ArrayLiteral(parameters),
                                IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                            ),
                        },
                    ],
                },
                IrType::Class("ConstructorInfo".to_string()),
                Ownership::Shared,
            )
        })
        .collect()
}

fn type_object_interfaces_from_ir(ty: &IrType, env: &TypeEnv, include_members: bool) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let Some(type_name) = type_object_lookup_name(ty) else {
        return Vec::new();
    };
    env.bases
        .get(type_name)
        .into_iter()
        .flat_map(|bases| bases.iter())
        .map(|base| type_object_expr_from_ir_shallow(&IrType::Unknown(base.clone()), env))
        .collect()
}

fn builtin_methods_from_ir(ty: &IrType, env: &TypeEnv) -> Vec<TypedExpr> {
    let mut methods = Vec::new();
    let entries: &[(&str, &[&str])] = match ty {
        IrType::String => &[
            ("ToLower", &[]),
            ("ToLowerInvariant", &[]),
            ("Substring", &["int"]),
            ("Split", &["string"]),
            ("Replace", &["string", "string"]),
            ("Trim", &[]),
            ("StartsWith", &["string", "StringComparison"]),
            ("Equals", &["string", "StringComparison"]),
            ("Contains", &["string"]),
        ],
        IrType::Class(name) | IrType::Struct(name) if name == "DateTime" || name == "System.DateTime" => {
            &[("CompareTo", &["DateTime"])]
        }
        IrType::Class(name) | IrType::Struct(name) if name == "Type" || name == "System.Type" => {
            &[("GetGenericTypeDefinition", &[]), ("GetGenericArguments", &[]), ("GetMethod", &["string", "Type[]"]), ("GetProperty", &["string", "object"]), ("GetProperties", &[])]
        }
        _ => &[],
    };
    for (name, params) in entries {
        let parameter_types = params
            .iter()
            .map(|param| match *param {
                "int" => type_object_expr_from_ir_shallow(&IrType::Int, env),
                "string" => type_object_expr_from_ir_shallow(&IrType::String, env),
                "DateTime" => type_object_expr_from_ir_shallow(&IrType::Unknown("DateTime".to_string()), env),
                "StringComparison" => type_object_expr_from_ir_shallow(&IrType::Unknown("StringComparison".to_string()), env),
                "object" => type_object_expr_from_ir_shallow(&IrType::Unknown("object".to_string()), env),
                "Type[]" => type_object_expr_from_ir_shallow(&IrType::Array(Box::new(IrType::Class("Type".to_string()))), env),
                _ => type_object_expr_from_ir_shallow(&IrType::Unknown((*param).to_string()), env),
            })
            .collect::<Vec<_>>();
        methods.push(typed_expr_with_ownership(
            TypedExprKind::NewObject {
                type_name: "MethodInfo".to_string(),
                constructor: None,
                args: Vec::new(),
                fields: vec![
                    TypedFieldInit {
                        name: "Name".to_string(),
                        expr: typed_expr(TypedExprKind::String((*name).to_string()), IrType::String),
                    },
                    TypedFieldInit {
                        name: "ReturnType".to_string(),
                        expr: typed_expr(
                            TypedExprKind::String(if *name == "GetGenericArguments" {
                                "Type[]".to_string()
                            } else if *name == "GetProperties" {
                                "PropertyInfo[]".to_string()
                            } else if *name == "GetMethod" {
                                "MethodInfo".to_string()
                            } else if *name == "GetProperty" {
                                "PropertyInfo".to_string()
                            } else if *name == "GetGenericTypeDefinition" {
                                "Type".to_string()
                            } else if *name == "CompareTo" {
                                "int".to_string()
                            } else {
                                "void".to_string()
                            }),
                            IrType::String,
                        ),
                    },
                    TypedFieldInit {
                        name: "ParameterTypes".to_string(),
                        expr: typed_expr(
                            TypedExprKind::ArrayLiteral(parameter_types),
                            IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                        ),
                    },
                    TypedFieldInit {
                        name: "IsStatic".to_string(),
                        expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                    },
                ],
            },
            IrType::Class("MethodInfo".to_string()),
            Ownership::Shared,
        ));
    }
    methods
}

fn type_object_lookup_name(ty: &IrType) -> Option<&str> {
    match ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) | IrType::Unknown(name) => {
            Some(name.rsplit('.').next().unwrap_or(name))
        }
        IrType::String => Some("string"),
        IrType::Bool => Some("bool"),
        IrType::Int => Some("int"),
        IrType::Long => Some("long"),
        IrType::Double => Some("double"),
        IrType::UInt => Some("uint"),
        IrType::Byte => Some("byte"),
        IrType::Short => Some("short"),
        IrType::Decimal => Some("decimal"),
        _ => None,
    }
}

fn type_object_full_name_from_ir(ty: &IrType) -> String {
    match ty {
        IrType::Bool => "bool".to_string(),
        IrType::Byte => "byte".to_string(),
        IrType::Short => "short".to_string(),
        IrType::Int => "int".to_string(),
        IrType::Long => "long".to_string(),
        IrType::UInt => "uint".to_string(),
        IrType::Double => "double".to_string(),
        IrType::Decimal => "decimal".to_string(),
        IrType::String => "string".to_string(),
        IrType::Void => "void".to_string(),
        IrType::Array(inner) => format!("{}[]", type_object_full_name_from_ir(inner)),
        IrType::Ref(inner) => format!("ref {}", type_object_full_name_from_ir(inner)),
        IrType::Struct(name)
        | IrType::Class(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => name.clone(),
        IrType::Exception => "Exception".to_string(),
        IrType::List(inner) => format!("List<{}>", type_object_full_name_from_ir(inner)),
        IrType::Dictionary(key, value) => format!(
            "Dictionary<{},{}>",
            type_object_full_name_from_ir(key),
            type_object_full_name_from_ir(value)
        ),
        IrType::Enumerable(inner) => format!("ICollection<{}>", type_object_full_name_from_ir(inner)),
        IrType::Thread => "Thread".to_string(),
        IrType::Task(inner) => format!("Task<{}>", type_object_full_name_from_ir(inner)),
        IrType::Function { params, return_type } => format!(
            "Func<{}{}>",
            params
                .iter()
                .map(type_object_full_name_from_ir)
                .collect::<Vec<_>>()
                .join(","),
            if matches!(return_type.as_ref(), IrType::Void) {
                String::new()
            } else {
                format!(
                    "{}{}",
                    if params.is_empty() { "" } else { "," },
                    type_object_full_name_from_ir(return_type)
                )
            }
        ),
        IrType::Weak(inner) => format!("weakref<{}>", type_object_full_name_from_ir(inner)),
        IrType::Nullable(inner) => format!("{}?", type_object_full_name_from_ir(inner)),
    }
}

fn type_object_generic_definition_name_from_ir(ty: &IrType) -> String {
    match ty {
        IrType::List(_) => "List<>".to_string(),
        IrType::Dictionary(_, _) => "Dictionary<,>".to_string(),
        IrType::Enumerable(_) => "ICollection<>".to_string(),
        IrType::Task(_) => "Task<>".to_string(),
        IrType::Nullable(_) => "Nullable<>".to_string(),
        IrType::Function { params, return_type } => {
            let placeholders = std::iter::repeat("")
                .take(params.len() + if matches!(return_type.as_ref(), IrType::Void) { 0 } else { 1 })
                .collect::<Vec<_>>()
                .join(",");
            format!("Func<{}>", placeholders)
        }
        _ => type_object_full_name_from_ir(ty),
    }
}

fn type_object_generic_arguments_from_ir(ty: &IrType, env: &TypeEnv) -> Vec<TypedExpr> {
    match ty {
        IrType::List(inner)
        | IrType::Enumerable(inner)
        | IrType::Task(inner)
        | IrType::Nullable(inner)
        | IrType::Weak(inner) => vec![type_object_expr_from_ir_shallow(inner, env)],
        IrType::Dictionary(key, value) => vec![
            type_object_expr_from_ir_shallow(key, env),
            type_object_expr_from_ir_shallow(value, env),
        ],
        IrType::Function { params, return_type } => {
            let mut args = params
                .iter()
                .map(|param| type_object_expr_from_ir_shallow(param, env))
                .collect::<Vec<_>>();
            if !matches!(return_type.as_ref(), IrType::Void) {
                args.push(type_object_expr_from_ir_shallow(return_type, env));
            }
            args
        }
        _ => Vec::new(),
    }
}

fn allows_shared_reference_flow(_env: &TypeEnv, ty: &IrType) -> bool {
    matches!(ty, IrType::Class(_) | IrType::Interface(_))
}

fn is_csharp_shared_handle_name(name: &str) -> bool {
    let simple = name.rsplit('.').next().unwrap_or(name);
    let looks_like_interface = simple
        .strip_prefix('I')
        .and_then(|rest| rest.chars().next())
        .is_some_and(|ch| ch.is_ascii_uppercase());
    looks_like_interface
        || matches!(
            simple,
            "IServiceCollection"
                | "ServiceCollection"
                | "ControllerBase"
                | "ActionResult"
                | "WebApplication"
                | "WebApplicationBuilder"
                | "HttpContext"
                | "HttpRequest"
                | "HttpResponse"
        )
        || simple.ends_with("Options")
        || simple.ends_with("Settings")
        || simple.ends_with("Configuration")
        || simple.ends_with("Manager")
        || simple.ends_with("Builder")
        || simple.ends_with("Collection")
        || simple.ends_with("Logger")
        || simple.ends_with("Provider")
        || simple.ends_with("Context")
        || simple.ends_with("Factory")
}

fn derives_from(env: &TypeEnv, name: &str, expected_base: &str) -> bool {
    let Some(bases) = env.bases.get(name) else {
        return false;
    };
    bases.iter().any(|base| {
        base == expected_base
            || base.rsplit('.').next() == Some(expected_base)
            || derives_from(env, base, expected_base)
    })
}

fn resolve_method_call(
    env: &TypeEnv,
    target: &TypedExpr,
    name: &str,
    args: &[TypedExpr],
) -> Result<(IrType, Ownership, String, CallResolution), String> {
    if matches!(
        &target.ty,
        IrType::Unknown(target_name) | IrType::Class(target_name)
            if matches!(base_type_name(target_name), "object" | "Object")
    ) && name == "Equals"
    {
        let resolution = if args.len() == 2 {
            CallResolution::StaticFunction
        } else {
            CallResolution::InstanceMethod
        };
        return Ok((
            IrType::Bool,
            Ownership::Copy,
            "object.Equals".to_string(),
            resolution,
        ));
    }
    match (&target.ty, name) {
        (IrType::String, _) => {
            for string_type in ["string", "String", "System.String"] {
                if let Some(signature) = env.resolve_method_call(string_type, name, args)? {
                    if signature.is_static {
                        return Ok((
                            signature.return_type.clone(),
                            signature.return_ownership.clone(),
                            signature.symbol.clone(),
                            CallResolution::StaticFunction,
                        ));
                    }
                    return Ok((
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                        signature.symbol.clone(),
                        CallResolution::InstanceMethod,
                    ));
                }
            }
            if let Some(signature) = env.resolve_function_call(name, args)? {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::StaticFunction,
                ));
            }
        }
        (IrType::Unknown(target_name) | IrType::Class(target_name), _) => {
            if let Some(signature) = env.resolve_method_call(target_name, name, args)? {
                if signature.is_static {
                    return Ok((
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                        signature.symbol.clone(),
                        CallResolution::StaticFunction,
                    ));
                }
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            if let Some(signature) = env.resolve_function_call(name, args)? {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::StaticFunction,
                ));
            }
        }
        _ => {}
    }
    match (&target.ty, name) {
        (IrType::Class(type_name), "MapGet" | "MapPost") if type_name == "WebApplication" => {
            if let [TypedExpr {
                kind: TypedExprKind::String(path),
                ..
            }, TypedExpr {
                kind: TypedExprKind::FunctionSymbol(handler),
                ty:
                    IrType::Function {
                        params,
                        return_type,
                    },
                ..
            }] = args
            {
                if params.is_empty() && return_type.as_ref() == &IrType::String {
                    return Ok((
                        IrType::Void,
                        Ownership::Copy,
                        name.to_string(),
                        CallResolution::EndpointHandlerRegistration {
                            http_method: if name == "MapGet" { "GET" } else { "POST" }.to_string(),
                            path: path.clone(),
                            handler: handler.clone(),
                        },
                    ));
                }
            }
            if let Some(signature) = env.resolve_method_call(type_name, name, args)? {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else if let Some(signature) =
                env.resolve_extension_method_call(type_name, name, target, args)?
            {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else {
                Ok((
                    IrType::Unknown(name.to_string()),
                    Ownership::Shared,
                    name.to_string(),
                    CallResolution::InstanceMethod,
                ))
            }
        }
        (IrType::Unknown(target) | IrType::Class(target), "Run") if target == "Task" => {
            let result = args
                .first()
                .map(|arg| arg.ty.clone())
                .unwrap_or(IrType::Void);
            Ok((
                IrType::Task(Box::new(result)),
                Ownership::Owned,
                name.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "FromResult")
            if target == "Task"
                || target == "ValueTask"
                || target == "System.Threading.Tasks.Task"
                || target == "System.Threading.Tasks.ValueTask" =>
        {
            let result = args
                .first()
                .map(|arg| arg.ty.clone())
                .unwrap_or(IrType::Void);
            Ok((
                IrType::Task(Box::new(result)),
                Ownership::Owned,
                name.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "ReadAllText")
            if target == "File" || target == "System.IO.File" =>
        {
            Ok((
                IrType::String,
                Ownership::Owned,
                "System_IO_File_ReadAllText".to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "WriteAllText")
            if target == "File" || target == "System.IO.File" =>
        {
            Ok((
                IrType::Void,
                Ownership::Copy,
                "System_IO_File_WriteAllText".to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "WriteLine")
            if target == "Console" || target == "System.Console" =>
        {
            let arg_ty = args.first().map(|arg| &arg.ty).unwrap_or(&IrType::Void);
            let symbol = match arg_ty {
                IrType::String => "System_Console_WriteLine_String",
                IrType::Byte | IrType::Short | IrType::Int | IrType::Long | IrType::UInt => {
                    "System_Console_WriteLine_I64"
                }
                IrType::Double => "System_Console_WriteLine_Double",
                IrType::Bool => "System_Console_WriteLine_Bool",
                _ => "System_Console_WriteLine_String",
            };
            Ok((
                IrType::Void,
                Ownership::Copy,
                symbol.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::List(_), method_name) => {
            let collection_resolution = match method_name {
                "ToArray" => Some((
                    IrType::Unknown("ToArray".to_string()),
                    Ownership::Owned,
                    name.to_string(),
                    CallResolution::CollectionMethod,
                )),
                "Contains" => Some((
                    IrType::Bool,
                    Ownership::Copy,
                    name.to_string(),
                    CallResolution::CollectionMethod,
                )),
                "Add" | "Clear" => Some((
                    IrType::Void,
                    Ownership::Copy,
                    name.to_string(),
                    CallResolution::CollectionMethod,
                )),
                _ => None,
            };
            if let Some(result) = collection_resolution {
                return Ok(result);
            }
            if let Some(signature) = env.resolve_method_call("List", method_name, args)? {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            if let Some(signature) =
                env.resolve_extension_method_call("List", method_name, target, args)?
            {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            Ok((
                IrType::Unknown(method_name.to_string()),
                Ownership::Shared,
                method_name.to_string(),
                CallResolution::Unknown,
            ))
        }
        (IrType::Enumerable(_), method_name) => {
            if let Some(signature) = env.resolve_method_call("IEnumerable", method_name, args)? {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            if let Some(signature) =
                env.resolve_extension_method_call("IEnumerable", method_name, target, args)?
            {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            Ok((
                IrType::Unknown(method_name.to_string()),
                Ownership::Shared,
                method_name.to_string(),
                CallResolution::Unknown,
            ))
        }
        (
            IrType::Class(type_name) | IrType::Struct(type_name) | IrType::Interface(type_name),
            method_name,
        ) => {
            if let Some(signature) = env.resolve_method_call(type_name, method_name, args)? {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else if let Some(signature) =
                env.resolve_extension_method_call(type_name, method_name, target, args)?
            {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else {
                Ok((
                    IrType::Unknown(name.to_string()),
                    Ownership::Shared,
                    name.to_string(),
                    CallResolution::InstanceMethod,
                ))
            }
        }
        (IrType::Task(inner), "GetResult") => Ok((
            inner.as_ref().clone(),
            ownership_for_type(inner),
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Task(inner), "GetAwaiter") => Ok((
            IrType::Task(inner.clone()),
            Ownership::Owned,
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Task(inner), "AsTask") => Ok((
            IrType::Task(inner.clone()),
            Ownership::Owned,
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Weak(_), "TryGetTarget") => {
            Ok((IrType::Bool, Ownership::Copy, name.to_string(), CallResolution::WeakMethod))
        }
        (IrType::Dictionary(_, _), "ContainsKey" | "Remove" | "TryGetValue") => {
            Ok((
                IrType::Bool,
                Ownership::Copy,
                name.to_string(),
                CallResolution::CollectionMethod,
            ))
        }
        (IrType::Dictionary(_, _), method_name) => {
            let collection_resolution = match method_name {
                "Add" | "Clear" => Some((
                    IrType::Void,
                    Ownership::Copy,
                    name.to_string(),
                    CallResolution::CollectionMethod,
                )),
                "GetEnumerator" => Some(
                    env.resolve_method_call("Dictionary", method_name, args)?
                        .map(|signature| {
                            (
                                signature.return_type.clone(),
                                signature.return_ownership.clone(),
                                name.to_string(),
                                CallResolution::CollectionMethod,
                            )
                        })
                        .unwrap_or((
                            IrType::Unknown(method_name.to_string()),
                            Ownership::Owned,
                            name.to_string(),
                            CallResolution::CollectionMethod,
                        )),
                ),
                _ => None,
            };
            if let Some(result) = collection_resolution {
                return Ok(result);
            }
            if let Some(signature) = env.resolve_method_call("Dictionary", method_name, args)? {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            if let Some(signature) =
                env.resolve_extension_method_call("Dictionary", method_name, target, args)?
            {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            Ok((
                IrType::Unknown(method_name.to_string()),
                Ownership::Shared,
                method_name.to_string(),
                CallResolution::Unknown,
            ))
        }
        (_, "Wait") => Ok((IrType::Void, Ownership::Copy, name.to_string(), CallResolution::TaskMethod)),
        _ => Ok((
            IrType::Unknown(name.to_string()),
            Ownership::Shared,
            name.to_string(),
            CallResolution::Unknown,
        )),
    }
}

fn type_syntax_to_ir(ty: &TypeSyntax, env: &TypeEnv) -> IrType {
    type_syntax_to_ir_with_subst(ty, env, &HashMap::new())
}

fn type_syntax_to_ir_with_subst(
    ty: &TypeSyntax,
    env: &TypeEnv,
    subst: &HashMap<String, TypeSyntax>,
) -> IrType {
    match ty {
        TypeSyntax::GenericNamed { name, args }
            if name == "own"
                || name == "shared"
                || name == "borrow"
                || name == "view"
                || name == "System.Ownership.own"
                || name == "System.Ownership.shared"
                || name == "System.Ownership.borrow"
                || name == "System.Ownership.view" =>
        {
            if let Some(first_arg) = args.first() {
                type_syntax_to_ir_with_subst(first_arg, env, subst)
            } else {
                IrType::Unknown("T".to_string())
            }
        }
        TypeSyntax::Scalar(scalar) => scalar_to_ir(*scalar),
        TypeSyntax::String => IrType::String,
        TypeSyntax::Array(inner) => {
            IrType::Array(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Ref(inner) => {
            IrType::Ref(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Named(name) if name == "Exception" || name == "System.Exception" => {
            IrType::Exception
        }
        TypeSyntax::Named(name) if name == "Action" || name == "System.Action" => {
            IrType::Function {
                params: vec![],
                return_type: Box::new(IrType::Void),
            }
        }
        TypeSyntax::Named(name) => {
            if let Some(replacement) = subst.get(name) {
                if matches!(replacement, TypeSyntax::Named(replacement_name) if replacement_name == name) {
                    return IrType::Unknown(name.clone());
                }
                return type_syntax_to_ir_with_subst(replacement, env, subst);
            }
            if let Some(signature) = env.delegates.get(name) {
                return delegate_signature_to_ir(signature, &[], env, subst);
            }
            match env.kinds.get(name) {
                Some(TypeKind::Class) => IrType::Class(name.clone()),
                Some(TypeKind::Interface) => IrType::Interface(name.clone()),
                Some(TypeKind::Enum) => IrType::Int,
                Some(_) => IrType::Struct(name.clone()),
                None => IrType::Unknown(name.clone()),
            }
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "Rc" || name == "System.Ownership.Rc" =>
        {
            if let Some(first_arg) = args.first() {
                IrType::Struct(rc_runtime_type_name(&type_syntax_to_ir_with_subst(
                    first_arg, env, subst,
                )))
            } else {
                IrType::Struct("Rc_T".to_string())
            }
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "Weak"
                || name == "WeakReference"
                || name == "System.WeakReference"
                || name == "System.Ownership.weakref"
                || name == "weakref" =>
        {
            if let Some(first_arg) = args.first() {
                IrType::Weak(Box::new(type_syntax_to_ir_with_subst(first_arg, env, subst)))
            } else {
                IrType::Weak(Box::new(IrType::Unknown("T".to_string())))
            }
        }
        TypeSyntax::GenericNamed { name, args } if name == "Func" || name == "System.Func" => {
            if args.is_empty() {
                IrType::Function {
                    params: vec![],
                    return_type: Box::new(IrType::Void),
                }
            } else {
                let (params_args, ret_arg) = args.split_at(args.len() - 1);
                let params = params_args
                    .iter()
                    .map(|arg| type_syntax_to_ir_with_subst(arg, env, subst))
                    .collect();
                let return_type = Box::new(type_syntax_to_ir_with_subst(&ret_arg[0], env, subst));
                IrType::Function {
                    params,
                    return_type,
                }
            }
        }
        TypeSyntax::GenericNamed { name, args } if name == "Action" || name == "System.Action" => {
            let params = args
                .iter()
                .map(|arg| type_syntax_to_ir_with_subst(arg, env, subst))
                .collect();
            IrType::Function {
                params,
                return_type: Box::new(IrType::Void),
            }
        }
        TypeSyntax::GenericNamed { name, args } => {
            if let Some(replacement) = subst.get(name) {
                if matches!(replacement, TypeSyntax::Named(replacement_name) if replacement_name == name) {
                    return IrType::Unknown(format!("{name}<>"));
                }
                return type_syntax_to_ir_with_subst(replacement, env, subst);
            }
            if (name == "List" || name == "System.Collections.Generic.List") && args.len() == 1 {
                return IrType::List(Box::new(type_syntax_to_ir_with_subst(
                    &args[0],
                    env,
                    subst,
                )));
            }
            if (name == "Dictionary" || name == "System.Collections.Generic.Dictionary") && args.len() == 2 {
                return IrType::Dictionary(
                    Box::new(type_syntax_to_ir_with_subst(&args[0], env, subst)),
                    Box::new(type_syntax_to_ir_with_subst(&args[1], env, subst)),
                );
            }
            if let Some(signature) = env.delegates.get(name) {
                return delegate_signature_to_ir(signature, args, env, subst);
            }
            if is_collection_interface_name(name) {
                if args.len() == 1 {
                    return IrType::Enumerable(Box::new(type_syntax_to_ir_with_subst(
                        &args[0],
                        env,
                        subst,
                    )));
                }
                return IrType::Unknown(format!("{name}<>"));
            }
            if name == "IReadOnlyDictionary" || name == "System.Collections.Generic.IReadOnlyDictionary" {
                if args.len() == 2 {
                    return IrType::Dictionary(
                        Box::new(type_syntax_to_ir_with_subst(&args[0], env, subst)),
                        Box::new(type_syntax_to_ir_with_subst(&args[1], env, subst)),
                    );
                }
            }
            let concrete_args = args
                .iter()
                .map(|arg| type_syntax_to_ir_with_subst(arg, env, subst))
                .collect::<Vec<_>>();
            let concrete_name = monomorphized_type_name(name, &concrete_args);
            match env.kinds.get(name) {
                Some(TypeKind::Class) => IrType::Class(concrete_name),
                Some(TypeKind::Interface) => IrType::Interface(concrete_name),
                Some(TypeKind::Enum) => IrType::Int,
                Some(_) => IrType::Struct(concrete_name),
                None => IrType::Unknown(concrete_name),
            }
        }
        TypeSyntax::List(inner) => {
            IrType::List(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Dictionary(key, value) => IrType::Dictionary(
            Box::new(type_syntax_to_ir_with_subst(key, env, subst)),
            Box::new(type_syntax_to_ir_with_subst(value, env, subst)),
        ),
        TypeSyntax::IEnumerable(inner) => {
            IrType::Enumerable(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Thread => IrType::Thread,
        TypeSyntax::Task(inner) => {
            IrType::Task(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Nullable(inner) => {
            let inner = type_syntax_to_ir_with_subst(inner, env, subst);
            if is_nullable_value_type(&inner) {
                IrType::Nullable(Box::new(inner))
            } else {
                inner
            }
        }
        TypeSyntax::Void => IrType::Void,
    }
}

fn is_nullable_value_type(ty: &IrType) -> bool {
    matches!(
        ty,
        IrType::Bool
            | IrType::Byte
            | IrType::Short
            | IrType::Int
            | IrType::Long
            | IrType::UInt
            | IrType::Double
            | IrType::Decimal
            | IrType::Struct(_)
            | IrType::Unknown(_)
    )
}

fn ownership_for_declared_type_syntax(ty: &TypeSyntax, env: &TypeEnv) -> Ownership {
    match ty {
        TypeSyntax::GenericNamed { name, args }
            if name == "own" || name == "System.Ownership.own" =>
        {
            let _ = args;
            Ownership::Owned
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "shared" || name == "System.Ownership.shared" =>
        {
            let _ = args;
            Ownership::Shared
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "borrow" || name == "System.Ownership.borrow" =>
        {
            let _ = args;
            Ownership::Borrowed
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "view" || name == "System.Ownership.view" =>
        {
            let _ = args;
            Ownership::View
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "weakref" || name == "System.Ownership.weakref" =>
        {
            let _ = args;
            Ownership::Copy
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "IReadOnlyDictionary"
                || name == "System.Collections.Generic.IReadOnlyDictionary" =>
        {
            let _ = args;
            Ownership::View
        }
        TypeSyntax::GenericNamed { name, .. } if is_collection_interface_name(name) => {
            Ownership::View
        }
        _ => ownership_for_type(&type_syntax_to_ir(ty, env)),
    }
}

fn is_collection_interface_name(name: &str) -> bool {
    matches!(
        name,
        "IEnumerable"
            | "System.Collections.Generic.IEnumerable"
            | "ICollection"
            | "System.Collections.Generic.ICollection"
    )
}

fn delegate_signature_to_ir(
    signature: &DelegateSignature,
    args: &[TypeSyntax],
    env: &TypeEnv,
    outer_subst: &HashMap<String, TypeSyntax>,
) -> IrType {
    let mut subst = outer_subst.clone();
    for (index, generic_name) in signature.generic_params.iter().enumerate() {
        let replacement = args
            .get(index)
            .cloned()
            .unwrap_or_else(|| TypeSyntax::Named(format!("__open_generic_{generic_name}")));
        subst.insert(generic_name.clone(), replacement);
    }
    let params = signature
        .params
        .iter()
        .map(|param| type_syntax_to_ir_with_subst(param, env, &subst))
        .collect();
    let return_type = Box::new(type_syntax_to_ir_with_subst(&signature.return_type, env, &subst));
    IrType::Function {
        params,
        return_type,
    }
}

fn scalar_to_ir(scalar: ScalarType) -> IrType {
    match scalar {
        ScalarType::Bool => IrType::Bool,
        ScalarType::Byte => IrType::Byte,
        ScalarType::Short => IrType::Short,
        ScalarType::I32 => IrType::Int,
        ScalarType::I64 => IrType::Long,
        ScalarType::U32 => IrType::UInt,
        ScalarType::F64 => IrType::Double,
        ScalarType::Decimal => IrType::Decimal,
    }
}

pub(crate) fn ownership_for_type(ty: &IrType) -> Ownership {
    match ty {
        IrType::Bool
        | IrType::Byte
        | IrType::Short
        | IrType::Int
        | IrType::Long
        | IrType::UInt
        | IrType::Double
        | IrType::Decimal
        | IrType::Void
        | IrType::Weak(_) => Ownership::Copy,
        IrType::Function { .. } | IrType::Nullable(_) => Ownership::Shared,
        IrType::Ref(_) => Ownership::Borrowed,
        IrType::Enumerable(_) => Ownership::View,
        IrType::Class(_) | IrType::Interface(_) => Ownership::Shared,
        IrType::String
        | IrType::Array(_)
        | IrType::Struct(_)
        | IrType::List(_)
        | IrType::Dictionary(_, _)
        | IrType::Thread
        | IrType::Task(_)
        | IrType::Exception => Ownership::Owned,
        IrType::Unknown(_) => Ownership::Shared,
    }
}

pub(crate) fn drop_kind_for_type(ty: &IrType, ownership: &Ownership) -> DropKind {
    match ownership {
        Ownership::Copy => DropKind::None,
        Ownership::Borrowed => DropKind::BorrowOnly,
        Ownership::View => DropKind::ViewOnly,
        Ownership::Moved => DropKind::None,
        Ownership::Shared => match ty {
            IrType::Class(_) | IrType::Interface(_) | IrType::Nullable(_) => DropKind::DropClass,
            IrType::Function { .. } => DropKind::DropDelegate,
            _ => DropKind::None,
        },
        Ownership::Owned => match ty {
            IrType::String | IrType::Array(_) => DropKind::Free,
            IrType::Struct(_) => DropKind::DropStruct,
            IrType::Class(_) | IrType::Interface(_) => DropKind::DropClass,
            IrType::List(_) | IrType::Dictionary(_, _) => DropKind::DropCollection,
            IrType::Task(_) => DropKind::DropTask,
            IrType::Exception => DropKind::DropException,
            _ => DropKind::None,
        },
    }
}
