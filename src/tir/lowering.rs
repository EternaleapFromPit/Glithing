use super::*;

pub(super) fn lower_type(ty: &TypeDef, symbol_id: usize, env: &TypeEnv) -> Result<TypedType, String> {
    let this_type = collection_this_type(ty);
    let fields = env
        .all_field_infos(&ty.name)
        .into_iter()
        .map(|(name, field)| TypedBinding {
            name,
            ownership: field.ownership,
            ty: field.ty,
        })
        .collect::<Vec<_>>();
    let this_binding = TypedBinding {
        name: "this".to_string(),
        ty: this_type,
        ownership: Ownership::Borrowed,
    };
    let constructors = ty
        .constructors
        .iter()
        .map(|constructor| {
            let function = Function {
                package_id: ty.package_id.clone(),
                visibility: constructor.visibility,
                namespace: constructor.namespace.clone(),
                attributes: constructor.attributes.clone(),
                is_async: false,
                is_extern: false,
                is_static: false,
                is_extension: false,
                name: ty.name.clone(),
                generic_params: Vec::new(),
                params: constructor.params.clone(),
                return_type: TypeSyntax::Void,
                body: constructor.body.clone(),
            };
            lower_function(
                &function,
                env,
                std::slice::from_ref(&this_binding),
                Some(ty.name.clone()),
                Some(type_constructor_symbol(ty, symbol_id, constructor, env)),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let methods = ty
        .methods
        .iter()
        .map(|method| {
            let implicit_params: &[TypedBinding] = if method.is_extension || method.is_static {
                &[]
            } else {
                std::slice::from_ref(&this_binding)
            };
            lower_function(
                method,
                env,
                implicit_params,
                Some(ty.name.clone()),
                Some(type_method_symbol(ty, symbol_id, method, env)),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TypedType {
        package_id: ty.package_id.clone(),
        visibility: ty.visibility,
        name: ty.name.clone(),
        namespace: ty.namespace.clone(),
        generic_params: ty
            .generic_params
            .iter()
            .map(|param| param.name.clone())
            .collect(),
        symbol_id,
        is_extension: ty.is_extension,
        kind: ty.kind,
        bases: ty.bases.clone(),
        fields,
        constructors,
        methods,
    })
}

pub(super) fn lower_function(
    function: &Function,
    env: &TypeEnv,
    implicit_params: &[TypedBinding],
    current_type: Option<String>,
    symbol_override: Option<String>,
) -> Result<TypedFunction, String> {
    let return_type = type_syntax_to_ir(&function.return_type, env);
    let mut scopes = vec![HashMap::new()];
    let mut params = Vec::new();
    for param in implicit_params {
        scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), param.clone());
        params.push(param.clone());
    }
    if let Some(current_type) = &current_type {
        scopes
            .last_mut()
            .unwrap()
            .insert(
                "__glitching_current_type".to_string(),
                TypedBinding {
                    name: "__glitching_current_type".to_string(),
                    ty: IrType::Class(current_type.clone()),
                    ownership: Ownership::Shared,
                },
            );
    }
    for param in &function.params {
        let ty = type_syntax_to_ir(&param.ty, env);
        let binding = TypedBinding {
            name: param.name.clone(),
            ownership: ownership_for_declared_type_syntax(&param.ty, env),
            ty,
        };
        scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), binding.clone());
        params.push(binding);
    }
    let mut locals = Vec::new();
    lower_stmts(&function.body, env, &mut scopes, &mut locals)?;
    let mut typed_scopes = vec![HashMap::new()];
    for param in &params {
        typed_scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), param.clone());
    }
    if let Some(current_type) = current_type {
        typed_scopes
            .last_mut()
            .unwrap()
            .insert(
                "__glitching_current_type".to_string(),
                TypedBinding {
                    name: "__glitching_current_type".to_string(),
                    ty: IrType::Class(current_type),
                    ownership: Ownership::Shared,
                },
            );
    }
    let body = lower_typed_stmts(&function.body, env, &mut typed_scopes, Some(&return_type))?;
    Ok(TypedFunction {
        package_id: function.package_id.clone(),
        visibility: function.visibility,
        name: function.name.clone(),
        symbol: symbol_override.unwrap_or_else(|| {
            env.resolve_function(
                &function.name,
                &function
                    .params
                    .iter()
                    .map(|param| type_syntax_to_ir(&param.ty, env))
                    .collect::<Vec<_>>(),
            )
            .map(|signature| signature.symbol.clone())
            .unwrap_or_else(|| function.name.clone())
        }),
        is_async: function.is_async,
        generic_params: function
            .generic_params
            .iter()
            .map(|param| param.name.clone())
            .collect(),
        is_extern: function.is_extern,
        required_params: function
            .params
            .iter()
            .take_while(|param| param.default.is_none())
            .count(),
        return_ownership: ownership_for_declared_type_syntax(&function.return_type, env),
        return_type,
        params,
        locals,
        body,
    })
}

pub(super) fn collection_this_type(ty: &TypeDef) -> IrType {
    match ty.name.as_str() {
        "List" | "System.Collections.Generic.List" => {
            let item = ty
                .generic_params
                .first()
                .map(|param| IrType::Unknown(param.name.clone()))
                .unwrap_or_else(|| IrType::Unknown("T".to_string()));
            IrType::List(Box::new(item))
        }
        "Dictionary" | "System.Collections.Generic.Dictionary" => {
            let key = ty
                .generic_params
                .first()
                .map(|param| IrType::Unknown(param.name.clone()))
                .unwrap_or_else(|| IrType::Unknown("TKey".to_string()));
            let value = ty
                .generic_params
                .get(1)
                .map(|param| IrType::Unknown(param.name.clone()))
                .unwrap_or_else(|| IrType::Unknown("TValue".to_string()));
            IrType::Dictionary(Box::new(key), Box::new(value))
        }
        "IEnumerable" | "System.Collections.Generic.IEnumerable" => {
            let item = ty
                .generic_params
                .first()
                .map(|param| IrType::Unknown(param.name.clone()))
                .unwrap_or_else(|| IrType::Unknown("T".to_string()));
            IrType::Enumerable(Box::new(item))
        }
        _ => match ty.kind {
            TypeKind::Class => {
                if ty.generic_params.is_empty() {
                    IrType::Class(ty.name.clone())
                } else {
                    IrType::Class(monomorphized_type_name(
                        &ty.name,
                        &ty.generic_params
                            .iter()
                            .map(|param| IrType::Unknown(param.name.clone()))
                            .collect::<Vec<_>>(),
                    ))
                }
            }
            TypeKind::Interface => {
                if ty.generic_params.is_empty() {
                    IrType::Interface(ty.name.clone())
                } else {
                    IrType::Interface(monomorphized_type_name(
                        &ty.name,
                        &ty.generic_params
                            .iter()
                            .map(|param| IrType::Unknown(param.name.clone()))
                            .collect::<Vec<_>>(),
                    ))
                }
            }
            _ => {
                let name = if ty.generic_params.is_empty() {
                    ty.name.clone()
                } else {
                    monomorphized_type_name(
                        &ty.name,
                        &ty.generic_params
                            .iter()
                            .map(|param| IrType::Unknown(param.name.clone()))
                            .collect::<Vec<_>>(),
                    )
                };
                IrType::Ref(Box::new(IrType::Struct(name)))
            }
        },
    }
}

pub(super) fn find_expected_types(candidates: &[FunctionSignature], args: &[Expr]) -> Vec<Option<IrType>> {
    let mut expected = vec![None; args.len()];
    for sig in candidates {
        if let Some(element_ty) = &sig.params_element_type {
            let fixed_len = sig.params.len().saturating_sub(1);
            if args.len() >= fixed_len {
                for (i, param_ty) in sig.params.iter().take(fixed_len).enumerate() {
                    if expected[i].is_none() {
                        expected[i] = Some(param_ty.clone());
                    }
                }
                for i in fixed_len..args.len() {
                    if expected[i].is_none() {
                        expected[i] = Some(element_ty.clone());
                    }
                }
            }
        } else if sig.params.len() == args.len() {
            for (i, param_ty) in sig.params.iter().enumerate() {
                if expected[i].is_none() {
                    expected[i] = Some(param_ty.clone());
                }
            }
        }
    }
    expected
}

