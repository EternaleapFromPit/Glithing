use super::*;

impl LlvmEmitter {
    pub(super) fn emit_drop_glue(&mut self) {
        let objects = self.object_types.values().cloned().collect::<Vec<_>>();
        for object in objects {
            let llvm_name = llvm_object_name(&object.name);
            let retain_name = retain_symbol(&object.name);
            let drop_name = drop_symbol(&object.name);
            if object.name.starts_with("Nullable_") {
                self.body.push_str(&format!(
                    "define void @{retain_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %retain\nretain:\n  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 0\n  %rc = load i64, ptr %rc_ptr\n  %next = add i64 %rc, 1\n  store i64 %next, ptr %rc_ptr\n  br label %done\ndone:\n  ret void\n}}\n\n"
                ));
                self.body.push_str(&format!(
                    "define void @{drop_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 0\n  %rc = load i64, ptr %rc_ptr\n  %next = sub i64 %rc, 1\n  %destroy = icmp eq i64 %next, 0\n  br i1 %destroy, label %destroy_object, label %keep\ndestroy_object:\n  %has_value_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 2\n  %has_value = load i1, ptr %has_value_ptr\n  br i1 %has_value, label %drop_value, label %done\ndrop_value:\n"
                ));
                self.emit_nullable_payload_drop(&object, &llvm_name);
                self.body
                    .push_str("  call void @glitch_free(ptr %object)\n  br label %done\nkeep:\n  store i64 %next, ptr %rc_ptr\n  br label %done\ndone:\n  ret void\n}\n\n");
                continue;
            }
            if matches!(object.kind, TypeKind::Class | TypeKind::Interface) {
                let destroy_name = destroy_symbol(&object.name);
                self.body.push_str(&format!(
                    "define void @{destroy_name}(ptr %object) {{\nentry:\n"
                ));
                self.emit_field_drops(&object, &llvm_name);
                self.body
                    .push_str("  call void @glitch_free(ptr %object)\n  ret void\n}\n\n");
                self.body.push_str(&format!(
                    "define void @{retain_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %retain\nretain:\n  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 0\n  %rc = load i64, ptr %rc_ptr\n  %next = add i64 %rc, 1\n  store i64 %next, ptr %rc_ptr\n  br label %done\ndone:\n  ret void\n}}\n\n"
                ));
                self.body.push_str(&format!(
                    "define void @{drop_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 0\n  %rc = load i64, ptr %rc_ptr\n  %next = sub i64 %rc, 1\n  %destroy = icmp eq i64 %next, 0\n  br i1 %destroy, label %destroy_object, label %keep\ndestroy_object:\n  %drop_ptr_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 1\n  %drop_ptr = load ptr, ptr %drop_ptr_ptr\n  %has_dynamic_drop = icmp ne ptr %drop_ptr, null\n  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop\ndynamic_drop:\n  call void %drop_ptr(ptr %object)\n  br label %done\nstatic_drop:\n  call void @{destroy_name}(ptr %object)\n  br label %done\n"
                ));
                self.body.push_str(
                    "keep:\n  store i64 %next, ptr %rc_ptr\n  br label %done\ndone:\n  ret void\n}\n\n",
                );
            } else {
                self.body.push_str(&format!(
                    "define void @{drop_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %destroy_object\ndestroy_object:\n"
                ));
                self.emit_field_drops(&object, &llvm_name);
                self.body
                    .push_str("  call void @glitch_free(ptr %object)\n  br label %done\ndone:\n  ret void\n}\n\n");
            }
        }
    }

    pub(super) fn emit_field_drops(&mut self, object: &LlObjectType, llvm_name: &str) {
        for field in object.fields.values() {
            if matches!(field.drop_kind, DropKind::DropCollection) {
                let ptr_name = format!("%field_{}_ptr", field.index);
                let value_name = format!("%field_{}", field.index);
                self.body.push_str(&format!(
                    "  {ptr_name} = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  {value_name} = load ptr, ptr {ptr_name}\n",
                    field.index
                ));
                self.emit_collection_drop_value(&field.ty, &value_name);
                continue;
            }
            if let IrType::Nullable(inner) = &field.ty {
                let ptr_name = format!("%field_{}_ptr", field.index);
                let value_name = format!("%field_{}", field.index);
                self.body.push_str(&format!(
                    "  {ptr_name} = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  {value_name} = load ptr, ptr {ptr_name}\n",
                    field.index
                ));
                let type_name = LlvmEmitter::nullable_type_name(inner);
                if self.object_types.contains_key(&type_name) {
                    self.emit_drop(&type_name, &value_name);
                }
                continue;
            }
            if matches!(field.drop_kind, DropKind::Free) && matches!(field.ty, IrType::Array(_)) {
                let ptr_name = format!("%field_{}_ptr", field.index);
                let value_name = format!("%field_{}", field.index);
                self.body.push_str(&format!(
                    "  {ptr_name} = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  {value_name} = load ptr, ptr {ptr_name}\n",
                    field.index
                ));
                if let IrType::Array(element) = &field.ty {
                    self.emit_array_drop_value(&value_name, element);
                } else if let IrType::Task(inner) = &field.ty {
                    self.emit_task_drop_value(&value_name, inner);
                }
                continue;
            }
            if matches!(field.drop_kind, DropKind::Free) && is_string_like_type(&field.ty) {
                self.body.push_str(&format!(
                    "  %field_{}_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %field_{} = load ptr, ptr %field_{}_ptr\n  call void @glitch_string_release(ptr %field_{})\n",
                    field.index, field.index, field.index, field.index, field.index
                ));
                continue;
            }
            if matches!(field.drop_kind, DropKind::DropDelegate) {
                self.body.push_str(&format!(
                    "  %field_{}_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %field_{} = load ptr, ptr %field_{}_ptr\n  call void @glitch_delegate_release(ptr %field_{})\n",
                    field.index, field.index, field.index, field.index, field.index
                ));
                continue;
            }
            if matches!(&field.ty, IrType::Unknown(name) if name == "object") {
                self.body.push_str(&format!(
                    "  %field_{}_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %field_{} = load ptr, ptr %field_{}_ptr\n  call void @glitch_box_release(ptr %field_{})\n",
                    field.index, field.index, field.index, field.index, field.index
                ));
                continue;
            }
            let Some(type_name) = object_type_name(&field.ty) else {
                continue;
            };
            if !self.object_types.contains_key(type_name) {
                continue;
            }
            if !matches!(field.drop_kind, DropKind::DropClass | DropKind::DropStruct) {
                continue;
            }
            let drop_name = drop_symbol(&self.object_types[type_name].name);
            self.body.push_str(&format!(
                "  %field_{}_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %field_{} = load ptr, ptr %field_{}_ptr\n  call void @{}(ptr %field_{})\n",
                field.index,
                field.index,
                field.index,
                field.index,
                drop_name,
                field.index
            ));
        }
    }

