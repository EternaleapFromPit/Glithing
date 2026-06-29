use super::*;

impl LlvmEmitter {
    pub(in crate::llvm) fn emit_typed_try(
        &mut self,
        try_body: &[TypedStmt],
        catch_name: Option<&str>,
        catch_body: &[TypedStmt],
        finally_body: &[TypedStmt],
    ) -> Result<(), String> {
        let outer_handler = self.exception_handler.clone();
        let has_catch = catch_name.is_some() || !catch_body.is_empty();
        let has_finally = !finally_body.is_empty();
        let catch_label = self.next_label("try_catch");
        let finally_label = self.next_label("try_finally");
        let end_label = self.next_label("try_end");
        let propagate_target = outer_handler
            .clone()
            .unwrap_or_else(|| self.current_unwind_label.clone());

        self.exception_handler = Some(if has_catch {
            catch_label.clone()
        } else if has_finally {
            finally_label.clone()
        } else {
            propagate_target.clone()
        });
        self.emit_typed_stmts(try_body)?;
        if !self.terminated {
            let target = if has_finally {
                &finally_label
            } else {
                &end_label
            };
            self.body.push_str(&format!("  br label %{target}\n"));
        }

        if has_catch {
            self.body.push_str(&format!("{catch_label}:\n"));
            self.terminated = false;
            let exception = self.tmp();
            self.body.push_str(&format!(
                "  {exception} = load ptr, ptr @glitch_exception_pending\n  store ptr null, ptr @glitch_exception_pending\n"
            ));
            if let Some(name) = catch_name {
                if let Some(var) = self.vars.get(name) {
                    self.body
                        .push_str(&format!("  store ptr {exception}, ptr {}\n", var.ptr));
                } else {
                    let ptr = self.tmp();
                    self.body.push_str(&format!(
                        "  {ptr} = alloca ptr\n  store ptr {exception}, ptr {ptr}\n"
                    ));
                    self.vars.insert(
                        name.to_string(),
                        LlVar {
                            ptr,
                            ty: LlType::Ptr,
                            ir_ty: IrType::Exception,
                            drop_kind: DropKind::DropException,
                        },
                    );
                    self.drop_order.push(name.to_string());
                }
            } else {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {exception})\n"
                ));
            }
            self.exception_handler = Some(if has_finally {
                finally_label.clone()
            } else {
                propagate_target.clone()
            });
            self.emit_typed_stmts(catch_body)?;
            if let Some(name) = catch_name {
                if let Some(var) = self.vars.remove(name) {
                    if let Some(pos) = self.drop_order.iter().position(|x| x == name) {
                        self.drop_order.remove(pos);
                    }
                    if !self.terminated {
                        self.emit_var_drop(&var);
                    }
                }
            }
            if !self.terminated {
                let target = if has_finally {
                    &finally_label
                } else {
                    &end_label
                };
                self.body.push_str(&format!("  br label %{target}\n"));
            }
        }

        if has_finally {
            self.body.push_str(&format!("{finally_label}:\n"));
            self.terminated = false;
            self.exception_handler = outer_handler.clone();
            self.emit_typed_stmts(finally_body)?;
            if !self.terminated {
                let pending = self.tmp();
                let has_exception = self.tmp();
                self.body.push_str(&format!(
                    "  {pending} = load ptr, ptr @glitch_exception_pending\n  {has_exception} = icmp ne ptr {pending}, null\n  br i1 {has_exception}, label %{propagate_target}, label %{end_label}\n"
                ));
            }
        }

        self.body.push_str(&format!("{end_label}:\n"));
        self.terminated = false;
        self.exception_handler = outer_handler;
        Ok(())
    }

    pub(in crate::llvm) fn emit_typed_for(
        &mut self,
        init: Option<&TypedStmt>,
        condition: Option<&TypedExpr>,
        increment: Option<&TypedStmt>,
        body: &[TypedStmt],
    ) -> Result<(), String> {
        if let Some(init) = init {
            self.emit_typed_stmt(init)?;
        }
        let condition_label = self.next_label("for_condition");
        let body_label = self.next_label("for_body");
        let increment_label = self.next_label("for_increment");
        let end_label = self.next_label("for_end");
        self.body.push_str(&format!(
            "  br label %{condition_label}\n{condition_label}:\n"
        ));
        if let Some(condition) = condition {
            let condition = self.emit_typed_expr(condition)?;
            let condition = self.to_i1(condition)?;
            self.body.push_str(&format!(
                "  br i1 {}, label %{body_label}, label %{end_label}\n",
                condition.value
            ));
        } else {
            self.body.push_str(&format!("  br label %{body_label}\n"));
        }
        self.body.push_str(&format!("{body_label}:\n"));
        self.terminated = false;
        self.loop_targets
            .push((increment_label.clone(), end_label.clone()));
        self.emit_typed_stmts(body)?;
        self.loop_targets.pop();
        if !self.terminated {
            self.body
                .push_str(&format!("  br label %{increment_label}\n"));
        }
        self.body.push_str(&format!("{increment_label}:\n"));
        self.terminated = false;
        if let Some(increment) = increment {
            self.emit_typed_stmt(increment)?;
        }
        if !self.terminated {
            self.body
                .push_str(&format!("  br label %{condition_label}\n"));
        }
        self.body.push_str(&format!("{end_label}:\n"));
        self.terminated = false;
        Ok(())
    }

    pub(in crate::llvm) fn emit_typed_switch(
        &mut self,
        expr: &TypedExpr,
        cases: &[TypedSwitchCase],
        default: &[TypedStmt],
    ) -> Result<(), String> {
        let value = self.emit_typed_expr(expr)?;
        let end_label = self.next_label("switch_end");
        let default_label = self.next_label("switch_default");
        let case_labels = cases
            .iter()
            .map(|_| self.next_label("switch_case"))
            .collect::<Vec<_>>();
        let compare_labels = cases
            .iter()
            .map(|_| self.next_label("switch_compare"))
            .collect::<Vec<_>>();

        if let Some(first) = compare_labels.first() {
            self.body.push_str(&format!("  br label %{first}\n"));
        } else {
            self.body
                .push_str(&format!("  br label %{default_label}\n"));
        }

        for (index, case) in cases.iter().enumerate() {
            self.body.push_str(&format!("{}:\n", compare_labels[index]));
            let next = compare_labels.get(index + 1).unwrap_or(&default_label);
            if matches!(case.value.kind, TypedExprKind::IsPattern { .. }) {
                let case_value = self.emit_typed_expr(&case.value)?;
                let case_value = self.to_i1(case_value)?;
                self.body.push_str(&format!(
                    "  br i1 {}, label %{}, label %{next}\n",
                    case_value.value, case_labels[index]
                ));
            } else {
                let case_value = self.emit_typed_expr(&case.value)?;
                let case_value = self.cast_value(case_value, &value.ty)?;
                let equal = self.tmp();
                self.emit_equality(&expr.ty, &value.value, &case_value.value, &equal);
                self.body.push_str(&format!(
                    "  br i1 {equal}, label %{}, label %{next}\n",
                    case_labels[index]
                ));
            }
        }

        let inherited_continue = self
            .loop_targets
            .last()
            .map(|(continue_target, _)| continue_target.clone())
            .unwrap_or_default();
        for (index, case) in cases.iter().enumerate() {
            self.body.push_str(&format!("{}:\n", case_labels[index]));
            self.terminated = false;
            self.loop_targets
                .push((inherited_continue.clone(), end_label.clone()));
            self.emit_typed_stmts(&case.body)?;
            self.loop_targets.pop();
            if !self.terminated {
                self.body.push_str(&format!("  br label %{end_label}\n"));
            }
        }

        self.body.push_str(&format!("{default_label}:\n"));
        self.terminated = false;
        self.loop_targets
            .push((inherited_continue, end_label.clone()));
        self.emit_typed_stmts(default)?;
        self.loop_targets.pop();
        if !self.terminated {
            self.body.push_str(&format!("  br label %{end_label}\n"));
        }
        self.body.push_str(&format!("{end_label}:\n"));
        self.terminated = false;
        Ok(())
    }

    pub(in crate::llvm) fn emit_typed_foreach(
        &mut self,
        item: &TypedBinding,
        collection: &TypedExpr,
        body: &[TypedStmt],
    ) -> Result<(), String> {
        if let IrType::Dictionary(key, value) = &collection.ty {
            return self.emit_dictionary_foreach(item, collection, body, key, value);
        }
        let element_type = match &collection.ty {
            IrType::Array(element) | IrType::List(element) | IrType::Enumerable(element) => {
                element.as_ref()
            }
            IrType::Unknown(_) => {
                self.emit_typed_expr(collection)?;
                return Ok(());
            }
            IrType::Class(_) | IrType::Interface(_) => {
                self.emit_typed_expr(collection)?;
                return Ok(());
            }
            other => {
                return Err(format!(
                    "LLVM TIR backend: foreach is unsupported for {other:?}"
                ));
            }
        };
        let collection_value = self.emit_typed_expr(collection)?;
        let layout = if matches!(collection.ty, IrType::Array(_)) {
            "%glitch.array"
        } else {
            "%glitch.list"
        };
        let data_index = if matches!(collection.ty, IrType::Array(_)) {
            1
        } else {
            2
        };
        let len_ptr = self.tmp();
        let data_ptr = self.tmp();
        let len = self.tmp();
        let data = self.tmp();
        let index_ptr = self.tmp();
        let item_ptr = self.tmp();
        let condition_label = self.next_label("foreach_condition");
        let body_label = self.next_label("foreach_body");
        let advance_label = self.next_label("foreach_advance");
        let end_label = self.next_label("foreach_end");
        let element_llvm_type = llvm_ir_type(element_type);
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds {layout}, ptr {}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds {layout}, ptr {}, i32 0, i32 {data_index}\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n  {index_ptr} = alloca i64\n  {item_ptr} = alloca {}\n  store i64 0, ptr {index_ptr}\n  br label %{condition_label}\n{condition_label}:\n",
            collection_value.value,
            collection_value.value,
            element_llvm_type.as_ir()
        ));
        let index = self.tmp();
        let in_range = self.tmp();
        self.body.push_str(&format!(
            "  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{body_label}, label %{end_label}\n{body_label}:\n"
        ));
        let slot = self.tmp();
        let value = self.tmp();
        self.body.push_str(&format!(
            "  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  {value} = load {}, ptr {slot}\n  store {} {value}, ptr {item_ptr}\n",
            element_llvm_type.as_ir(),
            element_llvm_type.as_ir(),
            element_llvm_type.as_ir()
        ));

        let previous = self.vars.insert(
            item.name.clone(),
            LlVar {
                ptr: item_ptr,
                ty: element_llvm_type,
                ir_ty: element_type.clone(),
                drop_kind: DropKind::BorrowOnly,
            },
        );
        self.loop_targets
            .push((advance_label.clone(), end_label.clone()));
        self.terminated = false;
        self.emit_typed_stmts(body)?;
        self.loop_targets.pop();
        if !self.terminated {
            self.body
                .push_str(&format!("  br label %{advance_label}\n"));
        }
        self.body.push_str(&format!("{advance_label}:\n"));
        let current_index = self.tmp();
        let next_index = self.tmp();
        self.body.push_str(&format!(
            "  {current_index} = load i64, ptr {index_ptr}\n  {next_index} = add i64 {current_index}, 1\n  store i64 {next_index}, ptr {index_ptr}\n  br label %{condition_label}\n{end_label}:\n"
        ));
        self.emit_temporary_drop(collection, &collection_value);
        self.terminated = false;
        if let Some(previous) = previous {
            self.vars.insert(item.name.clone(), previous);
        } else {
            self.vars.remove(&item.name);
        }
        Ok(())
    }

    pub(in crate::llvm) fn emit_exception_check(&mut self) {
        let pending = self.tmp();
        let has_exception = self.tmp();
        let continue_label = self.next_label("call_continue");
        let target = self
            .exception_handler
            .clone()
            .unwrap_or_else(|| self.current_unwind_label.clone());
        self.body.push_str(&format!(
            "  {pending} = load ptr, ptr @glitch_exception_pending\n  {has_exception} = icmp ne ptr {pending}, null\n  br i1 {has_exception}, label %{target}, label %{continue_label}\n{continue_label}:\n"
        ));
        self.terminated = false;
    }

    pub(in crate::llvm) fn emit_exception_branch(&mut self) {
        let target = self
            .exception_handler
            .clone()
            .unwrap_or_else(|| self.current_unwind_label.clone());
        self.body.push_str(&format!("  br label %{target}\n"));
        self.terminated = true;
    }

    pub(in crate::llvm) fn emit_throw_value(&mut self, expr: &TypedExpr) -> Result<LlValue, String> {
        let exception = self.emit_typed_expr(expr)?;
        let exception = self.cast_value(exception, &LlType::Ptr)?;
        self.body.push_str(&format!(
            "  store ptr {}, ptr @glitch_exception_pending\n",
            exception.value
        ));
        self.emit_exception_branch();
        self.default_typed_value(&expr.ty)
    }

    pub(in crate::llvm) fn emit_coalesce_value(
        &mut self,
        left: &TypedExpr,
        right: &TypedExpr,
        result_ty: &IrType,
    ) -> Result<LlValue, String> {
        let left_value = self.emit_typed_expr(left)?;
        let result_ll_ty = llvm_ir_type(result_ty);
        let result_ptr = self.tmp();
        let left_label = self.next_label("coalesce_left");
        let right_label = self.next_label("coalesce_right");
        let end_label = self.next_label("coalesce_end");
        let is_null = match &left_value.ty {
            LlType::Ptr => {
                let check = self.tmp();
                self.body.push_str(&format!(
                    "  {check} = icmp eq ptr {}, null\n",
                    left_value.value
                ));
                check
            }
            ty if ty.is_integer() => {
                let check = self.tmp();
                self.body.push_str(&format!(
                    "  {check} = icmp eq {} {}, 0\n",
                    ty.as_ir(),
                    left_value.value
                ));
                check
            }
            LlType::Double => {
                let check = self.tmp();
                self.body.push_str(&format!(
                    "  {check} = fcmp oeq double {}, 0.0\n",
                    left_value.value
                ));
                check
            }
            _ => {
                let check = self.tmp();
                self.body
                    .push_str(&format!("  {check} = icmp eq ptr null, null\n"));
                check
            }
        };
        self.body.push_str(&format!(
            "  {result_ptr} = alloca {}\n  br i1 {}, label %{right_label}, label %{left_label}\n{left_label}:\n",
            result_ll_ty.as_ir(),
            is_null
        ));
        let left_cast = self.cast_value(left_value, &result_ll_ty)?;
        self.body.push_str(&format!(
            "  store {} {}, ptr {result_ptr}\n  br label %{end_label}\n{right_label}:\n",
            result_ll_ty.as_ir(),
            left_cast.value
        ));
        let right_value = self.emit_typed_expr(right)?;
        if !self.terminated {
            let right_cast = self.cast_value(right_value, &result_ll_ty)?;
            self.body.push_str(&format!(
                "  store {} {}, ptr {result_ptr}\n  br label %{end_label}\n",
                result_ll_ty.as_ir(),
                right_cast.value
            ));
        }
        self.body.push_str(&format!("{end_label}:\n"));
        self.terminated = false;
        let result = self.tmp();
        self.body.push_str(&format!(
            "  {result} = load {}, ptr {result_ptr}\n",
            result_ll_ty.as_ir()
        ));
        Ok(LlValue {
            value: result,
            ty: result_ll_ty,
        })
    }

}