pub(super) fn lower_lambda(
    params: &[String],
    body: &LambdaBody,
    expected_type: Option<&IrType>,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedExpr, String> {
    let mut captures = Vec::new();
    collect_lambda_captures(body, params, &mut captures);
    captures.sort();
    captures.dedup();
    for capture in &captures {
        if let Some(binding) = lookup(scopes, capture) {
            if matches!(binding.ownership, Ownership::Borrowed | Ownership::View) {
                return Err(format!(
                    "ownership checker: lambda capture '{capture}' may outlive borrowed/view source; move the value into the closure or use an owned copy"
                ));
            }
        }
    }
    let param_types: Vec<IrType> = if let Some(IrType::Function {
        params: expected_params,
        ..
    }) = expected_type
    {
        expected_params.iter().cloned().collect()
    } else {
        vec![IrType::Unknown("lambda_param".to_string()); params.len()]
    };
    let mut new_scopes = scopes.to_vec();
    let mut lambda_scope = HashMap::new();
    for (name, ty) in params.iter().zip(param_types.iter()) {
        lambda_scope.insert(
            name.clone(),
            TypedBinding {
                name: name.clone(),
                ty: ty.clone(),
                ownership: ownership_for_type(ty),
            },
        );
    }
    new_scopes.push(lambda_scope);
    let expected_return = expected_type.and_then(|ty| match ty {
        IrType::Function { return_type, .. } => Some(return_type.as_ref().clone()),
        _ => None,
    });
    let (typed_body, return_type) = match body {
        LambdaBody::Expr(expr) => {
            let typed_body = lower_typed_expr_with_expected(
                expr,
                env,
                &new_scopes,
                expected_return.as_ref(),
            )?;
            let return_type = typed_body.ty.clone();
            (TypedLambdaBody::Expr(Box::new(typed_body)), return_type)
        }
        LambdaBody::Block(stmts) => {
            let mut lambda_scopes = new_scopes.clone();
            let typed_body = lower_typed_stmts(
                stmts,
                env,
                &mut lambda_scopes,
                expected_return.as_ref(),
            )?;
            (
                TypedLambdaBody::Block(typed_body),
                expected_return.unwrap_or(IrType::Void),
            )
        }
    };
    let lambda_ty = IrType::Function {
        params: param_types,
        return_type: Box::new(return_type),
    };
    Ok(typed_expr_with_ownership(
        TypedExprKind::Lambda {
            params: params.to_vec(),
            body: typed_body,
        },
        lambda_ty,
        Ownership::Shared,
    ))
}

pub(super) fn collect_lambda_captures(body: &LambdaBody, params: &[String], out: &mut Vec<String>) {
    match body {
        LambdaBody::Expr(expr) => collect_lambda_captures_expr(expr, params, out),
        LambdaBody::Block(stmts) => {
            for stmt in stmts {
                collect_lambda_captures_stmt(stmt, params, out);
            }
        }
    }
}

fn collect_lambda_captures_expr(expr: &Expr, params: &[String], out: &mut Vec<String>) {
    match expr {
        Expr::Var(name) | Expr::Move(name) | Expr::Borrow { name, .. } => {
            if !params.contains(name) {
                out.push(name.clone());
            }
        }
        Expr::ArrayLiteral(values) => {
            for value in values {
                collect_lambda_captures_expr(value, params, out);
            }
        }
        Expr::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_lambda_captures_expr(length, params, out);
            }
            for value in values {
                collect_lambda_captures_expr(value, params, out);
            }
        }
        Expr::Index { target, index } => {
            collect_lambda_captures_expr(target, params, out);
            collect_lambda_captures_expr(index, params, out);
        }
        Expr::Field { target, .. } => collect_lambda_captures_expr(target, params, out),
        Expr::IsPattern { expr, .. } | Expr::Await(expr) | Expr::Unary { expr, .. } => {
            collect_lambda_captures_expr(expr, params, out)
        }
        Expr::Throw(expr) => collect_lambda_captures_expr(expr, params, out),
        Expr::Assign { target, value } => {
            collect_lambda_captures_expr(target, params, out);
            collect_lambda_captures_expr(value, params, out);
        }
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_lambda_captures_expr(condition, params, out);
            collect_lambda_captures_expr(when_true, params, out);
            collect_lambda_captures_expr(when_false, params, out);
        }
        Expr::Binary { left, right, .. } => {
            collect_lambda_captures_expr(left, params, out);
            collect_lambda_captures_expr(right, params, out);
        }
        Expr::MethodCall { target, args, .. } => {
            collect_lambda_captures_expr(target, params, out);
            for arg in args {
                collect_lambda_captures_expr(arg, params, out);
            }
        }
        Expr::IncDec { name, .. } => {
            if !params.contains(name) {
                out.push(name.clone());
            }
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_lambda_captures_expr(arg, params, out);
            }
        }
        Expr::NewObject { args, fields, .. } => {
            for arg in args {
                collect_lambda_captures_expr(arg, params, out);
            }
            for field in fields {
                collect_lambda_captures_expr(&field.expr, params, out);
            }
        }
        Expr::Lambda { params: inner_params, body } => {
            let mut merged = params.to_vec();
            merged.extend(inner_params.iter().cloned());
            collect_lambda_captures(body, &merged, out);
        }
        Expr::Int(_)
        | Expr::Float(_)
        | Expr::Bool(_)
        | Expr::Null
        | Expr::String(_)
        | Expr::NewCollection(_)
        | Expr::NewThread(_)
        => {}
        Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
            collect_lambda_captures_expr(expr, params, out);
        }
    }
}

fn collect_lambda_captures_stmt(stmt: &Stmt, params: &[String], out: &mut Vec<String>) {
    match stmt {
        Stmt::Let { expr, .. }
        | Stmt::Assign { expr, .. }
        | Stmt::Print(expr)
        | Stmt::Expr(expr)
        | Stmt::Throw(expr) => collect_lambda_captures_expr(expr, params, out),
        Stmt::AssignTarget { target, expr } => {
            collect_lambda_captures_expr(target, params, out);
            collect_lambda_captures_expr(expr, params, out);
        }
        Stmt::Block(body) => {
            for stmt in body {
                collect_lambda_captures_stmt(stmt, params, out);
            }
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            collect_lambda_captures_expr(condition, params, out);
            for stmt in then_body {
                collect_lambda_captures_stmt(stmt, params, out);
            }
            for stmt in else_body {
                collect_lambda_captures_stmt(stmt, params, out);
            }
        }
        Stmt::Try {
            try_body,
            catch,
            finally_body,
        } => {
            for stmt in try_body {
                collect_lambda_captures_stmt(stmt, params, out);
            }
            if let Some(catch) = catch {
                for stmt in &catch.body {
                    collect_lambda_captures_stmt(stmt, params, out);
                }
            }
            for stmt in finally_body {
                collect_lambda_captures_stmt(stmt, params, out);
            }
        }
        Stmt::Switch { expr, cases, default } => {
            collect_lambda_captures_expr(expr, params, out);
            for case in cases {
                collect_lambda_captures_expr(&case.value, params, out);
                for stmt in &case.body {
                    collect_lambda_captures_stmt(stmt, params, out);
                }
            }
            for stmt in default {
                collect_lambda_captures_stmt(stmt, params, out);
            }
        }
        Stmt::While { condition, body } => {
            collect_lambda_captures_expr(condition, params, out);
            for stmt in body {
                collect_lambda_captures_stmt(stmt, params, out);
            }
        }
        Stmt::For {
            init,
            condition,
            increment,
            body,
        } => {
            if let Some(init) = init {
                collect_lambda_captures_stmt(init, params, out);
            }
            if let Some(condition) = condition {
                collect_lambda_captures_expr(condition, params, out);
            }
            if let Some(increment) = increment {
                collect_lambda_captures_stmt(increment, params, out);
            }
            for stmt in body {
                collect_lambda_captures_stmt(stmt, params, out);
            }
        }
        Stmt::ForEach { collection, body, .. } => {
            collect_lambda_captures_expr(collection, params, out);
            for stmt in body {
                collect_lambda_captures_stmt(stmt, params, out);
            }
        }
        Stmt::Return(Some(expr)) => collect_lambda_captures_expr(expr, params, out),
        Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
    }
}

