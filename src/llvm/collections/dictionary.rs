use super::*;

impl LlvmEmitter {
    pub(in crate::llvm) fn resolve_dictionary_enumerator_type(
        &self,
        key_ty: &IrType,
        value_ty: &IrType,
    ) -> Option<LlObjectType> {
        let mut candidates = self
            .object_types
            .values()
            .filter(|object| {
                base_type_name(&object.name) == "DictionaryEnumerator"
                    && object
                        .fields
                        .get("Keys")
                        .is_some_and(|field| field.ty == IrType::Array(Box::new(key_ty.clone())))
                    && object
                        .fields
                        .get("Values")
                        .is_some_and(|field| field.ty == IrType::Array(Box::new(value_ty.clone())))
            })
            .cloned()
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| left.name.cmp(&right.name));
        candidates.dedup_by(|left, right| left.name == right.name);
        if candidates.len() == 1 {
            return candidates.pop();
        }
        let mut fallback = self
            .object_types
            .values()
            .filter(|object| base_type_name(&object.name) == "DictionaryEnumerator")
            .cloned()
            .collect::<Vec<_>>();
        fallback.sort_by(|left, right| left.name.cmp(&right.name));
        fallback.dedup_by(|left, right| left.name == right.name);
        (fallback.len() == 1).then(|| fallback.remove(0))
    }

    pub(in crate::llvm) fn resolve_key_value_pair_type(
        &self,
        key_ty: &IrType,
        value_ty: &IrType,
    ) -> Option<LlObjectType> {
        let mut candidates = self
            .object_types
            .values()
            .filter(|object| {
                base_type_name(&object.name) == "KeyValuePair"
                    && object
                        .fields
                        .get("Key")
                        .is_some_and(|field| field.ty == *key_ty)
                    && object
                        .fields
                        .get("Value")
                        .is_some_and(|field| field.ty == *value_ty)
            })
            .cloned()
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| left.name.cmp(&right.name));
        candidates.dedup_by(|left, right| left.name == right.name);
        if candidates.len() == 1 {
            return candidates.pop();
        }
        let mut fallback = self
            .object_types
            .values()
            .filter(|object| base_type_name(&object.name) == "KeyValuePair")
            .cloned()
            .collect::<Vec<_>>();
        fallback.sort_by(|left, right| left.name.cmp(&right.name));
        fallback.dedup_by(|left, right| left.name == right.name);
        (fallback.len() == 1).then(|| fallback.remove(0))
    }

    pub(in crate::llvm) fn emit_dictionary_snapshot_array(
        &mut self,
        dict: &str,
        field_index: i32,
        element_ty: &IrType,
        temp_name: &str,
    ) -> String {
        let len_ptr = self.tmp();
        let len = self.tmp();
        let data_ptr = self.tmp();
        let data = self.tmp();
        let array = self.tmp();
        let array_data = self.tmp();
        let array_len_ptr = self.tmp();
        let array_data_ptr = self.tmp();
        let index_ptr = self.tmp();
        let loop_label = self.next_label(&format!("{temp_name}_loop"));
        let copy_label = self.next_label(&format!("{temp_name}_copy"));
        let done_label = self.next_label(&format!("{temp_name}_done"));
        let index = self.tmp();
        let in_range = self.tmp();
        let slot = self.tmp();
        let item = self.tmp();
        let array_slot = self.tmp();
        let next = self.tmp();
        let element_ll_ty = llvm_ir_type(element_ty);
        let element_size = self.emit_type_size(element_ty);
        let synthetic = TypedExpr {
            kind: TypedExprKind::Var(format!("<{temp_name}>")),
            ty: element_ty.clone(),
            ownership: Ownership::Owned,
            drop_kind: DropKind::None,
        };
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 0\n  {len} = load i64, ptr {len_ptr}\n  {data_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 {field_index}\n  {data} = load ptr, ptr {data_ptr}\n  {array} = call ptr @glitch_calloc(i64 1, i64 16)\n  {array_data} = call ptr @glitch_calloc(i64 {len}, i64 {element_size})\n  {array_len_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 0\n  store i64 {len}, ptr {array_len_ptr}\n  {array_data_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 1\n  store ptr {array_data}, ptr {array_data_ptr}\n  {index_ptr} = alloca i64\n  store i64 0, ptr {index_ptr}\n  br label %{loop_label}\n{loop_label}:\n  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{copy_label}, label %{done_label}\n{copy_label}:\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  {item} = load {}, ptr {slot}\n",
            element_ll_ty.as_ir(),
            element_ll_ty.as_ir()
        ));
        self.retain_for_store(element_ty, &synthetic, &item);
        self.body.push_str(&format!(
            "  {array_slot} = getelementptr inbounds {}, ptr {array_data}, i64 {index}\n  store {} {item}, ptr {array_slot}\n  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{done_label}:\n",
            element_ll_ty.as_ir(),
            element_ll_ty.as_ir()
        ));
        array
    }

    pub(in crate::llvm) fn emit_dictionary_enumerator(
        &mut self,
        collection_value: &str,
        key_ty: &IrType,
        value_ty: &IrType,
    ) -> Result<LlValue, String> {
        let object = self
            .resolve_dictionary_enumerator_type(key_ty, value_ty)
            .ok_or_else(|| {
                format!(
                    "LLVM TIR backend: dictionary enumerator layout for {:?}, {:?} is missing",
                    key_ty, value_ty
                )
            })?;
        let llvm_name = llvm_object_name(&object.name);
        let size_ptr = self.tmp();
        let size = self.tmp();
        let enumerator = self.tmp();
        self.body.push_str(&format!(
            "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {enumerator} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
        ));
        if matches!(object.kind, TypeKind::Class) {
            let rc_ptr = self.tmp();
            let drop_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {enumerator}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {enumerator}, i32 0, i32 1\n  store ptr @{}, ptr {drop_ptr}\n",
                destroy_symbol(&object.name)
            ));
        }
        let keys_array = self.emit_dictionary_snapshot_array(collection_value, 2, key_ty, "dict_enum_keys");
        let values_array =
            self.emit_dictionary_snapshot_array(collection_value, 3, value_ty, "dict_enum_values");
        let len_ptr = self.tmp();
        let len = self.tmp();
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {collection_value}, i32 0, i32 0\n  {len} = load i64, ptr {len_ptr}\n"
        ));

        let keys_field = object
            .fields
            .get("Keys")
            .cloned()
            .ok_or_else(|| format!("LLVM TIR backend: '{}' is missing field 'Keys'", object.name))?;
        let values_field = object
            .fields
            .get("Values")
            .cloned()
            .ok_or_else(|| format!("LLVM TIR backend: '{}' is missing field 'Values'", object.name))?;
        let count_field = object
            .fields
            .get("Count")
            .cloned()
            .ok_or_else(|| format!("LLVM TIR backend: '{}' is missing field 'Count'", object.name))?;
        let index_field = object
            .fields
            .get("Index")
            .cloned()
            .ok_or_else(|| format!("LLVM TIR backend: '{}' is missing field 'Index'", object.name))?;
        let keys_ptr = self.tmp();
        let values_ptr = self.tmp();
        let count_ptr = self.tmp();
        let index_ptr = self.tmp();
        let count_i32 = self.tmp();
        self.body.push_str(&format!(
            "  {keys_ptr} = getelementptr inbounds %{llvm_name}, ptr {enumerator}, i32 0, i32 {}\n  store ptr {keys_array}, ptr {keys_ptr}\n  {values_ptr} = getelementptr inbounds %{llvm_name}, ptr {enumerator}, i32 0, i32 {}\n  store ptr {values_array}, ptr {values_ptr}\n  {count_ptr} = getelementptr inbounds %{llvm_name}, ptr {enumerator}, i32 0, i32 {}\n  {count_i32} = trunc i64 {len} to i32\n  store i32 {count_i32}, ptr {count_ptr}\n  {index_ptr} = getelementptr inbounds %{llvm_name}, ptr {enumerator}, i32 0, i32 {}\n  store i32 -1, ptr {index_ptr}\n",
            keys_field.index, values_field.index, count_field.index, index_field.index
        ));
        Ok(LlValue {
            value: enumerator,
            ty: LlType::Ptr,
        })
    }

    pub(in crate::llvm) fn emit_key_value_pair_object(
        &mut self,
        pair_ty: &IrType,
        key_ty: &IrType,
        key_value: &str,
        value_ty: &IrType,
        value_value: &str,
    ) -> Result<LlValue, String> {
        let object = if let Some(type_name) = object_type_name(pair_ty) {
            self.object_types
                .get(type_name)
                .cloned()
                .or_else(|| self.resolve_key_value_pair_type(key_ty, value_ty))
                .ok_or_else(|| format!("LLVM TIR backend: type '{type_name}' has no LLVM layout"))?
        } else {
            self.resolve_key_value_pair_type(key_ty, value_ty).ok_or_else(|| {
                format!(
                    "LLVM TIR backend: dictionary foreach item type {:?} has no object layout",
                    pair_ty
                )
            })?
        };
        let key_field = object
            .fields
            .get("Key")
            .cloned()
            .ok_or_else(|| format!("LLVM TIR backend: '{}' is missing field 'Key'", object.name))?;
        let value_field = object
            .fields
            .get("Value")
            .cloned()
            .ok_or_else(|| format!("LLVM TIR backend: '{}' is missing field 'Value'", object.name))?;
        let llvm_name = llvm_object_name(&object.name);
        let size_ptr = self.tmp();
        let size = self.tmp();
        let pair = self.tmp();
        self.body.push_str(&format!(
            "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {pair} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
        ));
        let key_ptr = self.tmp();
        let value_ptr = self.tmp();
        let synthetic_key = TypedExpr {
            kind: TypedExprKind::Var("<dict_key>".to_string()),
            ty: key_ty.clone(),
            ownership: Ownership::Owned,
            drop_kind: DropKind::None,
        };
        let synthetic_value = TypedExpr {
            kind: TypedExprKind::Var("<dict_value>".to_string()),
            ty: value_ty.clone(),
            ownership: Ownership::Owned,
            drop_kind: DropKind::None,
        };
        self.retain_for_store(key_ty, &synthetic_key, key_value);
        self.retain_for_store(value_ty, &synthetic_value, value_value);
        self.body.push_str(&format!(
            "  {key_ptr} = getelementptr inbounds %{llvm_name}, ptr {pair}, i32 0, i32 {}\n  store {} {key_value}, ptr {key_ptr}\n  {value_ptr} = getelementptr inbounds %{llvm_name}, ptr {pair}, i32 0, i32 {}\n  store {} {value_value}, ptr {value_ptr}\n",
            key_field.index,
            llvm_ir_type(key_ty).as_ir(),
            value_field.index,
            llvm_ir_type(value_ty).as_ir()
        ));
        Ok(LlValue {
            value: pair,
            ty: LlType::Ptr,
        })
    }

    pub(in crate::llvm) fn emit_dictionary_foreach(
        &mut self,
        item: &TypedBinding,
        collection: &TypedExpr,
        body: &[TypedStmt],
        key_ty: &IrType,
        value_ty: &IrType,
    ) -> Result<(), String> {
        let collection_value = self.emit_typed_expr(collection)?;
        let len_ptr = self.tmp();
        let keys_ptr = self.tmp();
        let values_ptr = self.tmp();
        let len = self.tmp();
        let keys = self.tmp();
        let values = self.tmp();
        let index_ptr = self.tmp();
        let item_ptr = self.tmp();
        let condition_label = self.next_label("dict_foreach_condition");
        let body_label = self.next_label("dict_foreach_body");
        let advance_label = self.next_label("dict_foreach_advance");
        let end_label = self.next_label("dict_foreach_end");
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {}, i32 0, i32 0\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {}, i32 0, i32 2\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {}, i32 0, i32 3\n  {len} = load i64, ptr {len_ptr}\n  {keys} = load ptr, ptr {keys_ptr}\n  {values} = load ptr, ptr {values_ptr}\n  {index_ptr} = alloca i64\n  {item_ptr} = alloca ptr\n  store i64 0, ptr {index_ptr}\n  store ptr null, ptr {item_ptr}\n  br label %{condition_label}\n",
            collection_value.value, collection_value.value, collection_value.value
        ));
        self.body.push_str(&format!("{condition_label}:\n"));
        let index = self.tmp();
        let in_range = self.tmp();
        self.body.push_str(&format!(
            "  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{body_label}, label %{end_label}\n{body_label}:\n"
        ));
        let old_item = self.tmp();
        self.body.push_str(&format!(
            "  {old_item} = load ptr, ptr {item_ptr}\n"
        ));
        if let Some(type_name) = object_type_name(&item.ty) {
            if self.object_types.contains_key(type_name) {
                self.emit_drop(type_name, &old_item);
            }
        }
        let key_slot = self.tmp();
        let key_value = self.tmp();
        let value_slot = self.tmp();
        let value_value = self.tmp();
        self.body.push_str(&format!(
            "  {key_slot} = getelementptr inbounds {}, ptr {keys}, i64 {index}\n  {key_value} = load {}, ptr {key_slot}\n  {value_slot} = getelementptr inbounds {}, ptr {values}, i64 {index}\n  {value_value} = load {}, ptr {value_slot}\n",
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir()
        ));
        let pair = self.emit_key_value_pair_object(&item.ty, key_ty, &key_value, value_ty, &value_value)?;
        self.body
            .push_str(&format!("  store ptr {}, ptr {item_ptr}\n", pair.value));

        let previous = self.vars.insert(
            item.name.clone(),
            LlVar {
                ptr: item_ptr.clone(),
                ty: LlType::Ptr,
                ir_ty: item.ty.clone(),
                drop_kind: item.drop_kind(),
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
        let final_item = self.tmp();
        self.body
            .push_str(&format!("  {final_item} = load ptr, ptr {item_ptr}\n"));
        if let Some(type_name) = object_type_name(&item.ty) {
            if self.object_types.contains_key(type_name) {
                self.emit_drop(type_name, &final_item);
            }
        }
        self.terminated = false;
        if let Some(previous) = previous {
            self.vars.insert(item.name.clone(), previous);
        } else {
            self.vars.remove(&item.name);
        }
        Ok(())
    }

}
