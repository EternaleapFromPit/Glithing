use super::*;

impl LlvmEmitter {
    pub(super) fn retain_task_payload(&mut self, result_ty: &IrType, value: &LlValue) {
    if is_string_like_type(result_ty) {
        self
            .body
            .push_str(&format!("  call void @glitch_string_retain(ptr {})\n", value.value));
    } else if matches!(result_ty, IrType::Task(_)) {
        self
            .body
            .push_str(&format!("  call void @GlitchTask_Retain(ptr {})\n", value.value));
    } else if matches!(result_ty, IrType::Function { .. }) {
        self
            .body
            .push_str(&format!("  call void @glitch_delegate_retain(ptr {})\n", value.value));
    } else if let Some(type_name) = object_type_name(result_ty) {
        if self.object_types.contains_key(type_name) {
            self.emit_retain(type_name, &value.value);
        }
    }
    }

    pub(super) fn emit_task_from_exception_value(
        &mut self,
        message_val: LlValue,
    ) -> Result<LlValue, String> {
        let task_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {} = call ptr @glitch_task_from_result_ptr(ptr null)\n",
            task_ptr
        ));
        let exception_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {} = getelementptr inbounds %glitch.task, ptr {}, i32 0, i32 4\n  store ptr {}, ptr {}\n",
            exception_ptr, task_ptr, message_val.value, exception_ptr
        ));
        Ok(LlValue {
            value: task_ptr,
            ty: LlType::Ptr,
        })
    }

    pub(super) fn emit_exception_message_value(
        &mut self,
        exception_expr: &TypedExpr,
    ) -> Result<LlValue, String> {
        if matches!(exception_expr.ty, IrType::Exception) {
            return self.emit_typed_expr(exception_expr);
        }
        let message_expr = TypedExpr {
            kind: TypedExprKind::Field {
                target: Box::new(exception_expr.clone()),
                name: "Message".to_string(),
            },
            ty: IrType::String,
            ownership: Ownership::Borrowed,
            drop_kind: DropKind::None,
        };
        let message_ptr = self.emit_field_ptr(&message_expr)?;
        let message = self.tmp();
        self.body.push_str(&format!(
            "  {} = load ptr, ptr {}\n",
            message, message_ptr.value
        ));
        Ok(LlValue {
            value: message,
            ty: LlType::Ptr,
        })
    }

    pub(super) fn emit_task_from_result_value(
        &mut self,
        result_ty: &IrType,
        value_val: LlValue,
    ) -> Result<LlValue, String> {
        let result_llvm_type = llvm_ir_type(result_ty);
        let task_ptr = self.tmp();
        let helper_name = if llvm_ir_type(result_ty) == LlType::I1 || is_bool_like_type(result_ty) {
            "glitch_task_from_result_bool"
        } else {
            match result_ty {
                IrType::Int | IrType::UInt => "glitch_task_from_result_i32",
                IrType::Long => "glitch_task_from_result_i64",
                IrType::Double | IrType::Decimal => "glitch_task_from_result_double",
                _ => "glitch_task_from_result_ptr",
            }
        };

        let helper_arg = if helper_name == "glitch_task_from_result_ptr" && value_val.ty != LlType::Ptr
        {
            let casted = self.tmp();
            match value_val.ty {
                LlType::I1 => self.body.push_str(&format!(
                    "  {casted}_i64 = zext i1 {} to i64\n  {casted} = inttoptr i64 {casted}_i64 to ptr\n",
                    value_val.value
                )),
                LlType::I8 | LlType::I16 | LlType::I32 | LlType::I64 => self.body.push_str(&format!(
                    "  {casted} = inttoptr {} {} to ptr\n",
                    value_val.ty.as_ir(),
                    value_val.value
                )),
                LlType::Double => self.body.push_str(&format!(
                    "  {casted}_bits = bitcast double {} to i64\n  {casted} = inttoptr i64 {casted}_bits to ptr\n",
                    value_val.value
                )),
                LlType::Void | LlType::Ptr => {}
            }
            casted
        } else {
            value_val.value.clone()
        };

        self.body.push_str(&format!(
            "  {} = call ptr @{}({} {})\n",
            task_ptr,
            helper_name,
            if helper_name == "glitch_task_from_result_ptr" {
                LlType::Ptr.as_ir()
            } else {
                result_llvm_type.as_ir()
            },
            helper_arg
        ));

        Ok(LlValue {
            value: task_ptr,
            ty: LlType::Ptr,
        })
    }

    pub(super) fn emit_task_from_exception_inline(
        &mut self,
        call: &TypedCall,
        return_type: &IrType,
    ) -> Result<LlValue, String> {
        let _ = return_type;
        let [exception_expr] = &call.args[..] else {
            return Err("Task.FromException expects exactly one argument".to_string());
        };
        let exception_val = self.emit_typed_expr(exception_expr)?;
        let message_val = self.emit_exception_message_value(exception_expr)?;
        self.retain_task_payload(&IrType::String, &message_val);
        self.emit_temporary_drop(exception_expr, &exception_val);
        self.emit_task_from_exception_value(message_val)
    }
}