pub(super) fn lower_call_args(
    args: &[Expr],
    expected_types: &[Option<IrType>],
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<Vec<TypedExpr>, String> {
    let mut lowered = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        let expected = expected_types.get(i).and_then(|x| x.as_ref());
        let typed_arg = match arg {
            Expr::Lambda { params, body } => lower_lambda(params, body, expected, env, scopes)?,
            Expr::NamedArg { name: _, expr } => {
                let inner = match expr.as_ref() {
                    Expr::Lambda { params, body } => {
                        lower_lambda(params, body, expected, env, scopes)?
                    }
                    _ => lower_typed_expr(expr, env, scopes)?,
                };
                inner
            }
            _ => lower_typed_expr(arg, env, scopes)?,
        };
        lowered.push(typed_arg);
    }
    Ok(lowered)
}

pub(super) fn pack_params_args(
    signature: &FunctionSignature,
    args: Vec<TypedExpr>,
) -> Vec<TypedExpr> {
    let Some(element_ty) = &signature.params_element_type else {
        return args;
    };
    let fixed_len = signature.params.len().saturating_sub(1);
    if args.len() == signature.params.len()
        && signature
            .params
            .last()
            .is_some_and(|expected| ir_arg_matches(expected, args.last().map(|arg| &arg.ty).unwrap()))
    {
        return args;
    }
    if args.len() < fixed_len {
        return args;
    }
    let mut packed = args[..fixed_len].to_vec();
    let tail = args[fixed_len..].to_vec();
    let array_ty = IrType::Array(Box::new(element_ty.clone()));
    packed.push(typed_expr_with_ownership(
        TypedExprKind::NewArray {
            element_type: element_ty.clone(),
            length: None,
            values: tail,
        },
        array_ty,
        Ownership::Owned,
    ));
    packed
}

fn lower_explicit_generic_args(args: &[TypeSyntax], env: &TypeEnv) -> Vec<IrType> {
    args.iter().map(|arg| type_syntax_to_ir(arg, env)).collect()
}

fn specialize_call_candidates(
    candidates: Vec<FunctionSignature>,
    generic_args: &[IrType],
    env: &TypeEnv,
    context: &str,
) -> Result<Vec<FunctionSignature>, String> {
    if generic_args.is_empty() || candidates.is_empty() {
        return Ok(candidates);
    }
    let mut specialized = Vec::new();
    let mut saw_matching_arity = false;
    for signature in candidates {
        if signature.generic_params.len() != generic_args.len() {
            continue;
        }
        saw_matching_arity = true;
        specialized.push(specialize_signature_with_explicit_generic_args(
            &signature,
            generic_args,
            env,
            context,
        )?);
    }
    if saw_matching_arity {
        Ok(specialized)
    } else {
        Ok(Vec::new())
    }
}

