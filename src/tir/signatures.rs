use super::*;

pub(super) fn populate_function_signatures(program: &Program, env: &mut TypeEnv) {
    let mut counts = HashMap::<String, usize>::new();
    for function in &program.functions {
        *counts.entry(function.name.clone()).or_default() += 1;
    }
    for function in &program.functions {
        let overloaded = counts.get(&function.name).copied().unwrap_or_default() > 1;
        let signature = function_signature(function, env, overloaded);
        env.functions
            .entry(function.name.clone())
            .or_default()
            .push(signature);
    }
}

pub(super) fn populate_delegate_signatures(program: &Program, env: &mut TypeEnv) {
    for delegate in &program.delegates {
        let signature = DelegateSignature {
            full_name: qualified_delegate_name(&delegate.namespace, &delegate.name),
            generic_params: delegate
                .generic_params
                .iter()
                .map(|param| param.name.clone())
                .collect(),
            params: delegate.params.iter().map(|param| param.ty.clone()).collect(),
            return_type: delegate.return_type.clone(),
        };
        env.delegates
            .entry(signature.full_name.clone())
            .or_insert_with(|| signature.clone());
        env.delegates
            .entry(delegate.name.clone())
            .or_insert(signature);
    }
}

pub(super) fn populate_method_signatures(program: &Program, env: &mut TypeEnv) {
    for (type_index, ty) in program.types.iter().enumerate() {
        for method in &ty.methods {
            let overloaded = ty
                .methods
                .iter()
                .filter(|candidate| candidate.name == method.name)
                .count()
                > 1;
            let params = method
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let param_ownerships = method
                .params
                .iter()
                .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
                .collect::<Vec<_>>();
            let suffix = params
                .iter()
                .map(ir_symbol_suffix)
                .collect::<Vec<_>>()
                .join("_");
            let base = format!(
                "{}_{}__g{}",
                qualified_type_symbol_name(
                    &ty.namespace,
                    &ty.name,
                    ty.generic_params.len(),
                    type_index,
                ),
                sanitize_ir_symbol(&method.name),
                method.generic_params.len()
            );
            let base = if method.is_extension {
                format!("{base}__ext")
            } else {
                base
            };
            // Preserve the C# source method name for lookup, but give each overload
            // a stable lowered symbol that later IR/codegen stages can call directly.
            let symbol = if overloaded {
                if suffix.is_empty() {
                    format!("{base}__overload")
                } else {
                    format!("{base}__{suffix}")
                }
            } else {
                base
            };
            let return_type = type_syntax_to_ir(&method.return_type, env);
            let return_ownership = ownership_for_declared_type_syntax(&method.return_type, env);
            let params_element_type = params_element_type(&method.params, env);
            let required_params = method.params.iter().take_while(|param| param.default.is_none()).count();
            let key = if method.is_extension || ty.is_extension {
                (
                    extension_receiver_type_key(method, ty, env),
                    method.name.clone(),
                )
            } else {
                (ty.name.clone(), method.name.clone())
            };
            let signature = FunctionSignature {
                package_id: method.package_id.clone().or_else(|| ty.package_id.clone()),
                visibility: method.visibility,
                generic_params: method
                    .generic_params
                    .iter()
                    .map(|param| param.name.clone())
                    .collect(),
                generic_constraints: method
                    .generic_params
                    .iter()
                    .map(|param| param.constraints.clone())
                    .collect(),
                params,
                param_ownerships,
                required_params,
                params_element_type,
                return_type,
                return_ownership,
                symbol,
                is_static: method.is_static,
            };
            if method.is_extension || ty.is_extension {
                env.extension_methods.entry(key).or_default().push(signature);
            } else {
                env.methods.entry(key).or_default().push(signature);
            }
        }
    }
}