    pub(super) fn emit_nullable_payload_drop(&mut self, object: &LlObjectType, llvm_name: &str) {
        let Some(field) = object.fields.get("Value").cloned() else {
            return;
        };
        if matches!(field.drop_kind, DropKind::DropCollection) {
            let ptr_name = "%nullable_value_ptr";
            let value_name = "%nullable_value";
            self.body.push_str(&format!(
                "  {ptr_name} = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  {value_name} = load ptr, ptr {ptr_name}\n",
                field.index
            ));
            self.emit_collection_drop_value(&field.ty, value_name);
            return;
        }
        if matches!(field.drop_kind, DropKind::Free) && matches!(field.ty, IrType::Array(_)) {
            let ptr_name = "%nullable_value_ptr";
            let value_name = "%nullable_value";
            self.body.push_str(&format!(
                "  {ptr_name} = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  {value_name} = load ptr, ptr {ptr_name}\n",
                field.index
            ));
            if let IrType::Array(element) = &field.ty {
                self.emit_array_drop_value(value_name, element);
            }
            return;
        }
        if matches!(field.drop_kind, DropKind::Free) && is_string_like_type(&field.ty) {
            self.body.push_str(&format!(
                "  %nullable_value_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %nullable_value = load ptr, ptr %nullable_value_ptr\n  call void @glitch_string_release(ptr %nullable_value)\n",
                field.index
            ));
            return;
        }
        if matches!(field.drop_kind, DropKind::DropDelegate) {
            self.body.push_str(&format!(
                "  %nullable_value_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %nullable_value = load ptr, ptr %nullable_value_ptr\n  call void @glitch_delegate_release(ptr %nullable_value)\n",
                field.index
            ));
            return;
        }
        if matches!(field.drop_kind, DropKind::DropClass | DropKind::DropStruct) {
            if let Some(type_name) = object_type_name(&field.ty) {
                if self.object_types.contains_key(type_name) {
                    let drop_name = drop_symbol(&self.object_types[type_name].name);
                    self.body.push_str(&format!(
                        "  %nullable_value_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %nullable_value = load ptr, ptr %nullable_value_ptr\n  call void @{}(ptr %nullable_value)\n",
                        field.index,
                        drop_name
                    ));
                }
            }
        }
    }

    pub(super) fn emit_boxed_scalar_value(&mut self, value: LlValue, ty: &IrType) -> Result<LlValue, String> {
        let Some(type_name) = self.ensure_boxed_scalar_object_type(ty) else {
            return Err(format!(
                "LLVM backend: unsupported boxing target for type {:?}",
                ty
            ));
        };
        let tag = boxed_scalar_tag(ty).ok_or_else(|| {
            format!("LLVM backend: unsupported boxed scalar tag for type {:?}", ty)
        })?;
        let llvm_name = llvm_object_name(&type_name);
        let size_ptr = self.tmp();
        let size = self.tmp();
        let object = self.tmp();
        self.body.push_str(&format!(
            "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {object} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
        ));
        let rc_ptr = self.tmp();
        let drop_ptr = self.tmp();
        let tag_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {object}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {object}, i32 0, i32 1\n  store ptr @glitch_destroy_boxed_scalar, ptr {drop_ptr}\n  {tag_ptr} = getelementptr inbounds %{llvm_name}, ptr {object}, i32 0, i32 2\n  store i32 {tag}, ptr {tag_ptr}\n"
        ));
        let payload_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {payload_ptr} = getelementptr inbounds %{llvm_name}, ptr {object}, i32 0, i32 3\n  store {} {}, ptr {payload_ptr}\n",
            llvm_ir_type(ty).as_ir(),
            value.value
        ));
        Ok(LlValue {
            value: object,
            ty: LlType::Ptr,
        })
    }

}
