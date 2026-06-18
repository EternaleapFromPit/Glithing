use super::*;
use super::helpers::*;
use super::support::*;

pub(super) fn emit_delegate_invoke(
    emitter: &mut LlvmEmitter,
    delegate_ptr: &str,
    args: &[LlValue],
    return_ty: &IrType,
) -> Result<LlValue, String> {
    let fn_ptr_addr = emitter.tmp();
    emitter.body.push_str(&format!(
        "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 1\n",
        fn_ptr_addr, delegate_ptr
    ));
    let fn_ptr = emitter.tmp();
    emitter
        .body
        .push_str(&format!("  {} = load ptr, ptr {}\n", fn_ptr, fn_ptr_addr));

    let env_ptr_addr = emitter.tmp();
    emitter.body.push_str(&format!(
        "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 2\n",
        env_ptr_addr, delegate_ptr
    ));
    let env_ptr = emitter.tmp();
    emitter
        .body
        .push_str(&format!("  {} = load ptr, ptr {}\n", env_ptr, env_ptr_addr));

    let mut call_args = vec![format!("ptr {env_ptr}")];
    for arg in args {
        call_args.push(format!("{} {}", arg.ty.as_ir(), arg.value));
    }

    if matches!(return_ty, IrType::Void) {
        emitter.body.push_str(&format!(
            "  call void {}({})\n",
            fn_ptr,
            call_args.join(", ")
        ));
        Ok(LlValue {
            value: String::new(),
            ty: LlType::Void,
        })
    } else {
        let result = emitter.tmp();
        let ret_ty = llvm_ir_type(return_ty);
        emitter.body.push_str(&format!(
            "  {} = call {} {}({})\n",
            result,
            ret_ty.as_ir(),
            fn_ptr,
            call_args.join(", ")
        ));
        Ok(LlValue {
            value: result,
            ty: ret_ty,
        })
    }
}