fn extension_receiver_type_key(method: &Function, owner: &TypeDef, env: &TypeEnv) -> String {
    let Some(receiver) = method
        .params
        .iter()
        .find(|param| param.modifier == ParamModifier::This)
    else {
        return owner.name.clone();
    };
    match type_syntax_to_ir(&receiver.ty, env) {
        IrType::Class(name)
        | IrType::Struct(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => name,
        IrType::String => "string".to_string(),
        IrType::List(_) => "List".to_string(),
        IrType::Dictionary(_, _) => "Dictionary".to_string(),
        IrType::Enumerable(_) => "IEnumerable".to_string(),
        other => render_monomorphized_ir_type(&other),
    }
}

pub(super) fn populate_constructor_signatures(program: &Program, env: &mut TypeEnv) {
    for (type_index, ty) in program.types.iter().enumerate() {
        let overloaded = ty.constructors.len() > 1;
        for constructor in &ty.constructors {
            let params = constructor
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let param_ownerships = constructor
                .params
                .iter()
                .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
                .collect::<Vec<_>>();
            let suffix = params
                .iter()
                .map(ir_symbol_suffix)
                .collect::<Vec<_>>()
                .join("_");
            let base = format!(
                "{}_ctor",
                qualified_type_symbol_name(
                    &ty.namespace,
                    &ty.name,
                    ty.generic_params.len(),
                    type_index,
                )
            );
            let symbol = if overloaded {
                if suffix.is_empty() {
                    format!("{base}__empty")
                } else {
                    format!("{base}__{suffix}")
                }
            } else {
                base
            };
            let params_element_type = params_element_type(&constructor.params, env);
            let required_params = constructor.params.iter().take_while(|param| param.default.is_none()).count();
            env.constructors
                .entry(ty.name.clone())
                .or_default()
                .push(FunctionSignature {
                    package_id: ty.package_id.clone(),
                    visibility: constructor.visibility,
                    generic_params: ty
                        .generic_params
                        .iter()
                        .map(|param| param.name.clone())
                        .collect(),
                    generic_constraints: ty
                        .generic_params
                        .iter()
                        .map(|param| param.constraints.clone())
                        .collect(),
                    params,
                    param_ownerships,
                    required_params,
                    params_element_type,
                    return_type: IrType::Void,
                    return_ownership: Ownership::Copy,
                    symbol,
                    is_static: false,
                });
        }
    }
}

impl TypeEnv {
    pub(super) fn from_program(program: &Program) -> Self {
        let mut env = TypeEnv::default();
        for enum_def in &program.enums {
            env.kinds.insert(enum_def.name.clone(), TypeKind::Enum);
            env.type_packages
                .insert(enum_def.name.clone(), enum_def.package_id.clone());
            env.type_visibilities
                .insert(enum_def.name.clone(), enum_def.visibility);
            if let Some(package_id) = &enum_def.package_id {
                let key = package_type_key(package_id, &enum_def.name);
                env.kinds.insert(key.clone(), TypeKind::Enum);
                env.type_packages.insert(key.clone(), enum_def.package_id.clone());
                env.type_visibilities.insert(key, enum_def.visibility);
            }
        }
        for ty in &program.types {
            env.kinds.insert(ty.name.clone(), ty.kind);
            env.type_packages
                .insert(ty.name.clone(), ty.package_id.clone());
            env.type_visibilities
                .insert(ty.name.clone(), ty.visibility);
            if let Some(package_id) = &ty.package_id {
                let key = package_type_key(package_id, &ty.name);
                env.kinds.insert(key.clone(), ty.kind);
                env.type_packages.insert(key.clone(), ty.package_id.clone());
                env.type_visibilities.insert(key, ty.visibility);
            }
            env.bases.insert(ty.name.clone(), ty.bases.clone());
        }
        populate_delegate_signatures(program, &mut env);
        populate_function_signatures(program, &mut env);
        populate_method_signatures(program, &mut env);
        populate_constructor_signatures(program, &mut env);
        for ty in &program.types {
            for field in &ty.fields {
                let field_type = type_syntax_to_ir(&field.ty, &env);
                let field_signature = FieldSignature {
                    package_id: ty.package_id.clone(),
                    visibility: field.visibility,
                    ty: field_type.clone(),
                    ownership: ownership_for_declared_type_syntax(&field.ty, &env),
                };
                env.fields
                    .insert((ty.name.clone(), field.name.clone()), field_signature.clone());
                env.field_order
                    .entry(ty.name.clone())
                    .or_default()
                    .push((field.name.clone(), field_signature));
            }
        }
        env
    }

    pub(super) fn single_function(&self, name: &str) -> Option<&FunctionSignature> {
        let signatures = self.functions.get(name)?;
        if signatures.len() == 1 {
            signatures.first()
        } else {
            None
        }
    }

    pub(super) fn lookup_type_visibility(
        &self,
        type_name: &str,
        caller_package: Option<&str>,
    ) -> Option<(Option<String>, Visibility)> {
        let package_candidate = |candidate: &str| {
            caller_package.map(|package_id| package_type_key(package_id, candidate))
        };
        for candidate in [
            type_name.to_string(),
            base_type_name(type_name).to_string(),
            type_name.rsplit('.').next().unwrap_or(type_name).to_string(),
        ] {
            if let Some(package_key) = package_candidate(&candidate) {
                if let Some(visibility) = self.type_visibilities.get(&package_key) {
                    let package_id = self
                        .type_packages
                        .get(&package_key)
                        .cloned()
                        .unwrap_or(None);
                    return Some((package_id, *visibility));
                }
            }
            if let Some(visibility) = self.type_visibilities.get(&candidate) {
                let package_id = self
                    .type_packages
                    .get(&candidate)
                    .cloned()
                    .unwrap_or(None);
                return Some((package_id, *visibility));
            }
        }
        None
    }

    pub(super) fn lookup_signature_by_symbol(&self, symbol: &str) -> Option<&FunctionSignature> {
        self.functions
            .values()
            .chain(self.methods.values())
            .chain(self.extension_methods.values())
            .chain(self.constructors.values())
            .flat_map(|signatures| signatures.iter())
            .find(|signature| signature.symbol == symbol)
    }

    pub(super) fn resolve_function(&self, name: &str, arg_types: &[IrType]) -> Option<&FunctionSignature> {
        let signatures = self.functions.get(name)?;
        resolve_signature(signatures, arg_types)
    }

    pub(super) fn resolve_function_call(
        &self,
        name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<&FunctionSignature>, String> {
        let Some(signatures) = self.functions.get(name) else {
            return Ok(None);
        };
        resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("call to '{name}'"),
        )
    }

    pub(super) fn resolve_function_call_with_generic_args(
        &self,
        name: &str,
        args: &[TypedExpr],
        generic_args: &[IrType],
    ) -> Result<Option<FunctionSignature>, String> {
        let Some(signatures) = self.functions.get(name) else {
            return Ok(None);
        };
        let resolved = resolve_explicit_call_signature(
            signatures,
            args,
            generic_args,
            self,
            &format!("call to '{name}'"),
        )?;
        if resolved.is_none()
            && !generic_args.is_empty()
            && !signatures
                .iter()
                .any(|signature| signature.generic_params.len() == generic_args.len())
        {
            return Err(explicit_generic_arity_error(
                signatures,
                generic_args,
                &format!("call to '{name}'"),
            ));
        }
        Ok(resolved)
    }

    pub(super) fn resolve_method(
        &self,
        type_name: &str,
        method_name: &str,
        arg_types: &[IrType],
    ) -> Option<FunctionSignature> {
        if let Some(signatures) = self
            .methods
            .get(&(type_name.to_string(), method_name.to_string()))
        {
            if let Some(sig) = resolve_signature(signatures, arg_types) {
                return Some(sig.clone());
            }
        }
        if let Some(signature) =
            self.resolve_specialized_method_signature(type_name, method_name, arg_types)
        {
            return Some(signature);
        }
        self.bases.get(type_name)?.iter().find_map(|base| {
            let simple = base.rsplit('.').next().unwrap_or(base);
            self.resolve_method(simple, method_name, arg_types)
        })
    }

    pub(super) fn resolve_method_call(
        &self,
        type_name: &str,
        method_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        let resolve = |args: &[TypedExpr]| -> Result<Option<FunctionSignature>, String> {
            if let Some(signatures) = self
                .methods
                .get(&(type_name.to_string(), method_name.to_string()))
            {
                if let Some(sig) = resolve_call_signature(
                    signatures,
                    args,
                    |expected, arg| ir_conversion_rank(expected, arg, self),
                    || format!("call to '{}.{}'", type_name, method_name),
                )? {
                    return Ok(Some(sig.clone()));
                }
            }
            Ok(None)
        };
        if let Some(sig) = resolve(args)? {
            return Ok(Some(sig));
        }
        if let Some(signature) =
            self.resolve_specialized_method_call_signature(type_name, method_name, args)?
        {
            return Ok(Some(signature));
        }
        if method_name.ends_with("Async") && args.len() > 1 {
            if let Some(sig) = resolve(&args[..args.len() - 1])? {
                return Ok(Some(sig));
            }
        }
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                if let Some(sig) = self.resolve_method_call(simple, method_name, args)? {
                    return Ok(Some(sig));
                }
            }
        }
        Ok(None)
    }

    pub(super) fn resolve_extension_method_call(
        &self,
        type_name: &str,
        method_name: &str,
        receiver: &TypedExpr,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        let mut combined = Vec::with_capacity(args.len() + 1);
        combined.push(receiver.clone());
        combined.extend(args.iter().cloned());
        let try_resolve = |owner: &str| -> Result<Option<FunctionSignature>, String> {
            let Some(signatures) = self
                .extension_methods
                .get(&(owner.to_string(), method_name.to_string()))
            else {
                return Ok(None);
            };
            Ok(resolve_call_signature(
                signatures,
                &combined,
                |expected, arg| ir_conversion_rank(expected, arg, self),
                || format!("extension call to '{}.{}'", owner, method_name),
            )?
            .cloned())
        };
        if let Some(signature) = try_resolve(type_name)? {
            return Ok(Some(signature));
        }
        let simple = base_type_name(type_name);
        if simple != type_name {
            if let Some(signature) = try_resolve(simple)? {
                return Ok(Some(signature));
            }
        }
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                if let Some(signature) = self.resolve_extension_method_call(base, method_name, receiver, args)? {
                    return Ok(Some(signature));
                }
                let base_simple = base.rsplit('.').next().unwrap_or(base);
                if base_simple != base {
                    if let Some(signature) =
                        self.resolve_extension_method_call(base_simple, method_name, receiver, args)?
                    {
                        return Ok(Some(signature));
                    }
                }
            }
        }
        Ok(None)
    }

    pub(super) fn resolve_extension_method_call_with_generic_args(
        &self,
        type_name: &str,
        method_name: &str,
        receiver: &TypedExpr,
        args: &[TypedExpr],
        generic_args: &[IrType],
    ) -> Result<Option<FunctionSignature>, String> {
        let mut combined = Vec::with_capacity(args.len() + 1);
        combined.push(receiver.clone());
        combined.extend(args.iter().cloned());
        let try_resolve = |owner: &str| -> Result<Option<FunctionSignature>, String> {
            let Some(signatures) = self
                .extension_methods
                .get(&(owner.to_string(), method_name.to_string()))
            else {
                return Ok(None);
            };
            resolve_explicit_call_signature(
                signatures,
                &combined,
                generic_args,
                self,
                &format!("extension call to '{}.{}'", owner, method_name),
            )
        };
        if let Some(signature) = try_resolve(type_name)? {
            return Ok(Some(signature));
        }
        let simple = base_type_name(type_name);
        if simple != type_name {
            if let Some(signature) = try_resolve(simple)? {
                return Ok(Some(signature));
            }
        }
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                if let Some(signature) = self.resolve_extension_method_call_with_generic_args(
                    base,
                    method_name,
                    receiver,
                    args,
                    generic_args,
                )? {
                    return Ok(Some(signature));
                }
                let base_simple = base.rsplit('.').next().unwrap_or(base);
                if base_simple != base {
                    if let Some(signature) = self.resolve_extension_method_call_with_generic_args(
                        base_simple,
                        method_name,
                        receiver,
                        args,
                        generic_args,
                    )? {
                        return Ok(Some(signature));
                    }
                }
            }
        }
        Ok(None)
    }

    pub(super) fn resolve_method_call_with_generic_args(
        &self,
        type_name: &str,
        method_name: &str,
        args: &[TypedExpr],
        generic_args: &[IrType],
    ) -> Result<Option<FunctionSignature>, String> {
        let resolve = |owner: &str, call_args: &[TypedExpr]| -> Result<Option<FunctionSignature>, String> {
            let Some(signatures) = self
                .methods
                .get(&(owner.to_string(), method_name.to_string()))
            else {
                return Ok(None);
            };
            resolve_explicit_call_signature(
                signatures,
                call_args,
                generic_args,
                self,
                &format!("call to '{}.{}'", owner, method_name),
            )
        };
        if let Some(sig) = resolve(type_name, args)? {
            return Ok(Some(sig));
        }
        if let Some(signature) = self.resolve_specialized_method_call_signature_with_generic_args(
            type_name,
            method_name,
            args,
            generic_args,
        )? {
            return Ok(Some(signature));
        }
        if method_name.ends_with("Async") && args.len() > 1 {
            if let Some(sig) = resolve(type_name, &args[..args.len() - 1])? {
                return Ok(Some(sig));
            }
        }
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                if let Some(sig) = self.resolve_method_call_with_generic_args(
                    simple,
                    method_name,
                    args,
                    generic_args,
                )? {
                    return Ok(Some(sig));
                }
            }
        }
        let all_candidates = self.method_candidate_signatures(type_name, method_name);
        if !generic_args.is_empty()
            && !all_candidates.is_empty()
            && !all_candidates
                .iter()
                .any(|signature| signature.generic_params.len() == generic_args.len())
        {
            return Err(explicit_generic_arity_error(
                &all_candidates,
                generic_args,
                &format!("call to '{}.{}'", type_name, method_name),
            ));
        }
        Ok(None)
    }

    pub(super) fn resolve_constructor(
        &self,
        type_name: &str,
        arg_types: &[IrType],
    ) -> Option<FunctionSignature> {
        if let Some(signatures) = self.constructors.get(type_name) {
            if let Some(signature) = resolve_signature(signatures, arg_types) {
                return Some(signature.clone());
            }
        }
        self.resolve_specialized_constructor_signature(type_name, arg_types)
    }

    pub(super) fn resolve_constructor_call(
        &self,
        type_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        if let Some(signatures) = self.constructors.get(type_name) {
            if let Some(signature) = resolve_call_signature(
                signatures,
                args,
                |expected, arg| ir_conversion_rank(expected, arg, self),
                || format!("constructor call '{type_name}'"),
            )? {
                return Ok(Some(signature.clone()));
            }
        }
        self.resolve_specialized_constructor_call_signature(type_name, args)
    }

    pub(super) fn resolve_field(&self, type_name: &str, field_name: &str) -> Option<FieldSignature> {
        if let Some(ty) = self
            .fields
            .get(&(type_name.to_string(), field_name.to_string()))
        {
            return Some(ty.clone());
        }
        if let Some(field) = self.resolve_specialized_field(type_name, field_name) {
            return Some(field);
        }
        self.bases.get(type_name)?.iter().find_map(|base| {
            let simple = base.rsplit('.').next().unwrap_or(base);
            self.resolve_field(simple, field_name)
        })
    }

    pub(super) fn all_fields(&self, type_name: &str) -> Vec<(String, IrType)> {
        self.all_field_infos(type_name)
            .into_iter()
            .map(|(name, field)| (name, field.ty))
            .collect()
    }

    pub(super) fn all_field_infos(&self, type_name: &str) -> Vec<(String, FieldSignature)> {
        let mut fields = Vec::new();
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                fields.extend(self.all_field_infos(simple));
            }
        }
        if let Some(own_fields) = self.field_order.get(type_name) {
            for (name, field) in own_fields {
                if let Some(existing) = fields.iter_mut().find(|(field, _)| field == name) {
                    *existing = (name.clone(), field.clone());
                } else {
                    fields.push((name.clone(), field.clone()));
                }
            }
        }
        if fields.is_empty() {
            if let Some(specialized_fields) = self.resolve_specialized_all_field_infos(type_name) {
                return specialized_fields;
            }
        }
        fields
    }

    pub(super) fn resolve_specialized_field(&self, type_name: &str, field_name: &str) -> Option<FieldSignature> {
        let (base_name, subst) = self.generic_owner_subst(type_name)?;
        let field = self.fields.get(&(base_name.clone(), field_name.to_string()))?;
        Some(substitute_field_signature(field, &subst))
    }

    pub(super) fn resolve_specialized_all_field_infos(
        &self,
        type_name: &str,
    ) -> Option<Vec<(String, FieldSignature)>> {
        let (base_name, subst) = self.generic_owner_subst(type_name)?;
        let own_fields = self.field_order.get(&base_name)?;
        let mut fields = Vec::new();
        if let Some(bases) = self.bases.get(&base_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                fields.extend(self.all_field_infos(simple));
            }
        }
        for (name, field) in own_fields {
            let field = substitute_field_signature(field, &subst);
            if let Some(existing) = fields.iter_mut().find(|(field_name, _)| field_name == name) {
                *existing = (name.clone(), field);
            } else {
                fields.push((name.clone(), field));
            }
        }
        Some(fields)
    }

    pub(super) fn resolve_specialized_method_signature(
        &self,
        type_name: &str,
        method_name: &str,
        arg_types: &[IrType],
    ) -> Option<FunctionSignature> {
        let (base_name, subst) = self.generic_owner_subst(type_name)?;
        let signatures = self.methods.get(&(base_name.clone(), method_name.to_string()))?;
        let signature = resolve_signature(signatures, arg_types)?;
        Some(substitute_function_signature(signature, &subst))
    }

    pub(super) fn resolve_specialized_method_call_signature(
        &self,
        type_name: &str,
        method_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        let Some((base_name, subst)) = self.generic_owner_subst(type_name) else {
            return Ok(None);
        };
        let Some(signatures) = self.methods.get(&(base_name.clone(), method_name.to_string())) else {
            return Ok(None);
        };
        let signature = resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("call to '{}.{}'", type_name, method_name),
        )?;
        Ok(signature.map(|sig| substitute_function_signature(sig, &subst)))
    }

    pub(super) fn resolve_specialized_method_call_signature_with_generic_args(
        &self,
        type_name: &str,
        method_name: &str,
        args: &[TypedExpr],
        generic_args: &[IrType],
    ) -> Result<Option<FunctionSignature>, String> {
        let Some((base_name, subst)) = self.generic_owner_subst(type_name) else {
            return Ok(None);
        };
        let Some(signatures) = self.methods.get(&(base_name.clone(), method_name.to_string())) else {
            return Ok(None);
        };
        let signature = resolve_explicit_call_signature(
            signatures,
            args,
            generic_args,
            self,
            &format!("call to '{}.{}'", type_name, method_name),
        )?;
        Ok(signature.map(|sig| substitute_function_signature(&sig, &subst)))
    }

    pub(super) fn method_candidate_signatures(
        &self,
        type_name: &str,
        method_name: &str,
    ) -> Vec<FunctionSignature> {
        let mut seen = HashSet::new();
        let mut results = Vec::new();
        self.collect_method_candidate_signatures(type_name, method_name, &mut seen, &mut results);
        results
    }

    fn collect_method_candidate_signatures(
        &self,
        type_name: &str,
        method_name: &str,
        seen: &mut HashSet<String>,
        results: &mut Vec<FunctionSignature>,
    ) {
        if !seen.insert(type_name.to_string()) {
            return;
        }
        if let Some(signatures) = self
            .methods
            .get(&(type_name.to_string(), method_name.to_string()))
        {
            results.extend(signatures.iter().cloned());
        }
        let simple = base_type_name(type_name);
        if simple != type_name {
            if let Some(signatures) = self
                .methods
                .get(&(simple.to_string(), method_name.to_string()))
            {
                results.extend(signatures.iter().cloned());
            }
        }
        if let Some((base_name, subst)) = self.generic_owner_subst(type_name) {
            if let Some(signatures) = self
                .methods
                .get(&(base_name.clone(), method_name.to_string()))
            {
                results.extend(
                    signatures
                        .iter()
                        .map(|signature| substitute_function_signature(signature, &subst)),
                );
            }
            if let Some(bases) = self.bases.get(&base_name) {
                for base in bases {
                    self.collect_method_candidate_signatures(base, method_name, seen, results);
                }
            }
        }
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                self.collect_method_candidate_signatures(base, method_name, seen, results);
            }
        }
    }

    pub(super) fn resolve_specialized_constructor_signature(
        &self,
        type_name: &str,
        arg_types: &[IrType],
    ) -> Option<FunctionSignature> {
        let (base_name, subst) = self.generic_owner_subst(type_name)?;
        let signatures = self.constructors.get(&base_name)?;
        let signature = resolve_signature(signatures, arg_types)?;
        Some(substitute_function_signature(signature, &subst))
    }

    pub(super) fn resolve_specialized_constructor_call_signature(
        &self,
        type_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<FunctionSignature>, String> {
        let Some((base_name, subst)) = self.generic_owner_subst(type_name) else {
            return Ok(None);
        };
        let Some(signatures) = self.constructors.get(&base_name) else {
            return Ok(None);
        };
        let signature = resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("constructor call '{type_name}'"),
        )?;
        Ok(signature.map(|sig| substitute_function_signature(sig, &subst)))
    }

    pub(super) fn generic_owner_subst(&self, type_name: &str) -> Option<(String, HashMap<String, IrType>)> {
        let (base, args) = split_monomorphized_type(type_name)?;
        let base_name = base_type_name(base).to_string();
        let generic_params = self.type_generic_params.get(&base_name)?;
        if generic_params.len() != args.len() || generic_params.is_empty() {
            return None;
        }
        let args = args
            .into_iter()
            .map(|arg| parse_monomorphized_ir_type(&arg, self))
            .collect::<Option<Vec<_>>>()?;
        let mut subst = HashMap::new();
        for (name, arg) in generic_params.iter().cloned().zip(args) {
            subst.insert(name, arg);
        }
        Some((base_name, subst))
    }
}

