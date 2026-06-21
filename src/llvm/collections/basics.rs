use super::*;

impl LlvmEmitter {
    pub(in crate::llvm) fn emit_new_array(
        &mut self,
        element_type: &IrType,
        length: Option<&TypedExpr>,
        values: &[TypedExpr],
    ) -> Result<LlValue, String> {
        let array = self.tmp();
        let data = self.tmp();
        let len_ptr = self.tmp();
        let data_ptr = self.tmp();
        let element_size = self.emit_type_size(element_type);
        let len_value = if let Some(length) = length {
            let length = self.emit_typed_expr(length)?;
            self.cast_value(length, &LlType::I64)?.value
        } else {
            values.len().to_string()
        };
        let alloc_len = if length.is_some() {
            len_value.clone()
        } else {
            values.len().to_string()
        };
        self.body.push_str(&format!(
            "  {array} = call ptr @glitch_calloc(i64 1, i64 16)\n  {data} = call ptr @glitch_calloc(i64 {}, i64 {element_size})\n  {len_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 0\n  store i64 {}, ptr {len_ptr}\n  {data_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 1\n  store ptr {data}, ptr {data_ptr}\n",
            alloc_len,
            len_value
        ));
        let element_ll_type = llvm_ir_type(element_type);
        for (index, source) in values.iter().enumerate() {
            let value = self.emit_typed_expr(source)?;
            let value = self.cast_value(value, &element_ll_type)?;
            self.retain_for_store(element_type, source, &value.value);
            let slot = self.tmp();
            self.body.push_str(&format!(
                "  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  store {} {}, ptr {slot}\n",
                element_ll_type.as_ir(),
                element_ll_type.as_ir(),
                value.value
            ));
        }
        Ok(LlValue {
            value: array,
            ty: LlType::Ptr,
        })
    }

