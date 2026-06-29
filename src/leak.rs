use crate::ast::Program;
use crate::tir::{
    Ownership, IrType, TypedExpr, TypedExprKind, TypedFunction, TypedLambdaBody, TypedProgram,
    TypedStmt, TypedStmtKind,
};

pub(crate) struct LeakAnalyzer {
    warnings: Vec<String>,
}

impl LeakAnalyzer {
    pub(crate) fn analyze_program(_program: &Program, typed: &TypedProgram) -> String {
        let mut analyzer = Self {
            warnings: Vec::new(),
        };
        for function in &typed.functions {
            analyzer.analyze_function(&format!("function {}", function.name), function);
        }
        for ty in &typed.types {
            for constructor in &ty.constructors {
                analyzer.analyze_function(&format!("constructor {}", ty.name), constructor);
            }
            for method in &ty.methods {
                analyzer.analyze_function(&format!("method {}.{}", ty.name, method.name), method);
            }
        }
        if analyzer.warnings.is_empty() {
            "No obvious owned temporary leaks detected.\n".to_string()
        } else {
            let mut report = String::from("Potential leak report:\n");
            for warning in analyzer.warnings {
                report.push_str("- ");
                report.push_str(&warning);
                report.push('\n');
            }
            report
        }
    }

    fn analyze_function(&mut self, context: &str, function: &TypedFunction) {
        self.analyze_stmts(context, &function.body);
    }

    fn analyze_stmts(&mut self, context: &str, stmts: &[TypedStmt]) {
        for stmt in stmts {
            self.analyze_stmt(context, stmt);
        }
    }

    fn analyze_stmt(&mut self, context: &str, stmt: &TypedStmt) {
        match &stmt.kind {
            TypedStmtKind::Expr(expr) => {
                if self.expr_may_create_owned_value(expr) {
                    self.warnings.push(format!(
                        "{context}: expression result is owned and discarded: {expr:?}"
                    ));
                }
                self.analyze_expr(context, expr);
            }
            TypedStmtKind::Let { expr, .. }
            | TypedStmtKind::Assign { expr, .. }
            | TypedStmtKind::AssignTarget { expr, .. }
            | TypedStmtKind::Print(expr)
            | TypedStmtKind::Return(Some(expr))
            | TypedStmtKind::Throw(expr) => self.analyze_expr(context, expr),
            TypedStmtKind::Block(body) | TypedStmtKind::While { body, .. } => {
                self.analyze_stmts(context, body)
            }
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr(context, condition);
                self.analyze_stmts(context, then_body);
                self.analyze_stmts(context, else_body);
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                self.analyze_stmts(context, try_body);
                self.analyze_stmts(context, catch_body);
                self.analyze_stmts(context, finally_body);
            }
            TypedStmtKind::Switch {
                expr,
                cases,
                default,
            } => {
                self.analyze_expr(context, expr);
                for case in cases {
                    self.analyze_expr(context, &case.value);
                    self.analyze_stmts(context, &case.body);
                }
                self.analyze_stmts(context, default);
            }
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                if let Some(init) = init {
                    self.analyze_stmt(context, init);
                }
                if let Some(condition) = condition {
                    self.analyze_expr(context, condition);
                }
                self.analyze_stmts(context, body);
                if let Some(increment) = increment {
                    self.analyze_stmt(context, increment);
                }
            }
            TypedStmtKind::ForEach {
                collection, body, ..
            } => {
                self.analyze_expr(context, collection);
                self.analyze_stmts(context, body);
            }
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
        }
    }

    fn analyze_expr(&mut self, context: &str, expr: &TypedExpr) {
        match &expr.kind {
            TypedExprKind::ArrayLiteral(values) => {
                for value in values {
                    self.analyze_expr(context, value);
                }
            }
            TypedExprKind::NewArray { length, values, .. } => {
                if let Some(length) = length {
                    self.analyze_expr(context, length);
                }
                for value in values {
                    self.analyze_expr(context, value);
                }
            }
            TypedExprKind::Index { target, index } => {
                self.analyze_expr(context, target);
                self.analyze_expr(context, index);
            }
            TypedExprKind::Field { target, .. } => self.analyze_expr(context, target),
            TypedExprKind::IsPattern { expr, .. } => self.analyze_expr(context, expr),
            TypedExprKind::Call(call) => {
                match &call.kind {
                    crate::tir::TypedCallKind::Function { .. } => {}
                    crate::tir::TypedCallKind::Method { target, .. } => {
                        self.analyze_expr(context, target);
                    }
                }
                for arg in &call.args {
                    self.analyze_expr(context, arg);
                }
            }
            TypedExprKind::NewObject { args, fields, .. } => {
                for arg in args {
                    self.analyze_expr(context, arg);
                }
                for field in fields {
                    self.analyze_expr(context, &field.expr);
                }
            }
            TypedExprKind::NewCollection(_) | TypedExprKind::NewThread(_) => {}
            TypedExprKind::Borrow { .. } => {}
            TypedExprKind::Await(inner)
            | TypedExprKind::Throw(inner)
            | TypedExprKind::Unary { expr: inner, .. } => self.analyze_expr(context, inner),
            TypedExprKind::Assign { target, value } => {
                self.analyze_expr(context, target);
                self.analyze_expr(context, value);
            }
            TypedExprKind::Lambda { body, .. } => match body {
                TypedLambdaBody::Expr(body) => self.analyze_expr(context, body),
                TypedLambdaBody::Block(stmts) => {
                    for stmt in stmts {
                        self.analyze_stmt(context, stmt);
                    }
                }
            },
            TypedExprKind::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.analyze_expr(context, condition);
                self.analyze_expr(context, when_true);
                self.analyze_expr(context, when_false);
            }
            TypedExprKind::IncDec { target, .. } => self.analyze_expr(context, target),
            TypedExprKind::Binary { left, right, .. } => {
                self.analyze_expr(context, left);
                self.analyze_expr(context, right);
            }
            TypedExprKind::NullableSome(inner) => self.analyze_expr(context, inner),
            TypedExprKind::Int(_)
            | TypedExprKind::Float(_)
            | TypedExprKind::Bool(_)
            | TypedExprKind::Null
            | TypedExprKind::String(_)
            | TypedExprKind::Var(_)
            | TypedExprKind::FunctionSymbol(_)
            | TypedExprKind::Move(_) => {
                let _ = context;
            }
        }
    }

    fn expr_may_create_owned_value(&self, expr: &TypedExpr) -> bool {
        match &expr.kind {
            TypedExprKind::String(_) => true,
            TypedExprKind::ArrayLiteral(_) => true,
            TypedExprKind::NewArray { .. }
            | TypedExprKind::NewCollection(_)
            | TypedExprKind::NewThread(_) => true,
            TypedExprKind::NewObject { type_name, .. } => !is_weak_reference_type_name(type_name),
            TypedExprKind::Call(call) => match &call.kind {
                crate::tir::TypedCallKind::Function { name, .. } => {
                    if is_internal_helper_function(name) {
                        false
                    } else {
                        type_may_own(&expr.ty)
                    }
                }
                crate::tir::TypedCallKind::Method { .. } => {
                    if is_framework_fluent_method(call) {
                        false
                    } else {
                        matches!(expr.ownership, Ownership::Owned)
                    }
                }
            },
            TypedExprKind::Field { target, name } => {
                matches!(expr.ownership, Ownership::Owned)
                    && (!matches!(target.ty, IrType::Task(_))
                        || matches!(name.as_str(), "Result" | "Exception"))
            }
            TypedExprKind::Index { .. }
            | TypedExprKind::Await(_)
            | TypedExprKind::Conditional { .. }
            | TypedExprKind::NullableSome(_)
            | TypedExprKind::Unary { .. }
            | TypedExprKind::Binary { .. } => matches!(expr.ownership, Ownership::Owned),
            TypedExprKind::Throw(_) | TypedExprKind::Lambda { .. } => false,
            TypedExprKind::Var(_)
            | TypedExprKind::FunctionSymbol(_)
            | TypedExprKind::Move(_)
            | TypedExprKind::Borrow { .. }
            | TypedExprKind::Int(_)
            | TypedExprKind::Float(_)
            | TypedExprKind::Bool(_)
            | TypedExprKind::Null
            | TypedExprKind::Assign { .. }
            | TypedExprKind::IsPattern { .. }
            | TypedExprKind::IncDec { .. } => false,
        }
    }
}

