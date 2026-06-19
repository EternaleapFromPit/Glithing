use super::*;

fn retain_task_payload(
    emitter: &mut LlvmEmitter,
    result_ty: &IrType,
    value: &LlValue,
) {
    if is_string_like_type(result_ty) {
        emitter
            .body
            .push_str(&format!("  call void @glitch_string_retain(ptr {})\n", value.value));
    } else if matches!(result_ty, IrType::Function { .. }) {
        emitter
            .body
            .push_str(&format!("  call void @glitch_delegate_retain(ptr {})\n", value.value));
    } else if let Some(type_name) = object_type_name(result_ty) {
        if emitter.object_types.contains_key(type_name) {
            emitter.emit_retain(type_name, &value.value);
        }
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
            emitter.emit_temporary_drop(tasks, &tasks_val);
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

    let result_llvm_type = llvm_ir_type(&result_ty);
    let task_ptr = emitter.tmp();
    if matches!(
        val_expr.kind,
        TypedExprKind::Var(_)
            | TypedExprKind::Field { .. }
            | TypedExprKind::Index { .. }
            | TypedExprKind::Borrow { .. }
    ) {
        retain_task_payload(emitter, &result_ty, &value_val);
    }
    let helper_name = if llvm_ir_type(&result_ty) == LlType::I1 || is_bool_like_type(&result_ty) {
        "glitch_task_from_result_bool"
    } else {
        match &result_ty {
        IrType::Int | IrType::UInt => "glitch_task_from_result_i32",
        IrType::Long => "glitch_task_from_result_i64",
        IrType::Double | IrType::Decimal => "glitch_task_from_result_double",
        _ => "glitch_task_from_result_ptr",
    }
    };

    let helper_arg = if helper_name == "glitch_task_from_result_ptr" && value_val.ty != LlType::Ptr {
        let casted = emitter.tmp();
        match value_val.ty {
            LlType::I1 => emitter.body.push_str(&format!(
                "  {casted}_i64 = zext i1 {} to i64\n  {casted} = inttoptr i64 {casted}_i64 to ptr\n",
                value_val.value
            )),
            LlType::I8 | LlType::I16 | LlType::I32 | LlType::I64 => emitter.body.push_str(&format!(
                "  {casted} = inttoptr {} {} to ptr\n",
                value_val.ty.as_ir(),
                value_val.value
            )),
            LlType::Double => emitter.body.push_str(&format!(
                "  {casted}_bits = bitcast double {} to i64\n  {casted} = inttoptr i64 {casted}_bits to ptr\n",
                value_val.value
            )),
            LlType::Void | LlType::Ptr => {}
        }
        casted
    } else {
        value_val.value.clone()
    };

    emitter.body.push_str(&format!(
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
