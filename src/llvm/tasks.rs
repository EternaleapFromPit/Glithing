use super::*;

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

    let result_llvm_type = llvm_ir_type(&result_ty);

    if matches!(result_ty, IrType::Void) {
        emitter.emit_delegate_invoke(&worker_val.value, &[], &IrType::Void)?;
        emitter.emit_temporary_drop(worker_expr, &worker_val);
        let task_ptr = emitter.tmp();
        emitter.body.push_str(&format!(
            "  {} = call ptr @glitch_task_from_result_ptr(ptr null)\n",
            task_ptr
        ));
        Ok(LlValue {
            value: task_ptr,
            ty: LlType::Ptr,
        })
    } else {
        let call_res = emitter.emit_delegate_invoke(&worker_val.value, &[], &result_ty)?;
        emitter.emit_temporary_drop(worker_expr, &worker_val);

        let task_ptr = emitter.tmp();
        let helper_name = match &result_ty {
            IrType::Int | IrType::UInt => "glitch_task_from_result_i32",
            IrType::Long => "glitch_task_from_result_i64",
            IrType::Double | IrType::Decimal => "glitch_task_from_result_double",
            _ => "glitch_task_from_result_ptr",
        };
        emitter.body.push_str(&format!(
            "  {} = call ptr @{}({} {})\n",
            task_ptr,
            helper_name,
            result_llvm_type.as_ir(),
            call_res.value
        ));
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
    let helper_name = match &result_ty {
        IrType::Int | IrType::UInt => "glitch_task_from_result_i32",
        IrType::Long => "glitch_task_from_result_i64",
        IrType::Double | IrType::Decimal => "glitch_task_from_result_double",
        _ => "glitch_task_from_result_ptr",
    };

    emitter.body.push_str(&format!(
        "  {} = call ptr @{}({} {})\n",
        task_ptr,
        helper_name,
        result_llvm_type.as_ir(),
        value_val.value
    ));

    Ok(LlValue {
        value: task_ptr,
        ty: LlType::Ptr,
    })
}
