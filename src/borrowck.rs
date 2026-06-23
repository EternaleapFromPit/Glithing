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
    terminated: bool,
    exits_switch: bool,
    loop_exit_snapshots: Vec<LoopExitSnapshot>,
    switch_exit_snapshots: Vec<BorrowState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopExitKind {
    Break,
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FlowContext {
    in_loop: bool,
    in_switch: bool,
}

impl FlowContext {
    fn new() -> Self {
        Self {
            in_loop: false,
            in_switch: false,
        }
    }

    fn with_loop(self) -> Self {
        Self {
            in_loop: true,
            in_switch: false,
        }
    }

    fn with_switch(self) -> Self {
        Self {
            in_loop: self.in_loop,
            in_switch: true,
        }
    }
}

#[derive(Debug, Clone)]
struct LoopExitSnapshot {
    kind: LoopExitKind,
    state: BorrowState,
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

    fn merge_from_branches(base: &BorrowState, branches: &[BorrowState]) -> BorrowState {
        let mut merged = base.clone();
        let active_branches = branches
            .iter()
            .filter(|branch| !branch.terminated && !branch.exits_switch)
            .collect::<Vec<_>>();
        for (name, state) in &mut merged.vars {
            let mut moved = false;
            let mut shared_borrows = 0usize;
            let mut mutable_borrowed = false;
            for branch in &active_branches {
                if let Some(branch_state) = branch.vars.get(name) {
                    moved |= branch_state.moved;
                    shared_borrows = shared_borrows.max(branch_state.shared_borrows);
                    mutable_borrowed |= branch_state.mutable_borrowed;
                }
            }
            state.moved = moved;
            state.shared_borrows = shared_borrows;
            state.mutable_borrowed = mutable_borrowed;
        }
        merged.ref_targets = base.ref_targets.clone();
        merged.loop_exit_snapshots = branches
            .iter()
            .flat_map(|branch| branch.loop_exit_snapshots.iter().cloned())
            .collect();
        merged.switch_exit_snapshots = branches
            .iter()
            .flat_map(|branch| branch.switch_exit_snapshots.iter().cloned())
            .collect();
        merged.terminated = active_branches.is_empty();
        merged.exits_switch = branches.iter().all(|branch| branch.exits_switch);
        merged
    }

    fn merge_loop_exit_states(base: &BorrowState, branches: &[BorrowState]) -> BorrowState {
        let mut merged = base.clone();
        for (name, state) in &mut merged.vars {
            let mut moved = false;
            let mut shared_borrows = 0usize;
            let mut mutable_borrowed = false;
            for branch in branches {
                if let Some(branch_state) = branch.vars.get(name) {
                    moved |= branch_state.moved;
                    shared_borrows = shared_borrows.max(branch_state.shared_borrows);
                    mutable_borrowed |= branch_state.mutable_borrowed;
                }
            }
            state.moved = moved;
            state.shared_borrows = shared_borrows;
            state.mutable_borrowed = mutable_borrowed;
        }
        merged.ref_targets = base.ref_targets.clone();
        merged.loop_exit_snapshots = branches
            .iter()
            .flat_map(|branch| branch.loop_exit_snapshots.iter().cloned())
            .collect();
        merged.switch_exit_snapshots = branches
            .iter()
            .flat_map(|branch| branch.switch_exit_snapshots.iter().cloned())
            .collect();
        merged.terminated = false;
        merged.exits_switch = branches.iter().all(|branch| branch.exits_switch);
        merged
    }

    fn snapshot_for_loop_exit(&self) -> BorrowState {
        let mut snapshot = self.clone();
        snapshot.terminated = false;
        snapshot.exits_switch = false;
        snapshot.loop_exit_snapshots.clear();
        snapshot.switch_exit_snapshots.clear();
        snapshot
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
                Self::check_stmts(&constructor.body, &mut state, FlowContext::new())?;
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
        Self::check_stmts(&function.body, &mut state, FlowContext::new())
    }

    fn apply_finally_to_state(
        state: &BorrowState,
        finally_body: &[Stmt],
        flow: FlowContext,
    ) -> Result<BorrowState, String> {
        let mut final_state = state.clone();
        let terminated_before = final_state.terminated;
        let existing_snapshots = final_state.loop_exit_snapshots.clone();
        let existing_switch_snapshots = final_state.switch_exit_snapshots.clone();
        final_state.push_scope();
        Self::check_stmts(finally_body, &mut final_state, flow)?;
        final_state.pop_scope();
        final_state.terminated |= terminated_before;

        let mut propagated_snapshots = Vec::new();
        for snapshot in existing_snapshots {
            let mut snapshot_state = snapshot.state.clone();
            snapshot_state.push_scope();
            Self::check_stmts(finally_body, &mut snapshot_state, flow)?;
            snapshot_state.pop_scope();
            propagated_snapshots.push(LoopExitSnapshot {
                kind: snapshot.kind,
                state: snapshot_state.snapshot_for_loop_exit(),
            });
        }
        final_state.loop_exit_snapshots.extend(propagated_snapshots);

        let mut propagated_switch_snapshots = Vec::new();
        for snapshot in existing_switch_snapshots {
            let mut snapshot_state = snapshot.clone();
            snapshot_state.push_scope();
            Self::check_stmts(finally_body, &mut snapshot_state, flow)?;
            snapshot_state.pop_scope();
            propagated_switch_snapshots.push(snapshot_state);
        }
        final_state.switch_exit_snapshots.extend(propagated_switch_snapshots);
        Ok(final_state)
    }

    fn check_stmts(stmts: &[Stmt], state: &mut BorrowState, flow: FlowContext) -> Result<(), String> {
        for stmt in stmts {
            if state.terminated || state.exits_switch {
                break;
            }
            Self::check_stmt(stmt, state, flow)?;
            if state.terminated || state.exits_switch {
                break;
            }
        }
        Ok(())
    }

    fn check_stmt(stmt: &Stmt, state: &mut BorrowState, flow: FlowContext) -> Result<(), String> {
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
                Self::check_stmts(body, state, flow)?;
                state.pop_scope();
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                Self::check_expr(condition, state)?;
                let base_state = state.clone();
                let mut then_state = base_state.clone();
                then_state.push_scope();
                Self::check_stmts(then_body, &mut then_state, flow)?;
                then_state.pop_scope();
                let mut else_state = base_state.clone();
                else_state.push_scope();
                Self::check_stmts(else_body, &mut else_state, flow)?;
                else_state.pop_scope();
                *state = BorrowState::merge_from_branches(&base_state, &[then_state, else_state]);
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                let base_state = state.clone();
                let mut try_state = base_state.clone();
                try_state.push_scope();
                Self::check_stmts(try_body, &mut try_state, flow)?;
                try_state.pop_scope();
                let try_state = Self::apply_finally_to_state(&try_state, finally_body, flow)?;
                let merged_state = if let Some(catch) = catch {
                    let mut catch_state = base_state.clone();
                    catch_state.push_scope();
                    if let Some(name) = &catch.name {
                        catch_state.declare(name);
                    }
                    Self::check_stmts(&catch.body, &mut catch_state, flow)?;
                    catch_state.pop_scope();
                    let catch_state = Self::apply_finally_to_state(&catch_state, finally_body, flow)?;
                    BorrowState::merge_from_branches(&base_state, &[try_state, catch_state])
                } else {
                    try_state
                };
                *state = merged_state;
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                Self::check_expr(expr, state)?;
                let base_state = state.clone();
                let mut branch_states = Vec::new();
                let mut switch_exit_states = Vec::new();
                for case in cases {
                    let mut case_value_state = base_state.clone();
                    Self::check_expr(&case.value, &mut case_value_state)?;
                    let mut case_state = base_state.clone();
                    case_state.push_scope();
                    Self::check_stmts(&case.body, &mut case_state, flow.with_switch())?;
                    case_state.pop_scope();
                    switch_exit_states.extend(case_state.switch_exit_snapshots.iter().cloned());
                    if !case_state.terminated && !case_state.exits_switch {
                        branch_states.push(case_state);
                    }
                }
                let mut default_state = base_state.clone();
                default_state.push_scope();
                Self::check_stmts(default, &mut default_state, flow.with_switch())?;
                default_state.pop_scope();
                switch_exit_states.extend(default_state.switch_exit_snapshots.iter().cloned());
                if !default_state.terminated && !default_state.exits_switch {
                    branch_states.push(default_state);
                }
                for snapshot in switch_exit_states {
                    let mut exit_state = snapshot.clone();
                    exit_state.exits_switch = false;
                    branch_states.push(exit_state);
                }
                *state = BorrowState::merge_from_branches(&base_state, &branch_states);
            }
            Stmt::While { condition, body } => {
                Self::check_expr(condition, state)?;
                let base_state = state.clone();
                let mut body_state = base_state.clone();
                body_state.push_scope();
                Self::check_stmts(body, &mut body_state, flow.with_loop())?;
                body_state.pop_scope();
                let mut loop_branches = vec![body_state.clone()];
                loop_branches.extend(
                    body_state
                        .loop_exit_snapshots
                        .iter()
                        .map(|snapshot| snapshot.state.clone()),
                );
                *state = BorrowState::merge_loop_exit_states(&base_state, &loop_branches);
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                let base_state = state.clone();
                let mut loop_state = base_state.clone();
                loop_state.push_scope();
                if let Some(init) = init {
                    Self::check_stmt(init, &mut loop_state, flow.with_loop())?;
                }
                if let Some(condition) = condition {
                    Self::check_expr(condition, &mut loop_state)?;
                }
                Self::check_stmts(body, &mut loop_state, flow.with_loop())?;
                loop_state.pop_scope();
                let mut loop_branches = Vec::new();
                if !loop_state.terminated {
                    let mut steady_state = loop_state.clone();
                    if let Some(increment) = increment {
                        Self::check_stmt(increment, &mut steady_state, flow.with_loop())?;
                    }
                    loop_branches.push(steady_state);
                }
                for snapshot in &loop_state.loop_exit_snapshots {
                    let mut branch_state = snapshot.state.clone();
                    match snapshot.kind {
                        LoopExitKind::Break => loop_branches.push(branch_state),
                        LoopExitKind::Continue => {
                            branch_state.terminated = false;
                            if let Some(increment) = increment {
                                Self::check_stmt(increment, &mut branch_state, flow.with_loop())?;
                            }
                            loop_branches.push(branch_state);
                        }
                    }
                }
                if loop_branches.is_empty() {
                    loop_branches.push(base_state.clone());
                }
                *state = BorrowState::merge_loop_exit_states(&base_state, &loop_branches);
            }
            Stmt::ForEach {
                item_name,
                collection,
                body,
                ..
            } => {
                Self::check_expr(collection, state)?;
                let base_state = state.clone();
                let mut loop_state = base_state.clone();
                loop_state.push_scope();
                loop_state.declare(item_name);
                Self::check_stmts(body, &mut loop_state, flow.with_loop())?;
                loop_state.pop_scope();
                let mut loop_branches = vec![loop_state.clone()];
                loop_branches.extend(
                    loop_state
                        .loop_exit_snapshots
                        .iter()
                        .map(|snapshot| snapshot.state.clone()),
                );
                *state = BorrowState::merge_loop_exit_states(&base_state, &loop_branches);
            }
            Stmt::Print(expr) | Stmt::Expr(expr) => {
                Self::check_expr(expr, state)?;
            }
            Stmt::Return(Some(expr)) => {
                Self::check_expr(expr, state)?;
                state.terminated = true;
            }
            Stmt::Return(None) => {
                state.terminated = true;
            }
            Stmt::Throw(expr) => {
                Self::check_expr(expr, state)?;
                state.terminated = true;
            }
            Stmt::Break | Stmt::Continue => {
                let kind = match stmt {
                    Stmt::Break => LoopExitKind::Break,
                    Stmt::Continue => LoopExitKind::Continue,
                    _ => unreachable!(),
                };
                if matches!(stmt, Stmt::Break) && flow.in_switch {
                    state.switch_exit_snapshots.push(state.snapshot_for_loop_exit());
                    state.exits_switch = true;
                } else {
                    state.loop_exit_snapshots.push(LoopExitSnapshot {
                        kind,
                        state: state.snapshot_for_loop_exit(),
                    });
                    state.terminated = true;
                }
            }
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
            Expr::NewArray { length, values, .. } => {
                if let Some(length) = length {
                    Self::check_expr(length, state)?;
                }
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
            Expr::Throw(expr) => Self::check_expr(expr, state)?,
            Expr::Assign { target, value } => {
                Self::check_expr(target, state)?;
                Self::check_expr(value, state)?;
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
                match body {
                    LambdaBody::Expr(body) => Self::check_expr(body, state)?,
                    LambdaBody::Block(stmts) => Self::check_stmts(stmts, state, FlowContext::new())?,
                }
                state.pop_scope();
            }
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                Self::check_expr(condition, state)?;
                let base_state = state.clone();
                let mut true_state = base_state.clone();
                Self::check_expr(when_true, &mut true_state)?;
                let mut false_state = base_state.clone();
                Self::check_expr(when_false, &mut false_state)?;
                *state = BorrowState::merge_from_branches(&base_state, &[true_state, false_state]);
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
