use super::*;

impl LlvmEmitter {
    pub(in crate::llvm) fn emit_service_provider_lookup(
        &mut self,
        target: &TypedExpr,
        name: &str,
        result_ty: &IrType,
        generic_args: &[IrType],
    ) -> Result<LlValue, String> {
        let receiver = self.emit_typed_expr(target)?;
        let requested_ty = match result_ty {
            IrType::Unknown(name) if name == "T" => generic_args.first().unwrap_or(result_ty),
            _ => result_ty,
        };
        if matches!(requested_ty, IrType::Unknown(name) if name == "T") {
            if let Some(object_name) = object_type_name(&target.ty) {
                if self.object_types.contains_key(object_name) {
                    self.emit_retain(object_name, &receiver.value);
                }
            }
            self.emit_temporary_drop(target, &receiver);
            return Ok(LlValue {
                value: receiver.value,
                ty: LlType::Ptr,
            });
        }
        let requested_name = self.service_registration_name(requested_ty);
        let registration = requested_name
            .as_deref()
            .and_then(|service_name| self.resolve_service_registration_from_target(target, service_name));
        if let Some(registration) = registration {
            let value = match registration.lifetime {
                ServiceLifetime::Singleton => {
                    let source_expr = registration.source_expr.as_ref().ok_or_else(|| {
                        "LLVM TIR backend: singleton service registration lost its source expression"
                            .to_string()
                    })?;
                    let loaded = self.emit_typed_expr(source_expr)?;
                    self.retain_for_store(&registration.stored_ty, source_expr, &loaded.value);
                    self.cast_value(
                        loaded,
                        &llvm_ir_type(requested_ty),
                    )?
                }
                ServiceLifetime::Transient | ServiceLifetime::Scoped => {
                    LlValue {
                        value: self.emit_endpoint_object_allocation(
                            &registration.implementation_name,
                            "service_lookup",
                        )?,
                        ty: LlType::Ptr,
                    }
                }
            };
            self.emit_temporary_drop(target, &receiver);
            return Ok(value);
        }
        let result = match requested_ty {
            IrType::Class(type_name) | IrType::Interface(type_name)
                if matches!(
                    base_type_name(type_name),
                    "IServiceProvider"
                        | "ServiceProvider"
                        | "IServiceScopeFactory"
                        | "Microsoft.Extensions.DependencyInjection.IServiceProvider"
                        | "Microsoft.Extensions.DependencyInjection.ServiceProvider"
                        | "Microsoft.Extensions.DependencyInjection.IServiceScopeFactory"
                ) =>
            {
                if let Some(object_name) = object_type_name(&target.ty) {
                    if self.object_types.contains_key(object_name) {
                        self.emit_retain(object_name, &receiver.value);
                    }
                }
                LlValue {
                    value: receiver.value.clone(),
                    ty: LlType::Ptr,
                }
            }
            IrType::Class(type_name) => {
                let implementation = self
                    .resolve_registered_service_implementation(type_name)
                    .unwrap_or_else(|| type_name.clone());
                if name == "GetService"
                    && implementation == *type_name
                    && !self.service_registrations.contains_key(type_name)
                    && !self
                        .service_registrations
                        .contains_key(base_type_name(type_name))
                {
                    self.emit_temporary_drop(target, &receiver);
                    return self.default_typed_value(result_ty);
                }
                let allocated =
                    self.emit_endpoint_object_allocation(&implementation, "service_lookup")?;
                self.emit_temporary_drop(target, &receiver);
                return Ok(LlValue {
                    value: allocated,
                    ty: LlType::Ptr,
                });
            }
            IrType::Interface(interface_name) => {
                let implementation = if let Some(registered) =
                    self.resolve_registered_service_implementation(interface_name)
                {
                    registered
                } else if let Some(resolved) =
                    self.resolve_interface_implementation(interface_name)
                {
                    resolved
                } else if name == "GetService" {
                    self.emit_temporary_drop(target, &receiver);
                    return self.default_typed_value(result_ty);
                } else {
                    return Err(format!(
                        "LLVM TIR backend: {name}<{interface_name}> requires a unique concrete implementation or an explicit AddTransient/AddScoped registration"
                    ));
                };
                let allocated =
                    self.emit_endpoint_object_allocation(&implementation, "service_lookup")?;
                self.emit_temporary_drop(target, &receiver);
                return Ok(LlValue {
                    value: allocated,
                    ty: LlType::Ptr,
                });
            }
            other => {
                self.emit_temporary_drop(target, &receiver);
                return Err(format!(
                    "LLVM TIR backend: unsupported DI service lookup result {other:?}; rewrite to an explicit concrete construction or supported interface"
                ));
            }
        };
        self.emit_temporary_drop(target, &receiver);
        Ok(result)
    }