pub(super) fn lower_typed_expr(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedExpr, String> {
    lower_typed_expr_with_expected(expr, env, scopes, None)
}

pub(super) fn lower_typed_expr_with_expected(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
    expected: Option<&IrType>,
) -> Result<TypedExpr, String> {
    let typed = match expr {
        Expr::Int(value) => typed_expr(TypedExprKind::Int(*value), IrType::Long),
        Expr::Float(value) => typed_expr(TypedExprKind::Float(*value), IrType::Double),
        Expr::Bool(value) => typed_expr(TypedExprKind::Bool(*value), IrType::Bool),
        Expr::Null => typed_expr(TypedExprKind::Null, IrType::Unknown("null".to_string())),
        Expr::String(value) => typed_expr_with_ownership(
            TypedExprKind::String(value.clone()),
            IrType::String,
            Ownership::Owned,
        ),
        Expr::Var(name) => {
            if let Some(binding) = lookup(scopes, name) {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    binding.ty,
                    binding.ownership,
                )
            } else if let Some((this_type, field_type, field_ownership)) =
                implicit_field(env, scopes, name)
            {
                let this_expr = typed_expr_with_ownership(
                    TypedExprKind::Var("this".to_string()),
                    this_type,
                    Ownership::Borrowed,
                );
                typed_expr_with_ownership(
                    TypedExprKind::Field {
                        target: Box::new(this_expr),
                        name: name.clone(),
                    },
                    field_type.clone(),
                    field_ownership,
                )
            } else if let Some(current_type) = current_enclosing_type(scopes) {
                if let Some(signature) = env.resolve_method_call(&current_type, name, &[])?
                {
                    if signature.is_static {
                        return Ok(typed_expr_with_ownership(
                            TypedExprKind::Call(TypedCall {
                                kind: TypedCallKind::Function {
                                    name: name.clone(),
                                    symbol: signature.symbol.clone(),
                                },
                                args: Vec::new(),
                                generic_args: Vec::new(),
                            }),
                            signature.return_type.clone(),
                            signature.return_ownership.clone(),
                        ));
                    }
                }
                if let Some(signature) = env.resolve_method(&current_type, &format!("get_{name}"), &[]) {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Call(TypedCall {
                            kind: TypedCallKind::Function {
                                name: format!("get_{name}"),
                                symbol: signature.symbol.clone(),
                            },
                            args: Vec::new(),
                            generic_args: Vec::new(),
                        }),
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                    ));
                }
                if let Some(initializer) =
                    env.static_fields.get(&(current_type.clone(), name.clone()))
                {
                    return lower_typed_expr_with_expected(initializer, env, scopes, expected);
                }
                if let Some(kind) = env.kinds.get(name) {
                    let ty = match kind {
                        TypeKind::Class => IrType::Class(name.clone()),
                        TypeKind::Interface => IrType::Interface(name.clone()),
                        TypeKind::Enum => IrType::Int,
                        _ => IrType::Struct(name.clone()),
                    };
                    typed_expr_with_ownership(TypedExprKind::Var(name.clone()), ty, Ownership::Shared)
                } else if name == "Exception" || name == "System.Exception" {
                    typed_expr_with_ownership(
                        TypedExprKind::Var(name.clone()),
                        IrType::Exception,
                        Ownership::Shared,
                    )
                } else if let Some(signature) = env.single_function(name) {
                    typed_expr_with_ownership(
                        TypedExprKind::FunctionSymbol(name.clone()),
                        IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        },
                        Ownership::Copy,
                    )
                } else {
                    typed_expr_with_ownership(
                        TypedExprKind::Var(name.clone()),
                        IrType::Unknown(name.clone()),
                        Ownership::Shared,
                    )
                }
            } else if let Some(kind) = env.kinds.get(name) {
                let ty = match kind {
                    TypeKind::Class => IrType::Class(name.clone()),
                    TypeKind::Interface => IrType::Interface(name.clone()),
                    TypeKind::Enum => IrType::Int,
                    _ => IrType::Struct(name.clone()),
                };
                typed_expr_with_ownership(TypedExprKind::Var(name.clone()), ty, Ownership::Shared)
            } else if name == "Exception" || name == "System.Exception" {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    IrType::Exception,
                    Ownership::Shared,
                )
            } else if let Some(signature) = env.single_function(name) {
                typed_expr_with_ownership(
                    TypedExprKind::FunctionSymbol(name.clone()),
                    IrType::Function {
                        params: signature.params.clone(),
                        return_type: Box::new(signature.return_type.clone()),
                    },
                    Ownership::Copy,
                )
            } else {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    IrType::Unknown(name.clone()),
                    Ownership::Shared,
                )
            }
        }
        Expr::Move(name) => {
            let ty = lookup(scopes, name)
                .map(|binding| binding.ty)
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            typed_expr_with_ownership(TypedExprKind::Move(name.clone()), ty, Ownership::Moved)
        }
        Expr::ArrayLiteral(values) => {
            let values = values
                .iter()
                .map(|value| lower_typed_expr(value, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            let element_ty = values
                .first()
                .map(|value| value.ty.clone())
                .unwrap_or(IrType::Long);
            typed_expr_with_ownership(
                TypedExprKind::ArrayLiteral(values),
                IrType::Array(Box::new(element_ty)),
                Ownership::Owned,
            )
        }
        Expr::NewArray {
            element_type,
            length,
            values,
        } => {
            let values = values
                .iter()
                .map(|value| lower_typed_expr(value, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            let element_type = if matches!(element_type, TypeSyntax::Named(name) if name == "var") {
                values
                    .first()
                    .map(|value| value.ty.clone())
                    .unwrap_or(IrType::Long)
            } else {
                type_syntax_to_ir(element_type, env)
            };
            let length = length
                .as_ref()
                .map(|length| lower_typed_expr(length, env, scopes))
                .transpose()?
                .map(Box::new);
            typed_expr_with_ownership(
                TypedExprKind::NewArray {
                    element_type: element_type.clone(),
                    length,
                    values,
                },
                IrType::Array(Box::new(element_type)),
                Ownership::Owned,
            )
        }
        Expr::Index { target, index } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let index = lower_typed_expr(index, env, scopes)?;
            if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name)
            | IrType::Unknown(type_name) = &target.ty
            {
                let simple = base_type_name(type_name);
                let index_args = vec![index.clone()];
                let resolved = if matches!(simple, "string" | "String") {
                    None
                } else {
                    env.resolve_method_call(type_name, "get_Item", &index_args)?
                        .or_else(|| {
                            env.resolve_extension_method_call(
                                type_name,
                                "get_Item",
                                &target,
                                &index_args,
                            )
                            .ok()
                            .flatten()
                        })
                };
                if let Some(signature) = resolved {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Call(TypedCall {
                            kind: TypedCallKind::Method {
                                target: Box::new(target),
                                name: "get_Item".to_string(),
                                symbol: signature.symbol.clone(),
                                resolution: CallResolution::InstanceMethod,
                            },
                            args: index_args,
                            generic_args: Vec::new(),
                        }),
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                    ));
                }
            }
            let ty = match &target.ty {
                IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => {
                    inner.as_ref().clone()
                }
                IrType::Dictionary(_, value) => value.as_ref().clone(),
                IrType::String => IrType::Byte,
                _ => IrType::Unknown("index".to_string()),
            };
            let ownership = if ty == IrType::String {
                Ownership::Borrowed
            } else {
                ownership_for_type(&ty)
            };
            typed_expr_with_ownership(
                TypedExprKind::Index {
                    target: Box::new(target),
                    index: Box::new(index),
                },
                ty,
                ownership,
            )
        }
        Expr::Field { target, name } => {
            let target = lower_typed_expr(target, env, scopes)?;
            if let TypedExprKind::Var(enum_name) = &target.kind {
                if let Some(value) = env.enum_values.get(&(enum_name.clone(), name.clone())) {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Int(*value),
                        IrType::Int,
                        Ownership::Copy,
                    ));
                }
            }
            if let IrType::Interface(type_name) = &target.ty {
                if base_type_name(type_name) == "IEnumerator" && name == "Current" {
                    if let Some(item_ty) = split_monomorphized_type(type_name)
                        .and_then(|(_, args)| args.first().cloned())
                        .and_then(|arg| parse_monomorphized_ir_type(&arg, env))
                    {
                        return Ok(typed_expr_with_ownership(
                            TypedExprKind::Field {
                                target: Box::new(target),
                                name: name.clone(),
                            },
                            item_ty.clone(),
                            ownership_for_type(&item_ty),
                        ));
                    }
                }
            }
            if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                let getter_name = format!("get_{name}");
                if let Some(signature) = env.resolve_method(type_name, &getter_name, &[]) {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Call(TypedCall {
                            kind: TypedCallKind::Method {
                                target: Box::new(target),
                                name: getter_name,
                                symbol: signature.symbol.clone(),
                                resolution: CallResolution::InstanceMethod,
                            },
                            args: Vec::new(),
                            generic_args: Vec::new(),
                        }),
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                    ));
                }
            }
            let field_info = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), field) | (IrType::Struct(type_name), field) => {
                    env.resolve_field(type_name, field)
                }
                (IrType::Unknown(type_name), field) => env.resolve_field(type_name, field),
                (IrType::List(_), "Count")
                | (IrType::Dictionary(_, _), "Count")
                | (IrType::Array(_), "Length" | "Count") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Interface(type_name), "Current")
                    if base_type_name(type_name) == "IEnumerator" =>
                {
                    split_monomorphized_type(type_name)
                        .and_then(|(_, args)| args.first().cloned())
                        .and_then(|arg| parse_monomorphized_ir_type(&arg, env))
                        .map(|ty| FieldSignature {
                            package_id: None,
                            visibility: Visibility::Public,
                            ownership: ownership_for_type(&ty),
                            ty,
                        })
                }
                (IrType::String, "Length") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Task(inner), "Result") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: inner.as_ref().clone(),
                    ownership: ownership_for_type(inner),
                }),
                (IrType::Task(_), "IsCompleted")
                | (IrType::Task(_), "IsCompletedSuccessfully")
                | (IrType::Task(_), "IsFaulted") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Bool,
                    ownership: Ownership::Copy,
                }),
                (IrType::Task(_), "Exception") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Exception,
                    ownership: Ownership::Owned,
                }),
                (IrType::Exception, "Message") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::String,
                    ownership: Ownership::Borrowed,
                }),
                (IrType::Weak(inner), "Target") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: inner.as_ref().clone(),
                    ownership: Ownership::Borrowed,
                }),
                _ => None,
            };
            let (ty, ownership) = field_info
                .map(|field| (field.ty, field.ownership))
                .unwrap_or_else(|| {
                    let ty = IrType::Unknown(name.clone());
                    let ownership = ownership_for_type(&ty);
                    (ty, ownership)
                });
            typed_expr_with_ownership(
                TypedExprKind::Field {
                    target: Box::new(target),
                    name: name.clone(),
                },
                ty,
                ownership,
            )
        }
        Expr::IsPattern { expr, ty, name } => {
            let expr = lower_typed_expr(expr, env, scopes)?;
            let pattern_type = type_syntax_to_ir(ty, env);
            let binding = name.as_ref().map(|name| TypedBinding {
                name: name.clone(),
                ty: pattern_type,
                ownership: Ownership::Borrowed,
            });
            typed_expr(
                TypedExprKind::IsPattern {
                    expr: Box::new(expr),
                    binding,
                },
                IrType::Bool,
            )
        }
        Expr::MethodCall { target, name, generic_args: explicit_generic_types, args } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let explicit_generic_args = lower_explicit_generic_args(explicit_generic_types, env);
            let mut candidates = Vec::new();
            if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name)
            | IrType::Unknown(type_name) = &target.ty
            {
                if let Some(sigs) = env.methods.get(&(type_name.clone(), name.clone())) {
                    candidates = sigs.clone();
                }
                if candidates.is_empty() {
                    let short = base_type_name(type_name);
                    if let Some(sigs) = env.methods.get(&(short.to_string(), name.clone())) {
                        candidates = sigs.clone();
                    }
                }
                if candidates.is_empty() {
                    candidates = env
                        .methods
                        .iter()
                        .filter(|((owner, method_name), _)| {
                            method_name == name && base_type_name(owner) == base_type_name(type_name)
                        })
                        .flat_map(|(_, signatures)| signatures.clone())
                        .collect();
                }
            }
            if candidates.is_empty() {
                if let Some(sigs) = env.functions.get(name) {
                    candidates = sigs.clone();
                }
            }
            let candidates = specialize_call_candidates(
                candidates,
                &explicit_generic_args,
                env,
                &format!("call to '{name}'"),
            )?;
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            let (ty, _ownership, symbol, resolution) =
                resolve_method_call(env, &target, name, &explicit_generic_args, &args)?;
            let resolved_signature = candidates
                .iter()
                .find(|signature| signature.symbol == symbol)
                .cloned()
                .or_else(|| match &target.ty {
                    IrType::Class(type_name)
                    | IrType::Struct(type_name)
                    | IrType::Interface(type_name)
                    | IrType::Unknown(type_name) => env
                        .resolve_method_call_with_generic_args(type_name, name, &args, &explicit_generic_args)
                        .ok()
                        .flatten(),
                    IrType::String => ["string", "String", "System.String"]
                        .iter()
                        .find_map(|string_type| {
                            env.resolve_method_call_with_generic_args(
                                string_type,
                                name,
                                &args,
                                &explicit_generic_args,
                            )
                            .ok()
                            .flatten()
                        }),
                    _ => None,
                })
                .or_else(|| {
                    env.resolve_function_call_with_generic_args(name, &args, &explicit_generic_args)
                        .ok()
                        .flatten()
                });
            let generic_args = if explicit_generic_args.is_empty() {
                resolved_signature.as_ref().map(|signature| {
                    infer_generic_args_from_signature_with_expected(signature, &args, expected)
                })
                .unwrap_or_default()
            } else {
                explicit_generic_args.clone()
            };
            let ty = resolved_signature
                .as_ref()
                .map(|signature| {
                    substitute_generic_args_in_ir_type(
                        &signature.return_type,
                        &signature.generic_params,
                        &generic_args,
                    )
                })
                .unwrap_or(ty);
            let ownership = ownership_for_type(&ty);
            let args = if let Some(signature) = resolved_signature.as_ref() {
                pack_params_args(signature, args)
            } else {
                args
            };
            typed_expr_with_ownership(
                TypedExprKind::Call(TypedCall {
                    kind: TypedCallKind::Method {
                        target: Box::new(target),
                        name: name.clone(),
                        symbol,
                        resolution,
                    },
                    args,
                    generic_args,
                }),
                ty,
                ownership,
            )
        }
        Expr::FunctionCall { name, generic_args: explicit_generic_types, args } => {
            if name == "sizeof" {
                let type_name = if let Some(Expr::Var(tn)) = args.first() {
                    tn.clone()
                } else {
                    "int".to_string()
                };
                return Ok(typed_expr(
                    TypedExprKind::Call(TypedCall {
                        kind: TypedCallKind::Function {
                            name: name.clone(),
                            symbol: name.clone(),
                        },
                        args: vec![typed_expr(
                            TypedExprKind::Var(type_name.clone()),
                            IrType::Unknown(type_name),
                        )],
                        generic_args: Vec::new(),
                    }),
                    IrType::Int,
                ));
            }
            let mut candidates = Vec::new();
            if let Some(sigs) = env.functions.get(name) {
                candidates = sigs.clone();
            }
            let current_type = current_enclosing_type(scopes);
            if let Some(current_type) = current_type.as_ref() {
                if let Some(sigs) = env.methods.get(&(current_type.clone(), name.clone())) {
                    candidates.extend(sigs.clone());
                }
            }
            let explicit_generic_args = lower_explicit_generic_args(explicit_generic_types, env);
            let candidates = specialize_call_candidates(
                candidates,
                &explicit_generic_args,
                env,
                &format!("call to '{name}'"),
            )?;
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            let method_signature = current_type.as_ref().and_then(|current_type| {
                env.resolve_method_call_with_generic_args(
                    current_type,
                    name,
                    &args,
                    &explicit_generic_args,
                )
                .ok()
                .flatten()
            });
            let function_signature = if method_signature.is_none() {
                env.resolve_function_call_with_generic_args(name, &args, &explicit_generic_args)?
            } else {
                None
            };
            let signature = method_signature.as_ref().or(function_signature.as_ref());
            let generic_args = if explicit_generic_args.is_empty() {
                signature.map(|signature| {
                    infer_generic_args_from_signature_with_expected(signature, &args, expected)
                })
                .unwrap_or_default()
            } else {
                explicit_generic_args.clone()
            };
            let args = if let Some(signature) = signature {
                pack_params_args(signature, args)
            } else {
                args
            };
            let ty = signature
                .map(|signature| {
                    substitute_generic_args_in_ir_type(
                        &signature.return_type,
                        &signature.generic_params,
                        &generic_args,
                    )
                })
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            let ownership = ownership_for_type(&ty);
            typed_expr_with_ownership(
                TypedExprKind::Call(TypedCall {
                    kind: if let Some(signature) = method_signature {
                        if signature.is_static {
                            TypedCallKind::Function {
                                name: name.clone(),
                                symbol: signature.symbol.clone(),
                            }
                        } else {
                            let current_this = lookup(scopes, "this");
                            TypedCallKind::Method {
                                target: Box::new(typed_expr(
                                    TypedExprKind::Var("this".to_string()),
                                    current_this
                                        .as_ref()
                                        .map(|binding| binding.ty.clone())
                                        .unwrap_or_else(|| IrType::Unknown("this".to_string())),
                                )),
                                name: name.clone(),
                                symbol: signature.symbol.clone(),
                                resolution: CallResolution::InstanceMethod,
                            }
                        }
                    } else {
                        TypedCallKind::Function {
                            name: name.clone(),
                            symbol: function_signature
                                .as_ref()
                                .map(|signature| signature.symbol.clone())
                                .unwrap_or_else(|| name.clone()),
                        }
                    },
                    args,
                    generic_args,
                }),
                ty,
                ownership,
            )
        }
        Expr::NewObject {
            type_name,
            args,
            fields,
        } => {
            let mut candidates = Vec::new();
            if let Some(sigs) = env.constructors.get(type_name) {
                candidates = sigs.clone();
            }
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            if let Some((base, generic_args)) = split_monomorphized_type(type_name) {
                if base_type_name(base) == "Rc" {
                    let inner_ty = generic_args
                        .first()
                        .and_then(|arg| parse_monomorphized_ir_type(arg, env))
                        .unwrap_or(IrType::Unknown("T".to_string()));
                    let runtime_name = rc_runtime_type_name(&inner_ty);
                    let ty = IrType::Struct(runtime_name.clone());
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::NewObject {
                            type_name: runtime_name,
                            constructor: None,
                            args,
                            fields: Vec::new(),
                        },
                        ty.clone(),
                        ownership_for_type(&ty),
                    ));
                }
            }
            if is_weak_reference_type_name(type_name) {
                let inner_ty = if let Some(arg) = args.first() {
                    arg.ty.clone()
                } else {
                    IrType::Unknown("T".to_string())
                };
                let ty = IrType::Weak(Box::new(inner_ty));
                return Ok(typed_expr_with_ownership(
                    TypedExprKind::NewObject {
                        type_name: type_name.clone(),
                        constructor: None,
                        args,
                        fields: Vec::new(),
                    },
                    ty.clone(),
                    Ownership::Copy,
                ));
            }
            let constructor_signature = env.resolve_constructor_call(type_name, &args)?;
            let args = if let Some(signature) = constructor_signature.as_ref() {
                pack_params_args(signature, args)
            } else {
                args
            };
            let constructor = constructor_signature.map(|signature| signature.symbol.clone());
            let fields = fields
                .iter()
                .map(|field| {
                    Ok(TypedFieldInit {
                        name: field.name.clone(),
                        expr: lower_typed_expr(&field.expr, env, scopes)?,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let fields = if type_name == "Type" {
                augment_type_metadata_fields(fields, env)?
            } else {
                fields
            };
            let ty = if type_name == "__anonymous" {
                IrType::Unknown("anonymous".to_string())
            } else {
                match env.kinds.get(type_name) {
                    Some(TypeKind::Class) => IrType::Class(type_name.clone()),
                    Some(TypeKind::Interface) => IrType::Interface(type_name.clone()),
                    _ if type_name == "Exception" || type_name == "System.Exception" => {
                        IrType::Exception
                    }
                    _ => IrType::Struct(type_name.clone()),
                }
            };
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: type_name.clone(),
                    constructor,
                    args,
                    fields,
                },
                ty.clone(),
                ownership_for_type(&ty),
            )
        }
        Expr::NewCollection(ty) => {
            let ty = type_syntax_to_ir(ty, env);
            typed_expr_with_ownership(
                TypedExprKind::NewCollection(ty.clone()),
                ty,
                Ownership::Owned,
            )
        }
        Expr::NewThread(entry) => typed_expr_with_ownership(
            TypedExprKind::NewThread(entry.clone()),
            IrType::Thread,
            Ownership::Owned,
        ),
        Expr::Borrow { mutable, name } => {
            let target = lookup(scopes, name)
                .map(|binding| binding.ty)
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            typed_expr_with_ownership(
                TypedExprKind::Borrow {
                    mutable: *mutable,
                    name: name.clone(),
                },
                IrType::Ref(Box::new(target)),
                Ownership::Borrowed,
            )
        }
        Expr::Await(inner) => {
            let inner = lower_typed_expr(inner, env, scopes)?;
            let ty = match &inner.ty {
                IrType::Task(result) => result.as_ref().clone(),
                other => IrType::Unknown(format!("await {other:?}")),
            };
            typed_expr(TypedExprKind::Await(Box::new(inner)), ty)
        }
        Expr::Throw(expr) => {
            let inner = lower_typed_expr_with_expected(expr, env, scopes, expected)?;
            let ty = expected.cloned().unwrap_or_else(|| inner.ty.clone());
            typed_expr_with_ownership(
                TypedExprKind::Throw(Box::new(inner)),
                ty,
                Ownership::Copy,
            )
        }
        Expr::Assign { target, value } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let value = lower_typed_expr_with_expected(value, env, scopes, Some(&target.ty))?;
            typed_expr_with_ownership(
                TypedExprKind::Assign {
                    target: Box::new(target),
                    value: Box::new(value.clone()),
                },
                value.ty.clone(),
                value.ownership.clone(),
            )
        }
        Expr::Unary { op, expr } => {
            let expr = lower_typed_expr(expr, env, scopes)?;
            let ty = match op {
                UnaryOp::Not => IrType::Bool,
                UnaryOp::Neg => expr.ty.clone(),
            };
            typed_expr(
                TypedExprKind::Unary {
                    op: *op,
                    expr: Box::new(expr),
                },
                ty,
            )
        }
        Expr::IncDec {
            name,
            delta,
            prefix,
        } => {
            let target = if let Some((this_type, field_type, field_ownership)) =
                implicit_field(env, scopes, name)
            {
                let this_expr = typed_expr_with_ownership(
                    TypedExprKind::Var("this".to_string()),
                    this_type,
                    Ownership::Borrowed,
                );
                typed_expr_with_ownership(
                    TypedExprKind::Field {
                        target: Box::new(this_expr),
                        name: name.clone(),
                    },
                    field_type.clone(),
                    field_ownership,
                )
            } else {
                let ty = lookup(scopes, name)
                    .map(|binding| binding.ty.clone())
                    .unwrap_or_else(|| IrType::Unknown(name.clone()));
                let binding = lookup(scopes, name);
                let ownership = binding.map(|b| b.ownership).unwrap_or(Ownership::Shared);
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    ty,
                    ownership,
                )
            };
            let ty = target.ty.clone();
            typed_expr(
                TypedExprKind::IncDec {
                    target: Box::new(target),
                    delta: *delta,
                    prefix: *prefix,
                },
                ty,
            )
        }
        Expr::Lambda { params, body } => lower_lambda(params, body, expected, env, scopes)?,
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            let condition = lower_typed_expr(condition, env, scopes)?;
            let when_true = lower_typed_expr(when_true, env, scopes)?;
            let when_false = lower_typed_expr(when_false, env, scopes)?;
            let ty = if when_true.ty == IrType::Unknown("null".to_string()) {
                when_false.ty.clone()
            } else {
                when_true.ty.clone()
            };
            typed_expr_with_ownership(
                TypedExprKind::Conditional {
                    condition: Box::new(condition),
                    when_true: Box::new(when_true),
                    when_false: Box::new(when_false),
                },
                ty.clone(),
                ownership_for_type(&ty),
            )
        }
        Expr::Binary { left, op, right } => {
            let left = lower_typed_expr(left, env, scopes)?;
            let right = if *op == BinaryOp::Coalesce {
                lower_typed_expr_with_expected(right, env, scopes, Some(&left.ty))?
            } else {
                lower_typed_expr(right, env, scopes)?
            };
            let ty = if op.is_comparison() {
                IrType::Bool
            } else {
                left.ty.clone()
            };
            typed_expr(
                TypedExprKind::Binary {
                    left: Box::new(left),
                    op: *op,
                    right: Box::new(right),
                },
                ty,
            )
        }
        Expr::NamedArg { expr, .. } => lower_typed_expr(expr, env, scopes)?,
        Expr::RefArg {
            modifier,
            expr: argument,
        } => {
            if matches!(
                modifier,
                ParamModifier::Out | ParamModifier::Ref | ParamModifier::In
            ) {
                if let Expr::Var(name) = argument.as_ref() {
                    let target = lookup(scopes, name).unwrap_or(TypedBinding {
                        name: name.clone(),
                        ty: IrType::Unknown(format!("out {name}")),
                        ownership: Ownership::Shared,
                    });
                    typed_expr_with_ownership(
                        TypedExprKind::Borrow {
                            mutable: !matches!(modifier, ParamModifier::In),
                            name: name.clone(),
                        },
                        IrType::Ref(Box::new(target.ty)),
                        Ownership::Borrowed,
                    )
                } else {
                    lower_typed_expr(argument, env, scopes)?
                }
            } else {
                lower_typed_expr(argument, env, scopes)?
            }
        }
    };
    Ok(coerce_for_expected_type(typed, expected))
}

