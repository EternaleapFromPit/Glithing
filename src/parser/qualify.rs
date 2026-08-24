use std::collections::HashMap;

use crate::ast::*;

/// A type declared somewhere that shares its simple name with at least one
/// other type in the program (a real, observed pattern: MediatR's
/// convention of nesting `Query`/`Command`/`Handler` inside a per-feature
/// grouping class means many unrelated features declare a type with the
/// exact same simple name — `Articles.Create`, `Comments.Create`,
/// `Users.Create`, etc.). Kept so references elsewhere in the program can be
/// resolved to the one candidate that's actually in scope.
#[derive(Clone)]
struct Candidate {
    namespace: Vec<String>,
    enclosing_types: Vec<String>,
    qualified: String,
}

/// Renames every type whose simple name collides with another type
/// somewhere else in the program to its full qualified name (namespace +
/// enclosing-type chain + simple name), then rewrites every reference to a
/// renamed type — base-class lists, field/parameter/return/local types,
/// `new X(...)` constructions — to the correct qualified form for the scope
/// each reference appears in. Types with a unique simple name (the vast
/// majority) are left completely untouched, so this only changes behavior
/// for the specific names that were actually ambiguous.
pub(crate) fn qualify_colliding_types(program: &mut Program) {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for ty in &program.types {
        *counts.entry(ty.name.clone()).or_default() += 1;
    }

    let mut candidates_by_old_name: HashMap<String, Vec<Candidate>> = HashMap::new();
    for ty in program.types.iter_mut() {
        if counts.get(&ty.name).copied().unwrap_or(0) <= 1 {
            continue;
        }
        let qualified = qualified_type_path(&ty.namespace, &ty.enclosing_types, &ty.name);
        candidates_by_old_name
            .entry(ty.name.clone())
            .or_default()
            .push(Candidate {
                namespace: ty.namespace.clone(),
                enclosing_types: ty.enclosing_types.clone(),
                qualified: qualified.clone(),
            });
        ty.name = qualified;
    }

    if candidates_by_old_name.is_empty() {
        return;
    }

    let resolver = Resolver {
        candidates: &candidates_by_old_name,
    };
    for ty in program.types.iter_mut() {
        let scope = Scope {
            namespace: ty.namespace.clone(),
            enclosing_types: ty.enclosing_types.clone(),
        };
        for base in ty.bases.iter_mut() {
            resolver.resolve_type_string(base, &scope);
        }
        for field in ty.fields.iter_mut() {
            resolver.resolve_type_syntax(&mut field.ty, &scope);
            if let Some(initializer) = &mut field.initializer {
                resolver.resolve_expr(initializer, &scope);
            }
        }
        for constructor in ty.constructors.iter_mut() {
            for param in constructor.params.iter_mut() {
                resolver.resolve_type_syntax(&mut param.ty, &scope);
            }
            resolver.resolve_stmts(&mut constructor.body, &scope);
        }
        for method in ty.methods.iter_mut() {
            resolver.resolve_function(method, &scope);
        }
    }
    for function in program.functions.iter_mut() {
        let scope = Scope {
            namespace: function.namespace.clone(),
            enclosing_types: Vec::new(),
        };
        resolver.resolve_function(function, &scope);
    }
}

struct Scope {
    namespace: Vec<String>,
    enclosing_types: Vec<String>,
}

struct Resolver<'a> {
    candidates: &'a HashMap<String, Vec<Candidate>>,
}

