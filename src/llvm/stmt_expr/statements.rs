use super::*;

impl LlvmEmitter {
    pub(in crate::llvm) fn emit_stmts(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            if self.terminated {
                break;
            }
            self.emit_stmt(stmt)?;
        }
        Ok(())
    }

    pub(in crate::llvm) fn emit_typed_stmts(&mut self, stmts: &[TypedStmt]) -> Result<(), String> {
        for stmt in stmts {
            if self.terminated {
                break;
            }
            self.emit_typed_stmt(stmt)?;
        }
        Ok(())
    }

    pub(in crate::llvm) fn emit_typed_stmt(&mut self, stmt: &TypedStmt) -> Result<(), String> {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                let value = self.emit_typed_expr(expr)?;
                let ty = llvm_ir_type(&binding.ty);
                let stored = self.cast_value(value, &ty)?;
                self.retain_for_store(&binding.ty, expr, &stored.value);
                self.move_raw_owned_source_after_store(&binding.ty, expr);
                let var = self.vars.get(&binding.name).cloned();
                let skip_drop = self.async_state_pc_ptr.is_some()
                    && self.async_uninitialized_locals.contains(&binding.name);
                if let Some(var) = &var {
                    if !skip_drop {
                        self.emit_var_drop(var);
                    }
                }
                let ptr = var.map(|var| var.ptr).unwrap_or_else(|| self.tmp());
                if !self.vars.contains_key(&binding.name) {
                    self.body
                        .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
                }
                self.body.push_str(&format!(
                    "  store {} {}, ptr {ptr}\n",
                    ty.as_ir(),
                    stored.value
                ));
                if let Some(var) = self.vars.get_mut(&binding.name) {
                    var.ty = ty;
                    var.ir_ty = binding.ty.clone();
                    var.drop_kind = binding.drop_kind();
                } else {
                    self.vars.insert(
                        binding.name.clone(),
                        LlVar {
                            ptr,
                            ty,
                            ir_ty: binding.ty.clone(),
                            drop_kind: binding.drop_kind(),
                        },
                    );
                    self.drop_order.push(binding.name.clone());
                }
                if self.async_state_pc_ptr.is_some() {
                    self.async_uninitialized_locals.remove(&binding.name);
                }
                if let Some(collection_key) = self.is_build_service_provider_call(expr) {
                    self.propagate_service_provider_registrations(
                        &binding.name,
                        &collection_key,
                    );
                }
                if let Some(builder_key) = self.is_web_application_build_call(expr) {
                    self.propagate_web_application_service_registrations(
                        &binding.name,
                        &builder_key,
                    );
                }
            }
            TypedStmtKind::Assign { name, expr } => {
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM TIR backend: unknown variable '{name}'"))?;
                let value = self.emit_typed_expr(expr)?;
                let value = self.cast_value(value, &var.ty)?;
                self.retain_for_store(&var.ir_ty, expr, &value.value);
                self.emit_var_drop(&var);
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    var.ty.as_ir(),
                    value.value,
                    var.ptr
                ));
                self.move_raw_owned_source_after_store(&var.ir_ty, expr);
                if let Some(collection_key) = self.is_build_service_provider_call(expr) {
                    self.propagate_service_provider_registrations(name, &collection_key);
                } else if let Some(builder_key) = self.is_web_application_build_call(expr) {
                    self.propagate_web_application_service_registrations(name, &builder_key);
                } else {
                    self.service_provider_registrations.remove(name);
                    self.service_provider_registrations
                        .remove(&format!("{name}.Services"));
                }
            }
            TypedStmtKind::AssignTarget { target, expr } => match &target.kind {
                TypedExprKind::Field { .. } => {
                    if self.is_opaque_field(target) {
                        self.emit_typed_expr(expr)?;
                        return Ok(());
                    }
                    let field_ptr = self.emit_field_ptr(target)?;
                    let value = self.emit_typed_expr(expr)?;
                    let field_ty = llvm_ir_type(&target.ty);
                    let value = self.cast_value(value, &field_ty)?;
                    self.retain_for_store(&target.ty, expr, &value.value);
                    self.move_raw_owned_source_after_store(&target.ty, expr);
                    if let Some(type_name) = object_type_name(&target.ty) {
                        if self.object_types.contains_key(type_name) {
                            let old = self.tmp();
                            self.body.push_str(&format!(
                                "  {old} = load ptr, ptr {}\n",
                                field_ptr.value
                            ));
                            self.emit_drop(type_name, &old);
                        }
                    } else if is_string_like_type(&target.ty) {
                        let old = self.tmp();
                        self.body.push_str(&format!(
                                "  {old} = load ptr, ptr {}\n  call void @glitch_string_release(ptr {old})\n",
                                field_ptr.value
                            ));
                    }
                    self.body.push_str(&format!(
                        "  store {} {}, ptr {}\n",
                        field_ty.as_ir(),
                        value.value,
                        field_ptr.value
                    ));
                }
                TypedExprKind::Index { target, index } => {
                    self.emit_index_assignment(target, index, expr)?;
                }
                _ => {
                    return Err(format!(
                        "LLVM TIR backend: unsupported assignment target {:?} with type {:?}",
                        target.kind, target.ty
                    ));
                }
            },
            TypedStmtKind::Block(body) => self.emit_typed_stmts(body)?,
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                let raw_condition = self.emit_typed_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                let then_label = self.next_label("if_then");
                let else_label = self.next_label("if_else");
                let end_label = self.next_label("if_end");
                self.body.push_str(&format!(
                    "  br i1 {}, label %{then_label}, label %{else_label}\n",
                    condition.value
                ));
                self.body.push_str(&format!("{then_label}:\n"));
                self.terminated = false;
                self.emit_typed_stmts(then_body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{end_label}\n"));
                }
                self.body.push_str(&format!("{else_label}:\n"));
                self.terminated = false;
                self.emit_typed_stmts(else_body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{end_label}\n"));
                }
                self.body.push_str(&format!("{end_label}:\n"));
                self.terminated = false;
            }
            TypedStmtKind::While { condition, body } => {
                let start = self.next_label("while_cond");
                let loop_body = self.next_label("while_body");
                let end = self.next_label("while_end");
                self.body.push_str(&format!("  br label %{start}\n"));
                self.body.push_str(&format!("{start}:\n"));
                let raw_condition = self.emit_typed_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                self.body.push_str(&format!(
                    "  br i1 {}, label %{loop_body}, label %{end}\n",
                    condition.value
                ));
                self.body.push_str(&format!("{loop_body}:\n"));
                self.terminated = false;
                self.loop_targets.push((start.clone(), end.clone()));
                self.emit_typed_stmts(body)?;
                self.loop_targets.pop();
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{start}\n"));
                }
                self.body.push_str(&format!("{end}:\n"));
                self.terminated = false;
            }
            TypedStmtKind::Print(expr) => {
                let value = self.emit_typed_expr(expr)?;
                self.emit_print(value.clone());
                self.emit_temporary_drop(expr, &value);
            }
            TypedStmtKind::Return(Some(expr)) => {
                let value = self.emit_typed_expr(expr)?;
                if self.current_return == LlType::Void {
                    if self.async_state_pc_ptr.is_none() {
                        self.emit_local_drops(expr_source_name(expr));
                    }
                    self.body.push_str("  ret void\n");
                    self.terminated = true;
                    return Ok(());
                }
                let value = self.cast_value(value, &self.current_return.clone())?;
                if let Some(type_name) = object_type_name(&expr.ty) {
                    if !matches!(
                        expr.kind,
                        TypedExprKind::NewObject { .. }
                            | TypedExprKind::Move(_)
                            | TypedExprKind::Var(_)
                    ) && self.object_types.contains_key(type_name)
                    {
                        self.emit_retain(type_name, &value.value);
                    }
                } else if is_string_like_type(&expr.ty)
                    && matches!(
                        expr.kind,
                        TypedExprKind::Field { .. } | TypedExprKind::Index { .. }
                    )
                {
                    self.body.push_str(&format!(
                        "  call void @glitch_string_retain(ptr {})\n",
                        value.value
                    ));
                }
                if self.async_state_pc_ptr.is_none() {
                    self.emit_local_drops(expr_source_name(expr));
                }
                self.body.push_str(&format!(
                    "  ret {} {}\n",
                    self.current_return.as_ir(),
                    value.value
                ));
                self.terminated = true;
            }
            TypedStmtKind::Return(None) => {
                self.emit_default_return();
                self.terminated = true;
            }
            TypedStmtKind::Expr(expr) => {
                let value = self.emit_typed_expr(expr)?;
                self.emit_temporary_drop(expr, &value);
            }
            TypedStmtKind::Try {
                try_body,
                catch_name,
                catch_body,
                finally_body,
            } => {
                self.emit_typed_try(try_body, catch_name.as_deref(), catch_body, finally_body)?;
            }
            TypedStmtKind::Throw(expr) => {
                let exception = self.emit_typed_expr(expr)?;
                let exception = self.cast_value(exception, &LlType::Ptr)?;
                self.body.push_str(&format!(
                    "  store ptr {}, ptr @glitch_exception_pending\n",
                    exception.value
                ));
                self.emit_exception_branch();
            }
            TypedStmtKind::ForEach {
                item,
                collection,
                body,
            } => self.emit_typed_foreach(item, collection, body)?,
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => self.emit_typed_for(
                init.as_deref(),
                condition.as_ref(),
                increment.as_deref(),
                body,
            )?,
            TypedStmtKind::Switch {
                expr,
                cases,
                default,
            } => self.emit_typed_switch(expr, cases, default)?,
            TypedStmtKind::Break => {
                let (_, break_target) = self.loop_targets.last().cloned().ok_or_else(|| {
                    "LLVM TIR backend: 'break' has no enclosing loop or switch".to_string()
                })?;
                self.body.push_str(&format!("  br label %{break_target}\n"));
                self.terminated = true;
            }
            TypedStmtKind::Continue => {
                let (continue_target, _) = self.loop_targets.last().cloned().ok_or_else(|| {
                    "LLVM TIR backend: 'continue' has no enclosing loop".to_string()
                })?;
                if continue_target.is_empty() {
                    return Err("LLVM TIR backend: 'continue' has no enclosing loop".to_string());
                }
                self.body
                    .push_str(&format!("  br label %{continue_target}\n"));
                self.terminated = true;
            }
        }
        Ok(())
    }

    pub(in crate::llvm) fn emit_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let {
                name,
                declared_type,
                expr,
                ..
            } => {
                let value = self.emit_expr(expr)?;
                let ty = declared_type
                    .as_ref()
                    .map(llvm_type)
                    .unwrap_or_else(|| value.ty.clone());
                let stored = self.cast_value(value, &ty)?;
                let ptr = self.tmp();
                self.body
                    .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
                self.body.push_str(&format!(
                    "  store {} {}, ptr {ptr}\n",
                    ty.as_ir(),
                    stored.value
                ));
                self.vars.insert(
                    name.clone(),
                    LlVar {
                        ptr,
                        ty,
                        ir_ty: IrType::Unknown(name.clone()),
                        drop_kind: DropKind::None,
                    },
                );
            }
            Stmt::Assign { name, expr } => {
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM backend: unknown variable '{name}'"))?;
                let value = self.emit_expr(expr)?;
                let value = self.cast_value(value, &var.ty)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    var.ty.as_ir(),
                    value.value,
                    var.ptr
                ));
            }
            Stmt::Block(body) => self.emit_stmts(body)?,
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let raw_condition = self.emit_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                let then_label = self.next_label("if_then");
                let else_label = self.next_label("if_else");
                let end_label = self.next_label("if_end");
                self.body.push_str(&format!(
                    "  br i1 {}, label %{then_label}, label %{else_label}\n",
                    condition.value
                ));
                self.body.push_str(&format!("{then_label}:\n"));
                self.terminated = false;
                self.emit_stmts(then_body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{end_label}\n"));
                }
                self.body.push_str(&format!("{else_label}:\n"));
                self.terminated = false;
                self.emit_stmts(else_body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{end_label}\n"));
                }
                self.body.push_str(&format!("{end_label}:\n"));
                self.terminated = false;
            }
            Stmt::While { condition, body } => {
                let start = self.next_label("while_cond");
                let loop_body = self.next_label("while_body");
                let end = self.next_label("while_end");
                self.body.push_str(&format!("  br label %{start}\n"));
                self.body.push_str(&format!("{start}:\n"));
                let raw_condition = self.emit_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                self.body.push_str(&format!(
                    "  br i1 {}, label %{loop_body}, label %{end}\n",
                    condition.value
                ));
                self.body.push_str(&format!("{loop_body}:\n"));
                self.terminated = false;
                self.emit_stmts(body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{start}\n"));
                }
                self.body.push_str(&format!("{end}:\n"));
                self.terminated = false;
            }
            Stmt::Print(expr) => {
                let value = self.emit_expr(expr)?;
                self.emit_print(value);
            }
            Stmt::Return(Some(expr)) => {
                let value = self.emit_expr(expr)?;
                let value = self.cast_value(value, &self.current_return.clone())?;
                self.body.push_str(&format!(
                    "  ret {} {}\n",
                    self.current_return.as_ir(),
                    value.value
                ));
                self.terminated = true;
            }
            Stmt::Return(None) => {
                self.emit_default_return();
                self.terminated = true;
            }
            Stmt::Expr(expr) => {
                self.emit_expr(expr)?;
            }
            Stmt::For { .. }
            | Stmt::ForEach { .. }
            | Stmt::Switch { .. }
            | Stmt::Try { .. }
            | Stmt::AssignTarget { .. }
            | Stmt::Throw(_)
            | Stmt::Break
            | Stmt::Continue => {
                return Err(format!(
                    "LLVM backend: unsupported statement in current slice: {stmt:?}"
                ));
            }
        }
        Ok(())
    }

}