pub(super) fn coerce_for_expected_type(mut typed: TypedExpr, expected: Option<&IrType>) -> TypedExpr {
    let Some(expected) = expected else {
        return typed;
    };
    match expected {
        IrType::Nullable(inner) => {
            if matches!(typed.ty, IrType::Nullable(_)) {
                return typed;
            }
            if matches!(typed.ty, IrType::Unknown(ref name) if name == "null") {
                typed.ty = expected.clone();
                typed.ownership = Ownership::Shared;
                typed.drop_kind = drop_kind_for_type(expected, &typed.ownership);
                return typed;
            }
            if is_nullable_value_type(inner) && ir_arg_matches(inner, &typed.ty) {
                let wrapped = TypedExpr {
                    kind: TypedExprKind::NullableSome(Box::new(typed)),
                    ty: expected.clone(),
                    ownership: Ownership::Shared,
                    drop_kind: drop_kind_for_type(expected, &Ownership::Shared),
                };
                return wrapped;
            }
            typed
        }
        _ => typed,
    }
}

pub(super) fn lower_expr(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedBinding, String> {
    let binding = match expr {
        Expr::Int(_) => typed_temp(IrType::Long),
        Expr::Float(_) => typed_temp(IrType::Double),
        Expr::Bool(_) => typed_temp(IrType::Bool),
        Expr::Null => typed_temp(IrType::Unknown("null".to_string())),
        Expr::String(_) => TypedBinding {
            name: "<expr>".to_string(),
            ty: IrType::String,
            ownership: Ownership::Owned,
        },
        Expr::Var(name) => lookup(scopes, name)
            .or_else(|| {
                if let Some(kind) = env.kinds.get(name) {
                    let ty = match kind {
                        TypeKind::Class => IrType::Class(name.clone()),
                        TypeKind::Interface => IrType::Interface(name.clone()),
                        TypeKind::Enum => IrType::Int,
                        _ => IrType::Struct(name.clone()),
                    };
                    Some(TypedBinding {
                        name: name.clone(),
                        ty,
                        ownership: Ownership::Shared,
                    })
                } else if name == "Exception" || name == "System.Exception" {
                    Some(TypedBinding {
                        name: name.clone(),
                        ty: IrType::Exception,
                        ownership: Ownership::Shared,
                    })
                } else {
                    env.single_function(name).map(|signature| TypedBinding {
                        name: name.clone(),
                        ty: IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        },
                        ownership: Ownership::Copy,
                    })
                }
            })
            .unwrap_or(TypedBinding {
                name: name.clone(),
                ty: IrType::Unknown(name.clone()),
                ownership: Ownership::Shared,
            }),
        Expr::Move(name) => {
            let mut binding = lookup(scopes, name).unwrap_or(TypedBinding {
                name: name.clone(),
                ty: IrType::Unknown(name.clone()),
                ownership: Ownership::Moved,
            });
            binding.ownership = Ownership::Moved;
            binding
        }
        Expr::ArrayLiteral(values) => {
            let mut lowered = Vec::with_capacity(values.len());
            for value in values {
                lowered.push(lower_expr(value, env, scopes)?);
            }
            let element_ty = lowered
                .first()
                .map(|value| value.ty.clone())
                .unwrap_or(IrType::Long);
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Array(Box::new(element_ty)),
                ownership: Ownership::Owned,
            }
        }
        Expr::NewArray {
            element_type,
            length,
            values,
        } => {
            let mut lowered = Vec::with_capacity(values.len());
            if let Some(length) = length {
                lower_expr(length, env, scopes)?;
            }
            for value in values {
                lowered.push(lower_expr(value, env, scopes)?);
            }
            let inferred_element = if matches!(element_type, TypeSyntax::Named(name) if name == "var") {
                lowered.first().map(|value| value.ty.clone()).unwrap_or(IrType::Long)
            } else {
                type_syntax_to_ir(element_type, env)
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Array(Box::new(inferred_element)),
                ownership: Ownership::Owned,
            }
        }
        Expr::Index { target, index } => {
            let target = lower_expr(target, env, scopes)?;
            lower_expr(index, env, scopes)?;
            let ty = match target.ty {
                IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => *inner,
                IrType::Dictionary(_, value) => *value,
                _ => IrType::Unknown("index".to_string()),
            };
            let ownership = if ty == IrType::String {
                Ownership::Borrowed
            } else {
                ownership_for_type(&ty)
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ty,
                ownership,
            }
        }
        Expr::Field { target, name } => {
            let target = lower_expr(target, env, scopes)?;
            if target.ty == IrType::Int {
                if let Some(value) = env.enum_values.get(&(target.name.clone(), name.clone())) {
                    let _ = value;
                    return Ok(TypedBinding {
                        name: "<expr>".to_string(),
                        ty: IrType::Int,
                        ownership: Ownership::Copy,
                    });
                }
            }
            let field_info = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), field) | (IrType::Struct(type_name), field) => {
                    env.resolve_field(type_name, field)
                }
                (IrType::List(_), "Count")
                | (IrType::Dictionary(_, _), "Count")
                | (IrType::Array(_), "Length" | "Count") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Interface(type_name), "Current")
                    if base_type_name(type_name) == "IEnumerator" =>
                {
                    split_monomorphized_type(type_name)
                        .and_then(|(_, args)| args.first().cloned())
                        .and_then(|arg| parse_monomorphized_ir_type(&arg, env))
                        .map(|ty| FieldSignature {
                            package_id: None,
                            visibility: Visibility::Public,
                            ownership: ownership_for_type(&ty),
                            ty,
                        })
                }
                (IrType::String, "Length") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Task(inner), "Result") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: inner.as_ref().clone(),
                    ownership: ownership_for_type(inner),
                }),
                (IrType::Task(_), "IsCompleted") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::Bool,
                    ownership: Ownership::Copy,
                }),
                (IrType::Exception, "Message") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: IrType::String,
                    ownership: Ownership::Borrowed,
                }),
                (IrType::Weak(inner), "Target") => Some(FieldSignature {
                    package_id: None,
                    visibility: Visibility::Public,
                    ty: inner.as_ref().clone(),
                    ownership: Ownership::Borrowed,
                }),
                _ => None,
            };
            let (ty, ownership) = field_info
                .map(|field| (field.ty, field.ownership))
                .unwrap_or_else(|| {
                    let ty = IrType::Unknown(name.clone());
                    let ownership = ownership_for_type(&ty);
                    (ty, ownership)
                });
            TypedBinding {
                name: "<expr>".to_string(),
                ty,
                ownership,
            }
        }
        Expr::IsPattern { expr, .. } => {
            lower_expr(expr, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Bool,
                ownership: Ownership::Copy,
            }
        }
        Expr::MethodCall { target, name, generic_args, args } => {
            let target = lower_expr(target, env, scopes)?;
            let args = args
                .iter()
                .map(|arg| lower_expr(arg, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            let explicit_generic_args = lower_explicit_generic_args(generic_args, env);
            let ty = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), "MapGet" | "MapPost")
                    if type_name == "WebApplication" =>
                {
                    IrType::Void
                }
                (IrType::Task(inner), "GetResult") => inner.as_ref().clone(),
                (IrType::Task(inner), "GetAwaiter") => IrType::Task(inner.clone()),
                (_, "Contains") | (_, "ContainsKey") | (_, "Remove") | (_, "TryGetValue") => IrType::Bool,
                (_, "Add") | (_, "Clear") | (_, "Wait") => IrType::Void,
                (IrType::Unknown(target) | IrType::Class(target), "Run") if target == "Task" => {
                    args.first()
                        .map(|arg| {
                            let result_ty = match &arg.ty {
                                IrType::Function { return_type, .. } => return_type.as_ref().clone(),
                                _ => arg.ty.clone(),
                            };
                            IrType::Task(Box::new(result_ty))
                        })
                        .unwrap_or_else(|| IrType::Task(Box::new(IrType::Void)))
                }
                (IrType::Unknown(target) | IrType::Class(target), "ReadAllText")
                    if target == "File" || target == "System.IO.File" =>
                {
                    IrType::String
                }
                (IrType::Unknown(target) | IrType::Class(target), "WriteAllText")
                    if target == "File" || target == "System.IO.File" =>
                {
                    IrType::Void
                }
                _ => IrType::Unknown(name.clone()),
            };
            let ownership = if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                env.resolve_method_call_with_generic_args(
                    type_name,
                    name,
                    &args
                        .iter()
                        .map(|arg| TypedExpr {
                            kind: TypedExprKind::Var(arg.name.clone()),
                            ty: arg.ty.clone(),
                            ownership: arg.ownership.clone(),
                            drop_kind: arg.drop_kind(),
                        })
                        .collect::<Vec<_>>(),
                    &explicit_generic_args,
                )
                    .ok()
                    .flatten()
                    .map(|signature| signature.return_ownership.clone())
                    .unwrap_or_else(|| ownership_for_type(&ty))
            } else {
                ownership_for_type(&ty)
            };
            let ty = if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                env.resolve_method_call_with_generic_args(
                    type_name,
                    name,
                    &args
                        .iter()
                        .map(|arg| TypedExpr {
                            kind: TypedExprKind::Var(arg.name.clone()),
                            ty: arg.ty.clone(),
                            ownership: arg.ownership.clone(),
                            drop_kind: arg.drop_kind(),
                        })
                        .collect::<Vec<_>>(),
                    &explicit_generic_args,
                )
                .ok()
                .flatten()
                .map(|signature| signature.return_type.clone())
                .unwrap_or(ty)
            } else {
                ty
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership,
                ty,
            }
        }
        Expr::FunctionCall { name, generic_args, args } => {
            if name == "sizeof" {
                return Ok(TypedBinding {
                    name: "<expr>".to_string(),
                    ownership: Ownership::Copy,
                    ty: IrType::Int,
                });
            }
            let args = args
                .iter()
                .map(|arg| lower_expr(arg, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            let explicit_generic_args = lower_explicit_generic_args(generic_args, env);
            let signature = env
                .resolve_function_call_with_generic_args(
                    name,
                    &args
                        .iter()
                        .map(|arg| TypedExpr {
                            kind: TypedExprKind::Var(arg.name.clone()),
                            ty: arg.ty.clone(),
                            ownership: arg.ownership.clone(),
                            drop_kind: arg.drop_kind(),
                        })
                        .collect::<Vec<_>>(),
                    &explicit_generic_args,
                )
                .ok()
                .flatten();
            let ty = signature
                .as_ref()
                .map(|signature| signature.return_type.clone())
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: signature
                    .as_ref()
                    .map(|signature| signature.return_ownership.clone())
                    .unwrap_or_else(|| Ownership::Shared),
                ty,
            }
        }
        Expr::NewObject {
            type_name,
            args,
            fields,
        } => {
            for arg in args {
                lower_expr(arg, env, scopes)?;
            }
            for field in fields {
                lower_expr(&field.expr, env, scopes)?;
            }
            let ty = match env.kinds.get(type_name) {
                Some(TypeKind::Class) => IrType::Class(type_name.clone()),
                _ if type_name == "Exception" || type_name == "System.Exception" => {
                    IrType::Exception
                }
                _ => IrType::Struct(type_name.clone()),
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: ownership_for_type(&ty),
                ty,
            }
        }
        Expr::NewCollection(ty) => {
            let ty = type_syntax_to_ir(ty, env);
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: Ownership::Owned,
                ty,
            }
        }
        Expr::NewThread(_) => TypedBinding {
            name: "<expr>".to_string(),
            ty: IrType::Thread,
            ownership: Ownership::Owned,
        },
        Expr::Borrow { name, .. } => {
            let target =
                lookup(scopes, name).unwrap_or_else(|| typed_temp(IrType::Unknown(name.clone())));
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Ref(Box::new(target.ty)),
                ownership: Ownership::Borrowed,
            }
        }
        Expr::Await(inner) => {
            let inner = lower_expr(inner, env, scopes)?;
            let ty = match inner.ty {
                IrType::Task(result) => *result,
                other => IrType::Unknown(format!("await {other:?}")),
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: ownership_for_type(&ty),
                ty,
            }
        }
        Expr::Throw(expr) => {
            lower_expr(expr, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Unknown("throw".to_string()),
                ownership: Ownership::Copy,
            }
        }
        Expr::Assign { target, value } => {
            lower_expr(target, env, scopes)?;
            let value = lower_expr(value, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: value.ty,
                ownership: value.ownership,
            }
        }
        Expr::Unary { expr, .. } => {
            lower_expr(expr, env, scopes)?;
            typed_temp(IrType::Bool)
        }
        Expr::Lambda { body, .. } => {
            match body {
                LambdaBody::Expr(body) => {
                    lower_expr(body, env, scopes)?;
                }
                LambdaBody::Block(stmts) => {
                    let mut lambda_scopes = scopes.to_vec();
                    let mut lambda_locals = Vec::new();
                    lower_stmts(stmts, env, &mut lambda_scopes, &mut lambda_locals)?;
                }
            }
            typed_temp(IrType::Unknown("lambda".to_string()))
        }
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            lower_expr(condition, env, scopes)?;
            let when_true = lower_expr(when_true, env, scopes)?;
            let when_false = lower_expr(when_false, env, scopes)?;
            if when_true.ty == IrType::Unknown("null".to_string()) {
                typed_temp(when_false.ty)
            } else {
                typed_temp(when_true.ty)
            }
        }
        Expr::Binary { left, op, right } => {
            let left = lower_expr(left, env, scopes)?;
            lower_expr(right, env, scopes)?;
            let ty = if op.is_comparison() {
                IrType::Bool
            } else {
                left.ty
            };
            typed_temp(ty)
        }
        Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => lower_expr(expr, env, scopes)?,
        Expr::IncDec { name, .. } => {
            let ty = if let Some((_, field_ty, _)) = implicit_field(env, scopes, name) {
                field_ty
            } else {
                lookup(scopes, name)
                    .map(|binding| binding.ty)
                    .unwrap_or_else(|| IrType::Unknown(name.clone()))
            };
            typed_temp(ty)
        }
    };
    Ok(binding)
}