pub(super) fn emit_task_completed_inline(emitter: &mut LlvmEmitter) -> LlValue {
    let task_ptr = emitter.tmp();
    emitter
        .body
        .push_str(&format!("  {} = call ptr @glitch_task_completed()\n", task_ptr));
    LlValue {
        value: task_ptr,
        ty: LlType::Ptr,
    }
}

pub(super) fn emit_task_when_all_inline(
    emitter: &mut LlvmEmitter,
    call: &TypedCall,
) -> Result<LlValue, String> {
    match call.args.as_slice() {
        [tasks] => {
            let tasks_val = emitter.emit_typed_expr(tasks)?;
            let task_ptr = emitter.tmp();
            emitter.body.push_str(&format!(
                "  {} = call ptr @glitch_task_when_all_array(ptr {})\n",
                task_ptr, tasks_val.value
            ));
            if let Some(var) = match &tasks.kind {
                TypedExprKind::Var(name) | TypedExprKind::Move(name) => {
                    emitter.vars.get(name).cloned()
                }
                _ => None,
            } {
                if matches!(var.ir_ty, IrType::Array(_))
                    && !matches!(var.drop_kind, DropKind::BorrowOnly | DropKind::ViewOnly)
                {
                    if let IrType::Array(element) = &var.ir_ty {
                        emitter.emit_array_drop_value(&tasks_val.value, element);
                    }
                    emitter.body.push_str(&format!(
                        "  store {} {}, ptr {}\n",
                        var.ty.as_ir(),
                        var.ty.default_value(),
                        var.ptr
                    ));
                } else {
                    emitter.emit_temporary_drop(tasks, &tasks_val);
                }
            } else {
                emitter.emit_temporary_drop(tasks, &tasks_val);
            }
            Ok(LlValue {
                value: task_ptr,
                ty: LlType::Ptr,
            })
        }
        [first, second] => {
            let first_val = emitter.emit_typed_expr(first)?;
            let second_val = emitter.emit_typed_expr(second)?;
            let task_ptr = emitter.tmp();
            emitter.body.push_str(&format!(
                "  {} = call ptr @glitch_task_when_all2(ptr {}, ptr {})\n",
                task_ptr, first_val.value, second_val.value
            ));
            emitter.emit_temporary_drop(first, &first_val);
            emitter.emit_temporary_drop(second, &second_val);
            Ok(LlValue {
                value: task_ptr,
                ty: LlType::Ptr,
            })
        }
        _ => Err("Task.WhenAll expects either a task array or two tasks".to_string()),
    }
}

pub(super) fn emit_task_run_inline(
    emitter: &mut LlvmEmitter,
    call: &TypedCall,
    return_type: &IrType,
) -> Result<LlValue, String> {
    let [worker_expr] = &call.args[..] else {
        return Err("Task.Run expects exactly one argument".to_string());
    };
    let worker_val = emitter.emit_typed_expr(worker_expr)?;

    let result_ty = match return_type {
        IrType::Task(inner) => inner.as_ref().clone(),
        _ => match &worker_expr.ty {
            IrType::Function { return_type, .. } => return_type.as_ref().clone(),
            _ => IrType::Void,
        },
    };

    emitter.body.push_str(&format!(
        "  call void @glitch_delegate_retain(ptr {})\n",
        worker_val.value
    ));

    if matches!(result_ty, IrType::Void) {
        let task_ptr = emitter.tmp();
        emitter.body.push_str(&format!(
            "  {} = call ptr @glitch_task_run_void(ptr {})\n",
            task_ptr, worker_val.value
        ));
        emitter.emit_temporary_drop(worker_expr, &worker_val);
        Ok(LlValue {
            value: task_ptr,
            ty: LlType::Ptr,
        })
    } else {
        let task_ptr = emitter.tmp();
        let helper_name = if llvm_ir_type(&result_ty) == LlType::I1 || is_bool_like_type(&result_ty) {
            "glitch_task_run_bool"
        } else {
            match &result_ty {
            IrType::Int | IrType::UInt => "glitch_task_run_i32",
            IrType::Long => "glitch_task_run_i64",
            IrType::Double | IrType::Decimal => "glitch_task_run_double",
            _ => "glitch_task_run_ptr",
        }
        };
        emitter.body.push_str(&format!(
            "  {} = call ptr @{}(ptr {})\n",
            task_ptr,
            helper_name,
            worker_val.value
        ));
        emitter.emit_temporary_drop(worker_expr, &worker_val);
        Ok(LlValue {
            value: task_ptr,
            ty: LlType::Ptr,
        })
    }
}

pub(super) fn emit_task_from_result_inline(
    emitter: &mut LlvmEmitter,
    call: &TypedCall,
    return_type: &IrType,
) -> Result<LlValue, String> {
    let [val_expr] = &call.args[..] else {
        return Err("Task.FromResult expects exactly one argument".to_string());
    };
    let value_val = emitter.emit_typed_expr(val_expr)?;

    let result_ty = match return_type {
        IrType::Task(inner) => inner.as_ref().clone(),
        _ => val_expr.ty.clone(),
    };

    if matches!(
        val_expr.kind,
        TypedExprKind::Var(_)
            | TypedExprKind::Field { .. }
            | TypedExprKind::Index { .. }
            | TypedExprKind::Borrow { .. }
    ) {
        emitter.retain_task_payload(&result_ty, &value_val);
    }
    emitter.emit_task_from_result_value(&result_ty, value_val)
}
