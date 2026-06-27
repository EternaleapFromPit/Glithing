use super::*;
use super::helpers::*;

impl LlvmEmitter {
    pub(super) fn emit_dynamic_object_drop(&mut self, value: &str) {
        self.body
            .push_str(&format!("  call void @glitch_object_drop(ptr {value})\n"));
    }

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
            let resolved_constructor = self
                .specialized_instance_symbols
                .get(&(constructor.to_string(), type_name.to_string()))
                .cloned()
                .unwrap_or_else(|| constructor.to_string());
            self.emit_typed_call(
                &resolved_constructor,
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
            self.move_raw_owned_source_after_store(&field.ty, &field_init.expr);
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
        if matches!(ty, IrType::Task(_)) {
            if matches!(
                expr.kind,
                TypedExprKind::Var(_)
                    | TypedExprKind::Field { .. }
                    | TypedExprKind::Index { .. }
                    | TypedExprKind::Borrow { .. }
            ) {
                self.body
                    .push_str(&format!("  call void @GlitchTask_Retain(ptr {value})\n"));
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

    pub(super) fn move_raw_owned_source_after_store(&mut self, ty: &IrType, expr: &TypedExpr) {
        if !matches!(
            ty,
            IrType::Array(_)
                | IrType::Struct(_)
                | IrType::List(_)
                | IrType::Dictionary(_, _)
                | IrType::Exception
        ) {
            return;
        }
        let Some(name) = expr_source_name(expr) else {
            return;
        };
        if !matches!(expr.kind, TypedExprKind::Var(_)) {
            return;
        }
        let Some(var) = self.vars.get(name).cloned() else {
            return;
        };
        self.body.push_str(&format!(
            "  store {} {}, ptr {}\n",
            var.ty.as_ir(),
            var.ty.default_value(),
            var.ptr
        ));
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
        body: &TypedLambdaBody,
        lambda_ty: &IrType,
    ) -> Result<LlValue, String> {
        delegates::emit_lambda_function(self, params, body, lambda_ty)
    }

    pub(super) fn emit_temporary_drop(&mut self, expr: &TypedExpr, value: &LlValue) {
        if matches!(
            expr.kind,
            TypedExprKind::Var(_)
                | TypedExprKind::Move(_)
                | TypedExprKind::Borrow { .. }
                | TypedExprKind::Index { .. }
        ) {
            return;
        }
        if let TypedExprKind::Field { target, name } = &expr.kind {
            if !(matches!(target.ty, IrType::Task(_)) && name == "Result") {
                return;
            }
        }
        if matches!(expr.kind, TypedExprKind::FunctionSymbol(_)) {
            self.body.push_str(&format!(
                "  call void @glitch_delegate_release(ptr {})\n",
                value.value
            ));
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
                    if self.object_types.contains_key(type_name) {
                        self.emit_drop(type_name, &value.value);
                    } else if matches!(expr.ty, IrType::Class(_) | IrType::Interface(_)) {
                        self.emit_dynamic_object_drop(&value.value);
                    }
                }
            }
            DropKind::DropDelegate => {
                self.body.push_str(&format!(
                    "  call void @glitch_delegate_release(ptr {})\n",
                    value.value
                ));
            }
            DropKind::DropException => {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {})\n",
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
        let needs_drop = (matches!(var.drop_kind, DropKind::Free) && matches!(var.ir_ty, IrType::String))
            || (matches!(var.ir_ty, IrType::Array(_)) && !matches!(var.drop_kind, DropKind::BorrowOnly | DropKind::ViewOnly))
            || matches!(var.drop_kind, DropKind::DropCollection)
            || matches!(var.ir_ty, IrType::Nullable(_))
            || matches!(var.drop_kind, DropKind::DropDelegate)
            || matches!(var.drop_kind, DropKind::DropException)
            || matches!(var.drop_kind, DropKind::DropTask)
            || matches!(&var.ir_ty, IrType::Unknown(name) if name == "object")
            || matches!(var.drop_kind, DropKind::DropClass | DropKind::DropStruct);

        if !needs_drop {
            return;
        }

        let value = self.tmp();
        self.body.push_str(&format!(
            "  {value} = load ptr, ptr {}\n  store ptr null, ptr {}\n",
            var.ptr, var.ptr
        ));

        match &var.ir_ty {
            IrType::String | IrType::Exception => {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {value})\n"
                ));
            }
            IrType::Function { .. } => {
                self.body.push_str(&format!(
                    "  call void @glitch_delegate_release(ptr {value})\n"
                ));
            }
            IrType::Array(element) => {
                self.emit_array_drop_value(&value, element);
            }
            IrType::List(_) | IrType::Dictionary(_, _) => {
                self.emit_collection_drop_value(&var.ir_ty, &value);
            }
            IrType::Task(inner) => {
                self.emit_task_drop_value(&value, inner);
            }
            IrType::Unknown(name) if name == "object" => {
                self.body.push_str(&format!(
                    "  call void @glitch_box_release(ptr {value})\n"
                ));
            }
            IrType::Nullable(inner) => {
                let type_name = LlvmEmitter::nullable_type_name(inner);
                if self.object_types.contains_key(&type_name) {
                    self.emit_drop(&type_name, &value);
                }
            }
            _ => {
                if let Some(type_name) = object_type_name(&var.ir_ty) {
                    if self.object_types.contains_key(type_name) {
                        self.emit_drop(type_name, &value);
                    } else if matches!(var.ir_ty, IrType::Class(_) | IrType::Interface(_)) {
                        self.emit_dynamic_object_drop(&value);
                    }
                } else if matches!(var.ir_ty, IrType::Class(_) | IrType::Interface(_)) {
                    self.emit_dynamic_object_drop(&value);
                }
            }
        }
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