pub(super) fn lookup(scopes: &[HashMap<String, TypedBinding>], name: &str) -> Option<TypedBinding> {
    scopes
        .iter()
        .rev()
        .find_map(|scope| scope.get(name).cloned())
}

pub(super) fn current_enclosing_type(scopes: &[HashMap<String, TypedBinding>]) -> Option<String> {
    let binding = lookup(scopes, "__glitching_current_type")?;
    match binding.ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => Some(name),
        _ => None,
    }
}

pub(super) fn current_enclosing_type_from_state(state: &OwnershipState) -> Option<String> {
    let binding = state.get("__glitching_current_type")?;
    match &binding.ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => {
            Some(name.clone())
        }
        _ => None,
    }
}

pub(super) fn implicit_field(
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
    name: &str,
) -> Option<(IrType, IrType, Ownership)> {
    let this_type = lookup(scopes, "this")?.ty;
    let owner = match &this_type {
        IrType::Class(name) | IrType::Struct(name) => name.as_str(),
        IrType::Ref(inner) => match inner.as_ref() {
            IrType::Struct(name) | IrType::Class(name) => name.as_str(),
            _ => return None,
        },
        _ => return None,
    };
    let field_type = env.resolve_field(owner, name)?;
    Some((this_type, field_type.ty, field_type.ownership))
}