    pub(in crate::llvm) fn record_service_registration(&mut self, service_ty: &IrType, implementation_ty: &IrType) {
        let Some(service_name) = self.service_registration_name(service_ty) else {
            return;
        };
        let Some(implementation_name) = self.service_registration_name(implementation_ty) else {
            return;
        };
        self.service_registrations
            .insert(service_name.clone(), implementation_name.clone());
        self.service_registrations.insert(
            base_type_name(&service_name).to_string(),
            implementation_name,
        );
    }

    pub(in crate::llvm) fn expr_tracking_key(&self, expr: &TypedExpr) -> Option<String> {
        match &expr.kind {
            TypedExprKind::Var(name) => Some(name.clone()),
            TypedExprKind::Field { target, name } => self
                .expr_tracking_key(target)
                .map(|prefix| format!("{prefix}.{name}")),
            _ => None,
        }
    }

    pub(in crate::llvm) fn is_web_application_build_call(&self, expr: &TypedExpr) -> Option<String> {
        let TypedExprKind::Call(call) = &expr.kind else {
            return None;
        };
        let TypedCallKind::Method { target, name, .. } = &call.kind else {
            return None;
        };
        if name != "Build" {
            return None;
        }
        match &target.ty {
            IrType::Class(type_name) | IrType::Unknown(type_name)
                if matches!(base_type_name(type_name), "WebApplicationBuilder") =>
            {
                self.expr_tracking_key(target)
            }
            _ => None,
        }
    }

    pub(in crate::llvm) fn is_build_service_provider_call(&self, expr: &TypedExpr) -> Option<String> {
        let TypedExprKind::Call(call) = &expr.kind else {
            return None;
        };
        let TypedCallKind::Method { target, name, .. } = &call.kind else {
            return None;
        };
        if name != "BuildServiceProvider" || !is_service_collection_type(&target.ty) {
            return None;
        }
        self.expr_tracking_key(target)
    }

    pub(in crate::llvm) fn propagate_service_provider_registrations(
        &mut self,
        provider_name: &str,
        collection_key: &str,
    ) {
        if let Some(registrations) = self
            .service_collection_registrations
            .get(collection_key)
            .cloned()
        {
            self.service_provider_registrations
                .insert(provider_name.to_string(), registrations);
        } else {
            self.service_provider_registrations.remove(provider_name);
        }
    }

    pub(in crate::llvm) fn propagate_web_application_service_registrations(
        &mut self,
        app_name: &str,
        builder_key: &str,
    ) {
        let collection_key = format!("{builder_key}.Services");
        if let Some(registrations) = self
            .service_collection_registrations
            .get(&collection_key)
            .cloned()
        {
            self.service_provider_registrations
                .insert(format!("{app_name}.Services"), registrations);
        } else {
            self.service_provider_registrations
                .remove(&format!("{app_name}.Services"));
        }
    }

    pub(in crate::llvm) fn emit_hidden_entry_alloca(&mut self, ty: &LlType) -> String {
        let ptr = self.tmp();
        let insert = format!("  {ptr} = alloca {}\n", ty.as_ir());
        if let Some(pos) = self.entry_insert_pos {
            self.body.insert_str(pos, &insert);
            self.entry_insert_pos = Some(pos + insert.len());
        } else {
            self.body.push_str(&insert);
        }
        ptr
    }

    pub(in crate::llvm) fn resolve_service_registration_from_target(
        &self,
        target: &TypedExpr,
        service_name: &str,
    ) -> Option<ServiceRegistration> {
        let key = self.expr_tracking_key(target)?;
        self.service_provider_registrations
            .get(&key)
            .and_then(|services| {
                services
                    .get(service_name)
                    .cloned()
                    .or_else(|| services.get(base_type_name(service_name)).cloned())
            })
    }

    pub(in crate::llvm) fn resolve_registered_service_implementation(&self, service_name: &str) -> Option<String> {
        self.service_registrations
            .get(service_name)
            .cloned()
            .or_else(|| {
                self.service_registrations
                    .get(base_type_name(service_name))
                    .cloned()
            })
    }

    pub(in crate::llvm) fn service_registration_name(&self, ty: &IrType) -> Option<String> {
        match ty {
            IrType::Class(name)
            | IrType::Struct(name)
            | IrType::Interface(name)
            | IrType::Unknown(name) => Some(name.clone()),
            _ => None,
        }
    }

