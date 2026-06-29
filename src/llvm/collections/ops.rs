use super::*;

impl LlvmEmitter {
    pub(in crate::llvm) fn resolve_dictionary_enumerator_layout_from_target(
        &self,
        target: &TypedExpr,
    ) -> Option<LlObjectType> {
        match &target.ty {
            IrType::Class(name) | IrType::Struct(name) if base_type_name(name) == "DictionaryEnumerator" => {
                self.object_types
                    .values()
                    .find(|object| object.name == *name || base_type_name(&object.name) == "DictionaryEnumerator")
                    .cloned()
            }
            IrType::Interface(interface_name) if base_type_name(interface_name) == "IEnumerator" => {
                let mut exact = self
                    .object_types
                    .values()
                    .filter(|object| {
                        base_type_name(&object.name) == "DictionaryEnumerator"
                            && object.bases.iter().any(|base| base == interface_name)
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                exact.sort_by(|left, right| left.name.cmp(&right.name));
                exact.dedup_by(|left, right| left.name == right.name);
                if exact.len() == 1 {
                    return exact.pop();
                }
                if let Some(implementation) = self.resolve_interface_implementation(interface_name) {
                    if base_type_name(&implementation) == "DictionaryEnumerator" {
                        return self
                            .object_types
                            .values()
                            .find(|object| object.name == implementation || base_type_name(&object.name) == "DictionaryEnumerator")
                            .cloned();
                    }
                }
                let mut candidates = self
                    .object_types
                    .values()
                    .filter(|object| base_type_name(&object.name) == "DictionaryEnumerator")
                    .cloned()
                    .collect::<Vec<_>>();
                candidates.sort_by(|left, right| left.name.cmp(&right.name));
                candidates.dedup_by(|left, right| left.name == right.name);
                (candidates.len() == 1).then(|| candidates.remove(0))
            }
            _ => None,
        }
    }

    pub(in crate::llvm) fn emit_dictionary_enumerator_method(
        &mut self,
        target: &TypedExpr,
        name: &str,
        result_ty: &IrType,
    ) -> Result<Option<LlValue>, String> {
        let Some(object) = self.resolve_dictionary_enumerator_layout_from_target(target) else {
            return Ok(None);
        };
        let receiver = self.emit_typed_expr(target)?;
        let llvm_name = llvm_object_name(&object.name);
        let result = match name {
            "MoveNext" => {
                let index_field = object.fields.get("Index").ok_or_else(|| {
                    format!("LLVM TIR backend: '{}' is missing field 'Index'", object.name)
                })?;
                let count_field = object.fields.get("Count").ok_or_else(|| {
                    format!("LLVM TIR backend: '{}' is missing field 'Count'", object.name)
                })?;
                let index_ptr = self.tmp();
                let current = self.tmp();
                let next = self.tmp();
                let count_ptr = self.tmp();
                let count = self.tmp();
                let in_range = self.tmp();
                self.body.push_str(&format!(
                    "  {index_ptr} = getelementptr inbounds %{llvm_name}, ptr {}, i32 0, i32 {}\n  {current} = load i32, ptr {index_ptr}\n  {next} = add i32 {current}, 1\n  store i32 {next}, ptr {index_ptr}\n  {count_ptr} = getelementptr inbounds %{llvm_name}, ptr {}, i32 0, i32 {}\n  {count} = load i32, ptr {count_ptr}\n  {in_range} = icmp slt i32 {next}, {count}\n",
                    receiver.value,
                    index_field.index,
                    receiver.value,
                    count_field.index
                ));
                LlValue {
                    value: in_range,
                    ty: LlType::I1,
                }
            }
            "Reset" => {
                let index_field = object.fields.get("Index").ok_or_else(|| {
                    format!("LLVM TIR backend: '{}' is missing field 'Index'", object.name)
                })?;
                let index_ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {index_ptr} = getelementptr inbounds %{llvm_name}, ptr {}, i32 0, i32 {}\n  store i32 -1, ptr {index_ptr}\n",
                    receiver.value,
                    index_field.index
                ));
                void_value()
            }
            "Dispose" => void_value(),
            "get_Current" => {
                let keys_field = object.fields.get("Keys").ok_or_else(|| {
                    format!("LLVM TIR backend: '{}' is missing field 'Keys'", object.name)
                })?;
                let values_field = object.fields.get("Values").ok_or_else(|| {
                    format!("LLVM TIR backend: '{}' is missing field 'Values'", object.name)
                })?;
                let index_field = object.fields.get("Index").ok_or_else(|| {
                    format!("LLVM TIR backend: '{}' is missing field 'Index'", object.name)
                })?;
                let pair_object = if let Some(pair_name) = object_type_name(result_ty) {
                    self.object_types
                        .get(pair_name)
                        .cloned()
                        .ok_or_else(|| {
                            format!("LLVM TIR backend: type '{pair_name}' has no LLVM layout")
                        })?
                } else {
                    let key_ty = match &keys_field.ty {
                        IrType::Array(inner) => inner.as_ref().clone(),
                        other => {
                            self.emit_temporary_drop(target, &receiver);
                            return Err(format!(
                                "LLVM TIR backend: dictionary enumerator Keys field is not an array: {other:?}"
                            ));
                        }
                    };
                    let value_ty = match &values_field.ty {
                        IrType::Array(inner) => inner.as_ref().clone(),
                        other => {
                            self.emit_temporary_drop(target, &receiver);
                            return Err(format!(
                                "LLVM TIR backend: dictionary enumerator Values field is not an array: {other:?}"
                            ));
                        }
                    };
                    self.resolve_key_value_pair_type(&key_ty, &value_ty).ok_or_else(|| {
                        "LLVM TIR backend: dictionary enumerator Current requires a concrete KeyValuePair result".to_string()
                    })?
                };
                let key_ty = pair_object
                    .fields
                    .get("Key")
                    .map(|field| field.ty.clone())
                    .ok_or_else(|| format!("LLVM TIR backend: '{}' is missing field 'Key'", pair_object.name))?;
                let value_ty = pair_object
                    .fields
                    .get("Value")
                    .map(|field| field.ty.clone())
                    .ok_or_else(|| format!("LLVM TIR backend: '{}' is missing field 'Value'", pair_object.name))?;
                let keys_ptr_ptr = self.tmp();
                let keys_array = self.tmp();
                let values_ptr_ptr = self.tmp();
                let values_array = self.tmp();
                let index_ptr = self.tmp();
                let index_i32 = self.tmp();
                let index = self.tmp();
                self.body.push_str(&format!(
                    "  {keys_ptr_ptr} = getelementptr inbounds %{llvm_name}, ptr {}, i32 0, i32 {}\n  {keys_array} = load ptr, ptr {keys_ptr_ptr}\n  {values_ptr_ptr} = getelementptr inbounds %{llvm_name}, ptr {}, i32 0, i32 {}\n  {values_array} = load ptr, ptr {values_ptr_ptr}\n  {index_ptr} = getelementptr inbounds %{llvm_name}, ptr {}, i32 0, i32 {}\n  {index_i32} = load i32, ptr {index_ptr}\n  {index} = sext i32 {index_i32} to i64\n",
                    receiver.value,
                    keys_field.index,
                    receiver.value,
                    values_field.index,
                    receiver.value,
                    index_field.index
                ));
                let keys_data_ptr = self.tmp();
                let keys_data = self.tmp();
                let values_data_ptr = self.tmp();
                let values_data = self.tmp();
                let key_slot = self.tmp();
                let key_value = self.tmp();
                let value_slot = self.tmp();
                let value_value = self.tmp();
                self.body.push_str(&format!(
                    "  {keys_data_ptr} = getelementptr inbounds %glitch.array, ptr {keys_array}, i32 0, i32 1\n  {keys_data} = load ptr, ptr {keys_data_ptr}\n  {values_data_ptr} = getelementptr inbounds %glitch.array, ptr {values_array}, i32 0, i32 1\n  {values_data} = load ptr, ptr {values_data_ptr}\n  {key_slot} = getelementptr inbounds {}, ptr {keys_data}, i64 {index}\n  {key_value} = load {}, ptr {key_slot}\n  {value_slot} = getelementptr inbounds {}, ptr {values_data}, i64 {index}\n  {value_value} = load {}, ptr {value_slot}\n",
                    llvm_ir_type(&key_ty).as_ir(),
                    llvm_ir_type(&key_ty).as_ir(),
                    llvm_ir_type(&value_ty).as_ir(),
                    llvm_ir_type(&value_ty).as_ir()
                ));
                let concrete_pair_ty = match result_ty {
                    IrType::Class(_) | IrType::Struct(_) => result_ty.clone(),
                    _ => IrType::Struct(pair_object.name.clone()),
                };
                self.emit_key_value_pair_object(
                    &concrete_pair_ty,
                    &key_ty,
                    &key_value,
                    &value_ty,
                    &value_value,
                )?
            }
            _ => {
                self.emit_temporary_drop(target, &receiver);
                return Ok(None);
            }
        };
        self.emit_temporary_drop(target, &receiver);
        Ok(Some(result))
    }