fn type_may_own(ty: &IrType) -> bool {
    match ty {
        IrType::String
        | IrType::Array(_)
        | IrType::Struct(_)
        | IrType::List(_)
        | IrType::Dictionary(_, _)
        | IrType::Thread
        | IrType::Task(_)
        | IrType::Exception => true,
        IrType::Nullable(inner) => type_may_own(inner),
        IrType::Class(_) | IrType::Interface(_) | IrType::Function { .. } | IrType::Unknown(_) => {
            true
        }
        IrType::Bool
        | IrType::Byte
        | IrType::Short
        | IrType::Int
        | IrType::Long
        | IrType::UInt
        | IrType::Double
        | IrType::Decimal
        | IrType::Void
        | IrType::Ref(_)
        | IrType::Enumerable(_)
        | IrType::Weak(_) => false,
    }
}

fn is_weak_reference_type_name(name: &str) -> bool {
    name == "Weak"
        || name == "WeakReference"
        || name == "System.WeakReference"
        || name == "System.Ownership.Weak"
        || name.starts_with("Weak<")
        || name.starts_with("WeakReference<")
        || name.starts_with("System.WeakReference<")
        || name.starts_with("WeakReference_")
        || name.starts_with("System_WeakReference_")
        || name.starts_with("System_Ownership_Weak")
        || name.starts_with("Weak_")
}

fn is_framework_fluent_method(call: &crate::tir::TypedCall) -> bool {
    let crate::tir::TypedCallKind::Method { name, .. } = &call.kind else {
        return false;
    };
    matches!(
        name.as_str(),
        "Use"
            | "UseTrace"
            | "MapGet"
            | "MapPost"
            | "MapPut"
            | "MapDelete"
            | "AddDbContext"
            | "AddLocalization"
            | "AddCors"
            | "AddMvc"
            | "AddJsonOptions"
            | "Run"
            | "RunOnce"
            | "Build"
            | "BuildServiceProvider"
            | "Configure"
    )
}

fn is_internal_helper_function(name: &str) -> bool {
    matches!(name, "configure" | "function")
        || name.starts_with("GlitchRestHost_")
        || name.starts_with("glitch_register_")
        || name.starts_with("glitch_endpoint_handlers_")
}