pub(super) fn resolve_signature<'a>(
    signatures: &'a [FunctionSignature],
    arg_types: &[IrType],
) -> Option<&'a FunctionSignature> {
    let args = arg_types
        .iter()
        .map(|ty| TypedExpr {
            kind: TypedExprKind::Var("<type-only>".to_string()),
            ty: ty.clone(),
            ownership: ownership_for_type(ty),
            drop_kind: drop_kind_for_type(ty, &ownership_for_type(ty)),
        })
        .collect::<Vec<_>>();
    resolve_call_signature(
        signatures,
        &args,
        |expected, arg| {
            if ir_arg_matches(expected, &arg.ty) {
                Some(0)
            } else {
                None
            }
        },
        || "type-only overload lookup".to_string(),
    )
    .ok()
    .flatten()
}

pub(super) fn ir_arg_matches(expected: &IrType, actual: &IrType) -> bool {
    expected == actual || matches!((expected, actual), (IrType::Int, IrType::Long))
}

pub(super) fn resolve_call_signature<'a, F, C>(
    signatures: &'a [FunctionSignature],
    args: &[TypedExpr],
    rank: F,
    context: C,
) -> Result<Option<&'a FunctionSignature>, String>
where
    F: Fn(&IrType, &TypedExpr) -> Option<u16>,
    C: Fn() -> String,
{
    let mut applicable = Vec::<(&FunctionSignature, Vec<u16>)>::new();
    let mut seen = HashSet::<String>::new();
    for signature in signatures {
        let key = format!(
            "{:?}|{:?}|{:?}|{:?}|{}",
            signature.params,
            signature.params_element_type,
            signature.return_type,
            signature.return_ownership,
            signature.is_static
        );
        if !seen.insert(key) {
            continue;
        }
        if args.len() >= signature.required_params && args.len() <= signature.params.len() {
            if let Some(ranks) = signature
                .params
                .iter()
                .take(args.len())
                .zip(args.iter())
                .map(|(expected, arg)| rank(expected, arg))
                .collect::<Option<Vec<_>>>()
            {
                applicable.push((signature, ranks));
                continue;
            }
        }
        let Some(params_element_type) = &signature.params_element_type else {
            continue;
        };
        let fixed_len = signature.params.len().saturating_sub(1);
        if args.len() < fixed_len {
            continue;
        }
        let mut ranks = Vec::new();
        let mut applicable_signature = true;
        for (expected, arg) in signature
            .params
            .iter()
            .take(fixed_len)
            .zip(args.iter().take(fixed_len))
        {
            let Some(rank_value) = rank(expected, arg) else {
                applicable_signature = false;
                break;
            };
            ranks.push(rank_value);
        }
        if !applicable_signature {
            continue;
        }
        for arg in args.iter().skip(fixed_len) {
            let Some(rank_value) = rank(params_element_type, arg) else {
                applicable_signature = false;
                break;
            };
            ranks.push(rank_value);
        }
        if applicable_signature {
            applicable.push((signature, ranks));
        }
    }

    if applicable.is_empty() {
        return Ok(None);
    }

    let Some((best, best_ranks)) = applicable.iter().find(|(_, ranks)| {
        applicable.iter().all(|(_, other)| {
            ranks == other
                || ranks
                    .iter()
                    .zip(other.iter())
                    .all(|(candidate, other)| candidate <= other)
        })
    }) else {
        let best_generic_arity = applicable
            .iter()
            .map(|(signature, _)| signature.generic_params.len())
            .min();
        if let Some(best_generic_arity) = best_generic_arity {
            let least_generic = applicable
                .iter()
                .filter(|(signature, _)| signature.generic_params.len() == best_generic_arity)
                .collect::<Vec<_>>();
            if least_generic.len() == 1 {
                return Ok(Some(least_generic[0].0));
            }
        }
        let candidates = applicable
            .iter()
            .map(|(signature, ranks)| {
                format!(
                    "{:?} -> {:?} ranks={:?}",
                    signature.params, signature.return_type, ranks
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        return Err(format!(
            "ambiguous overload resolution for {}: {}",
            context(),
            candidates
        ));
    };

    let tied = applicable
        .iter()
        .filter(|(_, ranks)| {
            ranks == best_ranks
                || (ranks
                    .iter()
                    .zip(best_ranks.iter())
                    .all(|(candidate, best)| candidate <= best)
                    && best_ranks
                        .iter()
                        .zip(ranks.iter())
                        .all(|(best, candidate)| best <= candidate))
                })
        .count();
    if tied > 1 {
        let tied_candidates = applicable
            .iter()
            .filter(|(_, ranks)| ranks == best_ranks)
            .collect::<Vec<_>>();
        let best_specificity = tied_candidates
            .iter()
            .map(|(signature, _)| signature_specificity(signature))
            .min();
        if let Some(best_specificity) = best_specificity {
            let specialized = tied_candidates
                .into_iter()
                .filter(|(signature, _)| signature_specificity(signature) == best_specificity)
                .collect::<Vec<_>>();
            if specialized.len() == 1 {
                return Ok(Some(specialized[0].0));
            }
            let best_generic_arity = specialized
                .iter()
                .map(|(signature, _)| signature.generic_params.len())
                .min();
            if let Some(best_generic_arity) = best_generic_arity {
                let least_generic = specialized
                    .into_iter()
                    .filter(|(signature, _)| signature.generic_params.len() == best_generic_arity)
                    .collect::<Vec<_>>();
                if least_generic.len() == 1 {
                    return Ok(Some(least_generic[0].0));
                }
            }
        }
        let candidates = applicable
            .iter()
            .map(|(signature, ranks)| {
                format!(
                    "{:?} -> {:?} ranks={:?}",
                    signature.params, signature.return_type, ranks
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        return Err(format!(
            "ambiguous overload resolution for {}: {}",
            context(),
            candidates
        ));
    }

    Ok(Some(*best))
}

pub(super) fn resolve_explicit_call_signature(
    signatures: &[FunctionSignature],
    args: &[TypedExpr],
    generic_args: &[IrType],
    env: &TypeEnv,
    context: &str,
) -> Result<Option<FunctionSignature>, String> {
    if generic_args.is_empty() {
        return Ok(resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, env),
            || context.to_string(),
        )?
        .cloned());
    }
    let mut arity_matched = Vec::new();
    for signature in signatures {
        if signature.generic_params.len() == generic_args.len() {
            arity_matched.push(specialize_signature_with_explicit_generic_args(
                signature,
                generic_args,
                env,
                context,
            )?);
        }
    }
    if arity_matched.is_empty() {
        return Ok(None);
    }
    resolve_call_signature(
        &arity_matched,
        args,
        |expected, arg| ir_conversion_rank(expected, arg, env),
        || context.to_string(),
    )
    .map(|signature| signature.cloned())
}

pub(super) fn explicit_generic_arity_error(
    signatures: &[FunctionSignature],
    generic_args: &[IrType],
    context: &str,
) -> String {
    let supported = signatures
        .iter()
        .map(|signature| signature.generic_params.len())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .map(|arity| arity.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let supported = if supported.is_empty() {
        "no generic overloads".to_string()
    } else {
        format!("available generic arities: {supported}")
    };
    format!(
        "{context} expects {} type argument(s); {supported}",
        generic_args.len()
    )
}

pub(super) fn signature_specificity(signature: &FunctionSignature) -> u16 {
    signature
        .params
        .iter()
        .map(ir_type_specificity)
        .sum()
}

pub(super) fn ir_type_specificity(ty: &IrType) -> u16 {
    match ty {
        IrType::Enumerable(_) => 0,
        IrType::List(_) => 1,
        IrType::Array(_) => 0,
        IrType::Dictionary(_, _) => 0,
        IrType::Ref(inner)
        | IrType::Task(inner)
        | IrType::Weak(inner)
        | IrType::Nullable(inner) => ir_type_specificity(inner),
        IrType::Function {
            params,
            return_type,
        } => params.iter().map(ir_type_specificity).sum::<u16>() + ir_type_specificity(return_type),
        _ => 0,
    }
}

pub(super) fn function_signature(function: &Function, env: &TypeEnv, overloaded: bool) -> FunctionSignature {
    let params = function
        .params
        .iter()
        .map(|param| type_syntax_to_ir(&param.ty, env))
        .collect::<Vec<_>>();
    let param_ownerships = function
        .params
        .iter()
        .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
        .collect::<Vec<_>>();
    let params_element_type = params_element_type(&function.params, env);
    let required_params = function
        .params
        .iter()
        .take_while(|param| param.default.is_none())
        .count();
    let base_symbol = if function.generic_params.is_empty() {
        qualified_function_symbol_name(&function.namespace, &function.name)
    } else {
        format!(
            "{}__g{}",
            qualified_function_symbol_name(&function.namespace, &function.name),
            function.generic_params.len()
        )
    };
    FunctionSignature {
        package_id: function.package_id.clone(),
        visibility: function.visibility,
        generic_params: function
            .generic_params
            .iter()
            .map(|param| param.name.clone())
            .collect(),
        generic_constraints: function
            .generic_params
            .iter()
            .map(|param| param.constraints.clone())
            .collect(),
        symbol: if overloaded {
            overloaded_function_symbol(&base_symbol, &params)
        } else {
            base_symbol
        },
        params,
        param_ownerships,
        required_params,
        params_element_type,
        return_type: type_syntax_to_ir(&function.return_type, env),
        return_ownership: ownership_for_declared_type_syntax(&function.return_type, env),
        is_static: false,
    }
}

pub(super) fn params_element_type(params: &[Param], env: &TypeEnv) -> Option<IrType> {
    let last = params.last()?;
    if !last.is_params {
        return None;
    }
    match type_syntax_to_ir(&last.ty, env) {
        IrType::Array(inner) => Some(*inner),
        other => Some(other),
    }
}

pub(super) fn overloaded_function_symbol(name: &str, params: &[IrType]) -> String {
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    if suffix.is_empty() {
        format!("{name}__overload")
    } else {
        format!("{name}__{suffix}")
    }
}

pub(super) fn qualified_function_symbol_name(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        sanitize_ir_symbol(name)
    } else {
        format!(
            "{}_{}",
            sanitize_ir_symbol(&namespace.join("_")),
            sanitize_ir_symbol(name)
        )
    }
}

pub(super) fn qualified_type_symbol_name(
    namespace: &[String],
    name: &str,
    generic_arity: usize,
    type_id: usize,
) -> String {
    if namespace.is_empty() {
        format!("{}__g{}__t{}", sanitize_ir_symbol(name), generic_arity, type_id)
    } else {
        format!(
            "{}_{}__g{}__t{}",
            sanitize_ir_symbol(&namespace.join("_")),
            sanitize_ir_symbol(name),
            generic_arity,
            type_id
        )
    }
}

pub(super) fn type_method_symbol(ty: &TypeDef, type_id: usize, method: &Function, env: &TypeEnv) -> String {
    let overloaded = ty
        .methods
        .iter()
        .filter(|candidate| candidate.name == method.name)
        .count()
        > 1;
    let params = method
        .params
        .iter()
        .map(|param| type_syntax_to_ir(&param.ty, env))
        .collect::<Vec<_>>();
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    let base = format!(
        "{}_{}__g{}",
        qualified_type_symbol_name(
            &ty.namespace,
            &ty.name,
            ty.generic_params.len(),
            type_id,
        ),
        sanitize_ir_symbol(&method.name),
        method.generic_params.len()
    );
    let base = if method.is_extension {
        format!("{base}__ext")
    } else {
        base
    };
    if overloaded {
        if suffix.is_empty() {
            format!("{base}__overload")
        } else {
            format!("{base}__{suffix}")
        }
    } else {
        base
    }
}

pub(super) fn type_constructor_symbol(
    ty: &TypeDef,
    type_id: usize,
    constructor: &Constructor,
    env: &TypeEnv,
) -> String {
    let overloaded = ty.constructors.len() > 1;
    let params = constructor
        .params
        .iter()
        .map(|param| type_syntax_to_ir(&param.ty, env))
        .collect::<Vec<_>>();
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    let base = format!(
        "{}_ctor",
        qualified_type_symbol_name(
            &ty.namespace,
            &ty.name,
            ty.generic_params.len(),
            type_id,
        )
    );
    if overloaded {
        if suffix.is_empty() {
            format!("{base}__empty")
        } else {
            format!("{base}__{suffix}")
        }
    } else {
        base
    }
}

pub(crate) fn ir_symbol_suffix(ty: &IrType) -> String {
    match ty {
        IrType::Bool => "bool".to_string(),
        IrType::Byte => "byte".to_string(),
        IrType::Short => "short".to_string(),
        IrType::Int => "int".to_string(),
        IrType::Long => "long".to_string(),
        IrType::UInt => "uint".to_string(),
        IrType::Double => "double".to_string(),
        IrType::Decimal => "decimal".to_string(),
        IrType::String => "string".to_string(),
        IrType::Void => "void".to_string(),
        IrType::Class(name)
        | IrType::Struct(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => sanitize_ir_symbol(name),
        IrType::Weak(inner) => format!("weak_{}", ir_symbol_suffix(inner)),
        IrType::Array(inner) => format!("array_{}", ir_symbol_suffix(inner)),
        IrType::Ref(inner) => format!("ref_{}", ir_symbol_suffix(inner)),
        IrType::List(inner) => format!("list_{}", ir_symbol_suffix(inner)),
        IrType::Dictionary(key, value) => {
            format!("dict_{}_{}", ir_symbol_suffix(key), ir_symbol_suffix(value))
        }
        IrType::Enumerable(inner) => format!("ienumerable_{}", ir_symbol_suffix(inner)),
        IrType::Nullable(inner) => format!("nullable_{}", ir_symbol_suffix(inner)),
        IrType::Thread => "thread".to_string(),
        IrType::Task(inner) => format!("task_{}", ir_symbol_suffix(inner)),
        IrType::Function { params, return_type } => {
            let mut parts = params.iter().map(ir_symbol_suffix).collect::<Vec<_>>();
            if !matches!(return_type.as_ref(), IrType::Void) {
                parts.push(format!("ret_{}", ir_symbol_suffix(return_type)));
            }
            if parts.is_empty() {
                "fn".to_string()
            } else {
                format!("fn_{}", parts.join("_"))
            }
        }
        IrType::Exception => "exception".to_string(),
    }
}

pub(super) fn render_monomorphized_ir_type(ty: &IrType) -> String {
    match ty {
        IrType::Bool => "bool".to_string(),
        IrType::Byte => "byte".to_string(),
        IrType::Short => "short".to_string(),
        IrType::Int => "int".to_string(),
        IrType::Long => "long".to_string(),
        IrType::UInt => "uint".to_string(),
        IrType::Double => "double".to_string(),
        IrType::Decimal => "decimal".to_string(),
        IrType::String => "string".to_string(),
        IrType::Void => "void".to_string(),
        IrType::Array(inner) => format!("{}[]", render_monomorphized_ir_type(inner)),
        IrType::Ref(inner) => format!("ref {}", render_monomorphized_ir_type(inner)),
        IrType::Struct(name) | IrType::Class(name) | IrType::Interface(name) | IrType::Unknown(name) => {
            name.clone()
        }
        IrType::List(inner) => format!("List<{}>", render_monomorphized_ir_type(inner)),
        IrType::Dictionary(key, value) => format!(
            "Dictionary<{},{}>",
            render_monomorphized_ir_type(key),
            render_monomorphized_ir_type(value)
        ),
        IrType::Enumerable(inner) => {
            format!("IEnumerable<{}>", render_monomorphized_ir_type(inner))
        }
        IrType::Thread => "Thread".to_string(),
        IrType::Task(inner) => format!("Task<{}>", render_monomorphized_ir_type(inner)),
        IrType::Nullable(inner) => format!("Nullable<{}>", render_monomorphized_ir_type(inner)),
        IrType::Function { params, return_type } => {
            let mut all = params
                .iter()
                .map(render_monomorphized_ir_type)
                .collect::<Vec<_>>();
            all.push(render_monomorphized_ir_type(return_type));
            format!("Func<{}>", all.join(","))
        }
        IrType::Exception => "Exception".to_string(),
        IrType::Weak(inner) => format!("Weak<{}>", render_monomorphized_ir_type(inner)),
    }
}

pub(super) fn monomorphized_type_name(name: &str, args: &[IrType]) -> String {
    format!(
        "{}<{}>",
        name,
        args.iter()
            .map(render_monomorphized_ir_type)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(super) fn rc_runtime_type_name(inner: &IrType) -> String {
    let rendered = render_monomorphized_ir_type(inner);
    let mut suffix = String::new();
    for ch in rendered.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            suffix.push(ch);
        } else {
            suffix.push('_');
        }
    }
    format!("Rc_{suffix}")
}

pub(super) fn is_weak_reference_type_name(name: &str) -> bool {
    name.starts_with("Weak_")
        || name.starts_with("System_WeakReference_")
        || name.starts_with("WeakReference_")
        || name.starts_with("Weak<")
        || name.starts_with("WeakReference<")
        || name.starts_with("System.WeakReference<")
}

pub(super) fn base_type_name(name: &str) -> &str {
    name.split('<')
        .next()
        .unwrap_or(name)
        .rsplit('.')
        .next()
        .unwrap_or(name)
}

pub(super) fn split_monomorphized_type(text: &str) -> Option<(&str, Vec<String>)> {
    let mut depth = 0usize;
    let mut open = None;
    let mut close = None;
    for (index, ch) in text.char_indices() {
        match ch {
            '<' => {
                if depth == 0 {
                    open = Some(index);
                }
                depth += 1;
            }
            '>' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    close = Some(index);
                    break;
                }
            }
            _ => {}
        }
    }
    let open = open?;
    let close = close?;
    let base = &text[..open];
    let args = &text[open + 1..close];
    let mut parsed_args = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    for ch in args.chars() {
        match ch {
            '<' => {
                depth += 1;
                current.push(ch);
            }
            '>' => {
                depth = depth.saturating_sub(1);
                current.push(ch);
            }
            ',' if depth == 0 => {
                parsed_args.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        parsed_args.push(current.trim().to_string());
    }
    Some((base, parsed_args))
}

pub(super) fn parse_monomorphized_ir_type(text: &str, env: &TypeEnv) -> Option<IrType> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    match text {
        "bool" => Some(IrType::Bool),
        "byte" => Some(IrType::Byte),
        "short" => Some(IrType::Short),
        "int" => Some(IrType::Int),
        "long" => Some(IrType::Long),
        "uint" => Some(IrType::UInt),
        "double" => Some(IrType::Double),
        "decimal" => Some(IrType::Decimal),
        "string" => Some(IrType::String),
        "void" => Some(IrType::Void),
        "Exception" | "System.Exception" => Some(IrType::Exception),
        _ => {
            if let Some(inner) = text.strip_suffix("[]") {
                return Some(IrType::Array(Box::new(parse_monomorphized_ir_type(inner, env)?)));
            }
            if let Some((base, args)) = split_monomorphized_type(text) {
                let base_name = base_type_name(base);
                return match base_name {
                    "List" => Some(IrType::List(Box::new(parse_monomorphized_ir_type(
                        args.first()?,
                        env,
                    )?))),
                    "Dictionary" => Some(IrType::Dictionary(
                        Box::new(parse_monomorphized_ir_type(args.first()?, env)?),
                        Box::new(parse_monomorphized_ir_type(args.get(1)?, env)?),
                    )),
                    "IEnumerable" | "ICollection" => Some(IrType::Enumerable(Box::new(
                        parse_monomorphized_ir_type(args.first()?, env)?,
                    ))),
                    "Task" | "ValueTask" => Some(IrType::Task(Box::new(
                        parse_monomorphized_ir_type(args.first()?, env)?,
                    ))),
                    "Weak" | "WeakReference" => Some(IrType::Weak(Box::new(
                        parse_monomorphized_ir_type(args.first()?, env)?,
                    ))),
                    "Nullable" => Some(IrType::Nullable(Box::new(
                        parse_monomorphized_ir_type(args.first()?, env)?,
                    ))),
                    _ => {
                        let kind = env.kinds.get(base_name).copied()?;
                        Some(match kind {
                            TypeKind::Class => IrType::Class(text.to_string()),
                            TypeKind::Interface => IrType::Interface(text.to_string()),
                            TypeKind::Enum => IrType::Int,
                            TypeKind::Struct | TypeKind::RefStruct => {
                                IrType::Struct(text.to_string())
                            }
                        })
                    }
                };
            }
            match env.kinds.get(base_type_name(text)).copied() {
                Some(TypeKind::Class) => Some(IrType::Class(text.to_string())),
                Some(TypeKind::Interface) => Some(IrType::Interface(text.to_string())),
                Some(TypeKind::Enum) => Some(IrType::Int),
                Some(TypeKind::Struct | TypeKind::RefStruct) => {
                    Some(IrType::Struct(text.to_string()))
                }
                None => Some(IrType::Unknown(text.to_string())),
            }
        }
    }
}

pub(super) fn sanitize_ir_symbol(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

pub(super) fn qualified_delegate_name(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", namespace.join("."), name)
    }
}

pub(super) fn collect_instantiation(ty: &IrType, output: &mut Vec<GenericInstantiation>) {
    let instantiation = match ty {
        IrType::List(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "List".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Dictionary(key, value) => {
            collect_instantiation(key, output);
            collect_instantiation(value, output);
            Some(GenericInstantiation {
                name: "Dictionary".to_string(),
                args: vec![key.as_ref().clone(), value.as_ref().clone()],
            })
        }
        IrType::Enumerable(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "IEnumerable".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Task(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "Task".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Array(inner) | IrType::Ref(inner) | IrType::Weak(inner) => {
            collect_instantiation(inner, output);
            None
        }
        IrType::Function {
            params,
            return_type,
        } => {
            for param in params {
                collect_instantiation(param, output);
            }
            collect_instantiation(return_type, output);
            None
        }
        _ => None,
    };
    if let Some(instantiation) = instantiation {
        if !output.contains(&instantiation) {
            output.push(instantiation);
        }
    }
}

pub(super) fn collect_generic_call_instantiations(
    functions: &[TypedFunction],
    types: &[TypedType],
    output: &mut Vec<GenericInstantiation>,
) {
    let mut generic_symbols = HashSet::new();
    for function in functions {
        if !function.generic_params.is_empty() {
            generic_symbols.insert(function.symbol.clone());
        }
    }
    for ty in types {
        for constructor in &ty.constructors {
            if !constructor.generic_params.is_empty() {
                generic_symbols.insert(constructor.symbol.clone());
            }
        }
        for method in &ty.methods {
            if !method.generic_params.is_empty() {
                generic_symbols.insert(method.symbol.clone());
            }
        }
    }

    for function in functions {
        collect_generic_call_instantiations_function(
            function,
            &generic_symbols,
            output,
        );
    }
    for ty in types {
        for constructor in &ty.constructors {
            collect_generic_call_instantiations_function(
                constructor,
                &generic_symbols,
                output,
            );
        }
        for method in &ty.methods {
            collect_generic_call_instantiations_function(
                method,
                &generic_symbols,
                output,
            );
        }
    }
}

pub(super) fn collect_generic_call_instantiations_function(
    function: &TypedFunction,
    generic_symbols: &HashSet<String>,
    output: &mut Vec<GenericInstantiation>,
) {
    collect_generic_call_instantiations_stmts(
        &function.body,
        generic_symbols,
        output,
    );
}

pub(super) fn collect_generic_call_instantiations_stmts(
    stmts: &[TypedStmt],
    generic_symbols: &HashSet<String>,
    output: &mut Vec<GenericInstantiation>,
) {
    for stmt in stmts {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                collect_instantiation(&binding.ty, output);
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Assign { expr, .. }
            | TypedStmtKind::Print(expr)
            | TypedStmtKind::Expr(expr)
            | TypedStmtKind::Return(Some(expr))
            | TypedStmtKind::Throw(expr) => {
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::AssignTarget { target, expr } => {
                collect_generic_call_instantiations_expr(
                    target,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Block(body)
            | TypedStmtKind::While { body, .. }
            | TypedStmtKind::For { body, .. }
            | TypedStmtKind::ForEach { body, .. } => {
                collect_generic_call_instantiations_stmts(
                    body,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_generic_call_instantiations_expr(
                    condition,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    then_body,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    else_body,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                collect_generic_call_instantiations_stmts(
                    try_body,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    catch_body,
                    generic_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    finally_body,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Switch { expr, cases, default } => {
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    output,
                );
                for case in cases {
                    collect_generic_call_instantiations_expr(
                        &case.value,
                        generic_symbols,
                        output,
                    );
                    collect_generic_call_instantiations_stmts(
                        &case.body,
                        generic_symbols,
                        output,
                    );
                }
                collect_generic_call_instantiations_stmts(
                    default,
                    generic_symbols,
                    output,
                );
            }
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
        }
    }
}

pub(super) fn collect_generic_call_instantiations_expr(
    expr: &TypedExpr,
    generic_symbols: &HashSet<String>,
    output: &mut Vec<GenericInstantiation>,
) {
    match &expr.kind {
        TypedExprKind::NullableSome(value) => {
            collect_generic_call_instantiations_expr(value, generic_symbols, output);
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                collect_generic_call_instantiations_expr(value, generic_symbols, output);
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_generic_call_instantiations_expr(length, generic_symbols, output);
            }
            for value in values {
                collect_generic_call_instantiations_expr(value, generic_symbols, output);
            }
        }
        TypedExprKind::Index { target, index } => {
            collect_generic_call_instantiations_expr(target, generic_symbols, output);
            collect_generic_call_instantiations_expr(index, generic_symbols, output);
        }
        TypedExprKind::Field { target, .. } => {
            collect_generic_call_instantiations_expr(target, generic_symbols, output);
        }
        TypedExprKind::IsPattern { expr, .. } => {
            collect_generic_call_instantiations_expr(expr, generic_symbols, output);
        }
        TypedExprKind::Throw(expr) | TypedExprKind::Await(expr) => {
            collect_generic_call_instantiations_expr(expr, generic_symbols, output);
        }
        TypedExprKind::Assign { target, value } => {
            collect_generic_call_instantiations_expr(target, generic_symbols, output);
            collect_generic_call_instantiations_expr(value, generic_symbols, output);
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_generic_call_instantiations_expr(condition, generic_symbols, output);
            collect_generic_call_instantiations_expr(when_true, generic_symbols, output);
            collect_generic_call_instantiations_expr(when_false, generic_symbols, output);
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_generic_call_instantiations_expr(left, generic_symbols, output);
            collect_generic_call_instantiations_expr(right, generic_symbols, output);
        }
        TypedExprKind::Unary { expr: inner, .. }
        | TypedExprKind::IncDec { target: inner, .. } => {
            collect_generic_call_instantiations_expr(inner, generic_symbols, output);
        }
        TypedExprKind::Lambda { body, .. } => match body {
            TypedLambdaBody::Expr(body) => {
                collect_generic_call_instantiations_expr(body, generic_symbols, output);
            }
            TypedLambdaBody::Block(stmts) => {
                collect_generic_call_instantiations_stmts(stmts, generic_symbols, output);
            }
        },
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Function { symbol, .. } => {
                    if generic_symbols.contains(symbol) && !call.generic_args.is_empty() {
                        let instantiation = GenericInstantiation {
                            name: symbol.clone(),
                            args: call.generic_args.clone(),
                        };
                        if !output.contains(&instantiation) {
                            output.push(instantiation);
                        }
                    }
                }
                TypedCallKind::Method { target, symbol, resolution, .. } => {
                    collect_generic_call_instantiations_expr(target, generic_symbols, output);
                    if generic_symbols.contains(symbol) && !call.generic_args.is_empty() {
                        let instantiation = GenericInstantiation {
                            name: symbol.clone(),
                            args: call.generic_args.clone(),
                        };
                        if !output.contains(&instantiation) {
                            output.push(instantiation);
                        }
                    }
                    let _ = resolution;
                }
            }
            for arg in &call.args {
                collect_generic_call_instantiations_expr(arg, generic_symbols, output);
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_generic_call_instantiations_expr(arg, generic_symbols, output);
            }
            for field in fields {
                collect_generic_call_instantiations_expr(&field.expr, generic_symbols, output);
            }
        }
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. } => {}
    }
}