    pub(in crate::llvm) fn emit_collection_method(
        &mut self,
        target: &TypedExpr,
        name: &str,
        args: &[TypedExpr],
    ) -> Result<LlValue, String> {
        let collection = self.emit_typed_expr(target)?;
        match (&target.ty, name) {
            (IrType::List(element), "Add") => {
                let [arg] = args else {
                    return Err("LLVM TIR backend: List.Add expects one argument".to_string());
                };
                let value = self.emit_typed_expr(arg)?;
                let value = self.cast_value(value, &llvm_ir_type(element))?;
                self.emit_list_add(&collection.value, element, &value.value, arg);
                Ok(void_value())
            }
            (IrType::Dictionary(key, value_ty), "Add") => {
                let [key_arg, value_arg] = args else {
                    return Err(
                        "LLVM TIR backend: Dictionary.Add expects two arguments".to_string()
                    );
                };
                let key_value = self.emit_typed_expr(key_arg)?;
                let key_value = self.cast_value(key_value, &llvm_ir_type(key))?;
                let value = self.emit_typed_expr(value_arg)?;
                let value = self.cast_value(value, &llvm_ir_type(value_ty))?;
                self.emit_dict_add(
                    &collection.value,
                    key,
                    &key_value.value,
                    key_arg,
                    value_ty,
                    &value.value,
                    value_arg,
                );
                Ok(void_value())
            }
            (IrType::Dictionary(key, value_ty), "GetEnumerator") => {
                self.emit_dictionary_enumerator(&collection.value, key, value_ty)
            }
            (IrType::List(element), "Contains") => {
                let [arg] = args else {
                    return Err("LLVM TIR backend: List.Contains expects one argument".to_string());
                };
                let needle = self.emit_typed_expr(arg)?;
                let needle = self.cast_value(needle, &llvm_ir_type(element))?;
                self.emit_list_contains(&collection.value, element, &needle.value)
            }
            (IrType::Dictionary(key, _), "ContainsKey") => {
                let [arg] = args else {
                    return Err(
                        "LLVM TIR backend: Dictionary.ContainsKey expects one argument".to_string(),
                    );
                };
                let needle = self.emit_typed_expr(arg)?;
                let needle = self.cast_value(needle, &llvm_ir_type(key))?;
                self.emit_dict_find(&collection.value, key, &needle.value)
                    .map(|(_, found)| found)
            }
            (IrType::Dictionary(key, value), "TryGetValue") => {
                let [key_arg, out_arg] = args else {
                    return Err(
                        "LLVM TIR backend: Dictionary.TryGetValue expects two arguments"
                            .to_string(),
                    );
                };
                let key_value = self.emit_typed_expr(key_arg)?;
                let key_value = self.cast_value(key_value, &llvm_ir_type(key))?;
                let out_ptr = self.emit_typed_expr(out_arg)?;
                let (found_index, found) =
                    self.emit_dict_find(&collection.value, key, &key_value.value)?;
                let load_label = self.next_label("dict_tryget_load");
                let done_label = self.next_label("dict_tryget_done");
                let value_ptr = self.tmp();
                let values = self.tmp();
                let slot = self.tmp();
                let loaded = self.tmp();
                let default_value = llvm_ir_type(value).default_value();
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    llvm_ir_type(value).as_ir(),
                    default_value,
                    out_ptr.value
                ));
                self.body.push_str(&format!(
                    "  br i1 {}, label %{load_label}, label %{done_label}\n",
                    found.value
                ));
                self.body.push_str(&format!("\n{load_label}:\n"));
                self.body.push_str(&format!(
                    "  {value_ptr} = getelementptr inbounds %glitch.dict, ptr {}, i32 0, i32 3\n",
                    collection.value
                ));
                self.body.push_str(&format!("  {values} = load ptr, ptr {value_ptr}\n"));
                self.body.push_str(&format!(
                    "  {slot} = getelementptr inbounds {}, ptr {values}, i64 {}\n",
                    llvm_ir_type(value).as_ir(),
                    found_index.value
                ));
                self.body.push_str(&format!(
                    "  {loaded} = load {}, ptr {slot}\n",
                    llvm_ir_type(value).as_ir()
                ));
                self.body.push_str(&format!(
                    "  store {} {loaded}, ptr {}\n  br label %{done_label}\n\n{done_label}:\n",
                    llvm_ir_type(value).as_ir(),
                    out_ptr.value
                ));
                Ok(found)
            }
            (IrType::Dictionary(key, value), "Remove") => {
                let [arg] = args else {
                    return Err(
                        "LLVM TIR backend: Dictionary.Remove expects one argument".to_string()
                    );
                };
                let needle = self.emit_typed_expr(arg)?;
                let needle = self.cast_value(needle, &llvm_ir_type(key))?;
                self.emit_dict_remove(&collection.value, key, value, &needle.value)
            }
            (IrType::List(element), "Clear") => {
                self.emit_collection_clear(&collection.value, element, "glitch.list", 0);
                Ok(void_value())
            }
            (IrType::List(element), "ToArray") => self.emit_list_to_array(&collection.value, element),
            (IrType::Dictionary(_, value), "Clear") => {
                self.emit_collection_clear(&collection.value, value, "glitch.dict", 0);
                Ok(void_value())
            }
            _ => Err(format!(
                "LLVM TIR backend: unsupported collection method '{name}' on {:?}",
                target.ty
            )),
        }
    }

    pub(in crate::llvm) fn emit_list_add(&mut self, list: &str, element: &IrType, value: &str, source: &TypedExpr) {
        let len_ptr = self.tmp();
        let cap_ptr = self.tmp();
        let data_ptr = self.tmp();
        let len = self.tmp();
        let cap = self.tmp();
        let data = self.tmp();
        let full = self.tmp();
        let grow = self.next_label("list_grow");
        let ready = self.next_label("list_ready");
        let grown_cap = self.tmp();
        let bytes = self.tmp();
        let grown_data = self.tmp();
        let current_data = self.tmp();
        let slot = self.tmp();
        let next_len = self.tmp();
        let element_size = self.emit_type_size(element);
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 0\n  {cap_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 1\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {cap} = load i64, ptr {cap_ptr}\n  {data} = load ptr, ptr {data_ptr}\n  {full} = icmp eq i64 {len}, {cap}\n  br i1 {full}, label %{grow}, label %{ready}\n{grow}:\n  {grown_cap} = mul i64 {cap}, 2\n  {bytes} = mul i64 {grown_cap}, {element_size}\n  {grown_data} = call ptr @glitch_realloc(ptr {data}, i64 {bytes})\n  store i64 {grown_cap}, ptr {cap_ptr}\n  store ptr {grown_data}, ptr {data_ptr}\n  br label %{ready}\n{ready}:\n  {current_data} = load ptr, ptr {data_ptr}\n  {slot} = getelementptr inbounds {}, ptr {current_data}, i64 {len}\n",
            llvm_ir_type(element).as_ir()
        ));
        if should_drop_argument_after_call(source) {
            self.retain_task_payload(
                element,
                &LlValue {
                    value: value.to_string(),
                    ty: llvm_ir_type(element),
                },
            );
        } else {
            self.retain_for_store(element, source, value);
        }
        self.body.push_str(&format!(
            "  store {} {value}, ptr {slot}\n  {next_len} = add i64 {len}, 1\n  store i64 {next_len}, ptr {len_ptr}\n",
            llvm_ir_type(element).as_ir()
        ));
        if should_drop_argument_after_call(source) {
            self.emit_temporary_drop(
                source,
                &LlValue {
                    value: value.to_string(),
                    ty: llvm_ir_type(element),
                },
            );
        }
        self.move_raw_owned_source_after_store(element, source);
    }

    pub(in crate::llvm) fn emit_dict_add(
        &mut self,
        dict: &str,
        key_ty: &IrType,
        key: &str,
        key_source: &TypedExpr,
        value_ty: &IrType,
        value: &str,
        source: &TypedExpr,
    ) {
        let len_ptr = self.tmp();
        let cap_ptr = self.tmp();
        let keys_ptr = self.tmp();
        let values_ptr = self.tmp();
        let len = self.tmp();
        let cap = self.tmp();
        let keys = self.tmp();
        let values = self.tmp();
        let full = self.tmp();
        let grow = self.next_label("dict_grow");
        let ready = self.next_label("dict_ready");
        let grown_cap = self.tmp();
        let key_bytes = self.tmp();
        let value_bytes = self.tmp();
        let grown_keys = self.tmp();
        let grown_values = self.tmp();
        let current_keys = self.tmp();
        let current_values = self.tmp();
        let key_slot = self.tmp();
        let value_slot = self.tmp();
        let next_len = self.tmp();
        let key_size = self.emit_type_size(key_ty);
        let value_size = self.emit_type_size(value_ty);
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 0\n  {cap_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 1\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 2\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 3\n  {len} = load i64, ptr {len_ptr}\n  {cap} = load i64, ptr {cap_ptr}\n  {keys} = load ptr, ptr {keys_ptr}\n  {values} = load ptr, ptr {values_ptr}\n  {full} = icmp eq i64 {len}, {cap}\n  br i1 {full}, label %{grow}, label %{ready}\n{grow}:\n  {grown_cap} = mul i64 {cap}, 2\n  {key_bytes} = mul i64 {grown_cap}, {key_size}\n  {value_bytes} = mul i64 {grown_cap}, {value_size}\n  {grown_keys} = call ptr @glitch_realloc(ptr {keys}, i64 {key_bytes})\n  {grown_values} = call ptr @glitch_realloc(ptr {values}, i64 {value_bytes})\n  store i64 {grown_cap}, ptr {cap_ptr}\n  store ptr {grown_keys}, ptr {keys_ptr}\n  store ptr {grown_values}, ptr {values_ptr}\n  br label %{ready}\n{ready}:\n  {current_keys} = load ptr, ptr {keys_ptr}\n  {current_values} = load ptr, ptr {values_ptr}\n  {key_slot} = getelementptr inbounds {}, ptr {current_keys}, i64 {len}\n  {value_slot} = getelementptr inbounds {}, ptr {current_values}, i64 {len}\n",
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir()
        ));
        if should_drop_argument_after_call(key_source) {
            self.retain_task_payload(
                key_ty,
                &LlValue {
                    value: key.to_string(),
                    ty: llvm_ir_type(key_ty),
                },
            );
        } else {
            self.retain_for_store(key_ty, key_source, key);
        }
        self.body.push_str(&format!(
            "  store {} {key}, ptr {key_slot}\n",
            llvm_ir_type(key_ty).as_ir()
        ));
        if should_drop_argument_after_call(source) {
            self.retain_task_payload(
                value_ty,
                &LlValue {
                    value: value.to_string(),
                    ty: llvm_ir_type(value_ty),
                },
            );
        } else {
            self.retain_for_store(value_ty, source, value);
        }
        self.body.push_str(&format!(
            "  store {} {value}, ptr {value_slot}\n  {next_len} = add i64 {len}, 1\n  store i64 {next_len}, ptr {len_ptr}\n",
            llvm_ir_type(value_ty).as_ir()
        ));
        if should_drop_argument_after_call(key_source) {
            self.emit_temporary_drop(
                key_source,
                &LlValue {
                    value: key.to_string(),
                    ty: llvm_ir_type(key_ty),
                },
            );
        }
        if should_drop_argument_after_call(source) {
            self.emit_temporary_drop(
                source,
                &LlValue {
                    value: value.to_string(),
                    ty: llvm_ir_type(value_ty),
                },
            );
        }
        self.move_raw_owned_source_after_store(value_ty, source);
    }

    pub(in crate::llvm) fn emit_list_contains(
        &mut self,
        list: &str,
        element: &IrType,
        needle: &str,
    ) -> Result<LlValue, String> {
        let len_ptr = self.tmp();
        let data_ptr = self.tmp();
        let len = self.tmp();
        let data = self.tmp();
        let index_ptr = self.tmp();
        let result_ptr = self.tmp();
        let loop_label = self.next_label("list_contains_loop");
        let found_label = self.next_label("list_contains_found");
        let next_label = self.next_label("list_contains_next");
        let done_label = self.next_label("list_contains_done");
        let index = self.tmp();
        let in_range = self.tmp();
        let slot = self.tmp();
        let item = self.tmp();
        let equal = self.tmp();
        let next = self.tmp();
        let result = self.tmp();
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n  {index_ptr} = alloca i64\n  {result_ptr} = alloca i1\n  store i64 0, ptr {index_ptr}\n  store i1 false, ptr {result_ptr}\n  br label %{loop_label}\n{loop_label}:\n  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{next_label}, label %{done_label}\n{next_label}:\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  {item} = load {}, ptr {slot}\n",
            llvm_ir_type(element).as_ir(),
            llvm_ir_type(element).as_ir()
        ));
        self.emit_equality(element, &item, needle, &equal);
        self.body.push_str(&format!(
            "  br i1 {equal}, label %{found_label}, label %list_contains_advance_{}\nlist_contains_advance_{}:\n  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{found_label}:\n  store i1 true, ptr {result_ptr}\n  br label %{done_label}\n{done_label}:\n  {result} = load i1, ptr {result_ptr}\n",
            self.label, self.label
        ));
        self.label += 1;
        Ok(LlValue {
            value: result,
            ty: LlType::I1,
        })
    }

    pub(in crate::llvm) fn emit_list_to_array(&mut self, list: &str, element: &IrType) -> Result<LlValue, String> {
        if matches!(
            element,
            IrType::Array(_)
                | IrType::List(_)
                | IrType::Dictionary(_, _)
                | IrType::Enumerable(_)
        ) {
            return Err(
                "LLVM TIR backend: List.ToArray is not supported for nested owned collections"
                    .to_string(),
            );
        }
        let len_ptr = self.tmp();
        let data_ptr = self.tmp();
        let len = self.tmp();
        let data = self.tmp();
        let array = self.tmp();
        let array_data = self.tmp();
        let array_len_ptr = self.tmp();
        let array_data_ptr = self.tmp();
        let index_ptr = self.tmp();
        let loop_label = self.next_label("list_to_array_loop");
        let copy_label = self.next_label("list_to_array_copy");
        let done_label = self.next_label("list_to_array_done");
        let index = self.tmp();
        let in_range = self.tmp();
        let slot = self.tmp();
        let item = self.tmp();
        let array_slot = self.tmp();
        let next = self.tmp();
        let element_ty = llvm_ir_type(element);
        let synthetic = TypedExpr {
            kind: TypedExprKind::Var("<list_to_array>".to_string()),
            ty: element.clone(),
            ownership: Ownership::Owned,
            drop_kind: DropKind::None,
        };
        let element_size = self.emit_type_size(element);
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n  {array} = call ptr @glitch_calloc(i64 1, i64 16)\n  {array_data} = call ptr @glitch_calloc(i64 {len}, i64 {element_size})\n  {array_len_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 0\n  store i64 {len}, ptr {array_len_ptr}\n  {array_data_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 1\n  store ptr {array_data}, ptr {array_data_ptr}\n  {index_ptr} = alloca i64\n  store i64 0, ptr {index_ptr}\n  br label %{loop_label}\n{loop_label}:\n  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{copy_label}, label %{done_label}\n{copy_label}:\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  {item} = load {}, ptr {slot}\n",
            element_ty.as_ir(),
            element_ty.as_ir()
        ));
        if matches!(element, IrType::Task(_)) {
            self.retain_task_payload(
                element,
                &LlValue {
                    value: item.clone(),
                    ty: element_ty.clone(),
                },
            );
        } else {
            self.retain_for_store(element, &synthetic, &item);
        }
        self.body.push_str(&format!(
            "  {array_slot} = getelementptr inbounds {}, ptr {array_data}, i64 {index}\n  store {} {item}, ptr {array_slot}\n  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{done_label}:\n",
            element_ty.as_ir(),
            element_ty.as_ir(),
        ));
        Ok(LlValue {
            value: array,
            ty: LlType::Ptr,
        })
    }

    pub(in crate::llvm) fn emit_dict_find(
        &mut self,
        dict: &str,
        key_ty: &IrType,
        needle: &str,
    ) -> Result<(LlValue, LlValue), String> {
        let len_ptr = self.tmp();
        let keys_ptr = self.tmp();
        let len = self.tmp();
        let keys = self.tmp();
        let index_ptr = self.tmp();
        let found_ptr = self.tmp();
        let loop_label = self.next_label("dict_find_loop");
        let inspect_label = self.next_label("dict_find_inspect");
        let found_label = self.next_label("dict_find_found");
        let advance_label = self.next_label("dict_find_advance");
        let done_label = self.next_label("dict_find_done");
        let index = self.tmp();
        let in_range = self.tmp();
        let slot = self.tmp();
        let item = self.tmp();
        let equal = self.tmp();
        let next = self.tmp();
        let result_index = self.tmp();
        let result_found = self.tmp();
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 0\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {keys} = load ptr, ptr {keys_ptr}\n  {index_ptr} = alloca i64\n  {found_ptr} = alloca i1\n  store i64 0, ptr {index_ptr}\n  store i1 false, ptr {found_ptr}\n  br label %{loop_label}\n{loop_label}:\n  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{inspect_label}, label %{done_label}\n{inspect_label}:\n  {slot} = getelementptr inbounds {}, ptr {keys}, i64 {index}\n  {item} = load {}, ptr {slot}\n",
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(key_ty).as_ir()
        ));
        self.emit_equality(key_ty, &item, needle, &equal);
        self.body.push_str(&format!(
            "  br i1 {equal}, label %{found_label}, label %{advance_label}\n{advance_label}:\n  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{found_label}:\n  store i1 true, ptr {found_ptr}\n  br label %{done_label}\n{done_label}:\n  {result_index} = load i64, ptr {index_ptr}\n  {result_found} = load i1, ptr {found_ptr}\n"
        ));
        Ok((
            LlValue {
                value: result_index,
                ty: LlType::I64,
            },
            LlValue {
                value: result_found,
                ty: LlType::I1,
            },
        ))
    }

    pub(in crate::llvm) fn emit_dict_remove(
        &mut self,
        dict: &str,
        key_ty: &IrType,
        value_ty: &IrType,
        needle: &str,
    ) -> Result<LlValue, String> {
        let (index, found) = self.emit_dict_find(dict, key_ty, needle)?;
        let remove_label = self.next_label("dict_remove");
        let compact_label = self.next_label("dict_remove_compact");
        let done_label = self.next_label("dict_remove_done");
        let len_ptr = self.tmp();
        let keys_ptr = self.tmp();
        let values_ptr = self.tmp();
        let len = self.tmp();
        let next_len = self.tmp();
        let needs_compact = self.tmp();
        let keys = self.tmp();
        let values = self.tmp();
        let last_key_ptr = self.tmp();
        let last_value_ptr = self.tmp();
        let last_key = self.tmp();
        let last_value = self.tmp();
        let removed_key_ptr = self.tmp();
        let removed_value_ptr = self.tmp();
        self.body.push_str(&format!(
            "  br i1 {}, label %{remove_label}, label %{done_label}\n{remove_label}:\n  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 0\n  {len} = load i64, ptr {len_ptr}\n  {next_len} = sub i64 {len}, 1\n  store i64 {next_len}, ptr {len_ptr}\n  {needs_compact} = icmp ne i64 {}, {next_len}\n  br i1 {needs_compact}, label %{compact_label}, label %{done_label}\n{compact_label}:\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 2\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 3\n  {keys} = load ptr, ptr {keys_ptr}\n  {values} = load ptr, ptr {values_ptr}\n  {last_key_ptr} = getelementptr inbounds {}, ptr {keys}, i64 {next_len}\n  {last_value_ptr} = getelementptr inbounds {}, ptr {values}, i64 {next_len}\n  {last_key} = load {}, ptr {last_key_ptr}\n  {last_value} = load {}, ptr {last_value_ptr}\n  {removed_key_ptr} = getelementptr inbounds {}, ptr {keys}, i64 {}\n  {removed_value_ptr} = getelementptr inbounds {}, ptr {values}, i64 {}\n  store {} {last_key}, ptr {removed_key_ptr}\n  store {} {last_value}, ptr {removed_value_ptr}\n  br label %{done_label}\n{done_label}:\n",
            found.value,
            index.value,
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir(),
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir(),
            llvm_ir_type(key_ty).as_ir(),
            index.value,
            llvm_ir_type(value_ty).as_ir(),
            index.value,
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir()
        ));
        Ok(found)
    }

    pub(in crate::llvm) fn emit_collection_clear(
        &mut self,
        collection: &str,
        _element: &IrType,
        layout: &str,
        len_index: usize,
    ) {
        let len_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %{layout}, ptr {collection}, i32 0, i32 {len_index}\n  store i64 0, ptr {len_ptr}\n"
        ));
    }

    pub(in crate::llvm) fn emit_collection_index(
        &mut self,
        target: &TypedExpr,
        index: &TypedExpr,
    ) -> Result<LlValue, String> {
        let collection = self.emit_typed_expr(target)?;
        match &target.ty {
            IrType::Array(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let data_ptr = self.tmp();
                let data = self.tmp();
                let slot = self.tmp();
                let value = self.tmp();
                let ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {data_ptr} = getelementptr inbounds %glitch.array, ptr {}, i32 0, i32 1\n  {data} = load ptr, ptr {data_ptr}\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {}\n  {value} = load {}, ptr {slot}\n",
                    collection.value,
                    ty.as_ir(),
                    index.value,
                    ty.as_ir()
                ));
                Ok(LlValue { value, ty })
            }
            IrType::List(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let data_ptr = self.tmp();
                let data = self.tmp();
                let slot = self.tmp();
                let value = self.tmp();
                let ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {data_ptr} = getelementptr inbounds %glitch.list, ptr {}, i32 0, i32 2\n  {data} = load ptr, ptr {data_ptr}\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {}\n  {value} = load {}, ptr {slot}\n",
                    collection.value,
                    ty.as_ir(),
                    index.value,
                    ty.as_ir()
                ));
                Ok(LlValue { value, ty })
            }
            IrType::Dictionary(key_ty, value_ty) => {
                let key = self.emit_typed_expr(index)?;
                let key = self.cast_value(key, &llvm_ir_type(key_ty))?;
                let (found_index, found) =
                    self.emit_dict_find(&collection.value, key_ty, &key.value)?;
                let values_ptr = self.tmp();
                let values = self.tmp();
                let slot = self.tmp();
                let loaded = self.tmp();
                let default_label = self.next_label("dict_index_default");
                let load_label = self.next_label("dict_index_load");
                let done_label = self.next_label("dict_index_done");
                let result_ptr = self.tmp();
                let result = self.tmp();
                let ty = llvm_ir_type(value_ty);
                self.body.push_str(&format!(
                    "  {result_ptr} = alloca {}\n  br i1 {}, label %{load_label}, label %{default_label}\n{load_label}:\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {}, i32 0, i32 3\n  {values} = load ptr, ptr {values_ptr}\n  {slot} = getelementptr inbounds {}, ptr {values}, i64 {}\n  {loaded} = load {}, ptr {slot}\n  store {} {loaded}, ptr {result_ptr}\n  br label %{done_label}\n{default_label}:\n  store {} {}, ptr {result_ptr}\n  br label %{done_label}\n{done_label}:\n  {result} = load {}, ptr {result_ptr}\n",
                    ty.as_ir(),
                    found.value,
                    collection.value,
                    ty.as_ir(),
                    found_index.value,
                    ty.as_ir(),
                    ty.as_ir(),
                    ty.as_ir(),
                    ty.default_value(),
                    ty.as_ir()
                ));
                Ok(LlValue { value: result, ty })
            }
            ty if is_string_like_type(ty) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let slot = self.tmp();
                let value = self.tmp();
                self.body.push_str(&format!(
                    "  {slot} = getelementptr inbounds i8, ptr {}, i64 {}\n  {value} = load i8, ptr {slot}\n",
                    collection.value, index.value
                ));
                Ok(LlValue {
                    value,
                    ty: LlType::I8,
                })
            }
            IrType::Ref(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let slot = self.tmp();
                let value = self.tmp();
                let ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds {}, ptr {}, i64 {}\n  {} = load {}, ptr {}\n",
                    slot, ty.as_ir(), collection.value, index.value, value, ty.as_ir(), slot
                ));
                Ok(LlValue { value, ty })
            }
            IrType::Unknown(_) => {
                self.emit_typed_expr(index)?;
                Ok(LlValue {
                    value: LlType::Ptr.default_value().to_string(),
                    ty: LlType::Ptr,
                })
            }
            other => Err(format!(
                "LLVM TIR backend: indexing is unsupported for {other:?}; move the logic into a concrete specialization or a runtime helper before indexing generic package state"
            )),
        }
    }

    pub(in crate::llvm) fn emit_index_assignment(
        &mut self,
        target: &TypedExpr,
        index: &TypedExpr,
        source: &TypedExpr,
    ) -> Result<(), String> {
        if matches!(target.ty, IrType::Unknown(_)) || self.is_opaque_field(target) {
            self.emit_typed_expr(target)?;
            self.emit_typed_expr(index)?;
            self.emit_typed_expr(source)?;
            return Ok(());
        }

        let collection = self.emit_typed_expr(target)?;
        match &target.ty {
            IrType::Array(element) | IrType::List(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let data_field = if matches!(target.ty, IrType::Array(_)) {
                    ("%glitch.array", 1)
                } else {
                    ("%glitch.list", 2)
                };
                let data_ptr = self.tmp();
                let data = self.tmp();
                let slot = self.tmp();
                let element_ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {data_ptr} = getelementptr inbounds {}, ptr {}, i32 0, i32 {}\n  {data} = load ptr, ptr {data_ptr}\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {}\n",
                    data_field.0,
                    collection.value,
                    data_field.1,
                    element_ty.as_ir(),
                    index.value
                ));
                let value = self.emit_typed_expr(source)?;
                let value = self.cast_value(value, &element_ty)?;
                self.retain_for_store(element, source, &value.value);
                self.move_raw_owned_source_after_store(element, source);
                if let Some(type_name) = object_type_name(element) {
                    if self.object_types.contains_key(type_name) {
                        let old = self.tmp();
                        self.body.push_str(&format!(
                            "  {old} = load {}, ptr {slot}\n",
                            element_ty.as_ir()
                        ));
                        self.emit_drop(type_name, &old);
                    }
                } else if is_string_like_type(element.as_ref()) {
                    let old = self.tmp();
                    self.body.push_str(&format!(
                        "  {old} = load ptr, ptr {slot}\n  call void @glitch_string_release(ptr {old})\n"
                    ));
                }
                self.body.push_str(&format!(
                    "  store {} {}, ptr {slot}\n",
                    element_ty.as_ir(),
                    value.value
                ));
                self.move_raw_owned_source_after_store(element, source);
                Ok(())
            }
            IrType::Dictionary(key_ty, value_ty) => {
                let key_source = index;
                let key = self.emit_typed_expr(key_source)?;
                let key = self.cast_value(key, &llvm_ir_type(key_ty))?;
                let value = self.emit_typed_expr(source)?;
                let value = self.cast_value(value, &llvm_ir_type(value_ty))?;
                let (found_index, found) =
                    self.emit_dict_find(&collection.value, key_ty, &key.value)?;
                let update_label = self.next_label("dict_set_update");
                let insert_label = self.next_label("dict_set_insert");
                let done_label = self.next_label("dict_set_done");
                self.body.push_str(&format!(
                    "  br i1 {}, label %{update_label}, label %{insert_label}\n{update_label}:\n",
                    found.value
                ));
                let values_ptr = self.tmp();
                let values = self.tmp();
                let slot = self.tmp();
                let value_ty_llvm = llvm_ir_type(value_ty);
                self.body.push_str(&format!(
                    "  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {}, i32 0, i32 3\n  {values} = load ptr, ptr {values_ptr}\n  {slot} = getelementptr inbounds {}, ptr {values}, i64 {}\n",
                    collection.value,
                    value_ty_llvm.as_ir(),
                    found_index.value
                ));
                self.retain_for_store(value_ty, source, &value.value);
                if let Some(type_name) = object_type_name(value_ty) {
                    if self.object_types.contains_key(type_name) {
                        let old = self.tmp();
                        self.body.push_str(&format!(
                            "  {old} = load {}, ptr {slot}\n",
                            value_ty_llvm.as_ir()
                        ));
                        self.emit_drop(type_name, &old);
                    }
                } else if is_string_like_type(value_ty.as_ref()) {
                    let old = self.tmp();
                    self.body.push_str(&format!(
                        "  {old} = load ptr, ptr {slot}\n  call void @glitch_string_release(ptr {old})\n"
                    ));
                }
                self.body.push_str(&format!(
                    "  store {} {}, ptr {slot}\n  br label %{done_label}\n{insert_label}:\n",
                    value_ty_llvm.as_ir(),
                    value.value
                ));
                self.emit_dict_add(
                    &collection.value,
                    key_ty,
                    &key.value,
                    key_source,
                    value_ty,
                    &value.value,
                    source,
                );
                self.body
                    .push_str(&format!("  br label %{done_label}\n{done_label}:\n"));
                Ok(())
            }
            IrType::Ref(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let slot = self.tmp();
                let element_ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds {}, ptr {}, i64 {}\n",
                    slot, element_ty.as_ir(), collection.value, index.value
                ));
                let value = self.emit_typed_expr(source)?;
                let value = self.cast_value(value, &element_ty)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    element_ty.as_ir(), value.value, slot
                ));
                Ok(())
            }
            other => Err(format!(
                "LLVM TIR backend: index assignment is unsupported for {other:?}"
            )),
        }
    }

}
