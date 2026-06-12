use std::collections::HashMap;

use crate::ast::*;

#[derive(Debug, Clone, Default)]
struct VarState {
    moved: bool,
    shared_borrows: usize,
    mutable_borrowed: bool,
}

#[derive(Debug, Clone, Default)]
struct BorrowState {
    vars: HashMap<String, VarState>,
    ref_targets: HashMap<String, String>,
    scopes: Vec<Vec<String>>,
}

impl BorrowState {
    fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
            ..Self::default()
        }
    }

    fn declare(&mut self, name: &str) {
        self.vars.entry(name.to_string()).or_default();
        self.scopes.last_mut().unwrap().push(name.to_string());
    }

    fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn pop_scope(&mut self) {
        let Some(scope) = self.scopes.pop() else {
            return;
        };
        for name in scope.iter().rev() {
            if let Some(target) = self.ref_targets.remove(name) {
                if let Some(target_state) = self.vars.get_mut(&target) {
                    if target_state.mutable_borrowed {
                        target_state.mutable_borrowed = false;
                    } else {
                        target_state.shared_borrows = target_state.shared_borrows.saturating_sub(1);
                    }
                }
            }
            self.vars.remove(name);
        }
    }

    fn ensure_available(&self, name: &str) -> Result<(), String> {
        let Some(state) = self.vars.get(name) else {
            return Ok(());
        };
        if state.moved {
            Err(format!("borrow checker: use of moved value '{name}'"))
        } else {
            Ok(())
        }
    }

    fn ensure_assignable(&self, name: &str) -> Result<(), String> {
        let Some(state) = self.vars.get(name) else {
            return Ok(());
        };
        if state.shared_borrows > 0 || state.mutable_borrowed {
            Err(format!(
                "borrow checker: cannot assign to '{name}' while it is borrowed"
            ))
        } else {
            Ok(())
        }
    }

    fn move_value(&mut self, name: &str) -> Result<(), String> {
        self.ensure_available(name)?;
        let Some(state) = self.vars.get_mut(name) else {
            return Ok(());
        };
        if state.shared_borrows > 0 || state.mutable_borrowed {
            return Err(format!(
                "borrow checker: cannot move '{name}' while it is borrowed"
            ));
        }
        state.moved = true;
        Ok(())
    }

    fn borrow(&mut self, name: &str, mutable: bool) -> Result<(), String> {
        self.ensure_available(name)?;
        let Some(state) = self.vars.get_mut(name) else {
            return Ok(());
        };
        if mutable {
            if state.shared_borrows > 0 || state.mutable_borrowed {
                return Err(format!(
                    "borrow checker: cannot mutably borrow '{name}' while it is borrowed"
                ));
            }
            state.mutable_borrowed = true;
        } else if state.mutable_borrowed {
            return Err(format!(
                "borrow checker: cannot borrow '{name}' while it is mutably borrowed"
            ));
        } else {
            state.shared_borrows += 1;
        }
        Ok(())
    }

    fn bind_borrow(&mut self, reference_name: &str, target: &str) {
        self.ref_targets
            .insert(reference_name.to_string(), target.to_string());
    }
}

pub(crate) struct BorrowChecker;

impl BorrowChecker {
    pub(crate) fn check_program(program: &Program) -> Result<(), String> {
        for function in &program.functions {
            Self::check_function(function, &[])?;
        }
        for ty in &program.types {
            for constructor in &ty.constructors {
                let mut state = BorrowState::new();
                state.declare("this");
                for param in &constructor.params {
                    state.declare(&param.name);
                }
                Self::check_stmts(&constructor.body, &mut state)?;
            }
            for method in &ty.methods {
                Self::check_function(method, &["this"])?;
            }
        }
        Ok(())
    }

    fn check_function(function: &Function, implicit_params: &[&str]) -> Result<(), String> {
        let mut state = BorrowState::new();
        for param in implicit_params {
            state.declare(param);
        }
        for param in &function.params {
            state.declare(&param.name);
        }
        Self::check_stmts(&function.body, &mut state)
    }

    fn check_stmts(stmts: &[Stmt], state: &mut BorrowState) -> Result<(), String> {
        for stmt in stmts {
            Self::check_stmt(stmt, state)?;
        }
        Ok(())
    }