impl Resolver<'_> {
    /// Picks the candidate for a colliding simple name that's actually
    /// reachable from `scope`, preferring (in order): an exact match on the
    /// scope's own namespace + enclosing-type chain, then a match on
    /// namespace alone, then — only if just one candidate remains after
    /// that — the sole survivor. An unresolved ambiguity is left as-is
    /// rather than guessed at.
    fn pick<'c>(&self, candidates: &'c [Candidate], scope: &Scope) -> Option<&'c Candidate> {
        if let Some(exact) = candidates
            .iter()
            .find(|c| c.namespace == scope.namespace && c.enclosing_types == scope.enclosing_types)
        {
            return Some(exact);
        }
        let same_namespace = candidates
            .iter()
            .filter(|c| c.namespace == scope.namespace)
            .collect::<Vec<_>>();
        if same_namespace.len() == 1 {
            return Some(same_namespace[0]);
        }
        if candidates.len() == 1 {
            return Some(&candidates[0]);
        }
        None
    }

    /// Resolves one raw type-name string (bare `"Query"`, one-level
    /// qualified `"List.Query"`, or an unrelated non-colliding name) against
    /// `scope`, returning the canonical qualified name if it refers to a
    /// renamed (colliding) type.
    fn resolve_name(&self, raw: &str, scope: &Scope) -> Option<String> {
        let leaf = raw.rsplit('.').next().unwrap_or(raw);
        let candidates = self.candidates.get(leaf)?;
        if raw.contains('.') {
            let prefix = &raw[..raw.len() - leaf.len() - 1];
            let matching = candidates
                .iter()
                .filter(|c| c.enclosing_types.last().map(|s| s.as_str()) == Some(prefix))
                .cloned()
                .collect::<Vec<_>>();
            return self.pick(&matching, scope).map(|c| c.qualified.clone());
        }
        self.pick(candidates, scope).map(|c| c.qualified.clone())
    }

    /// Resolves a base-list / generic-instantiation style string such as
    /// `"IRequestHandler<Query,Envelope>"`, rewriting the head and each
    /// generic argument independently and rejoining them.
    fn resolve_type_string(&self, raw: &mut String, scope: &Scope) {
        if let Some(open) = raw.find('<') {
            if raw.ends_with('>') {
                let head = raw[..open].to_string();
                let args_text = &raw[open + 1..raw.len() - 1];
                let mut args = split_top_level_commas(args_text)
                    .into_iter()
                    .map(|arg| arg.trim().to_string())
                    .collect::<Vec<_>>();
                let mut changed = false;
                let resolved_head = self.resolve_name(&head, scope);
                let head = resolved_head.unwrap_or(head);
                for arg in args.iter_mut() {
                    if let Some(resolved) = self.resolve_name(arg, scope) {
                        *arg = resolved;
                        changed = true;
                    }
                }
                if changed || self.candidates.contains_key(head.rsplit('.').next().unwrap_or(&head)) {
                    *raw = format!("{head}<{}>", args.join(","));
                }
                return;
            }
        }
        if let Some(resolved) = self.resolve_name(raw, scope) {
            *raw = resolved;
        }
    }

    fn resolve_type_syntax(&self, ty: &mut TypeSyntax, scope: &Scope) {
        match ty {
            TypeSyntax::Named(name) => {
                if let Some(resolved) = self.resolve_name(name, scope) {
                    *name = resolved;
                }
            }
            TypeSyntax::GenericNamed { name, args } => {
                if let Some(resolved) = self.resolve_name(name, scope) {
                    *name = resolved;
                }
                for arg in args.iter_mut() {
                    self.resolve_type_syntax(arg, scope);
                }
            }
            TypeSyntax::Array(inner)
            | TypeSyntax::Ref(inner)
            | TypeSyntax::List(inner)
            | TypeSyntax::IEnumerable(inner)
            | TypeSyntax::Task(inner)
            | TypeSyntax::Nullable(inner) => self.resolve_type_syntax(inner, scope),
            TypeSyntax::Dictionary(key, value) => {
                self.resolve_type_syntax(key, scope);
                self.resolve_type_syntax(value, scope);
            }
            TypeSyntax::Scalar(_) | TypeSyntax::String | TypeSyntax::Thread | TypeSyntax::Void => {}
        }
    }

    fn resolve_function(&self, function: &mut Function, scope: &Scope) {
        for param in function.params.iter_mut() {
            self.resolve_type_syntax(&mut param.ty, scope);
        }
        self.resolve_type_syntax(&mut function.return_type, scope);
        self.resolve_stmts(&mut function.body, scope);
    }

    fn resolve_stmts(&self, stmts: &mut [Stmt], scope: &Scope) {
        for stmt in stmts.iter_mut() {
            self.resolve_stmt(stmt, scope);
        }
    }

    fn resolve_stmt(&self, stmt: &mut Stmt, scope: &Scope) {
        match stmt {
            Stmt::Let { declared_type, expr, .. } => {
                if let Some(ty) = declared_type {
                    self.resolve_type_syntax(ty, scope);
                }
                self.resolve_expr(expr, scope);
            }
            Stmt::Assign { expr, .. } => self.resolve_expr(expr, scope),
            Stmt::AssignTarget { target, expr } => {
                self.resolve_expr(target, scope);
                self.resolve_expr(expr, scope);
            }
            Stmt::Block(body) => self.resolve_stmts(body, scope),
            Stmt::If { condition, then_body, else_body } => {
                self.resolve_expr(condition, scope);
                self.resolve_stmts(then_body, scope);
                self.resolve_stmts(else_body, scope);
            }
            Stmt::Try { try_body, catch, finally_body } => {
                self.resolve_stmts(try_body, scope);
                if let Some(catch) = catch {
                    if let Some(ty) = &mut catch.exception_type {
                        self.resolve_type_syntax(ty, scope);
                    }
                    self.resolve_stmts(&mut catch.body, scope);
                }
                self.resolve_stmts(finally_body, scope);
            }
            Stmt::Switch { expr, cases, default } => {
                self.resolve_expr(expr, scope);
                for case in cases.iter_mut() {
                    self.resolve_expr(&mut case.value, scope);
                    self.resolve_stmts(&mut case.body, scope);
                }
                self.resolve_stmts(default, scope);
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition, scope);
                self.resolve_stmts(body, scope);
            }
            Stmt::For { init, condition, increment, body } => {
                if let Some(init) = init {
                    self.resolve_stmt(init, scope);
                }
                if let Some(condition) = condition {
                    self.resolve_expr(condition, scope);
                }
                if let Some(increment) = increment {
                    self.resolve_stmt(increment, scope);
                }
                self.resolve_stmts(body, scope);
            }
            Stmt::ForEach { item_type, collection, body, .. } => {
                self.resolve_type_syntax(item_type, scope);
                self.resolve_expr(collection, scope);
                self.resolve_stmts(body, scope);
            }
            Stmt::Print(expr) | Stmt::Expr(expr) | Stmt::Throw(expr) => self.resolve_expr(expr, scope),
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.resolve_expr(expr, scope);
                }
            }
            Stmt::Break | Stmt::Continue => {}
        }
    }

    fn resolve_expr(&self, expr: &mut Expr, scope: &Scope) {
        match expr {
            Expr::NewObject { type_name, args, fields } => {
                self.resolve_type_string(type_name, scope);
                for arg in args.iter_mut() {
                    self.resolve_expr(arg, scope);
                }
                for field in fields.iter_mut() {
                    self.resolve_expr(&mut field.expr, scope);
                }
            }
            Expr::NewArray { element_type, length, values } => {
                self.resolve_type_syntax(element_type, scope);
                if let Some(length) = length {
                    self.resolve_expr(length, scope);
                }
                for value in values.iter_mut() {
                    self.resolve_expr(value, scope);
                }
            }
            Expr::NewCollection(ty) => self.resolve_type_syntax(ty, scope),
            Expr::ArrayLiteral(values) => {
                for value in values.iter_mut() {
                    self.resolve_expr(value, scope);
                }
            }
            Expr::Index { target, index } => {
                self.resolve_expr(target, scope);
                self.resolve_expr(index, scope);
            }
            Expr::Field { target, .. } => self.resolve_expr(target, scope),
            Expr::IsPattern { expr, ty, .. } => {
                self.resolve_expr(expr, scope);
                self.resolve_type_syntax(ty, scope);
            }
            Expr::MethodCall { target, generic_args, args, .. } => {
                self.resolve_expr(target, scope);
                for arg in generic_args.iter_mut() {
                    self.resolve_type_syntax(arg, scope);
                }
                for arg in args.iter_mut() {
                    self.resolve_expr(arg, scope);
                }
            }
            Expr::FunctionCall { generic_args, args, .. } => {
                for arg in generic_args.iter_mut() {
                    self.resolve_type_syntax(arg, scope);
                }
                for arg in args.iter_mut() {
                    self.resolve_expr(arg, scope);
                }
            }
            Expr::Throw(inner)
            | Expr::Await(inner)
            | Expr::Unary { expr: inner, .. }
            | Expr::RefArg { expr: inner, .. }
            | Expr::NamedArg { expr: inner, .. } => self.resolve_expr(inner, scope),
            Expr::Conditional { condition, when_true, when_false } => {
                self.resolve_expr(condition, scope);
                self.resolve_expr(when_true, scope);
                self.resolve_expr(when_false, scope);
            }
            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left, scope);
                self.resolve_expr(right, scope);
            }
            Expr::Assign { target, value } => {
                self.resolve_expr(target, scope);
                self.resolve_expr(value, scope);
            }
            Expr::Lambda { body, .. } => match body {
                LambdaBody::Expr(inner) => self.resolve_expr(inner, scope),
                LambdaBody::Block(stmts) => self.resolve_stmts(stmts, scope),
            },
            Expr::Int(_)
            | Expr::Float(_)
            | Expr::Bool(_)
            | Expr::Null
            | Expr::String(_)
            | Expr::Var(_)
            | Expr::Move(_)
            | Expr::NewThread(_)
            | Expr::Borrow { .. }
            | Expr::IncDec { .. } => {}
        }
    }
}

/// Splits a generic-argument list on top-level commas, respecting nested
/// `<...>` so `"Dictionary<string,int>,Envelope"` splits into two arguments,
/// not four.
fn split_top_level_commas(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (index, ch) in text.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(&text[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    parts.push(&text[start..]);
    parts
}
