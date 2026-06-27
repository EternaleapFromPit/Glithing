use super::*;

impl LlvmEmitter {
    pub(in crate::llvm) fn emit_expr(&mut self, expr: &Expr) -> Result<LlValue, String> {
        match expr {
            Expr::Int(value) => Ok(LlValue {
                value: value.to_string(),
                ty: LlType::I64,
            }),
            Expr::Float(value) => Ok(LlValue {
                value: value.to_string(),
                ty: LlType::Double,
            }),
            Expr::Bool(value) => Ok(LlValue {
                value: if *value { "1" } else { "0" }.to_string(),
                ty: LlType::I1,
            }),
            Expr::Null => Ok(LlValue {
                value: "null".to_string(),
                ty: LlType::Ptr,
            }),
            Expr::String(value) => Ok(LlValue {
                value: self.string_global(value),
                ty: LlType::Ptr,
            }),
            Expr::Var(name) | Expr::Move(name) => {
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM backend: unknown variable '{name}'"))?;
                let tmp = self.tmp();
                self.body.push_str(&format!(
                    "  {tmp} = load {}, ptr {}\n",
                    var.ty.as_ir(),
                    var.ptr
                ));
                Ok(LlValue {
                    value: tmp,
                    ty: var.ty,
                })
            }
            Expr::Assign { target, value } => {
                let target = self.emit_expr(target)?;
                let value = self.emit_expr(value)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    value.ty.as_ir(),
                    value.value,
                    target.value
                ));
                Ok(value)
            }
            Expr::Binary { left, op, right } => {
                let left_expr = left;
                let right_expr = right;
                let left = self.emit_expr(left_expr)?;
                if *op == BinaryOp::Coalesce {
                    return Ok(left);
                }
                if matches!(op, BinaryOp::And | BinaryOp::Or) {
                    let right = self.emit_expr(right_expr)?;
                    return self.emit_logical_value(left.clone(), *op, right.clone());
                }
                let raw_right = self.emit_expr(right_expr)?; 
                let right = self.cast_value(raw_right, &left.ty)?;
                let tmp = self.tmp();
                if op.is_comparison() {
                    if left.ty == LlType::Double {
                        let pred = match op {
                            BinaryOp::Eq => "oeq",
                            BinaryOp::NotEq => "one",
                            BinaryOp::Less => "olt",
                            BinaryOp::LessEq => "ole",
                            BinaryOp::Greater => "ogt",
                            BinaryOp::GreaterEq => "oge",
                            BinaryOp::Add
                            | BinaryOp::Sub
                            | BinaryOp::Mul
                            | BinaryOp::Div
                            | BinaryOp::Mod
                            | BinaryOp::Coalesce
                            | BinaryOp::And
                            | BinaryOp::Or
                            | BinaryOp::BitAnd
                            | BinaryOp::BitOr => unreachable!(),
                        };
                        self.body.push_str(&format!(
                            "  {tmp} = fcmp {pred} double {}, {}\n",
                            left.value, right.value
                        ));
                    } else {
                        let pred = match op {
                            BinaryOp::Eq => "eq",
                            BinaryOp::NotEq => "ne",
                            BinaryOp::Less => "slt",
                            BinaryOp::LessEq => "sle",
                            BinaryOp::Greater => "sgt",
                            BinaryOp::GreaterEq => "sge",
                            BinaryOp::Add
                            | BinaryOp::Sub
                            | BinaryOp::Mul
                            | BinaryOp::Div
                            | BinaryOp::Mod
                            | BinaryOp::Coalesce
                            | BinaryOp::And
                            | BinaryOp::Or
                            | BinaryOp::BitAnd
                            | BinaryOp::BitOr => unreachable!(),
                        };
                        self.body.push_str(&format!(
                            "  {tmp} = icmp {pred} {} {}, {}\n",
                            left.ty.as_ir(),
                            left.value,
                            right.value
                        ));
                    }
                    let result = LlValue {
                        value: tmp,
                        ty: LlType::I1,
                    };
                    Ok(result)
                } else if matches!(op, BinaryOp::BitAnd | BinaryOp::BitOr) {
                    let op_name = if *op == BinaryOp::BitAnd { "and" } else { "or" };
                    self.body.push_str(&format!(
                        "  {tmp} = {op_name} {} {}, {}\n",
                        left.ty.as_ir(),
                        left.value, right.value
                    ));
                    let result = LlValue {
                        value: tmp,
                        ty: left.ty,
                    };
                    Ok(result)
                } else if left.ty == LlType::Double {
                    let op_name = match op {
                        BinaryOp::Sub => "fsub",
                        BinaryOp::Mul => "fmul",
                        BinaryOp::Div => "fdiv",
                        BinaryOp::Mod => "frem",
                        _ => "fadd",
                    };
                    self.body.push_str(&format!(
                        "  {tmp} = {op_name} double {}, {}\n",
                        left.value, right.value
                    ));
                    let result = LlValue {
                        value: tmp,
                        ty: LlType::Double,
                    };
                    Ok(result)
                } else {
                    let op_name = match op {
                        BinaryOp::Sub => "sub",
                        BinaryOp::Mul => "mul",
                        BinaryOp::Div => "sdiv",
                        BinaryOp::Mod => "srem",
                        _ => "add",
                    };
                    self.body.push_str(&format!(
                        "  {tmp} = {op_name} {} {}, {}\n",
                        left.ty.as_ir(),
                        left.value,
                        right.value
                    ));
                    let result = LlValue {
                        value: tmp,
                        ty: left.ty,
                    };
                    Ok(result)
                }
            }
            Expr::Unary { op, expr } => {
                let value = self.emit_expr(expr)?;
                match op {
                    UnaryOp::Not => self.emit_not_value(value),
                    UnaryOp::Neg => self.emit_neg_value(value),
                }
            }
            Expr::IncDec {
                name,
                delta,
                prefix,
            } => self.emit_inc_dec_untyped(name, *delta, *prefix),
            Expr::IsPattern { expr, .. } => {
                let _ = self.emit_expr(expr);
                Ok(LlValue {
                    value: "true".to_string(),
                    ty: LlType::I1,
                })
            }
            Expr::Lambda { .. } => Err(
                "LLVM backend: lambda/delegate lowering is not supported in current slice"
                    .to_string(),
            ),
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                let raw_condition = self.emit_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                let when_true = self.emit_expr(when_true)?;
                let raw_false = self.emit_expr(when_false)?;
                let when_false = self.cast_value(raw_false, &when_true.ty)?;
                self.emit_select_value(condition, when_true, when_false)
            }
            Expr::FunctionCall { name, args, .. } => {
                let mut rendered_args = Vec::new();
                for arg in args {
                    let value = self.emit_expr(arg)?;
                    rendered_args.push(format!("{} {}", value.ty.as_ir(), value.value));
                }
                let ret = self
                    .functions
                    .get(name)
                    .map(|signature| signature.return_type.clone())
                    .unwrap_or(LlType::Void);
                if ret == LlType::Void {
                    self.body.push_str(&format!(
                        "  call void @{}({})\n",
                        sanitize(name),
                        rendered_args.join(", ")
                    ));
                    Ok(LlValue {
                        value: "0".to_string(),
                        ty: LlType::I32,
                    })
                } else {
                    let tmp = self.tmp();
                    self.body.push_str(&format!(
                        "  {tmp} = call {} @{}({})\n",
                        ret.as_ir(),
                        sanitize(name),
                        rendered_args.join(", ")
                    ));
                    Ok(LlValue {
                        value: tmp,
                        ty: ret,
                    })
                }
            }
            Expr::Borrow { name, .. } => {
                let var = self
                    .vars
                    .get(name)
                    .ok_or_else(|| format!("LLVM backend: unknown variable '{name}'"))?;
                Ok(LlValue {
                    value: var.ptr.clone(),
                    ty: LlType::Ptr,
                })
            }
            Expr::Await(inner) => Err(format!(
                "LLVM backend: await is not supported in current slice: {inner:?}"
            )),
            _ => Err(format!(
                "LLVM backend: unsupported expression in current slice: {expr:?}"
            )),
        }
    }

    pub(in crate::llvm) fn emit_typed_expr(&mut self, expr: &TypedExpr) -> Result<LlValue, String> {
        match &expr.kind {
            TypedExprKind::Int(value) => Ok(LlValue {
                value: value.to_string(),
                ty: LlType::I64,
            }),
            TypedExprKind::Float(value) => Ok(LlValue {
                value: value.to_string(),
                ty: LlType::Double,
            }),
            TypedExprKind::Bool(value) => Ok(LlValue {
                value: if *value { "1" } else { "0" }.to_string(),
                ty: LlType::I1,
            }),
            TypedExprKind::Null => Ok(LlValue {
                value: "null".to_string(),
                ty: LlType::Ptr,
            }),
            TypedExprKind::String(value) => Ok(LlValue {
                value: self.string_global(value),
                ty: LlType::Ptr,
            }),
            TypedExprKind::NullableSome(value) => {
                let inner = self.emit_typed_expr(value)?;
                let IrType::Nullable(payload_ty) = &expr.ty else {
                    return Err("LLVM TIR backend: NullableSome requires nullable target type".to_string());
                };
                let nullable_name = Self::nullable_type_name(payload_ty);
                let _ = self.ensure_nullable_object_type(payload_ty);
                let llvm_name = llvm_object_name(&nullable_name);
                let size_ptr = self.tmp();
                let size = self.tmp();
                let object = self.tmp();
                self.body.push_str(&format!(
                    "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {object} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
                ));
                let rc_ptr = self.tmp();
                let has_value_ptr = self.tmp();
                let value_ptr = self.tmp();
                let inner_ty = llvm_ir_type(payload_ty);
                let inner_value = self.cast_value(inner, &inner_ty)?;
                self.body.push_str(&format!(
                    "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {object}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {has_value_ptr} = getelementptr inbounds %{llvm_name}, ptr {object}, i32 0, i32 2\n  store i1 true, ptr {has_value_ptr}\n  {value_ptr} = getelementptr inbounds %{llvm_name}, ptr {object}, i32 0, i32 3\n  store {} {}, ptr {value_ptr}\n",
                    inner_ty.as_ir(),
                    inner_value.value
                ));
                Ok(LlValue {
                    value: object,
                    ty: LlType::Ptr,
                })
            }
            TypedExprKind::Var(name) => {
                if self.object_types.contains_key(name) {
                    return Ok(LlValue {
                        value: "null".to_string(),
                        ty: LlType::Ptr,
                    });
                }
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM TIR backend: unknown variable '{name}'"))?;
                let tmp = self.tmp();
                self.body.push_str(&format!(
                    "  {tmp} = load {}, ptr {}\n",
                    var.ty.as_ir(),
                    var.ptr
                ));
                Ok(LlValue {
                    value: tmp,
                    ty: var.ty,
                })
            }
            TypedExprKind::Move(name) => {
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM TIR backend: unknown variable '{name}'"))?;
                let tmp = self.tmp();
                self.body.push_str(&format!(
                    "  {tmp} = load {}, ptr {}\n",
                    var.ty.as_ir(),
                    var.ptr
                ));
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    var.ty.as_ir(),
                    var.ty.default_value(),
                    var.ptr
                ));
                Ok(LlValue {
                    value: tmp,
                    ty: var.ty,
                })
            }
            TypedExprKind::Field { .. } => {
                if let TypedExprKind::Field { target, name } = &expr.kind {
                    if name == "Result" && matches!(target.ty, IrType::Task(_)) {
                        let task_val = self.emit_typed_expr(target)?;
                        let result_ty = expr.ty.clone();
                        let result_llvm_type = llvm_ir_type(&result_ty);
                        if matches!(result_ty, IrType::Void) {
                            return Ok(void_value());
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
                            self.emit_temporary_drop(target, &task_val);
                            self.emit_exception_check();
                            return Ok(LlValue {
                                value: call_res,
                                ty: result_llvm_type,
                            });
                        }
                    }
                    if name == "Exception" && matches!(target.ty, IrType::Task(_)) {
                        let task_val = self.emit_typed_expr(target)?;
                        let exception = self.tmp();
                        self.body.push_str(&format!(
                            "  {} = call ptr @glitch_task_get_exception(ptr {})\n",
                            exception, task_val.value
                        ));
                        self.emit_temporary_drop(target, &task_val);
                        return Ok(LlValue {
                            value: exception,
                            ty: LlType::Ptr,
                        });
                    }
                    if matches!(name.as_str(), "IsCompleted" | "IsCompletedSuccessfully" | "IsFaulted")
                        && matches!(target.ty, IrType::Task(_))
                    {
                        let task_val = self.emit_typed_expr(target)?;
                        let completed = self.tmp();
                        let helper_name = match name.as_str() {
                            "IsCompleted" => "glitch_task_is_completed",
                            "IsCompletedSuccessfully" => "glitch_task_is_completed_successfully",
                            "IsFaulted" => "glitch_task_is_faulted",
                            _ => unreachable!(),
                        };
                        self.body.push_str(&format!(
                            "  {} = call i1 @{}(ptr {})\n",
                            completed, helper_name, task_val.value
                        ));
                        self.emit_temporary_drop(target, &task_val);
                        return Ok(LlValue {
                            value: completed,
                            ty: LlType::I1,
                        });
                    }
                    if name == "Target" && matches!(target.ty, IrType::Weak(_)) {
                        let weak_val = self.emit_typed_expr(target)?;
                        if let IrType::Weak(inner) = &target.ty {
                            if is_string_like_type(inner.as_ref()) {
                                self.body.push_str(&format!(
                                    "  call void @glitch_string_retain(ptr {})\n",
                                    weak_val.value
                                ));
                            } else if let Some(type_name) = object_type_name(inner) {
                                self.emit_retain(type_name, &weak_val.value);
                            }
                            return Ok(LlValue {
                                value: weak_val.value,
                                ty: llvm_ir_type(inner),
                            });
                        }
                    }
                    if name == "Message" && matches!(target.ty, IrType::Exception) {
                        return self.emit_typed_expr(target);
                    }
                    if let IrType::Nullable(inner) = &target.ty {
                        let nullable_name = Self::nullable_type_name(inner);
                        let _ = self.ensure_nullable_object_type(inner);
                        if name == "HasValue" {
                            let nullable = self.emit_typed_expr(target)?;
                            let has_value_ptr = self.tmp();
                            let has_value = self.tmp();
                            self.body.push_str(&format!(
                                "  {has_value_ptr} = getelementptr inbounds %{}, ptr {}, i32 0, i32 2\n  {has_value} = load i1, ptr {has_value_ptr}\n",
                                llvm_object_name(&nullable_name),
                                nullable.value
                            ));
                            return Ok(LlValue {
                                value: has_value,
                                ty: LlType::I1,
                            });
                        }
                        if matches!(name.as_str(), "Value" | "GetValueOrDefault") {
                            let nullable = self.emit_typed_expr(target)?;
                            let is_null = self.tmp();
                            let value_ptr = self.tmp();
                            let value = self.tmp();
                            let result = self.tmp();
                            let payload_ty = llvm_ir_type(inner);
                            self.body.push_str(&format!(
                                "  {is_null} = icmp eq ptr {}, null\n  {value_ptr} = getelementptr inbounds %{}, ptr {}, i32 0, i32 3\n  {value} = load {}, ptr {value_ptr}\n  {result} = select i1 {is_null}, {} {}, {} {}\n",
                                nullable.value,
                                llvm_object_name(&nullable_name),
                                nullable.value,
                                payload_ty.as_ir(),
                                payload_ty.as_ir(),
                                payload_ty.default_value(),
                                payload_ty.as_ir(),
                                value
                            ));
                            return Ok(LlValue {
                                value: result,
                                ty: payload_ty,
                            });
                        }
                    }
                    if name == "Count"
                        && matches!(target.ty, IrType::List(_) | IrType::Dictionary(_, _))
                    {
                        let collection = self.emit_typed_expr(target)?;
                        let len_ptr = self.tmp();
                        let len = self.tmp();
                        let layout = if matches!(target.ty, IrType::List(_)) {
                            "glitch.list"
                        } else {
                            "glitch.dict"
                        };
                        self.body.push_str(&format!(
                            "  {len_ptr} = getelementptr inbounds %{layout}, ptr {}, i32 0, i32 0\n  {len} = load i64, ptr {len_ptr}\n",
                            collection.value
                        ));
                        let count = self.tmp();
                        self.body
                            .push_str(&format!("  {count} = trunc i64 {len} to i32\n"));
                        return Ok(LlValue {
                            value: count,
                            ty: LlType::I32,
                        });
                    }
                    if matches!(name.as_str(), "Length" | "Count")
                        && matches!(target.ty, IrType::Array(_))
                    {
                        let array = self.emit_typed_expr(target)?;
                        let len_ptr = self.tmp();
                        let len = self.tmp();
                        let count = self.tmp();
                        self.body.push_str(&format!(
                            "  {len_ptr} = getelementptr inbounds %glitch.array, ptr {}, i32 0, i32 0\n  {len} = load i64, ptr {len_ptr}\n  {count} = trunc i64 {len} to i32\n",
                            array.value
                        ));
                        return Ok(LlValue {
                            value: count,
                            ty: LlType::I32,
                        });
                    }
                    if name == "Length" && is_string_like_type(&target.ty) {
                        let string = self.emit_typed_expr(target)?;
                        let len = self.tmp();
                        let count = self.tmp();
                        self.body.push_str(&format!(
                            "  {len} = call i64 @strlen(ptr {})\n  {count} = trunc i64 {len} to i32\n",
                            string.value
                        ));
                        return Ok(LlValue {
                            value: count,
                            ty: LlType::I32,
                        });
                    }
                    if name == "Current" {
                        if let Some(result) =
                            self.emit_dictionary_enumerator_method(target, "get_Current", &expr.ty)?
                        {
                            return Ok(result);
                        }
                    }
                    if self.is_opaque_field(expr) {
                        return self.default_typed_value(&expr.ty);
                    }
                }
                let TypedExprKind::Field { target, name } = &expr.kind else {
                    unreachable!();
                };
                let type_name = object_type_name(&target.ty).ok_or_else(|| {
                    format!(
                        "LLVM TIR backend: field '{}' target has no object layout: {:?}",
                        name, target.ty
                    )
                })?;
                let object = self.object_types.get(type_name).cloned().ok_or_else(|| {
                    format!("LLVM TIR backend: type '{type_name}' has no LLVM layout")
                })?;
                let field = object.fields.get(name).cloned().ok_or_else(|| {
                    format!("LLVM TIR backend: type '{type_name}' has no field '{name}'")
                })?;
                let receiver = self.emit_typed_expr(target)?;
                let ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {ptr} = getelementptr inbounds %{}, ptr {}, i32 0, i32 {}\n",
                    llvm_object_name(&object.name),
                    receiver.value,
                    field.index
                ));
                let ty = llvm_ir_type(&expr.ty);
                let value = self.tmp();
                self.body.push_str(&format!(
                    "  {value} = load {}, ptr {ptr}\n",
                    ty.as_ir()
                ));
                if !matches!(
                    target.kind,
                    TypedExprKind::Var(_)
                        | TypedExprKind::Field { .. }
                        | TypedExprKind::Index { .. }
                        | TypedExprKind::Move(_)
                        | TypedExprKind::Borrow { .. }
                ) {
                    self.retain_for_store(&expr.ty, expr, &value);
                    self.emit_temporary_drop(target, &receiver);
                }
                Ok(LlValue { value, ty })
            }
            TypedExprKind::IsPattern {
                expr: inner,
                binding,
            } => {
                let value = self.emit_typed_expr(inner)?;
                if let Some(binding) = binding {
                    let binding_ty = llvm_ir_type(&binding.ty);
                    let stored = self.cast_value(value.clone(), &binding_ty)?;
                    let ptr = if let Some(var) = self.vars.get(&binding.name) {
                        var.ptr.clone()
                    } else {
                        let ptr = self.tmp();
                        self.body
                            .push_str(&format!("  {ptr} = alloca {}\n", binding_ty.as_ir()));
                        self.vars.insert(
                            binding.name.clone(),
                            LlVar {
                                ptr: ptr.clone(),
                                ty: binding_ty.clone(),
                                ir_ty: binding.ty.clone(),
                                drop_kind: DropKind::BorrowOnly,
                            },
                        );
                        ptr
                    };
                    self.body.push_str(&format!(
                        "  store {} {}, ptr {ptr}\n",
                        binding_ty.as_ir(),
                        stored.value
                    ));
                }
                self.to_i1(value)
            }
            TypedExprKind::Assign { target, value } => {
                let value_expr = value.as_ref();
                let value = self.emit_typed_expr(value_expr)?;
                match &target.kind {
                    TypedExprKind::Var(name) => {
                        let var = self
                            .vars
                            .get(name)
                            .cloned()
                            .ok_or_else(|| format!("LLVM TIR backend: unknown variable '{name}'"))?;
                        let casted = self.cast_value(value, &var.ty)?;
                        self.retain_for_store(&var.ir_ty, value_expr, &casted.value);
                        self.emit_var_drop(&var);
                        self.body.push_str(&format!(
                            "  store {} {}, ptr {}\n",
                            var.ty.as_ir(),
                            casted.value,
                            var.ptr
                        ));
                        Ok(casted)
                    }
                    TypedExprKind::Field { .. } if !self.is_opaque_field(target) => {
                        let field_ptr = self.emit_field_ptr(target)?;
                        let field_ty = llvm_ir_type(&target.ty);
                        let casted = self.cast_value(value, &field_ty)?;
                        self.retain_for_store(&target.ty, value_expr, &casted.value);
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
                            casted.value,
                            field_ptr.value
                        ));
                        Ok(casted)
                    }
                    _ => Err(format!(
                        "LLVM TIR backend: unsupported assignment target {:?} with type {:?}",
                        target.kind, target.ty
                    )),
                }
            }
            TypedExprKind::Throw(expr) => self.emit_throw_value(expr),
            TypedExprKind::NewObject {
                type_name,
                constructor: _,
                args,
                fields: _,
            } if is_weak_reference_type_name(type_name) =>
            {
                if let Some(target) = args.first() {
                    self.emit_typed_expr(target)
                } else {
                    Ok(LlValue {
                        value: "null".to_string(),
                        ty: LlType::Ptr,
                    })
                }
            }
            TypedExprKind::NewObject {
                type_name,
                constructor: _,
                args,
                fields: _,
            } if type_name == "Exception" || type_name == "System.Exception" => {
                if let Some(message) = args.first() {
                    let message = self.emit_typed_expr(message)?;
                    self.cast_value(message, &LlType::Ptr)
                } else {
                    Ok(LlValue {
                        value: self.string_global(""),
                        ty: LlType::Ptr,
                    })
                }
            }
            TypedExprKind::NewObject {
                type_name,
                constructor,
                args,
                fields,
            } => self.emit_new_object(type_name, constructor.as_deref(), args, fields),
            TypedExprKind::NewCollection(ty) => self.emit_new_collection(ty),
            TypedExprKind::NewArray {
                element_type,
                length,
                values,
            } => self.emit_new_array(element_type, length.as_deref(), values),
            TypedExprKind::ArrayLiteral(values) => {
                let element_type = values
                    .first()
                    .map(|value| value.ty.clone())
                    .unwrap_or(IrType::Long);
                self.emit_new_array(&element_type, None, values)
            }
            TypedExprKind::Index { target, index } => self.emit_collection_index(target, index),
            TypedExprKind::Binary { left, op, right } => {
                if *op == BinaryOp::Coalesce {
                    return self.emit_coalesce_value(left, right, &expr.ty);
                }
                if let Some(value) =
                    self.emit_task_exception_null_compare(left, *op, right)?
                {
                    return Ok(value);
                }
                let left_expr = left;
                let right_expr = right;
                let left = self.emit_typed_expr(left_expr)?;
                if matches!(op, BinaryOp::And | BinaryOp::Or) {
                    let right = self.emit_typed_expr(right_expr)?;
                    let value = self.emit_logical_value(left.clone(), *op, right.clone());
                    self.emit_temporary_drop(left_expr, &left);
                    self.emit_temporary_drop(right_expr, &right);
                    return value;
                }
                if *op == BinaryOp::Add && is_string_like_type(&expr.ty) {
                    let left = self.cast_value(left, &LlType::Ptr)?;
                    let right = self.emit_typed_expr(right_expr)?;
                    let right = self.cast_value(right, &LlType::Ptr)?;
                    let value = self.tmp();
                    self.body.push_str(&format!(
                        "  {value} = call ptr @glitch_string_concat(ptr {}, ptr {})\n",
                        left.value, right.value
                    ));
                    let result = LlValue {
                        value,
                        ty: LlType::Ptr,
                    };
                    self.emit_temporary_drop(left_expr, &left);
                    self.emit_temporary_drop(right_expr, &right);
                    return Ok(result);
                }
                let raw_right = self.emit_typed_expr(right_expr)?;
                let raw_right_for_drop = raw_right.clone();
                let right = self.cast_value(raw_right, &left.ty)?;
                if left.ty == LlType::Ptr && !op.is_comparison() {
                    return self.default_typed_value(&expr.ty);
                }
                let result = self.emit_binary_value(left.clone(), *op, right.clone())?;
                self.emit_temporary_drop(left_expr, &left);
                self.emit_temporary_drop(right_expr, &raw_right_for_drop);
                Ok(result)
            }
            TypedExprKind::Unary { op, expr } => {
                let value = self.emit_typed_expr(expr)?;
                match op {
                    UnaryOp::Not => self.emit_not_value(value),
                    UnaryOp::Neg => self.emit_neg_value(value),
                }
            }
            TypedExprKind::IncDec {
                target,
                delta,
                prefix,
            } => self.emit_inc_dec_value(target, *delta, *prefix),
            TypedExprKind::Lambda { params, body } => {
                self.emit_lambda_function(params, body, &expr.ty)
            }
            TypedExprKind::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                let raw_condition = self.emit_typed_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                let result_ty = llvm_ir_type(&expr.ty);
                if result_ty == LlType::Void {
                    let true_label = self.next_label("conditional_true");
                    let false_label = self.next_label("conditional_false");
                    let end_label = self.next_label("conditional_end");
                    self.body.push_str(&format!(
                        "  br i1 {}, label %{true_label}, label %{false_label}\n{true_label}:\n",
                        condition.value
                    ));
                    let true_value = self.emit_typed_expr(when_true)?;
                    self.emit_temporary_drop(when_true, &true_value);
                    self.body.push_str(&format!("  br label %{end_label}\n{false_label}:\n"));
                    let false_value = self.emit_typed_expr(when_false)?;
                    self.emit_temporary_drop(when_false, &false_value);
                    self.body.push_str(&format!("  br label %{end_label}\n{end_label}:\n"));
                    return Ok(void_value());
                }
                let result_ptr = self.tmp();
                let true_label = self.next_label("conditional_true");
                let false_label = self.next_label("conditional_false");
                let end_label = self.next_label("conditional_end");
                self.body.push_str(&format!(
                    "  {result_ptr} = alloca {}\n  br i1 {}, label %{true_label}, label %{false_label}\n{true_label}:\n",
                    result_ty.as_ir(),
                    condition.value
                ));
                let true_value = self.emit_typed_expr(when_true)?;
                let true_value = self.cast_value(true_value, &result_ty)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {result_ptr}\n  br label %{end_label}\n{false_label}:\n",
                    result_ty.as_ir(),
                    true_value.value
                ));
                let false_value = self.emit_typed_expr(when_false)?;
                let false_value = self.cast_value(false_value, &result_ty)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {result_ptr}\n  br label %{end_label}\n{end_label}:\n",
                    result_ty.as_ir(),
                    false_value.value
                ));
                let result = self.tmp();
                self.body.push_str(&format!(
                    "  {result} = load {}, ptr {result_ptr}\n",
                    result_ty.as_ir()
                ));
                Ok(LlValue {
                    value: result,
                    ty: result_ty,
                })
            }
            TypedExprKind::Call(call) => {
                if let TypedCallKind::Method { target, name, .. } = &call.kind {
                    let is_task = is_task_surface_type(&target.ty);
                    let is_mediator = match &target.ty {
                        IrType::Interface(name) | IrType::Class(name) => {
                            base_type_name(name) == "IMediator"
                        }
                        _ => false,
                    };
                    if is_mediator && name == "Send" {
                        let response_ty = match &expr.ty {
                            IrType::Task(inner) => inner.as_ref().clone(),
                            other => other.clone(),
                        };
                        return self.emit_mediator_send(target, &call.args[0], &response_ty);
                    }
                    let is_mapper = match &target.ty {
                        IrType::Interface(name) | IrType::Class(name) => {
                            base_type_name(name) == "IMapper"
                        }
                        _ => false,
                    };
                    if is_mapper && name == "Map" && !call.args.is_empty() {
                        return self.emit_mapper_map(
                            target,
                            &call.args[0],
                            &call.generic_args,
                            &expr.ty,
                        );
                    }
                    if is_task && name == "Run" {
                        return self.emit_task_run_inline(call, &expr.ty);
                    }
                    if is_task && name == "FromResult" {
                        return self.emit_task_from_result_inline(call, &expr.ty);
                    }
                    if is_task && name == "FromException" {
                        return self.emit_task_from_exception_inline(call, &expr.ty);
                    }
                    if is_task && name == "WhenAll" {
                        return self.emit_task_when_all_inline(call);
                    }
                    if is_task && name == "CompletedTask" && call.args.is_empty() {
                        return Ok(self.emit_task_completed_inline());
                    }
                }
                match &call.kind {
                    TypedCallKind::Function { name, symbol } => {
                if name == "sizeof" {
                    let size = if let Some(arg) = call.args.first() {
                        if let TypedExprKind::Var(type_name) = &arg.kind {
                            if self.object_types.contains_key(type_name) {
                                let llvm_name = llvm_object_name(type_name);
                                let size_ptr = self.tmp();
                                let size = self.tmp();
                                self.body.push_str(&format!(
                                    "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n"
                                ));
                                return Ok(LlValue {
                                    value: size,
                                    ty: LlType::I64,
                                });
                            }
                            match type_name.as_str() {
                                "bool" => 1,
                                "byte" | "sbyte" => 1,
                                "short" | "ushort" => 2,
                                "int" | "uint" => 4,
                                "long" | "ulong" => 8,
                                "float" => 4,
                                "double" => 8,
                                _ => 8,
                            }
                        } else {
                            4
                        }
                            } else {
                                4
                            };
                            return Ok(LlValue {
                                value: size.to_string(),
                                ty: LlType::I32,
                            });
                        }
                        if matches!(
                            name.as_str(),
                            "Ok" | "Created" | "CreatedAtAction" | "Accepted"
                        ) {
                            return self.emit_mvc_result_value(name, &call.args);
                        }
                        if matches!(name.as_str(), "NoContent" | "NotFound" | "BadRequest") {
                            for arg in &call.args {
                                let value = self.emit_typed_expr(arg)?;
                                self.emit_temporary_drop(arg, &value);
                            }
                            return Ok(LlValue {
                                value: self.string_global(""),
                                ty: LlType::Ptr,
                            });
                        }
                        if name == "GlitchRestHost_Run" || symbol == "GlitchRestHost_Run" {
                            return self.emit_rest_host_run(&call.args);
                        }
                        if name == "glitch_register_attribute_routes"
                            || symbol == "glitch_register_attribute_routes"
                        {
                            for arg in &call.args {
                                let value = self.emit_typed_expr(arg)?;
                                self.emit_temporary_drop(arg, &value);
                            }
                            return Ok(void_value());
                        }
                        if name == "GlitchEndpointHandlers_Contains"
                            || symbol == "GlitchEndpointHandlers_Contains"
                        {
                            return self.emit_generated_endpoint_call(
                                "glitch_endpoint_handlers_contains",
                                &call.args,
                                LlType::I1,
                            );
                        }
                        if name == "GlitchEndpointHandlers_Invoke"
                            || symbol == "GlitchEndpointHandlers_Invoke"
                        {
                            return self.emit_generated_endpoint_call(
                                "glitch_endpoint_handlers_invoke",
                                &call.args,
                                LlType::Ptr,
                            );
                        }
                        if name == "GlitchMiddlewareHandlers_Apply"
                            || symbol == "GlitchMiddlewareHandlers_Apply"
                        {
                            let [app, text] = call.args.as_slice() else {
                                return Err(
                                "LLVM TIR backend: GlitchMiddlewareHandlers_Apply expects app and text"
                                    .to_string(),
                            );
                            };
                            let app_value = self.emit_typed_expr(app)?;
                            let text_value = self.emit_typed_expr(text)?;
                            self.emit_temporary_drop(app, &app_value);
                            self.body.push_str(&format!(
                                "  call void @glitch_string_retain(ptr {})\n",
                                text_value.value
                            ));
                            return self.cast_value(text_value, &LlType::Ptr);
                        }
                        if self.functions.contains_key(symbol) {
                            self.emit_typed_function_call(symbol, &call.generic_args, &call.args)
                        } else if self.vars.contains_key(symbol) {
                            let var = self.vars.get(symbol).unwrap().clone();
                            let delegate_ptr = self.tmp();
                            self.body.push_str(&format!(
                                "  {} = load ptr, ptr {}\n",
                                delegate_ptr, var.ptr
                            ));
                            let fn_ptr_addr = self.tmp();
                            self.body.push_str(&format!(
                            "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 1\n",
                            fn_ptr_addr, delegate_ptr
                        ));
                            let fn_ptr = self.tmp();
                            self.body.push_str(&format!(
                                "  {} = load ptr, ptr {}\n",
                                fn_ptr, fn_ptr_addr
                            ));
                            let env_ptr_addr = self.tmp();
                            self.body.push_str(&format!(
                            "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 2\n",
                            env_ptr_addr, delegate_ptr
                        ));
                            let env_ptr = self.tmp();
                            self.body.push_str(&format!(
                                "  {} = load ptr, ptr {}\n",
                                env_ptr, env_ptr_addr
                            ));
                            let mut arg_vals = Vec::new();
                            let mut arg_values = Vec::new();
                            arg_vals.push(format!("ptr {env_ptr}"));
                            for arg in &call.args {
                                let val = self.emit_typed_expr(arg)?;
                                arg_values.push(val.clone());
                                arg_vals.push(format!("{} {}", val.ty.as_ir(), val.value));
                            }
                            let result_reg = self.tmp();
                            let ret_ty = llvm_ir_type(&expr.ty);
                            self.body.push_str(&format!(
                                "  {} = call {} {}({})\n",
                                result_reg,
                                ret_ty.as_ir(),
                                fn_ptr,
                                arg_vals.join(", ")
                            ));
                            for (arg, value) in call.args.iter().zip(arg_values.iter()) {
                                if should_drop_argument_after_call(arg) {
                                    self.emit_temporary_drop(arg, value);
                                }
                            }
                            Ok(LlValue {
                                value: result_reg,
                                ty: ret_ty,
                            })
                        } else {
                            self.emit_opaque_call(None, &call.args, &expr.ty)
                        }
                    }
                    TypedCallKind::Method {
                        target,
                        name,
                        symbol,
                        resolution,
                    } => match resolution {
                        CallResolution::InstanceMethod => {
                            if is_configuration_manager_type(&target.ty) {
                                if name == "GetValue" {
                                    return self.emit_configuration_value_lookup(target, &call.args, &expr.ty);
                                }
                            }
                            if is_object_surface_type(&target.ty)
                                && name == "Equals"
                                && call.args.len() == 1
                            {
                                return self.emit_intrinsic_equals_pair(target, &call.args[0]);
                            }
                            if is_service_collection_type(&target.ty)
                                && matches!(name.as_str(), "AddTransient" | "AddScoped")
                            {
                                return self.emit_service_registration_marker(
                                    name,
                                    target,
                                    &call.args,
                                    &call.generic_args,
                                );
                            }
                            if is_service_collection_type(&target.ty)
                                && name == "AddSingleton"
                                && call.generic_args.len() == 1
                                && call.args.len() == 1
                            {
                                return self.emit_service_singleton_registration(
                                    target,
                                    &call.args,
                                    &call.generic_args,
                                );
                            }
                            if is_service_provider_lookup_type(&target.ty)
                                && matches!(name.as_str(), "GetRequiredService" | "GetService")
                                && call.args.is_empty()
                            {
                                return self.emit_service_provider_lookup(
                                    target,
                                    name,
                                    &expr.ty,
                                    &call.generic_args,
                                );
                            }
                            if matches!(target.ty, IrType::Dictionary(_, _)) && name == "GetEnumerator" {
                                return self.emit_collection_method(target, name, &call.args);
                            }
                            if let Some(result) =
                                self.emit_dictionary_enumerator_method(target, name, &expr.ty)?
                            {
                                return Ok(result);
                            }
                            let resolved_symbol = match &target.ty {
                                IrType::Class(type_name)
                                | IrType::Struct(type_name)
                                | IrType::Interface(type_name)
                                    if type_name.contains('<') =>
                                {
                                    if matches!(target.ty, IrType::Interface(_)) {
                                        if let Some(resolved) = self.resolve_interface_method_symbol(
                                            type_name,
                                            name,
                                            call.args.len(),
                                        ) {
                                            resolved
                                        } else {
                                            let specialized = self.specialized_instance_symbols
                                                .get(&(symbol.clone(), type_name.clone()))
                                                .cloned()
                                                .unwrap_or_else(|| symbol.clone());
                                            specialized
                                        }
                                    } else {
                                        let specialized = self.specialized_instance_symbols
                                        .get(&(symbol.clone(), type_name.clone()))
                                        .cloned()
                                        .unwrap_or_else(|| symbol.clone());
                                        if self.functions.contains_key(&specialized) {
                                            specialized
                                        } else {
                                            specialized
                                        }
                                    }
                                }
                                IrType::Interface(interface_name) => self
                                    .resolve_interface_method_symbol(
                                        interface_name,
                                        name,
                                        call.args.len(),
                                    )
                                    .unwrap_or_else(|| symbol.clone()),
                                _ => symbol.clone(),
                            };
                            if self.functions.contains_key(&resolved_symbol)
                                || matches!(target.ty, IrType::Interface(_))
                            {
                                let receiver = self.emit_typed_expr(target)?;
                                let result = self.emit_typed_call(
                                    &resolved_symbol,
                                    std::iter::once(receiver.clone()),
                                    &call.generic_args,
                                    &call.args,
                                )?;
                                self.emit_temporary_drop(target, &receiver);
                                Ok(result)
                            } else {
                                self.emit_opaque_call(Some(target), &call.args, &expr.ty)
                            }
                        }
                        CallResolution::StaticFunction => {
                            if is_object_surface_type(&target.ty)
                                && name == "Equals"
                                && call.args.len() == 2
                            {
                                return self.emit_intrinsic_equals_pair(
                                    &call.args[0],
                                    &call.args[1],
                                );
                            }
                            if is_task_surface_type(&target.ty) {
                                if name == "Run" {
                                    return self.emit_task_run_inline(call, &expr.ty);
                                }
                                if name == "FromResult" {
                                    return self.emit_task_from_result_inline(call, &expr.ty);
                                }
                                if name == "FromException" {
                                    return self.emit_task_from_exception_inline(call, &expr.ty);
                                }
                                if name == "WhenAll" {
                                    return self.emit_task_when_all_inline(call);
                                }
                                if name == "CompletedTask" && call.args.is_empty() {
                                    return Ok(self.emit_task_completed_inline());
                                }
                            }
                            if self.functions.contains_key(symbol) {
                                self.emit_typed_function_call(symbol, &call.generic_args, &call.args)
                            } else {
                                self.emit_opaque_call(Some(target), &call.args, &expr.ty)
                            }
                        }
                        CallResolution::CollectionMethod => {
                            self.emit_collection_method(target, name, &call.args)
                        }
                        CallResolution::TaskMethod => {
                            self.emit_task_method(target, name, &call.args, &expr.ty)
                        }
                        CallResolution::WeakMethod => {
                            self.emit_weak_method(target, name, &call.args)
                        }
                        CallResolution::EndpointHandlerRegistration { .. } => {
                            let receiver = self.emit_typed_expr(target)?;
                            for arg in &call.args {
                                if matches!(arg.kind, TypedExprKind::FunctionSymbol(_)) {
                                    continue;
                                }
                                let value = self.emit_typed_expr(arg)?;
                                self.emit_temporary_drop(arg, &value);
                            }
                            self.emit_temporary_drop(target, &receiver);
                            Ok(void_value())
                        }
                        CallResolution::Unknown => {
                            if is_service_collection_type(&target.ty)
                                && matches!(name.as_str(), "AddTransient" | "AddScoped")
                            {
                                return self.emit_service_registration_marker(
                                    name,
                                    target,
                                    &call.args,
                                    &call.generic_args,
                                );
                            }
                            if is_service_collection_type(&target.ty)
                                && name == "AddSingleton"
                                && call.generic_args.len() == 1
                                && call.args.len() == 1
                            {
                                return self.emit_service_singleton_registration(
                                    target,
                                    &call.args,
                                    &call.generic_args,
                                );
                            }
                            if is_service_provider_lookup_type(&target.ty)
                                && matches!(name.as_str(), "GetRequiredService" | "GetService")
                                && call.args.is_empty()
                            {
                                return self.emit_service_provider_lookup(
                                    target,
                                    name,
                                    &expr.ty,
                                    &call.generic_args,
                                );
                            }
                            if matches!(target.ty, IrType::Dictionary(_, _)) && name == "GetEnumerator" {
                                return self.emit_collection_method(target, name, &call.args);
                            }
                            if let Some(result) =
                                self.emit_dictionary_enumerator_method(target, name, &expr.ty)?
                            {
                                return Ok(result);
                            }
                            if let IrType::Interface(interface_name) = &target.ty {
                                if let Some(resolved_symbol) = self.resolve_interface_method_symbol(
                                    interface_name,
                                    name,
                                    call.args.len(),
                                ) {
                                    let receiver = self.emit_typed_expr(target)?;
                                    let result = self.emit_typed_call(
                                        &resolved_symbol,
                                        std::iter::once(receiver.clone()),
                                        &call.generic_args,
                                        &call.args,
                                    )?;
                                    self.emit_temporary_drop(target, &receiver);
                                    return Ok(result);
                                }
                            }
                            self.emit_opaque_call(Some(target), &call.args, &expr.ty)
                        }
                    },
                }
            }
            TypedExprKind::Borrow { name, .. } => {
                if !self.vars.contains_key(name) {
                    let inner_type = match &expr.ty {
                        IrType::Ref(inner) => inner.as_ref().clone(),
                        _ => IrType::Unknown(format!("out {name}")),
                    };
                    let ty = llvm_ir_type(&inner_type);
                    let ptr = self.tmp();
                    self.body.push_str(&format!(
                        "  {ptr} = alloca {}\n  store {} {}, ptr {ptr}\n",
                        ty.as_ir(),
                        ty.as_ir(),
                        ty.default_value()
                    ));
                    self.vars.insert(
                        name.clone(),
                        LlVar {
                            ptr,
                            ty,
                            ir_ty: inner_type,
                            drop_kind: DropKind::BorrowOnly,
                        },
                    );
                }
                let var = self.vars.get(name).expect("out variable was inserted");
                Ok(LlValue {
                    value: var.ptr.clone(),
                    ty: LlType::Ptr,
                })
            }
            TypedExprKind::Await(inner) => {
                let task_val = self.emit_typed_expr(inner)?;
                if let Some(pc_ptr) = self.async_state_pc_ptr.clone() {
                    self.async_suspend_index += 1;
                    self.body.push_str(&format!(
                        "  store i32 {}, ptr {pc_ptr}\n",
                        self.async_suspend_index
                    ));
                }
                let result_ty = expr.ty.clone();
                let result_llvm_type = llvm_ir_type(&result_ty);
                if matches!(result_ty, IrType::Void) {
                    self.emit_temporary_drop(inner, &task_val);
                    self.emit_exception_check();
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
                    self.emit_temporary_drop(inner, &task_val);
                    self.emit_exception_check();
                    Ok(LlValue {
                        value: call_res,
                        ty: result_llvm_type,
                    })
                }
            }
            TypedExprKind::FunctionSymbol(name) => {
                let id = self.lambda_id;
                self.lambda_id += 1;
                let wrapper_name = format!("glitch_delegate_wrapper_{name}_{id}");

                let (ret_ty, params) = if let Some(sig) = self.functions.get(name) {
                    (sig.return_type.clone(), sig.params.clone())
                } else {
                    (LlType::Ptr, vec![LlType::Ptr])
                };

                let mut wrapper_params = vec!["ptr %env".to_string()];
                let mut call_args = Vec::new();
                for (i, p) in params.iter().enumerate() {
                    wrapper_params.push(format!("{} %arg{}", p.as_ir(), i));
                    call_args.push(format!("{} %arg{}", p.as_ir(), i));
                }

                // Swap self.body to emit the wrapper function globally
                let saved_body = std::mem::take(&mut self.body);
                self.body.push_str(&format!(
                    "define {} @{}({}) {{\nentry:\n",
                    ret_ty.as_ir(),
                    wrapper_name,
                    wrapper_params.join(", ")
                ));
                if ret_ty == LlType::Void {
                    self.body.push_str(&format!(
                        "  call void @{}({})\n  ret void\n}}\n\n",
                        name,
                        call_args.join(", ")
                    ));
                } else {
                    let tmp_reg = format!("%t_wrap_{}", self.tmp);
                    self.tmp += 1;
                    self.body.push_str(&format!(
                        "  {} = call {} @{}({})\n  ret {} {}\n}}\n\n",
                        tmp_reg,
                        ret_ty.as_ir(),
                        name,
                        call_args.join(", "),
                        ret_ty.as_ir(),
                        tmp_reg
                    ));
                }
                let wrapper_func = std::mem::replace(&mut self.body, saved_body);
                self.globals.push(wrapper_func);

                // Allocate the delegate struct on the heap so it can escape the current scope.
                let delegate_ptr = self.tmp();
                let delegate_size_ptr = self.tmp();
                let delegate_size = self.tmp();
                self.body.push_str(&format!(
                    "  {delegate_size_ptr} = getelementptr %glitch.delegate, ptr null, i32 1\n  {delegate_size} = ptrtoint ptr {delegate_size_ptr} to i64\n  {delegate_ptr} = call ptr @glitch_calloc(i64 1, i64 {delegate_size})\n"
                ));
                let rc_field = self.tmp();
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 0\n  store i64 1, ptr {}\n",
                    rc_field, delegate_ptr, rc_field
                ));
                let fn_field = self.tmp();
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 1\n",
                    fn_field, delegate_ptr
                ));
                self.body.push_str(&format!(
                    "  store ptr @{}, ptr {}\n",
                    wrapper_name, fn_field
                ));
                let env_field = self.tmp();
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 2\n  store ptr null, ptr {}\n",
                    env_field, delegate_ptr, env_field
                ));
                let destroy_field = self.tmp();
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds %glitch.delegate, ptr {}, i32 0, i32 3\n  store ptr null, ptr {}\n",
                    destroy_field, delegate_ptr, destroy_field
                ));

                Ok(LlValue {
                    value: delegate_ptr,
                    ty: LlType::Ptr,
                })
            }
            _ => Err(format!(
                "LLVM TIR backend: unsupported expression in current slice: {expr:?}"
            )),
        }
    }

    pub(in crate::llvm) fn emit_typed_function_call(
        &mut self,
        name: &str,
        generic_args: &[IrType],
        args: &[TypedExpr],
    ) -> Result<LlValue, String> {
        self.emit_typed_call(name, std::iter::empty(), generic_args, args)
    }

    pub(in crate::llvm) fn emit_mvc_result_value(&mut self, name: &str, args: &[TypedExpr]) -> Result<LlValue, String> {
        let selected_index = if name == "CreatedAtAction" {
            args.len().checked_sub(1)
        } else {
            (!args.is_empty()).then_some(0)
        };
        let mut values = Vec::with_capacity(args.len());
        for arg in args {
            values.push(self.emit_typed_expr(arg)?);
        }
        let Some(selected_index) = selected_index else {
            return Ok(LlValue {
                value: self.string_global(""),
                ty: LlType::Ptr,
            });
        };
        let selected = values[selected_index].clone();
        let selected_expr = &args[selected_index];
        if !matches!(
            selected_expr.kind,
            TypedExprKind::NewObject { .. } | TypedExprKind::Move(_)
        ) {
            if let Some(type_name) = object_type_name(&selected_expr.ty) {
                if self.object_types.contains_key(type_name) {
                    self.emit_retain(type_name, &selected.value);
                }
            } else if is_string_like_type(&selected_expr.ty) {
                self.body.push_str(&format!(
                    "  call void @glitch_string_retain(ptr {})\n",
                    selected.value
                ));
            }
        }
        for (index, (arg, value)) in args.iter().zip(values.iter()).enumerate() {
            if index != selected_index {
                self.emit_temporary_drop(arg, value);
            }
        }
        self.cast_value(selected, &LlType::Ptr)
    }

    pub(in crate::llvm) fn emit_generated_endpoint_call(
        &mut self,
        name: &str,
        args: &[TypedExpr],
        return_type: LlType,
    ) -> Result<LlValue, String> {
        let expected = if name == "glitch_endpoint_handlers_invoke" {
            4
        } else {
            3
        };
        if args.len() != expected {
            return Err(format!(
                "LLVM TIR backend: {name} expects {expected} arguments"
            ));
        }
        let mut rendered = Vec::new();
        for arg in args {
            let value = self.emit_typed_expr(arg)?;
            let value = self.cast_value(value, &LlType::Ptr)?;
            rendered.push(value.value);
        }
        let result = self.tmp();
        self.body.push_str(&format!(
            "  {result} = call {} @{name}({})\n",
            return_type.as_ir(),
            rendered
                .iter()
                .map(|value| format!("ptr {value}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        Ok(LlValue {
            value: result,
            ty: return_type,
        })
    }

    pub(in crate::llvm) fn emit_configuration_value_lookup(
        &mut self,
        target: &TypedExpr,
        args: &[TypedExpr],
        result_ty: &IrType,
    ) -> Result<LlValue, String> {
        let [key_expr] = args else {
            return Err(
                "LLVM TIR backend: ConfigurationManager.GetValue expects one key argument"
                    .to_string(),
            );
        };
        let (receiver, full_key, key_value) =
            self.emit_configuration_lookup_key(target, key_expr)?;
        let result = match result_ty {
            IrType::String => {
                let value = self.tmp();
                let empty = self.string_global("");
                self.body.push_str(&format!(
                    "  {value} = call ptr @GlitchRestHost_read_env_string(ptr {}, ptr {})\n",
                    full_key, empty
                ));
                LlValue {
                    value,
                    ty: LlType::Ptr,
                }
            }
            IrType::Int | IrType::UInt => {
                let value = self.tmp();
                self.body.push_str(&format!(
                    "  {value} = call i32 @GlitchRestHost_read_env_int(ptr {}, i32 0)\n",
                    full_key
                ));
                LlValue {
                    value,
                    ty: LlType::I32,
                }
            }
            IrType::Long => {
                let value = self.tmp();
                self.body.push_str(&format!(
                    "  {value} = call i64 @GlitchRestHost_read_env_i64(ptr {}, i64 0)\n",
                    full_key
                ));
                LlValue {
                    value,
                    ty: LlType::I64,
                }
            }
            IrType::Bool => {
                let value = self.tmp();
                self.body.push_str(&format!(
                    "  {value} = call i1 @GlitchRestHost_read_env_bool(ptr {}, i1 false)\n",
                    full_key
                ));
                LlValue {
                    value,
                    ty: LlType::I1,
                }
            }
            other => {
                self.body
                    .push_str(&format!("  call void @glitch_string_release(ptr {full_key})\n"));
                self.emit_temporary_drop(key_expr, &key_value);
                self.emit_temporary_drop(target, &receiver);
                return Err(format!(
                    "LLVM TIR backend: ConfigurationManager.GetValue<{other:?}> is not supported yet; rewrite to string/int/long/bool lookup or bind the object explicitly"
                ));
            }
        };
        self.body
            .push_str(&format!("  call void @glitch_string_release(ptr {full_key})\n"));
        self.emit_temporary_drop(key_expr, &key_value);
        self.emit_temporary_drop(target, &receiver);
        Ok(result)
    }

    fn emit_configuration_lookup_key(
        &mut self,
        target: &TypedExpr,
        key_expr: &TypedExpr,
    ) -> Result<(LlValue, String, LlValue), String> {
        let type_name = object_type_name(&target.ty).ok_or_else(|| {
            format!(
                "LLVM TIR backend: ConfigurationManager lookup target has no object layout: {:?}",
                target.ty
            )
        })?;
        let object = self.object_types.get(type_name).cloned().ok_or_else(|| {
            format!("LLVM TIR backend: type '{type_name}' has no LLVM layout")
        })?;
        let prefix_field = object.fields.get("Prefix").cloned().ok_or_else(|| {
            format!("LLVM TIR backend: type '{type_name}' is missing field 'Prefix'")
        })?;
        let receiver = self.emit_typed_expr(target)?;
        let prefix_ptr = self.tmp();
        let prefix = self.tmp();
        self.body.push_str(&format!(
            "  {prefix_ptr} = getelementptr inbounds %{}, ptr {}, i32 0, i32 {}\n  {prefix} = load ptr, ptr {prefix_ptr}\n",
            llvm_object_name(&object.name),
            receiver.value,
            prefix_field.index
        ));
        let key_value = self.emit_typed_expr(key_expr)?;
        let key_value = self.cast_value(key_value, &LlType::Ptr)?;
        let full_key = self.tmp();
        self.body.push_str(&format!(
            "  {full_key} = call ptr @glitch_string_concat(ptr {prefix}, ptr {})\n",
            key_value.value
        ));
        Ok((
            receiver,
            full_key,
            key_value,
        ))
    }

    pub(in crate::llvm) fn emit_rest_host_run(&mut self, args: &[TypedExpr]) -> Result<LlValue, String> {
        let [app, port, max_requests] = args else {
            return Err(
                "LLVM TIR backend: GlitchRestHost_Run expects app, port, and max_requests"
                    .to_string(),
            );
        };
        let app_value = self.emit_typed_expr(app)?;
        let app_value = self.cast_value(app_value, &LlType::Ptr)?;
        let port_value = self.emit_typed_expr(port)?;
        let port_value = self.cast_value(port_value, &LlType::I32)?;
        let max_requests_value = self.emit_typed_expr(max_requests)?;
        let max_requests_value = self.cast_value(max_requests_value, &LlType::I32)?;
        self.body.push_str(&format!(
            "  call void @GlitchRestHost_Run(ptr {}, i32 {}, i32 {}, ptr @WebApplication_Handle, ptr @glitch_string_release)\n",
            app_value.value, port_value.value, max_requests_value.value
        ));
        self.emit_temporary_drop(app, &app_value);
        self.emit_temporary_drop(port, &port_value);
        self.emit_temporary_drop(max_requests, &max_requests_value);
        Ok(void_value())
    }

    pub(in crate::llvm) fn emit_typed_call<I>(
        &mut self,
        name: &str,
        prefix: I,
        generic_args: &[IrType],
        args: &[TypedExpr],
    ) -> Result<LlValue, String>
    where
        I: IntoIterator<Item = LlValue>,
    {
        let resolved_name = self
            .specialized_symbols
            .get(&(name.to_string(), generic_args.to_vec()))
            .cloned()
            .unwrap_or_else(|| name.to_string());
        let signature =
            self.functions.get(&resolved_name).cloned().ok_or_else(|| {
                format!("LLVM TIR backend: function '{name}' has no lowered body")
            })?;
        if is_safe_generic_wrapper_symbol(&resolved_name) {
            let [arg] = args else {
                return Err(format!(
                    "LLVM TIR backend: wrapper '{name}' expects one argument"
                ));
            };
            let value = self.emit_typed_expr(arg)?;
            return self.cast_value(value, &signature.return_type);
        }
        let prefix_values = prefix.into_iter().collect::<Vec<_>>();
        let prefix_len = prefix_values.len();
        let mut values = prefix_values.clone();
        let mut arg_values = Vec::new();
        for arg in args {
            let value = self.emit_typed_expr(arg)?;
            arg_values.push(value.clone());
            values.push(value);
        }
        if values.len() < signature.params.len() && values.len() >= signature.required_params {
            for expected in signature.params.iter().skip(values.len()) {
                values.push(LlValue {
                    value: expected.default_value().to_string(),
                    ty: expected.clone(),
                });
            }
        }
        if values.is_empty()
            && signature.params.len() == args.len() + 1
            && (name.contains("_get_") || name.starts_with("get_"))
            && self.vars.contains_key("this")
        {
            if let Some(this) = self.vars.get("this").cloned() {
                values.push(LlValue {
                    value: this.ptr,
                    ty: this.ty,
                });
            }
        }
        if values.len() != signature.params.len() {
            return Err(format!(
                "LLVM TIR backend: call to '{name}' expected {} arguments but got {}",
                signature.params.len(),
                values.len()
            ));
        }
        let mut rendered_args = Vec::new();
        for (value, expected) in prefix_values.into_iter().zip(signature.params.iter()) {
            let value = self.cast_value(value, expected)?;
            rendered_args.push(format!("{} {}", expected.as_ir(), value.value));
        }
        let mut casted_arg_values = Vec::new();
        for (arg_index, value) in arg_values.iter().cloned().enumerate() {
            let expected = &signature.params[prefix_len + arg_index];
            let value = self.cast_value(value, expected)?;
            casted_arg_values.push(value.clone());
            rendered_args.push(format!("{} {}", expected.as_ir(), value.value));
        }
        for expected in signature.params.iter().skip(prefix_len + arg_values.len()) {
            rendered_args.push(format!(
                "{} {}",
                expected.as_ir(),
                expected.default_value()
            ));
        }
        if signature.return_type == LlType::Void {
            self.body.push_str(&format!(
                "  call void @{}({})\n",
                sanitize(&resolved_name),
                rendered_args.join(", ")
            ));
            self.emit_exception_check();
            for (arg_index, ((arg, original_value), casted_value)) in args
                .iter()
                .zip(arg_values.iter())
                .zip(casted_arg_values.iter())
                .enumerate()
            {
                let expected = &signature.params[prefix_len + arg_index];
                if value_uses_boxed_scalar_temporary(original_value, expected) {
                    self.body.push_str(&format!(
                        "  call void @glitch_box_release(ptr {})\n",
                        casted_value.value
                    ));
                } else if should_drop_argument_after_call(arg) {
                    self.emit_temporary_drop(arg, original_value);
                }
            }
            Ok(LlValue {
                value: "0".to_string(),
                ty: LlType::I32,
            })
        } else {
            let tmp = self.tmp();
            self.body.push_str(&format!(
                "  {tmp} = call {} @{}({})\n",
                signature.return_type.as_ir(),
                sanitize(&resolved_name),
                rendered_args.join(", ")
            ));
            self.emit_exception_check();
            for (arg_index, ((arg, original_value), casted_value)) in args
                .iter()
                .zip(arg_values.iter())
                .zip(casted_arg_values.iter())
                .enumerate()
            {
                let expected = &signature.params[prefix_len + arg_index];
                if value_uses_boxed_scalar_temporary(original_value, expected) {
                    self.body.push_str(&format!(
                        "  call void @glitch_box_release(ptr {})\n",
                        casted_value.value
                    ));
                } else if should_drop_argument_after_call(arg) {
                    self.emit_temporary_drop(arg, original_value);
                }
            }
            Ok(LlValue {
                value: tmp,
                ty: signature.return_type,
            })
        }
    }

    pub(in crate::llvm) fn emit_intrinsic_equals_pair(
        &mut self,
        left_expr: &TypedExpr,
        right_expr: &TypedExpr,
    ) -> Result<LlValue, String> {
        let left_value = self.emit_typed_expr(left_expr)?;
        let right_value = self.emit_typed_expr(right_expr)?;
        let left_ptr = self.cast_value(left_value.clone(), &LlType::Ptr)?;
        let right_ptr = self.cast_value(right_value.clone(), &LlType::Ptr)?;

        let helper = if (is_string_like_type(&left_expr.ty) && is_string_like_type(&right_expr.ty))
            || (is_string_like_type(&left_expr.ty) && is_null_literal_expr(right_expr))
            || (is_string_like_type(&right_expr.ty) && is_null_literal_expr(left_expr))
        {
            "glitch_string_equals"
        } else {
            "glitch_object_equals"
        };

        let result = self.tmp();
        self.body.push_str(&format!(
            "  {result} = call i1 @{helper}(ptr {}, ptr {})\n",
            left_ptr.value, right_ptr.value
        ));

        for (expr, original, casted) in [
            (left_expr, &left_value, &left_ptr),
            (right_expr, &right_value, &right_ptr),
        ] {
            if value_uses_boxed_scalar_temporary(original, &LlType::Ptr) {
                self.body.push_str(&format!(
                    "  call void @glitch_box_release(ptr {})\n",
                    casted.value
                ));
            } else if should_drop_argument_after_call(expr) {
                self.emit_temporary_drop(expr, original);
            }
        }

        Ok(LlValue {
            value: result,
            ty: LlType::I1,
        })
    }

    fn emit_task_exception_null_compare(
        &mut self,
        left: &TypedExpr,
        op: BinaryOp,
        right: &TypedExpr,
    ) -> Result<Option<LlValue>, String> {
        fn is_null_expr(expr: &TypedExpr) -> bool {
            matches!(expr.kind, TypedExprKind::Null)
                || matches!(expr.ty, IrType::Unknown(ref name) if name == "null")
        }

        fn task_exception_target(expr: &TypedExpr) -> Option<&TypedExpr> {
            match &expr.kind {
                TypedExprKind::Field { target, name }
                    if name == "Exception" && matches!(target.ty, IrType::Task(_)) =>
                {
                    Some(target.as_ref())
                }
                _ => None,
            }
        }

        let (target_expr, invert) = if is_null_expr(right) {
            (task_exception_target(left), matches!(op, BinaryOp::Eq))
        } else if is_null_expr(left) {
            (task_exception_target(right), matches!(op, BinaryOp::Eq))
        } else {
            return Ok(None);
        };

        let Some(target_expr) = target_expr else {
            return Ok(None);
        };

        let task_value = self.emit_typed_expr(target_expr)?;
        let faulted = self.tmp();
        self.body.push_str(&format!(
            "  {faulted} = call i1 @glitch_task_is_faulted(ptr {})\n",
            task_value.value
        ));
        self.emit_temporary_drop(target_expr, &task_value);

        let value = if invert {
            self.emit_not_value(LlValue {
                value: faulted,
                ty: LlType::I1,
            })?
            .value
        } else {
            faulted
        };

        Ok(Some(LlValue {
            value,
            ty: LlType::I1,
        }))
    }

}

fn is_configuration_manager_type(ty: &IrType) -> bool {
    matches!(
        ty,
        IrType::Class(name) | IrType::Unknown(name) | IrType::Interface(name)
            if base_type_name(name) == "ConfigurationManager"
    )
}
