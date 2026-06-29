use super::*;

#[derive(Debug, Clone)]
pub(super) struct CheckedVar {
    pub(super) ty: IrType,
    pub(super) ownership: Ownership,
    pub(super) scope_depth: usize,
    pub(super) view_source: Option<String>,
}

#[derive(Debug, Clone)]
pub(super) struct OwnershipSnapshot {
    scopes: Vec<Vec<String>>,
    vars: HashMap<String, CheckedVar>,
    exits_function: bool,
    exits_loop: bool,
    exits_switch: bool,
}

pub(super) fn package_type_key(package_id: &str, type_name: &str) -> String {
    format!("{package_id}::{type_name}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LoopExitKind {
    Break,
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FlowContext {
    pub(super) in_loop: bool,
    pub(super) in_switch: bool,
}

impl FlowContext {
    pub(super) const fn new() -> Self {
        Self {
            in_loop: false,
            in_switch: false,
        }
    }

    pub(super) const fn with_loop(self) -> Self {
        Self {
            in_loop: true,
            in_switch: false,
        }
    }

    pub(super) const fn with_switch(self) -> Self {
        Self {
            in_loop: self.in_loop,
            in_switch: true,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct LoopExitSnapshot {
    kind: LoopExitKind,
    snapshot: OwnershipSnapshot,
}

#[derive(Debug, Clone)]
pub(super) struct OwnershipState {
    scopes: Vec<Vec<String>>,
    vars: HashMap<String, CheckedVar>,
    exits_function: bool,
    exits_loop: bool,
    exits_switch: bool,
    loop_exit_snapshots: Vec<LoopExitSnapshot>,
    switch_exit_snapshots: Vec<OwnershipSnapshot>,
}

impl OwnershipState {
    pub(super) fn new(params: Vec<TypedBinding>) -> Self {
        let mut state = Self {
            scopes: vec![Vec::new()],
            vars: HashMap::new(),
            exits_function: false,
            exits_loop: false,
            exits_switch: false,
            loop_exit_snapshots: Vec::new(),
            switch_exit_snapshots: Vec::new(),
        };
        for param in params {
            state.declare(param.name, param.ty, param.ownership, None);
        }
        state
    }

    pub(super) fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    pub(super) fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for name in scope {
                self.vars.remove(&name);
            }
        }
    }

    pub(super) fn depth(&self) -> usize {
        self.scopes.len().saturating_sub(1)
    }

    pub(super) fn declare(
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

    pub(super) fn assign(
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

    pub(super) fn get(&self, name: &str) -> Option<&CheckedVar> {
        self.vars.get(name)
    }

    pub(super) fn snapshot(&self) -> OwnershipSnapshot {
        OwnershipSnapshot {
            scopes: self.scopes.clone(),
            vars: self.vars.clone(),
            exits_function: self.exits_function,
            exits_loop: self.exits_loop,
            exits_switch: self.exits_switch,
        }
    }

    pub(super) fn from_snapshot(snapshot: &OwnershipSnapshot) -> Self {
        Self {
            scopes: snapshot.scopes.clone(),
            vars: snapshot.vars.clone(),
            exits_function: snapshot.exits_function,
            exits_loop: snapshot.exits_loop,
            exits_switch: snapshot.exits_switch,
            loop_exit_snapshots: Vec::new(),
            switch_exit_snapshots: Vec::new(),
        }
    }
}

pub(super) fn merge_checked_vars(left: &CheckedVar, right: &CheckedVar) -> CheckedVar {
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

pub(super) fn merge_ownership(left: &Ownership, right: &Ownership) -> Ownership {
    pub(super) fn rank(value: &Ownership) -> u8 {
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
pub(super) struct CheckedExpr {
    ty: IrType,
    ownership: Ownership,
    source_var: Option<String>,
    is_move: bool,
}

pub(super) struct OwnershipChecker;

impl OwnershipChecker {
    pub(super) fn check_function(
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

    pub(super) fn check_body(
        context: &str,
        body: &[Stmt],
        env: &TypeEnv,
        params: Vec<TypedBinding>,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<(), String> {
        let mut state = OwnershipState::new(params);
        Self::check_stmts(context, body, env, &mut state, return_type, return_ownership, FlowContext::new())
    }

    pub(super) fn check_stmts(
        context: &str,
        stmts: &[Stmt],
        env: &TypeEnv,
        state: &mut OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
        flow: FlowContext,
    ) -> Result<(), String> {
        for stmt in stmts {
            Self::check_stmt(context, stmt, env, state, return_type, return_ownership.clone(), flow)?;
            if state.exits_function || state.exits_loop || state.exits_switch {
                break;
            }
        }
        Ok(())
    }

    pub(super) fn check_block_snapshot(
        context: &str,
        stmts: &[Stmt],
        env: &TypeEnv,
        state: &OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
        flow: FlowContext,
    ) -> Result<OwnershipState, String> {
        let mut branch_state = state.clone();
        Self::check_stmts(
            context,
            stmts,
            env,
            &mut branch_state,
            return_type,
            return_ownership,
            flow,
        )?;
        Ok(branch_state)
    }

    pub(super) fn apply_finally_to_state(
        context: &str,
        finally_body: &[Stmt],
        env: &TypeEnv,
        state: &OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
        flow: FlowContext,
    ) -> Result<OwnershipState, String> {
        let mut final_state = state.clone();
        let exited_function = final_state.exits_function;
        let exited_loop = final_state.exits_loop;
        let exited_switch = final_state.exits_switch;
        let existing_snapshots = final_state.loop_exit_snapshots.clone();
        let existing_switch_snapshots = final_state.switch_exit_snapshots.clone();
        final_state.push_scope();
        Self::check_stmts(
            context,
            finally_body,
            env,
            &mut final_state,
            return_type,
            return_ownership.clone(),
            flow,
        )?;
        final_state.pop_scope();
        final_state.exits_function |= exited_function;
        final_state.exits_loop |= exited_loop;
        final_state.exits_switch |= exited_switch;

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
                flow,
            )?;
            snapshot_state.pop_scope();
            propagated_snapshots.push(LoopExitSnapshot {
                kind: snapshot.kind,
                snapshot: snapshot_state.snapshot(),
            });
        }
        final_state.loop_exit_snapshots.extend(propagated_snapshots);

        let mut propagated_switch_snapshots = Vec::new();
        for snapshot in existing_switch_snapshots {
            let mut snapshot_state = OwnershipState::from_snapshot(&snapshot);
            snapshot_state.push_scope();
            Self::check_stmts(
                context,
                finally_body,
                env,
                &mut snapshot_state,
                return_type,
                return_ownership.clone(),
                flow,
            )?;
            snapshot_state.pop_scope();
            propagated_switch_snapshots.push(snapshot_state.snapshot());
        }
        final_state.switch_exit_snapshots.extend(propagated_switch_snapshots);
        Ok(final_state)
    }

    pub(super) fn merge_branch_states(base: &OwnershipState, branches: &[OwnershipState]) -> OwnershipState {
        let mut merged = base.clone();
        let live_branches: Vec<&OwnershipState> = branches
            .iter()
            .filter(|branch| !branch.exits_function && !branch.exits_loop && !branch.exits_switch)
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
        merged.switch_exit_snapshots = branches
            .iter()
            .flat_map(|branch| branch.switch_exit_snapshots.iter().cloned())
            .collect();
        merged.exits_function = branches.iter().all(|branch| branch.exits_function);
        merged.exits_loop = branches.iter().all(|branch| branch.exits_loop);
        merged.exits_switch = branches.iter().all(|branch| branch.exits_switch);
        merged
    }

    pub(super) fn merge_loop_exit_states(base: &OwnershipState, branches: &[OwnershipState]) -> OwnershipState {
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
        merged.switch_exit_snapshots = branches
            .iter()
            .flat_map(|branch| branch.switch_exit_snapshots.iter().cloned())
            .collect();
        merged.exits_function = branches.iter().all(|branch| branch.exits_function);
        merged.exits_loop = branches.iter().all(|branch| branch.exits_loop);
        merged.exits_switch = branches.iter().all(|branch| branch.exits_switch);
        merged
    }

    pub(super) fn check_stmt(
        context: &str,
        stmt: &Stmt,
        env: &TypeEnv,
        state: &mut OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
        flow: FlowContext,
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
                Self::check_stmts(context, body, env, state, return_type, return_ownership.clone(), flow)?;
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
                    flow,
                )?;
                let else_state = Self::check_block_snapshot(
                    context,
                    else_body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                    flow,
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
                    flow,
                )?;
                let try_state = Self::apply_finally_to_state(
                    context,
                    finally_body,
                    env,
                    &try_state,
                    return_type,
                    return_ownership.clone(),
                    flow,
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
                        flow,
                    )?;
                    catch_state.pop_scope();
                    let catch_state = Self::apply_finally_to_state(
                        context,
                        finally_body,
                        env,
                        &catch_state,
                        return_type,
                        return_ownership.clone(),
                        flow,
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
                let mut loop_exit_snapshots = Vec::new();
                let mut switch_exit_snapshots = Vec::new();
                for case in cases {
                    Self::check_expr(&case.value, env, state)?;
                    let case_state = Self::check_block_snapshot(
                        context,
                        &case.body,
                        env,
                        &base,
                        return_type,
                        return_ownership.clone(),
                        flow.with_switch(),
                    )?;
                    loop_exit_snapshots.extend(case_state.loop_exit_snapshots.iter().cloned());
                    switch_exit_snapshots.extend(case_state.switch_exit_snapshots.iter().cloned());
                    if !case_state.exits_function && !case_state.exits_loop && !case_state.exits_switch {
                        branches.push(case_state);
                    }
                }
                let default_state = Self::check_block_snapshot(
                    context,
                    default,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                    flow.with_switch(),
                )?;
                loop_exit_snapshots.extend(default_state.loop_exit_snapshots.iter().cloned());
                switch_exit_snapshots.extend(default_state.switch_exit_snapshots.iter().cloned());
                if !default_state.exits_function && !default_state.exits_loop && !default_state.exits_switch {
                    branches.push(default_state);
                }
                for snapshot in switch_exit_snapshots {
                    let mut exit_state = OwnershipState::from_snapshot(&snapshot);
                    exit_state.exits_switch = false;
                    branches.push(exit_state);
                }
                let mut merged = Self::merge_branch_states(&base, &branches);
                merged.loop_exit_snapshots.extend(loop_exit_snapshots);
                merged.switch_exit_snapshots.clear();
                *state = merged;
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
                    flow.with_loop(),
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
                    Self::check_stmt(context, init, env, state, return_type, return_ownership.clone(), flow)?;
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
                    flow.with_loop(),
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
                            flow.with_loop(),
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
                                    flow.with_loop(),
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
                    flow.with_loop(),
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
                if matches!(stmt, Stmt::Break) && flow.in_switch {
                    state.switch_exit_snapshots.push(state.snapshot());
                    state.exits_switch = true;
                } else {
                    state.loop_exit_snapshots.push(LoopExitSnapshot {
                        kind,
                        snapshot: state.snapshot(),
                    });
                    state.exits_loop = true;
                }
            }
        }
        Ok(())
    }

    pub(super) fn check_expr(
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
                let mut checked = Vec::with_capacity(values.len());
                for value in values {
                    checked.push(Self::check_expr(value, env, state)?);
                }
                let element_ty = checked
                    .first()
                    .map(|value| value.ty.clone())
                    .unwrap_or(IrType::Long);
                CheckedExpr {
                    ty: IrType::Array(Box::new(element_ty)),
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
                let mut checked_values = Vec::with_capacity(values.len());
                if let Some(length) = length {
                    Self::check_expr(length, env, state)?;
                }
                for value in values {
                    checked_values.push(Self::check_expr(value, env, state)?);
                }
                let inferred_element = if matches!(element_type, TypeSyntax::Named(name) if name == "var") {
                    checked_values
                        .first()
                        .map(|value| value.ty.clone())
                        .unwrap_or(IrType::Long)
                } else {
                    type_syntax_to_ir(element_type, env)
                };
                CheckedExpr {
                    ty: IrType::Array(Box::new(inferred_element)),
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
                    (IrType::Task(_), "IsCompleted")
                    | (IrType::Task(_), "IsCompletedSuccessfully")
                    | (IrType::Task(_), "IsFaulted") => Some(FieldSignature {
                        package_id: None,
                        visibility: Visibility::Public,
                        ty: IrType::Bool,
                        ownership: Ownership::Copy,
                    }),
                    (IrType::Task(_), "Exception") => Some(FieldSignature {
                        package_id: None,
                        visibility: Visibility::Public,
                        ty: IrType::Exception,
                        ownership: Ownership::Owned,
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
            Expr::MethodCall { target, name, generic_args, args } => {
                let target = Self::check_expr(target, env, state)?;
                let args = args
                    .iter()
                    .map(|arg| Self::check_expr(arg, env, state))
                    .collect::<Result<Vec<_>, _>>()?;
                let explicit_generic_args = generic_args
                    .iter()
                    .map(|arg| type_syntax_to_ir(arg, env))
                    .collect::<Vec<_>>();
                let typed_args = args
                    .iter()
                    .map(|arg| TypedExpr {
                        kind: TypedExprKind::Var("<checked>".to_string()),
                        ty: arg.ty.clone(),
                        ownership: arg.ownership.clone(),
                        drop_kind: drop_kind_for_type(&arg.ty, &arg.ownership),
                    })
                    .collect::<Vec<_>>();
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
                            .map(|arg| {
                                let result_ty = match &arg.ty {
                                    IrType::Function { return_type, .. } => return_type.as_ref().clone(),
                                    _ => arg.ty.clone(),
                                };
                                IrType::Task(Box::new(result_ty))
                            })
                            .unwrap_or_else(|| IrType::Task(Box::new(IrType::Void)))
                    }
                    _ => IrType::Unknown(name.clone()),
                };
                let resolved_signature = if let IrType::Class(type_name)
                | IrType::Struct(type_name)
                | IrType::Interface(type_name) = &target.ty
                {
                    env.resolve_method_call_with_generic_args(
                        type_name,
                        name,
                        &typed_args,
                        &explicit_generic_args,
                    )
                    .ok()
                    .flatten()
                } else {
                    None
                };
                let ty = resolved_signature
                    .as_ref()
                    .map(|signature| signature.return_type.clone())
                    .unwrap_or(ty);
                let ownership = if target.ty == IrType::Exception && matches!(name.as_str(), "Message")
                {
                    Ownership::Borrowed
                } else {
                    resolved_signature
                        .as_ref()
                        .map(|signature| signature.return_ownership.clone())
                        .unwrap_or_else(|| ownership_for_type(&ty))
                };
                CheckedExpr {
                    ownership,
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::FunctionCall { name, generic_args, args } => {
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
            let explicit_generic_args = generic_args
                .iter()
                .map(|arg| type_syntax_to_ir(arg, env))
                .collect::<Vec<_>>();
            let typed_args = checked_args
                .iter()
                .map(|arg| TypedExpr {
                    kind: TypedExprKind::Var("<checked>".to_string()),
                    ty: arg.ty.clone(),
                    ownership: arg.ownership.clone(),
                    drop_kind: drop_kind_for_type(&arg.ty, &arg.ownership),
                })
                .collect::<Vec<_>>();
            let signature = env
                .resolve_function_call_with_generic_args(name, &typed_args, &explicit_generic_args)
                .ok()
                .flatten()
                .or_else(|| {
                    current_enclosing_type_from_state(state).and_then(|current_type| {
                        env.resolve_method_call_with_generic_args(
                            &current_type,
                            name,
                            &typed_args,
                            &explicit_generic_args,
                        )
                        .ok()
                        .flatten()
                    })
                });
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
                        .or_else(|| env.resolve_field(base_type_name(type_name), &field.name))
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
                match body {
                    LambdaBody::Expr(body) => {
                        Self::check_expr(body, env, state)?;
                    }
                    LambdaBody::Block(stmts) => {
                        let mut nested_state = state.clone();
                        Self::check_stmts(
                            "lambda",
                            stmts,
                            env,
                            &mut nested_state,
                            &IrType::Void,
                            Ownership::Copy,
                            FlowContext::new(),
                        )?;
                    }
                }
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

    pub(super) fn target_type(
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

    pub(super) fn check_assignment_safety(
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

    pub(super) fn coerce_for_target(target_ty: &IrType, mut expr: CheckedExpr) -> CheckedExpr {
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

    pub(super) fn check_view_source_lifetime(
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

pub(super) fn checked_temp(ty: IrType) -> CheckedExpr {
    CheckedExpr {
        ownership: ownership_for_type(&ty),
        ty,
        source_var: None,
        is_move: false,
    }
}

