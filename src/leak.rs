use std::collections::HashMap;

use crate::ast::*;
use crate::tir::TypedProgram;

pub(crate) struct LeakAnalyzer<'a> {
    functions: HashMap<&'a str, &'a TypeSyntax>,
    warnings: Vec<String>,
}

impl<'a> LeakAnalyzer<'a> {
    pub(crate) fn analyze_program(program: &'a Program, typed: &TypedProgram) -> String {
        let mut analyzer = Self {
            functions: program
                .functions
                .iter()
                .map(|f| (f.name.as_str(), &f.return_type))
                .collect(),
            warnings: Vec::new(),
        };
        let _ownership_summary = typed.ownership_summary();
        for function in &program.functions {
            analyzer.analyze_stmts(&format!("function {}", function.name), &function.body);
        }
        for ty in &program.types {
            for constructor in &ty.constructors {
                analyzer.analyze_stmts(&format!("constructor {}", ty.name), &constructor.body);
            }
            for method in &ty.methods {
                analyzer
                    .analyze_stmts(&format!("method {}.{}", ty.name, method.name), &method.body);
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

    fn analyze_stmts(&mut self, context: &str, stmts: &[Stmt]) {
        for stmt in stmts {
            self.analyze_stmt(context, stmt);
        }
    }

    fn analyze_stmt(&mut self, context: &str, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                if self.expr_may_create_owned_value(expr) {
                    self.warnings.push(format!(
                        "{context}: expression result is owned and discarded: {expr:?}"
                    ));
                }
                self.analyze_expr(context, expr);
            }
            Stmt::Let { expr, .. }
            | Stmt::Assign { expr, .. }
            | Stmt::AssignTarget { expr, .. }
            | Stmt::Print(expr)
            | Stmt::Return(Some(expr))
            | Stmt::Throw(expr) => self.analyze_expr(context, expr),
            Stmt::Block(body) | Stmt::While { body, .. } => self.analyze_stmts(context, body),
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr(context, condition);
                self.analyze_stmts(context, then_body);
                self.analyze_stmts(context, else_body);
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                self.analyze_stmts(context, try_body);
                if let Some(catch) = catch {
                    self.analyze_stmts(context, &catch.body);
                }
                self.analyze_stmts(context, finally_body);
            }
            Stmt::Switch {
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
            Stmt::For {
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
            Stmt::ForEach {
                collection, body, ..
            } => {
                self.analyze_expr(context, collection);
                self.analyze_stmts(context, body);
            }
            Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
        }
    }

    fn analyze_expr(&mut self, context: &str, expr: &Expr) {
        match expr {
            Expr::ArrayLiteral(values) => {
                for value in values {
                    self.analyze_expr(context, value);
                }
            }
            Expr::NewArray { length, values, .. } => {
                if let Some(length) = length {
                    self.analyze_expr(context, length);
                }
                for value in values {
                    self.analyze_expr(context, value);
                }
            }
            Expr::Index { target, index } => {
                self.analyze_expr(context, target);
                self.analyze_expr(context, index);
            }
            Expr::Field { target, .. } => self.analyze_expr(context, target),
            Expr::IsPattern { expr, .. } => self.analyze_expr(context, expr),
            Expr::MethodCall { target, args, .. } => {
                self.analyze_expr(context, target);
                for arg in args {
                    self.analyze_expr(context, arg);
                }
            }
            Expr::FunctionCall { args, .. } => {
                for arg in args {
                    self.analyze_expr(context, arg);
                }
            }
            Expr::Throw(expr) => self.analyze_expr(context, expr),
            Expr::Assign { target, value } => {
                self.analyze_expr(context, target);
                self.analyze_expr(context, value);
            }
            Expr::NewObject { args, fields, .. } => {
                for arg in args {
                    self.analyze_expr(context, arg);
                }
                for field in fields {
                    self.analyze_expr(context, &field.expr);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.analyze_expr(context, left);
                self.analyze_expr(context, right);
            }
            Expr::Unary { expr, .. } => self.analyze_expr(context, expr),
            Expr::IncDec { .. } => {}
            Expr::Lambda { body, .. } => self.analyze_expr(context, body),
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.analyze_expr(context, condition);
                self.analyze_expr(context, when_true);
                self.analyze_expr(context, when_false);
            }
            Expr::Await(inner) => self.analyze_expr(context, inner),
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                self.analyze_expr(context, expr)
            }
            Expr::Int(_)
            | Expr::Float(_)
            | Expr::Bool(_)
            | Expr::Null
            | Expr::String(_)
            | Expr::Var(_)
            | Expr::Move(_)
            | Expr::NewCollection(_)
            | Expr::NewThread(_)
            | Expr::Borrow { .. } => {
                let _ = context;
            }
        }
    }

    fn expr_may_create_owned_value(&self, expr: &Expr) -> bool {
        match expr {
            Expr::String(_)
            | Expr::NewArray { .. }
            | Expr::NewCollection(_)
            | Expr::NewThread(_) => true,
            Expr::NewObject { type_name, .. } => !is_weak_reference_type_name(type_name),
            Expr::FunctionCall { name, .. } => self
                .functions
                .get(name.as_str())
                .is_some_and(|ty| type_may_own(ty)),
            Expr::MethodCall { target, name, .. } => {
                matches!(name.as_str(), "GetResult")
                    || matches!(target.as_ref(), Expr::Var(type_name) if type_name == "Task")
            }
            Expr::Await(_) => true,
            Expr::Throw(expr) => self.expr_may_create_owned_value(expr),
            Expr::Assign { target, value } => {
                self.expr_may_create_owned_value(target)
                    || self.expr_may_create_owned_value(value)
            }
            Expr::Unary { expr, .. } => self.expr_may_create_owned_value(expr),
            Expr::Lambda { .. } => false,
            Expr::Conditional {
                when_true,
                when_false,
                ..
            } => {
                self.expr_may_create_owned_value(when_true)
                    || self.expr_may_create_owned_value(when_false)
            }
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                self.expr_may_create_owned_value(expr)
            }
            _ => false,
        }
    }
}

fn type_may_own(ty: &TypeSyntax) -> bool {
    match ty {
        TypeSyntax::String
        | TypeSyntax::Array(_)
        | TypeSyntax::List(_)
        | TypeSyntax::Dictionary(_, _)
        | TypeSyntax::Thread
        | TypeSyntax::Task(_) => true,
        TypeSyntax::IEnumerable(_) => false,
        TypeSyntax::Named(name) => !is_weak_reference_name(name),
        TypeSyntax::GenericNamed { name, .. } => {
            let name = name.as_str();
            matches!(name, "own" | "System.Ownership.own")
                || !matches!(
                    name,
                    "borrow"
                        | "view"
                        | "shared"
                        | "IReadOnlyDictionary"
                        | "System.Collections.Generic.IReadOnlyDictionary"
                        | "weakref"
                        | "System.Ownership.borrow"
                        | "System.Ownership.view"
                        | "System.Ownership.shared"
                        | "System.Ownership.weakref"
                ) && !is_weak_reference_name(name)
        }
        TypeSyntax::Nullable(inner) => type_may_own(inner),
        _ => false,
    }
}

fn is_weak_reference_name(name: &str) -> bool {
    matches!(
        name,
        "Weak"
            | "WeakReference"
            | "System.WeakReference"
            | "System.Ownership.Weak"
            | "weakref"
            | "System.Ownership.weakref"
    )
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