    pub(in crate::llvm) fn emit_service_registration_marker(
        &mut self,
        method_name: &str,
        target: &TypedExpr,
        args: &[TypedExpr],
        generic_args: &[IrType],
    ) -> Result<LlValue, String> {
        let receiver = self.emit_typed_expr(target)?;
        for arg in args {
            let value = self.emit_typed_expr(arg)?;
            self.emit_temporary_drop(arg, &value);
        }
        if generic_args.len() >= 2 {
            self.record_service_registration(&generic_args[0], &generic_args[1]);
            if let Some(collection_key) = self.expr_tracking_key(target) {
                if let (Some(service_name), Some(implementation_name)) = (
                    self.service_registration_name(&generic_args[0]),
                    self.service_registration_name(&generic_args[1]),
                ) {
                    let registration = ServiceRegistration {
                        lifetime: if method_name == "AddScoped" {
                            ServiceLifetime::Scoped
                        } else {
                            ServiceLifetime::Transient
                        },
                        implementation_name: implementation_name.clone(),
                        source_expr: None,
                        stored_ty: generic_args[1].clone(),
                    };
                    let entry = self
                        .service_collection_registrations
                        .entry(collection_key)
                        .or_default();
                    entry.insert(service_name.clone(), registration.clone());
                    entry.insert(base_type_name(&service_name).to_string(), registration);
                }
            }
        }
        self.emit_temporary_drop(target, &receiver);
        Ok(void_value())
    }

    pub(in crate::llvm) fn emit_service_singleton_registration(
        &mut self,
        target: &TypedExpr,
        args: &[TypedExpr],
        generic_args: &[IrType],
    ) -> Result<LlValue, String> {
        let receiver = self.emit_typed_expr(target)?;
        let Some(collection_key) = self.expr_tracking_key(target) else {
            self.emit_temporary_drop(target, &receiver);
            for arg in args {
                let value = self.emit_typed_expr(arg)?;
                self.emit_temporary_drop(arg, &value);
            }
            return Ok(void_value());
        };
        if generic_args.len() != 1 || args.len() != 1 {
            self.emit_temporary_drop(target, &receiver);
            for arg in args {
                let value = self.emit_typed_expr(arg)?;
                self.emit_temporary_drop(arg, &value);
            }
            return Ok(void_value());
        }
        let service_ty = &generic_args[0];
        let value_expr = &args[0];
        let value = self.emit_typed_expr(value_expr)?;
        let stable_singleton_source = matches!(
            value_expr.kind,
            TypedExprKind::Var(_) | TypedExprKind::Field { .. } | TypedExprKind::Index { .. }
        );
        let source_expr = if stable_singleton_source {
            if should_drop_argument_after_call(value_expr) {
                self.emit_temporary_drop(value_expr, &value);
            }
            value_expr.clone()
        } else {
            let slot_name = format!("__service_singleton_{}", self.tmp());
            let slot_ptr = self.emit_hidden_entry_alloca(&value.ty);
            self.retain_for_store(&value_expr.ty, value_expr, &value.value);
            self.body.push_str(&format!(
                "  store {} {}, ptr {slot_ptr}\n",
                value.ty.as_ir(),
                value.value
            ));
            let hidden_expr = TypedExpr {
                kind: TypedExprKind::Var(slot_name.clone()),
                ty: value_expr.ty.clone(),
                ownership: value_expr.ownership.clone(),
                drop_kind: value_expr.drop_kind.clone(),
            };
            self.vars.insert(
                slot_name.clone(),
                LlVar {
                    ptr: slot_ptr,
                    ty: value.ty.clone(),
                    ir_ty: value_expr.ty.clone(),
                    drop_kind: value_expr.drop_kind.clone(),
                },
            );
            self.drop_order.push(slot_name);
            hidden_expr
        };
        if stable_singleton_source {
            if let Some(service_name) = self.service_registration_name(service_ty) {
                if let Some(implementation_name) = self.service_registration_name(&value_expr.ty) {
                let registration = ServiceRegistration {
                    lifetime: ServiceLifetime::Singleton,
                    implementation_name,
                    source_expr: Some(source_expr),
                    stored_ty: value_expr.ty.clone(),
                };
                let entry = self
                    .service_collection_registrations
                    .entry(collection_key)
                    .or_default();
                entry.insert(service_name.clone(), registration.clone());
                entry.insert(base_type_name(&service_name).to_string(), registration);
            }
            }
        } else if let Some(service_name) = self.service_registration_name(service_ty) {
            if let Some(implementation_name) = self.service_registration_name(&value_expr.ty) {
                let registration = ServiceRegistration {
                    lifetime: ServiceLifetime::Singleton,
                    implementation_name,
                    source_expr: Some(source_expr),
                    stored_ty: value_expr.ty.clone(),
                };
                let entry = self
                    .service_collection_registrations
                    .entry(collection_key)
                    .or_default();
                entry.insert(service_name.clone(), registration.clone());
                entry.insert(base_type_name(&service_name).to_string(), registration);
            }
        }
        self.emit_temporary_drop(target, &receiver);
        Ok(void_value())
    }

}
