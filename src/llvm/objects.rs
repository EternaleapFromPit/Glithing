use super::*;

impl LlvmEmitter {
    pub(super) fn emit_equality(&mut self, ty: &IrType, left: &str, right: &str, result: &str) {
        if matches!(ty, IrType::String) {
            let cmp = self.tmp();
            self.body.push_str(&format!(
                "  {cmp} = call i32 @strcmp(ptr {left}, ptr {right})\n  {result} = icmp eq i32 {cmp}, 0\n"
            ));
        } else if matches!(llvm_ir_type(ty), LlType::Double) {
            self.body
                .push_str(&format!("  {result} = fcmp oeq double {left}, {right}\n"));
        } else {
            self.body.push_str(&format!(
                "  {result} = icmp eq {} {left}, {right}\n",
                llvm_ir_type(ty).as_ir()
            ));
        }
    }

    pub(super) fn emit_type_size(&mut self, ty: &IrType) -> String {
        match llvm_ir_type(ty) {
            LlType::I1 | LlType::I8 => "1".to_string(),
            LlType::I16 => "2".to_string(),
            LlType::I32 => "4".to_string(),
            LlType::I64 | LlType::Double | LlType::Ptr => "8".to_string(),
            LlType::Void => "1".to_string(),
        }
    }

    pub(super) fn emit_new_object(
        &mut self,
        type_name: &str,
        constructor: Option<&str>,
        args: &[TypedExpr],
        fields: &[TypedFieldInit],
    ) -> Result<LlValue, String> {
        let Some(object) = self.object_types.get(type_name).cloned() else {
            for arg in args {
                self.emit_typed_expr(arg)?;
            }
            for field in fields {
                self.emit_typed_expr(&field.expr)?;
            }
            return Ok(LlValue {
                value: "null".to_string(),
                ty: LlType::Ptr,
            });
        };
        if matches!(object.kind, TypeKind::Interface) {
            return Err(format!(
                "LLVM TIR backend: interface '{type_name}' cannot be allocated"
            ));
        }
        let llvm_name = llvm_object_name(&object.name);
        if type_name.starts_with("Rc_") {
            let [value_expr] = args else {
                return Err(format!(
                    "LLVM TIR backend: {type_name} constructor expects exactly one argument"
                ));
            };
            let size_ptr = self.tmp();
            let size = self.tmp();
            let value = self.tmp();
            self.body.push_str(&format!(
                "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {value} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
            ));
            let rc_ptr = self.tmp();
            let drop_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 1\n  store ptr @{}, ptr {drop_ptr}\n",
                destroy_symbol(&object.name)
            ));
            let field_value = self.emit_typed_expr(value_expr)?;
            let inner_field = object.fields.get("value").cloned().ok_or_else(|| {
                format!(
                    "LLVM TIR backend: {type_name} layout is missing the value field"
                )
            })?;
            let field_ty = llvm_ir_type(&inner_field.ty);
            let field_value = self.cast_value(field_value, &field_ty)?;
            let ptr = self.tmp();
            self.body.push_str(&format!(
                "  {ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 2\n  store {} {}, ptr {ptr}\n",
                field_ty.as_ir(),
                field_value.value
            ));
            let ref_count_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {ref_count_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 3\n  store i32 1, ptr {ref_count_ptr}\n"
            ));
            return Ok(LlValue { value, ty: LlType::Ptr });
        }
        let size_ptr = self.tmp();
        let size = self.tmp();
        let value = self.tmp();
        self.body.push_str(&format!(
            "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {value} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
        ));
        if matches!(object.kind, TypeKind::Class) {
            let rc_ptr = self.tmp();
            let drop_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 1\n  store ptr @{}, ptr {drop_ptr}\n",
                destroy_symbol(&object.name)
            ));
        }
        if let Some(constructor) = constructor {
            self.emit_typed_call(
                constructor,
                std::iter::once(LlValue {
                    value: value.clone(),
                    ty: LlType::Ptr,
                }),
                &[],
                args,
            )?;
        } else if !args.is_empty() {
            return Err(format!(
                "LLVM TIR backend: type '{type_name}' has no constructor accepting arguments"
            ));
        }
        for field_init in fields {
            let field = object.fields.get(&field_init.name).ok_or_else(|| {
                format!(
                    "LLVM TIR backend: type '{type_name}' has no field '{}'",
                    field_init.name
                )
            })?;
            let field_value = self.emit_typed_expr(&field_init.expr)?;
            let field_ty = llvm_ir_type(&field.ty);
            let field_value = self.cast_value(field_value, &field_ty)?;
            self.retain_for_store(&field.ty, &field_init.expr, &field_value.value);
            let ptr = self.tmp();
            self.body.push_str(&format!(
                "  {ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 {}\n  store {} {}, ptr {ptr}\n",
                field.index,
                field_ty.as_ir(),
                field_value.value
            ));
        }
        Ok(LlValue {
            value,
            ty: LlType::Ptr,
        })
    }

    pub(super) fn emit_field_ptr(&mut self, expr: &TypedExpr) -> Result<LlValue, String> {
        let TypedExprKind::Field { target, name } = &expr.kind else {
            return Err(format!(
                "LLVM TIR backend: unsupported assignment target {:?} with type {:?}",
                expr.kind, expr.ty
            ));
        };
        let type_name = object_type_name(&target.ty).ok_or_else(|| {
            format!(
                "LLVM TIR backend: field '{}' target has no object layout: {:?}",
                name, target.ty
            )
        })?;
        let object =
            self.object_types.get(type_name).cloned().ok_or_else(|| {
                format!("LLVM TIR backend: type '{type_name}' has no LLVM layout")
            })?;
        let field =
            object.fields.get(name).cloned().ok_or_else(|| {
                format!("LLVM TIR backend: type '{type_name}' has no field '{name}'")
            })?;
        let target = self.emit_typed_expr(target)?;
        let ptr = self.tmp();
        self.body.push_str(&format!(
            "  {ptr} = getelementptr inbounds %{}, ptr {}, i32 0, i32 {}\n",
            llvm_object_name(&object.name),
            target.value,
            field.index
        ));
        Ok(LlValue {
            value: ptr,
            ty: LlType::Ptr,
        })
    }

    pub(super) fn is_opaque_field(&self, expr: &TypedExpr) -> bool {
        let TypedExprKind::Field { target, name } = &expr.kind else {
            return false;
        };
        if matches!(target.ty, IrType::Unknown(_)) {
            return true;
        }
        object_type_name(&target.ty).is_some_and(|type_name| {
            self.object_types
                .get(type_name)
                .is_none_or(|object| !object.fields.contains_key(name))
        })
    }

    pub(super) fn emit_opaque_call(
        &mut self,
        target: Option<&TypedExpr>,
        args: &[TypedExpr],
        return_type: &IrType,
    ) -> Result<LlValue, String> {
        if let Some(target) = target {
            if !matches!(target.kind, TypedExprKind::Var(_))
                || self.vars.contains_key(match &target.kind {
                    TypedExprKind::Var(name) => name,
                    _ => unreachable!(),
                })
            {
                let value = self.emit_typed_expr(target)?;
                self.emit_temporary_drop(target, &value);
            }
        }
        for arg in args {
            let value = self.emit_typed_expr(arg)?;
            self.emit_temporary_drop(arg, &value);
        }
        self.default_typed_value(return_type)
    }

    pub(super) fn default_typed_value(&mut self, ty: &IrType) -> Result<LlValue, String> {
        let ll_type = llvm_ir_type(ty);
        let value = if matches!(ty, IrType::String) {
            self.string_global("")
        } else {
            ll_type.default_value().to_string()
        };
        Ok(LlValue { value, ty: ll_type })
    }

    pub(super) fn retain_for_store(&mut self, ty: &IrType, expr: &TypedExpr, value: &str) {
        if let IrType::Nullable(inner) = ty {
            if matches!(
                expr.kind,
                TypedExprKind::Var(_) | TypedExprKind::Field { .. } | TypedExprKind::Index { .. }
            ) {
                let type_name = LlvmEmitter::nullable_type_name(inner);
                if self.object_types.contains_key(&type_name) {
                    self.body.push_str(&format!(
                        "  call void @{}(ptr {value})\n",
                        retain_symbol(&type_name)
                    ));
                }
            }
            return;
        }
        if matches!(ty, IrType::Unknown(name) if name == "object") {
            return;
        }
        if matches!(ty, IrType::String) {
            if matches!(
                expr.kind,
                TypedExprKind::Var(_) | TypedExprKind::Field { .. } | TypedExprKind::Index { .. }
            ) {
                self.body
                    .push_str(&format!("  call void @glitch_string_retain(ptr {value})\n"));
            }
            return;
        }
        if matches!(ty, IrType::Function { .. }) {
            if matches!(
                expr.kind,
                TypedExprKind::Var(_)
                    | TypedExprKind::Field { .. }
                    | TypedExprKind::Index { .. }
            ) {
                self.body
                    .push_str(&format!("  call void @glitch_delegate_retain(ptr {value})\n"));
            }
            return;
        }
        let Some(type_name) = object_type_name(ty) else {
            return;
        };
        if type_name.starts_with("Box_") {
            return;
        }
        if !self.object_types.contains_key(type_name)
            || matches!(
                expr.kind,
                TypedExprKind::NewObject { .. }
                    | TypedExprKind::Call(_)
                    | TypedExprKind::Await(_)
                    | TypedExprKind::Move(_)
                    | TypedExprKind::Lambda { .. }
                    | TypedExprKind::FunctionSymbol(_)
            )
        {
            return;
        }
        self.emit_retain(type_name, value);
    }

    pub(super) fn emit_retain(&mut self, type_name: &str, value: &str) {
        if let Some(object) = self.object_types.get(type_name) {
            if !matches!(object.kind, TypeKind::Class | TypeKind::Interface) {
                return;
            }
            self.body.push_str(&format!(
                "  call void @{}(ptr {value})\n",
                retain_symbol(&object.name)
            ));
        }
    }

    /// Lift a lambda body to a named top-level LLVM function and return a
    /// function pointer to it.
    ///
    /// Captured variables (free variables referenced in the body that are not
    /// lambda parameters) are appended as extra `ptr` arguments so the lifted
    /// function does not require a heap-allocated closure environment.  The
    /// caller (the site that creates the lambda) passes the current values of
    /// those variables directly.
    pub(super) fn emit_lambda_function(
        &mut self,
        params: &[String],
        body: &TypedExpr,
    ) -> Result<LlValue, String> {
        delegates::emit_lambda_function(self, params, body)
    }

    pub(super) fn emit_temporary_drop(&mut self, expr: &TypedExpr, value: &LlValue) {
        if matches!(
            expr.kind,
            TypedExprKind::Var(_)
                | TypedExprKind::Field { .. }
                | TypedExprKind::Index { .. }
                | TypedExprKind::Move(_)
                | TypedExprKind::Borrow { .. }
        ) {
            return;
        }
        if let IrType::Nullable(inner) = &expr.ty {
            let type_name = LlvmEmitter::nullable_type_name(inner);
            if self.object_types.contains_key(&type_name) {
                self.emit_drop(&type_name, &value.value);
            }
            return;
        }
        if matches!(&expr.ty, IrType::Unknown(name) if name == "object") {
            self.body
                .push_str(&format!("  call void @glitch_box_release(ptr {})\n", value.value));
            return;
        }
        match expr.drop_kind {
            DropKind::Free if matches!(expr.ty, IrType::String) => {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {})\n",
                    value.value
                ));
            }
            DropKind::Free if matches!(expr.ty, IrType::Array(_)) => {
                if let IrType::Array(element) = &expr.ty {
                    self.emit_array_drop_value(&value.value, element);
                }
            }
            DropKind::DropCollection => {
                self.emit_collection_drop_value(&expr.ty, &value.value);
            }
            DropKind::DropClass | DropKind::DropStruct => {
                if let Some(type_name) = object_type_name(&expr.ty) {
                    self.emit_drop(type_name, &value.value);
                }
            }
            DropKind::DropDelegate => {
                self.body.push_str(&format!(
                    "  call void @glitch_delegate_release(ptr {})\n",
                    value.value
                ));
            }
            DropKind::DropTask => {
                if let IrType::Task(inner) = &expr.ty {
                    self.emit_task_drop_value(&value.value, inner);
                }
            }
            _ => {}
        }
    }

    pub(super) fn emit_drop(&mut self, type_name: &str, value: &str) {
        if let Some(object) = self.object_types.get(type_name) {
            self.body.push_str(&format!(
                "  call void @{}(ptr {value})\n",
                drop_symbol(&object.name)
            ));
        }
    }

    pub(super) fn emit_var_drop(&mut self, var: &LlVar) {
        if matches!(var.drop_kind, DropKind::Free) && matches!(var.ir_ty, IrType::String) {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            self.body.push_str(&format!(
                "  call void @glitch_string_release(ptr {value})\n"
            ));
            return;
        }
        if matches!(var.drop_kind, DropKind::Free) && matches!(var.ir_ty, IrType::Array(_)) {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            if let IrType::Array(element) = &var.ir_ty {
                self.emit_array_drop_value(&value, element);
            }
            return;
        }
        if matches!(var.drop_kind, DropKind::DropCollection) {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            self.emit_collection_drop_value(&var.ir_ty, &value);
            return;
        }
        if let IrType::Nullable(inner) = &var.ir_ty {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            let type_name = LlvmEmitter::nullable_type_name(inner);
            if self.object_types.contains_key(&type_name) {
                self.emit_drop(&type_name, &value);
            }
            return;
        }
        if matches!(var.drop_kind, DropKind::DropDelegate) {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            self.body.push_str(&format!(
                "  call void @glitch_delegate_release(ptr {value})\n"
            ));
            return;
        }
        if matches!(var.drop_kind, DropKind::DropTask) {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            if let IrType::Task(inner) = &var.ir_ty {
                self.emit_task_drop_value(&value, inner);
            }
            return;
        }
        if matches!(&var.ir_ty, IrType::Unknown(name) if name == "object") {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n  call void @glitch_box_release(ptr {value})\n", var.ptr));
            return;
        }
        if !matches!(var.drop_kind, DropKind::DropClass | DropKind::DropStruct) {
            return;
        }
        let Some(type_name) = object_type_name(&var.ir_ty) else {
            return;
        };
        if !self.object_types.contains_key(type_name) {
            return;
        }
        let value = self.tmp();
        self.body
            .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
        self.emit_drop(type_name, &value);
    }

    pub(super) fn emit_local_drops(&mut self, returned_local: Option<&str>) {
        let names = self.drop_order.iter().rev().cloned().collect::<Vec<_>>();
        for name in names {
            if returned_local == Some(name.as_str()) {
                continue;
            }
            if let Some(var) = self.vars.get(&name).cloned() {
                self.emit_var_drop(&var);
            }
        }
    }

    pub(super) fn emit_collection_drop_value(&mut self, ty: &IrType, value: &str) {
        let is_null = self.tmp();
        let release_label = self.next_label("collection_release");
        let done_label = self.next_label("collection_release_done");
        self.body.push_str(&format!(
            "  {is_null} = icmp eq ptr {value}, null\n  br i1 {is_null}, label %{done_label}, label %{release_label}\n{release_label}:\n"
        ));
        match ty {
            IrType::List(element) => {
                let len_ptr = self.tmp();
                let data_ptr = self.tmp();
                let len = self.tmp();
                let data = self.tmp();
                self.body.push_str(&format!(
                    "  {len_ptr} = getelementptr inbounds %glitch.list, ptr {value}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {value}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n"
                ));
                self.emit_buffer_element_drops(element, &data, &len);
                self.body.push_str(&format!(
                    "  call void @glitch_free(ptr {data})\n  call void @glitch_free(ptr {value})\n  br label %{done_label}\n"
                ));
            }
            IrType::Dictionary(key, item) => {
                let len_ptr = self.tmp();
                let keys_ptr = self.tmp();
                let values_ptr = self.tmp();
                let len = self.tmp();
                let keys = self.tmp();
                let values = self.tmp();
                self.body.push_str(&format!(
                    "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {value}, i32 0, i32 0\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {value}, i32 0, i32 2\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {value}, i32 0, i32 3\n  {len} = load i64, ptr {len_ptr}\n  {keys} = load ptr, ptr {keys_ptr}\n  {values} = load ptr, ptr {values_ptr}\n"
                ));
                self.emit_buffer_element_drops(key, &keys, &len);
                self.emit_buffer_element_drops(item, &values, &len);
                self.body.push_str(&format!(
                    "  call void @glitch_free(ptr {keys})\n  call void @glitch_free(ptr {values})\n  call void @glitch_free(ptr {value})\n  br label %{done_label}\n"
                ));
            }
            _ => {
                self.body.push_str(&format!(
                    "  call void @glitch_free(ptr {value})\n  br label %{done_label}\n"
                ));
            }
        }
        self.body.push_str(&format!("{done_label}:\n"));
    }

    pub(super) fn emit_array_drop_value(&mut self, value: &str, element: &IrType) {
        let is_null = self.tmp();
        let release_label = self.next_label("array_release");
        let done_label = self.next_label("array_release_done");
        let len_ptr = self.tmp();
        let len = self.tmp();
        let data_ptr = self.tmp();
        let data = self.tmp();
        self.body.push_str(&format!(
            "  {is_null} = icmp eq ptr {value}, null\n  br i1 {is_null}, label %{done_label}, label %{release_label}\n{release_label}:\n  {len_ptr} = getelementptr inbounds %glitch.array, ptr {value}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds %glitch.array, ptr {value}, i32 0, i32 1\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n"
        ));
        self.emit_buffer_element_drops(element, &data, &len);
        self.body.push_str(&format!(
            "  call void @glitch_free(ptr {data})\n  call void @glitch_free(ptr {value})\n  br label %{done_label}\n{done_label}:\n"
        ));
    }

    pub(super) fn emit_buffer_element_drops(&mut self, element: &IrType, data: &str, len: &str) {
        let drop_string = matches!(element, IrType::String);
        let drop_collection = matches!(element, IrType::List(_) | IrType::Dictionary(_, _));
        let drop_array = matches!(element, IrType::Array(_));
        let object = object_type_name(element)
            .filter(|name| self.object_types.contains_key(*name))
            .map(str::to_string);
        if !drop_string && !drop_collection && !drop_array && object.is_none() {
            return;
        }
        let index_ptr = self.tmp();
        let loop_label = self.next_label("element_drop_loop");
        let body_label = self.next_label("element_drop_body");
        let done_label = self.next_label("element_drop_done");
        let index = self.tmp();
        let in_range = self.tmp();
        let slot = self.tmp();
        let item = self.tmp();
        let next = self.tmp();
        let element_type = llvm_ir_type(element);
        self.body.push_str(&format!(
            "  {index_ptr} = alloca i64\n  store i64 0, ptr {index_ptr}\n  br label %{loop_label}\n{loop_label}:\n  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{body_label}, label %{done_label}\n{body_label}:\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  {item} = load {}, ptr {slot}\n",
            element_type.as_ir(),
            element_type.as_ir()
        ));
        if drop_string {
            self.body
                .push_str(&format!("  call void @glitch_string_release(ptr {item})\n"));
        } else if drop_collection {
            // Recursively drop inner collections (e.g. List<List<string>>).
            self.emit_collection_drop_value(element, &item);
        } else if drop_array {
            if let IrType::Array(inner_el) = element {
                self.emit_array_drop_value(&item, inner_el);
            }
        } else if let Some(type_name) = object {
            self.emit_drop(&type_name, &item);
        } else if let IrType::Task(inner) = element {
            self.emit_task_drop_value(&item, inner);
        }
        self.body.push_str(&format!(
            "  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{done_label}:\n"
        ));
    }

    pub(super) fn emit_task_drop_value(&mut self, task_value: &str, inner: &IrType) {
        let is_null = self.tmp();
        let release_label = self.next_label("task_release");
        let done_label = self.next_label("task_release_done");
        let result_ptr = self.tmp();
        let result = self.tmp();
        self.body.push_str(&format!(
            "  {is_null} = icmp eq ptr {task_value}, null\n  br i1 {is_null}, label %{done_label}, label %{release_label}\n{release_label}:\n  {result_ptr} = getelementptr inbounds %glitch.task, ptr {task_value}, i32 0, i32 1\n  {result} = load ptr, ptr {result_ptr}\n"
        ));
        match inner {
            IrType::String => self.body.push_str(&format!(
                "  call void @glitch_string_release(ptr {result})\n"
            )),
            IrType::Array(element) => self.emit_array_drop_value(&result, element),
            IrType::List(_) | IrType::Dictionary(_, _) => {
                self.emit_collection_drop_value(inner, &result)
            }
            _ => {
                if let Some(type_name) = object_type_name(inner) {
                    if self.object_types.contains_key(type_name) {
                        self.emit_drop(type_name, &result);
                    }
                }
            }
        }
        self.body.push_str(&format!(
            "  call void @glitch_free(ptr {task_value})\n  br label %{done_label}\n{done_label}:\n"
        ));
    }


}