pub(super) fn collect_condition_bindings(expr: &Expr, env: &TypeEnv) -> Vec<TypedBinding> {
    let mut bindings = Vec::new();
    collect_condition_bindings_inner(expr, env, &mut bindings);
    bindings
}

pub(super) fn collect_condition_bindings_inner(expr: &Expr, env: &TypeEnv, bindings: &mut Vec<TypedBinding>) {
    match expr {
        Expr::IsPattern {
            ty,
            name: Some(name),
            ..
        } => {
            let ty = type_syntax_to_ir(ty, env);
            bindings.push(TypedBinding {
                name: name.clone(),
                ty,
                ownership: Ownership::Borrowed,
            });
        }
        Expr::Binary { left, right, .. } => {
            collect_condition_bindings_inner(left, env, bindings);
            collect_condition_bindings_inner(right, env, bindings);
        }
        Expr::MethodCall { target, args, .. } => {
            collect_condition_bindings_inner(target, env, bindings);
            for arg in args {
                collect_condition_bindings_inner(arg, env, bindings);
            }
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_condition_bindings_inner(arg, env, bindings);
            }
        }
        Expr::RefArg {
            modifier: ParamModifier::Out,
            expr,
        } => {
            if let Expr::Var(name) = expr.as_ref() {
                bindings.push(TypedBinding {
                    name: name.clone(),
                    ty: IrType::Unknown(format!("out {name}")),
                    ownership: Ownership::Shared,
                });
            }
        }
        Expr::Unary { expr, .. } => collect_condition_bindings_inner(expr, env, bindings),
        _ => {}
    }
}

pub(super) fn typed_temp(ty: IrType) -> TypedBinding {
    TypedBinding {
        name: "<expr>".to_string(),
        ownership: ownership_for_type(&ty),
        ty,
    }
}

pub(super) fn typed_expr(kind: TypedExprKind, ty: IrType) -> TypedExpr {
    let ownership = ownership_for_type(&ty);
    typed_expr_with_ownership(kind, ty, ownership)
}

pub(super) fn typed_expr_with_ownership(kind: TypedExprKind, ty: IrType, ownership: Ownership) -> TypedExpr {
    let drop_kind = drop_kind_for_type(&ty, &ownership);
    TypedExpr {
        kind,
        ty,
        ownership,
        drop_kind,
    }
}