    pub(in crate::llvm) fn emit_new_collection(&mut self, ty: &IrType) -> Result<LlValue, String> {
        match ty {
            IrType::List(element) => {
                let list = self.tmp();
                let element_size = self.emit_type_size(element);
                let data = self.tmp();
                let cap_ptr = self.tmp();
                let data_ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {list} = call ptr @glitch_calloc(i64 1, i64 24)\n  {data} = call ptr @glitch_calloc(i64 4, i64 {element_size})\n"
                ));
                self.body.push_str(&format!(
                    "  {cap_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 1\n  store i64 4, ptr {cap_ptr}\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 2\n  store ptr {data}, ptr {data_ptr}\n"
                ));
                Ok(LlValue {
                    value: list,
                    ty: LlType::Ptr,
                })
            }
            IrType::Dictionary(key, value) => {
                let dict = self.tmp();
                let key_size = self.emit_type_size(key);
                let value_size = self.emit_type_size(value);
                let keys = self.tmp();
                let values = self.tmp();
                let cap_ptr = self.tmp();
                let keys_ptr = self.tmp();
                let values_ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {dict} = call ptr @glitch_calloc(i64 1, i64 32)\n  {keys} = call ptr @glitch_calloc(i64 4, i64 {key_size})\n  {values} = call ptr @glitch_calloc(i64 4, i64 {value_size})\n  {cap_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 1\n  store i64 4, ptr {cap_ptr}\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 2\n  store ptr {keys}, ptr {keys_ptr}\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 3\n  store ptr {values}, ptr {values_ptr}\n"
                ));
                Ok(LlValue {
                    value: dict,
                    ty: LlType::Ptr,
                })
            }
            other => Err(format!(
                "LLVM TIR backend: unsupported collection allocation {other:?}"
            )),
        }
    }

    pub(in crate::llvm) fn emit_task_run_inline(
        &mut self,
        call: &TypedCall,
        return_type: &IrType,
    ) -> Result<LlValue, String> {
        tasks::emit_task_run_inline(self, call, return_type)
    }

    pub(in crate::llvm) fn emit_task_completed_inline(&mut self) -> LlValue {
        tasks::emit_task_completed_inline(self)
    }

    pub(in crate::llvm) fn emit_task_from_result_inline(
        &mut self,
        call: &TypedCall,
        return_type: &IrType,
    ) -> Result<LlValue, String> {
        tasks::emit_task_from_result_inline(self, call, return_type)
    }

    pub(in crate::llvm) fn emit_task_when_all_inline(&mut self, call: &TypedCall) -> Result<LlValue, String> {
        tasks::emit_task_when_all_inline(self, call)
    }

    pub(in crate::llvm) fn emit_delegate_invoke(
        &mut self,
        delegate_ptr: &str,
        args: &[LlValue],
        return_ty: &IrType,
    ) -> Result<LlValue, String> {
        delegates::emit_delegate_invoke(self, delegate_ptr, args, return_ty)
    }

    pub(in crate::llvm) fn emit_weak_method(
        &mut self,
        target: &TypedExpr,
        name: &str,
        args: &[TypedExpr],
    ) -> Result<LlValue, String> {
        if name == "TryGetTarget" {
            if let IrType::Weak(inner) = &target.ty {
                let weak_val = self.emit_typed_expr(target)?;
                let out_ptr_val = self.emit_typed_expr(&args[0])?;

                let is_null = self.tmp();
                let non_null_lbl = self.next_label("tryget_not_null");
                let null_lbl = self.next_label("tryget_null");
                let done_lbl = self.next_label("tryget_done");
                let success = self.tmp();

                self.body.push_str(&format!(
                    "  {is_null} = icmp eq ptr {}, null\n  br i1 {is_null}, label %{null_lbl}, label %{non_null_lbl}\n",
                    weak_val.value
                ));

                // Non-null block
                self.body.push_str(&format!("\n{non_null_lbl}:\n"));
                if is_string_like_type(inner.as_ref()) {
                    self.body.push_str(&format!(
                        "  call void @glitch_string_retain(ptr {})\n",
                        weak_val.value
                    ));
                } else if let Some(type_name) = object_type_name(inner) {
                    self.emit_retain(type_name, &weak_val.value);
                }
                self.body.push_str(&format!(
                    "  store ptr {}, ptr {}\n  br label %{done_lbl}\n",
                    weak_val.value, out_ptr_val.value
                ));

                // Null block
                self.body.push_str(&format!("\n{null_lbl}:\n"));
                self.body.push_str(&format!(
                    "  store ptr null, ptr {}\n  br label %{done_lbl}\n",
                    out_ptr_val.value
                ));

                // Done block
                self.body.push_str(&format!("\n{done_lbl}:\n"));
                self.body.push_str(&format!(
                    "  {success} = phi i1 [ true, %{non_null_lbl} ], [ false, %{null_lbl} ]\n"
                ));

                return Ok(LlValue {
                    value: success,
                    ty: LlType::I1,
                });
            }
        }
        Err(format!("unsupported weak method: {name}"))
    }

    pub(in crate::llvm) fn emit_task_method(
        &mut self,
        target: &TypedExpr,
        name: &str,
        args: &[TypedExpr],
        expr_ty: &IrType,
    ) -> Result<LlValue, String> {
        let _ = args;
        let task_val = self.emit_typed_expr(target)?;
        if name == "Wait" {
            self.body
                .push_str(&format!("  call void @glitch_task_wait(ptr {})\n", task_val.value));
            Ok(void_value())
        } else if name == "GetResult" || name == "GetAwaiter" || name == "AsTask" {
            if name == "GetAwaiter" {
                Ok(task_val)
            } else if name == "AsTask" {
                Ok(task_val)
            } else {
                let result_ty = expr_ty.clone();
                let result_llvm_type = llvm_ir_type(&result_ty);
                if matches!(result_ty, IrType::Void) {
                    Ok(void_value())
                } else {
                    let call_res = self.tmp();
                    let helper_name = if llvm_ir_type(&result_ty) == LlType::I1
                        || is_bool_like_type(&result_ty)
                    {
                        "glitch_task_get_result_bool"
                    } else {
                        match &result_ty {
                            IrType::Int | IrType::UInt => "glitch_task_get_result_i32",
                            IrType::Long => "glitch_task_get_result_i64",
                            IrType::Double | IrType::Decimal => "glitch_task_get_result_double",
                            _ => "glitch_task_get_result_ptr",
                        }
                    };
                    self.body.push_str(&format!(
                        "  {} = call {} @{}(ptr {})\n",
                        call_res,
                        result_llvm_type.as_ir(),
                        helper_name,
                        task_val.value
                    ));
                    if is_string_like_type(&result_ty) {
                        self.body.push_str(&format!(
                            "  call void @glitch_string_retain(ptr {})\n",
                            call_res
                        ));
                    } else if let Some(type_name) = object_type_name(&result_ty) {
                        if self.object_types.contains_key(type_name) {
                            self.emit_retain(type_name, &call_res);
                        }
                    }
                    Ok(LlValue {
                        value: call_res,
                        ty: result_llvm_type,
                    })
                }
            }
        } else {
            Err(format!(
                "LLVM TIR backend: unsupported Task method '{name}'"
            ))
        }
    }

}