    fn check_stmt(stmt: &Stmt, state: &mut BorrowState) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, expr, .. } => {
                Self::check_expr(expr, state)?;
                state.declare(name);
                if let Expr::Borrow { name: target, .. } = expr {
                    state.bind_borrow(name, target);
                }
            }
            Stmt::Assign { name, expr } => {
                state.ensure_assignable(name)?;
                Self::check_expr(expr, state)?;
                if let Some(var) = state.vars.get_mut(name) {
                    var.moved = false;
                }
            }
            Stmt::AssignTarget { target, expr } => {
                Self::check_expr(target, state)?;
                Self::check_expr(expr, state)?;
            }
            Stmt::Block(body) => {
                state.push_scope();
                Self::check_stmts(body, state)?;
                state.pop_scope();
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                Self::check_expr(condition, state)?;
                let mut then_state = state.clone();
                then_state.push_scope();
                Self::check_stmts(then_body, &mut then_state)?;
                let mut else_state = state.clone();
                else_state.push_scope();
                Self::check_stmts(else_body, &mut else_state)?;
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                let mut try_state = state.clone();
                try_state.push_scope();
                Self::check_stmts(try_body, &mut try_state)?;
                if let Some(catch) = catch {
                    let mut catch_state = state.clone();
                    catch_state.push_scope();
                    if let Some(name) = &catch.name {
                        catch_state.declare(name);
                    }
                    Self::check_stmts(&catch.body, &mut catch_state)?;
                }
                let mut finally_state = state.clone();
                finally_state.push_scope();
                Self::check_stmts(finally_body, &mut finally_state)?;
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                Self::check_expr(expr, state)?;
                for case in cases {
                    Self::check_expr(&case.value, state)?;
                    let mut case_state = state.clone();
                    case_state.push_scope();
                    Self::check_stmts(&case.body, &mut case_state)?;
                }
                let mut default_state = state.clone();
                default_state.push_scope();
                Self::check_stmts(default, &mut default_state)?;
            }
            Stmt::While { condition, body } => {
                Self::check_expr(condition, state)?;
                let mut body_state = state.clone();
                body_state.push_scope();
                Self::check_stmts(body, &mut body_state)?;
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                let mut loop_state = state.clone();
                loop_state.push_scope();
                if let Some(init) = init {
                    Self::check_stmt(init, &mut loop_state)?;
                }
                if let Some(condition) = condition {
                    Self::check_expr(condition, &mut loop_state)?;
                }
                Self::check_stmts(body, &mut loop_state)?;
                if let Some(increment) = increment {
                    Self::check_stmt(increment, &mut loop_state)?;
                }
            }
            Stmt::ForEach {
                item_name,
                collection,
                body,
                ..
            } => {
                Self::check_expr(collection, state)?;
                let mut loop_state = state.clone();
                loop_state.push_scope();
                loop_state.declare(item_name);
                Self::check_stmts(body, &mut loop_state)?;
            }
            Stmt::Print(expr) | Stmt::Expr(expr) | Stmt::Throw(expr) => {
                Self::check_expr(expr, state)?;
            }
            Stmt::Return(Some(expr)) => Self::check_expr(expr, state)?,
            Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
        }
        Ok(())
    }

    fn check_expr(expr: &Expr, state: &mut BorrowState) -> Result<(), String> {
        match expr {
            Expr::Int(_) | Expr::Float(_) | Expr::Bool(_) | Expr::Null | Expr::String(_) => {}
            Expr::Var(name) => state.ensure_available(name)?,
            Expr::Move(name) => state.move_value(name)?,
            Expr::ArrayLiteral(values) => {
                for value in values {
                    Self::check_expr(value, state)?;
                }
            }
            Expr::NewArray { values, .. } => {
                for value in values {
                    Self::check_expr(value, state)?;
                }
            }
            Expr::Index { target, index } => {
                Self::check_expr(target, state)?;
                Self::check_expr(index, state)?;
            }
            Expr::Field { target, .. } => Self::check_expr(target, state)?,
            Expr::IsPattern { expr, .. } => Self::check_expr(expr, state)?,
            Expr::MethodCall { target, args, .. } => {
                Self::check_expr(target, state)?;
                for arg in args {
                    Self::check_expr(arg, state)?;
                }
            }
            Expr::FunctionCall { args, .. } => {
                for arg in args {
                    Self::check_expr(arg, state)?;
                }
            }
            Expr::NewObject { args, fields, .. } => {
                for arg in args {
                    Self::check_expr(arg, state)?;
                }
                for field in fields {
                    Self::check_expr(&field.expr, state)?;
                }
            }
            Expr::NewCollection(_) | Expr::NewThread(_) => {}
            Expr::Borrow { mutable, name } => state.borrow(name, *mutable)?,
            Expr::Await(inner) => Self::check_expr(inner, state)?,
            Expr::Lambda { params, body } => {
                state.push_scope();
                for param in params {
                    state.declare(param);
                }
                Self::check_expr(body, state)?;
                state.pop_scope();
            }
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                Self::check_expr(condition, state)?;
                Self::check_expr(when_true, state)?;
                Self::check_expr(when_false, state)?;
            }
            Expr::Unary { expr, .. } => Self::check_expr(expr, state)?,
            Expr::IncDec { name, .. } => state.ensure_available(name)?,
            Expr::Binary { left, right, .. } => {
                Self::check_expr(left, state)?;
                Self::check_expr(right, state)?;
            }
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                Self::check_expr(expr, state)?;
            }
        }
        Ok(())
    }
}