pub(super) fn emit_lambda_function(
    emitter: &mut LlvmEmitter,
    params: &[String],
    body: &TypedExpr,
) -> Result<LlValue, String> {
    let id = emitter.lambda_id;
    emitter.lambda_id += 1;
    let fn_name = format!("glitch_lambda_{id}");

    let mut free_vars: Vec<(String, LlVar)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    collect_free_vars_expr(body, params, &emitter.vars, &mut seen, &mut free_vars);

    let env_struct_name = format!("glitch.lambda.{id}.env");
    if !free_vars.is_empty() {
        let field_types = free_vars
            .iter()
            .map(|(_, lv)| lv.ty.as_ir().to_string())
            .collect::<Vec<_>>();
        emitter.type_defs.push(format!(
            "%{} = type {{ {} }}\n",
            env_struct_name,
            field_types.join(", ")
        ));
    }

    let saved_vars = emitter.vars.clone();
    let saved_drop_order = emitter.drop_order.clone();
    let saved_tmp = emitter.tmp;
    let saved_label = emitter.label;
    let saved_return = emitter.current_return.clone();
    let saved_is_main = emitter.current_is_main;
    let saved_unwind = emitter.current_unwind_label.clone();
    let saved_handler = emitter.exception_handler.clone();
    let saved_loop = emitter.loop_targets.clone();
    let saved_terminated = emitter.terminated;
    let saved_body = std::mem::take(&mut emitter.body);

    emitter.vars.clear();
    emitter.drop_order.clear();
    emitter.tmp = 0;
    emitter.label = 0;
    emitter.terminated = false;
    emitter.current_is_main = false;
    emitter.exception_handler = None;
    emitter.current_unwind_label = "exception_unwind".to_string();
    emitter.loop_targets.clear();
    emitter.current_return = LlType::Ptr;

    let mut param_decls = vec!["ptr %env".to_string()];
    for p in params {
        param_decls.push(format!("ptr %{}", sanitize(p)));
    }
    emitter.body.push_str(&format!(
        "define ptr @{fn_name}({}) {{\nentry:\n",
        param_decls.join(", ")
    ));

    for p in params {
        let ptr = emitter.tmp();
        emitter.body.push_str(&format!(
            "  {ptr} = alloca ptr\n  store ptr %{}, ptr {ptr}\n",
            sanitize(p)
        ));
        emitter.vars.insert(
            p.clone(),
            LlVar {
                ptr,
                ty: LlType::Ptr,
                ir_ty: IrType::Unknown("lambda_param".to_string()),
                drop_kind: DropKind::BorrowOnly,
            },
        );
    }

    for (i, (name, lv)) in free_vars.iter().enumerate() {
        let ptr = emitter.tmp();
        emitter.body
            .push_str(&format!("  {ptr} = alloca {}\n", lv.ty.as_ir()));
        let field_ptr = emitter.tmp();
        emitter.body.push_str(&format!(
            "  {} = getelementptr inbounds %{}, ptr %env, i32 0, i32 {}\n",
            field_ptr, env_struct_name, i
        ));
        let loaded_val = emitter.tmp();
        emitter.body.push_str(&format!(
            "  {} = load {}, ptr {}\n",
            loaded_val,
            lv.ty.as_ir(),
            field_ptr
        ));
        emitter.body.push_str(&format!(
            "  store {} {}, ptr {}\n",
            lv.ty.as_ir(),
            loaded_val,
            ptr
        ));
        emitter.vars.insert(
            name.clone(),
            LlVar {
                ptr,
                ty: lv.ty.clone(),
                ir_ty: lv.ir_ty.clone(),
                drop_kind: DropKind::BorrowOnly,
            },
        );
    }

    let result = emitter.emit_typed_expr(body);
    match result {
        Ok(val) => {
            if !emitter.terminated {
                let ret_val = if val.ty == LlType::Ptr {
                    val.value.clone()
                } else {
                    let cast = emitter.tmp();
                    emitter.body.push_str(&format!(
                        "  {cast} = inttoptr {} {} to ptr\n",
                        val.ty.as_ir(),
                        val.value
                    ));
                    cast
                };
                emitter.body.push_str(&format!("  ret ptr {ret_val}\n"));
            }
        }
        Err(_) => {
            if !emitter.terminated {
                emitter.body.push_str("  ret ptr null\n");
            }
        }
    }
    emitter.body.push_str(&format!("{}:\n", emitter.current_unwind_label));
    emitter.terminated = false;
    emitter.body.push_str("  ret ptr null\n");
    emitter.body.push_str("}\n\n");

    let lambda_func = std::mem::replace(&mut emitter.body, saved_body);
    emitter.globals.push(lambda_func);

    emitter.vars = saved_vars.clone();
    emitter.drop_order = saved_drop_order;
    emitter.tmp = saved_tmp;
    emitter.label = saved_label;
    emitter.current_return = saved_return;
    emitter.current_is_main = saved_is_main;
    emitter.current_unwind_label = saved_unwind;
    emitter.exception_handler = saved_handler;
    emitter.loop_targets = saved_loop;
    emitter.terminated = saved_terminated;

    let delegate_ptr = emitter.tmp();
    let delegate_size_ptr = emitter.tmp();
    let delegate_size = emitter.tmp();
    emitter.body.push_str(&format!(
        "  {delegate_size_ptr} = getelementptr %glitch.delegate, ptr null, i32 1\n  {delegate_size} = ptrtoint ptr {delegate_size_ptr} to i64\n  {delegate_ptr} = call ptr @glitch_calloc(i64 1, i64 {delegate_size})\n"
    ));
    let rc_field = emitter.tmp();
    emitter.body.push_str(&format!(
        "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 0\n  store i64 1, ptr {}\n",
        rc_field, delegate_ptr, rc_field
    ));
    let fn_field = emitter.tmp();
    emitter.body.push_str(&format!(
        "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 1\n",
        fn_field, delegate_ptr
    ));
    emitter
        .body
        .push_str(&format!("  store ptr @{}, ptr {}\n", fn_name, fn_field));
    let env_field = emitter.tmp();
    emitter.body.push_str(&format!(
        "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 2\n",
        env_field, delegate_ptr
    ));
    let destroy_field = emitter.tmp();
    emitter.body.push_str(&format!(
        "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 3\n",
        destroy_field, delegate_ptr
    ));

    if free_vars.is_empty() {
        emitter.body.push_str(&format!(
            "  store ptr null, ptr {}\n  store ptr null, ptr {}\n",
            env_field, destroy_field
        ));
    } else {
        let env_size_ptr = emitter.tmp();
        let env_size = emitter.tmp();
        let env_ptr = emitter.tmp();
        emitter.body.push_str(&format!(
            "  {env_size_ptr} = getelementptr %{}, ptr null, i32 1\n  {env_size} = ptrtoint ptr {env_size_ptr} to i64\n  {env_ptr} = call ptr @glitch_calloc(i64 1, i64 {env_size})\n",
            env_struct_name
        ));

        for (i, (name, lv)) in free_vars.iter().enumerate() {
            if let Some(var) = saved_vars.get(name).cloned() {
                let cap_val = emitter.tmp();
                emitter.body.push_str(&format!(
                    "  {} = load {}, ptr {}\n",
                    cap_val,
                    lv.ty.as_ir(),
                    var.ptr
                ));
                match lv.drop_kind {
                    DropKind::Free if matches!(lv.ir_ty, IrType::String) => emitter
                        .body
                        .push_str(&format!("  call void @glitch_string_retain(ptr {cap_val})\n")),
                    DropKind::DropDelegate => emitter
                        .body
                        .push_str(&format!("  call void @glitch_delegate_retain(ptr {cap_val})\n")),
                    DropKind::DropClass | DropKind::DropStruct => {
                        if let Some(type_name) = object_type_name(&lv.ir_ty) {
                            if let Some(object) = emitter.object_types.get(type_name) {
                                emitter.body.push_str(&format!(
                                    "  call void @{}(ptr {cap_val})\n",
                                    retain_symbol(&object.name)
                                ));
                            }
                        }
                    }
                    _ => {}
                }
                let field_ptr = emitter.tmp();
                emitter.body.push_str(&format!(
                    "  {} = getelementptr inbounds %{}, ptr {}, i32 0, i32 {}\n",
                    field_ptr, env_struct_name, env_ptr, i
                ));
                emitter.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    lv.ty.as_ir(),
                    cap_val,
                    field_ptr
                ));
            }
        }
        emitter.body.push_str(&format!(
            "  store ptr {}, ptr {}\n  store ptr @glitch_lambda_{id}_destroy, ptr {}\n",
            env_ptr, env_field, destroy_field
        ));
    }

    let mut destroy_func = String::new();
    destroy_func.push_str(&format!(
        "define void @glitch_lambda_{id}_destroy(ptr %env) {{\nentry:\n"
    ));
    for (i, (_, lv)) in free_vars.iter().enumerate() {
        let field_ptr = emitter.tmp();
        let loaded_val = emitter.tmp();
        destroy_func.push_str(&format!(
            "  {} = getelementptr inbounds %{}, ptr %env, i32 0, i32 {}\n  {} = load {}, ptr {}\n",
            field_ptr,
            env_struct_name,
            i,
            loaded_val,
            lv.ty.as_ir(),
            field_ptr
        ));
        match lv.drop_kind {
            DropKind::Free if matches!(lv.ir_ty, IrType::String) => {
                destroy_func.push_str(&format!(
                    "  call void @glitch_string_release(ptr {loaded_val})\n"
                ));
            }
            DropKind::DropDelegate => {
                destroy_func.push_str(&format!(
                    "  call void @glitch_delegate_release(ptr {loaded_val})\n"
                ));
            }
            DropKind::DropClass | DropKind::DropStruct => {
                if let Some(type_name) = object_type_name(&lv.ir_ty) {
                    if let Some(object) = emitter.object_types.get(type_name) {
                        destroy_func.push_str(&format!(
                            "  call void @{}(ptr {loaded_val})\n",
                            drop_symbol(&object.name)
                        ));
                    }
                }
            }
            _ => {}
        }
    }
    destroy_func.push_str("  call void @glitch_free(ptr %env)\n  ret void\n}\n\n");
    emitter.globals.push(destroy_func);

    Ok(LlValue {
        value: delegate_ptr,
        ty: LlType::Ptr,
    })
}