    pub(super) fn emit_owned_payload_drop_value(&mut self, ty: &IrType, value: &str) {
        match ty {
            IrType::Nullable(inner) => self.emit_owned_payload_drop_value(inner, value),
            IrType::String | IrType::Exception => self.body.push_str(&format!(
                "  call void @glitch_string_release(ptr {value})\n"
            )),
            IrType::Function { .. } => self.body.push_str(&format!(
                "  call void @glitch_delegate_release(ptr {value})\n"
            )),
            IrType::Array(element) => self.emit_array_drop_value(value, element),
            IrType::List(_) | IrType::Dictionary(_, _) => {
                self.emit_collection_drop_value(ty, value)
            }
            IrType::Task(inner) => self.emit_task_drop_value(value, inner),
            IrType::Unknown(name) if name == "object" => self.body.push_str(&format!(
                "  call void @glitch_box_release(ptr {value})\n"
            )),
            _ => {
                if let Some(type_name) = object_type_name(ty) {
                    if self.object_types.contains_key(type_name) {
                        self.emit_drop(type_name, value);
                    } else if matches!(ty, IrType::Class(_) | IrType::Interface(_)) {
                        self.emit_dynamic_object_drop(value);
                    }
                }
            }
        }
    }

    pub(super) fn emit_buffer_element_drops(&mut self, element: &IrType, data: &str, len: &str) {
        let needs_drop = matches!(
            element,
            IrType::String
                | IrType::Array(_)
                | IrType::List(_)
                | IrType::Dictionary(_, _)
                | IrType::Task(_)
                | IrType::Nullable(_)
        ) || object_type_name(element).is_some_and(|name| self.object_types.contains_key(name))
            || matches!(element, IrType::Class(_) | IrType::Interface(_))
            || matches!(element, IrType::Unknown(name) if name == "object");
        if !needs_drop {
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
        self.emit_owned_payload_drop_value(element, &item);
        self.body.push_str(&format!(
            "  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{done_label}:\n"
        ));
    }

    pub(super) fn emit_task_drop_value(&mut self, task_value: &str, inner: &IrType) {
        let is_null = self.tmp();
        let release_label = self.next_label("task_release");
        let drop_payload_label = self.next_label("task_drop_payload");
        let done_label = self.next_label("task_release_done");
        let result_ptr = self.tmp();
        let result = self.tmp();
        let destroyed = self.tmp();
        self.body.push_str(&format!(
            "  {is_null} = icmp eq ptr {task_value}, null\n  br i1 {is_null}, label %{done_label}, label %{release_label}\n{release_label}:\n  call void @glitch_task_wait(ptr {task_value})\n  {result_ptr} = getelementptr inbounds %glitch.task, ptr {task_value}, i32 0, i32 2\n  {result} = load ptr, ptr {result_ptr}\n  {destroyed} = call i1 @GlitchTask_Destroy(ptr {task_value})\n  br i1 {destroyed}, label %{drop_payload_label}, label %{done_label}\n{drop_payload_label}:\n"
        ));
        self.emit_owned_payload_drop_value(inner, &result);
        self.body.push_str(&format!(
            "  br label %{done_label}\n{done_label}:\n"
        ));
    }


}

