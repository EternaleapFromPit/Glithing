use super::*;

pub(super) fn infer_generic_args_from_signature(signature: &FunctionSignature, args: &[TypedExpr]) -> Vec<IrType> {
    infer_generic_args_from_signature_with_expected(signature, args, None)
}

pub(super) fn infer_generic_args_from_signature_with_expected(
    signature: &FunctionSignature,
    args: &[TypedExpr],
    expected: Option<&IrType>,
) -> Vec<IrType> {
    if signature.generic_params.is_empty() {
        return Vec::new();
    }
    let mut inferred = HashMap::<String, IrType>::new();
    let matches = if let Some(params_element_type) = &signature.params_element_type {
        let fixed_len = signature.params.len().saturating_sub(1);
        if args.len() < fixed_len {
            false
        } else {
            let mut ok = true;
            for (expected, arg) in signature.params.iter().take(fixed_len).zip(args.iter().take(fixed_len)) {
                if !infer_generic_args_from_expr(expected, arg, &signature.generic_params, &mut inferred) {
                    ok = false;
                    break;
                }
            }
            if ok {
                for arg in args.iter().skip(fixed_len) {
                    if !infer_generic_args_from_expr(
                        params_element_type,
                        arg,
                        &signature.generic_params,
                        &mut inferred,
                    ) {
                        ok = false;
                        break;
                    }
                }
            }
            ok
        }
    } else if signature.params.len() == args.len() {
        let mut ok = true;
        for (expected, arg) in signature.params.iter().zip(args.iter()) {
            if !infer_generic_args_from_expr(expected, arg, &signature.generic_params, &mut inferred) {
                ok = false;
                break;
            }
        }
        ok
    } else {
        false
    };
    if !matches {
        return Vec::new();
    }
    if let Some(expected) = expected {
        let _ = infer_generic_args_from_type(
            &signature.return_type,
            expected,
            &signature.generic_params,
            &mut inferred,
        );
    }
    let mut result = Vec::with_capacity(signature.generic_params.len());
    for name in &signature.generic_params {
        let Some(ty) = inferred.get(name) else {
            return Vec::new();
        };
        result.push(ty.clone());
    }
    result
}

pub(super) fn preferred_generic_inference_type(arg: &TypedExpr) -> IrType {
    match &arg.kind {
        TypedExprKind::Int(value) if i32::try_from(*value).is_ok() => IrType::Int,
        TypedExprKind::Int(_) => IrType::Long,
        _ => arg.ty.clone(),
    }
}

pub(super) fn infer_generic_args_from_expr(
    expected: &IrType,
    arg: &TypedExpr,
    generic_params: &[String],
    inferred: &mut HashMap<String, IrType>,
) -> bool {
    match expected {
        IrType::Unknown(name) if generic_params.iter().any(|param| param == name) => {
            let inferred_ty = preferred_generic_inference_type(arg);
            match inferred.get(name) {
                Some(existing) if existing == &inferred_ty => true,
                Some(existing) => match arg.kind {
                    TypedExprKind::Int(value) => {
                        int_literal_conversion_rank(existing, value).is_some()
                    }
                    _ => false,
                },
                None => {
                    inferred.insert(name.clone(), inferred_ty);
                    true
                }
            }
        }
        _ => infer_generic_args_from_type(expected, &arg.ty, generic_params, inferred),
    }
}

pub(super) fn substitute_generic_args_in_ir_type(
    ty: &IrType,
    generic_params: &[String],
    generic_args: &[IrType],
) -> IrType {
    let mut subst = HashMap::new();
    for (name, arg) in generic_params.iter().zip(generic_args.iter()) {
        subst.insert(name.clone(), arg.clone());
    }
    substitute_ir_type_with_map(ty, &subst)
}

pub(super) fn substitute_ir_type_with_map(ty: &IrType, subst: &HashMap<String, IrType>) -> IrType {
    match ty {
        IrType::Unknown(name) => subst.get(name).cloned().unwrap_or_else(|| ty.clone()),
        IrType::Struct(name) => substitute_object_ir_type_name(name, subst)
            .map(IrType::Struct)
            .unwrap_or_else(|| ty.clone()),
        IrType::Class(name) => substitute_object_ir_type_name(name, subst)
            .map(IrType::Class)
            .unwrap_or_else(|| ty.clone()),
        IrType::Interface(name) => substitute_object_ir_type_name(name, subst)
            .map(IrType::Interface)
            .unwrap_or_else(|| ty.clone()),
        IrType::Array(inner) => IrType::Array(Box::new(substitute_ir_type_with_map(inner, subst))),
        IrType::Ref(inner) => IrType::Ref(Box::new(substitute_ir_type_with_map(inner, subst))),
        IrType::List(inner) => IrType::List(Box::new(substitute_ir_type_with_map(inner, subst))),
        IrType::Dictionary(key, value) => IrType::Dictionary(
            Box::new(substitute_ir_type_with_map(key, subst)),
            Box::new(substitute_ir_type_with_map(value, subst)),
        ),
        IrType::Enumerable(inner) => {
            IrType::Enumerable(Box::new(substitute_ir_type_with_map(inner, subst)))
        }
        IrType::Task(inner) => IrType::Task(Box::new(substitute_ir_type_with_map(inner, subst))),
        IrType::Nullable(inner) => {
            IrType::Nullable(Box::new(substitute_ir_type_with_map(inner, subst)))
        }
        IrType::Function { params, return_type } => IrType::Function {
            params: params
                .iter()
                .map(|param| substitute_ir_type_with_map(param, subst))
                .collect(),
            return_type: Box::new(substitute_ir_type_with_map(return_type, subst)),
        },
        _ => ty.clone(),
    }
}

pub(super) fn substitute_object_ir_type_name(
    name: &str,
    subst: &HashMap<String, IrType>,
) -> Option<String> {
    let (base, args) = split_monomorphized_type(name)?;
    let rewritten = args
        .into_iter()
        .map(|arg| {
            let parsed = parse_monomorphized_ir_type(&arg, &TypeEnv::default())
                .unwrap_or(IrType::Unknown(arg));
            render_monomorphized_ir_type(&substitute_ir_type_with_map(&parsed, subst))
        })
        .collect::<Vec<_>>();
    Some(format!("{base}<{}>", rewritten.join(",")))
}

pub(super) fn substitute_field_signature(
    field: &FieldSignature,
    subst: &HashMap<String, IrType>,
) -> FieldSignature {
    FieldSignature {
        package_id: field.package_id.clone(),
        visibility: field.visibility,
        ty: substitute_ir_type_with_map(&field.ty, subst),
        ownership: field.ownership.clone(),
    }
}

pub(super) fn substitute_function_signature(
    signature: &FunctionSignature,
    subst: &HashMap<String, IrType>,
) -> FunctionSignature {
    FunctionSignature {
        package_id: signature.package_id.clone(),
        visibility: signature.visibility,
        generic_params: signature.generic_params.clone(),
        generic_constraints: signature.generic_constraints.clone(),
        params: signature
            .params
            .iter()
            .map(|param| substitute_ir_type_with_map(param, subst))
            .collect(),
        param_ownerships: signature.param_ownerships.clone(),
        required_params: signature.required_params,
        params_element_type: signature
            .params_element_type
            .as_ref()
            .map(|ty| substitute_ir_type_with_map(ty, subst)),
        return_type: substitute_ir_type_with_map(&signature.return_type, subst),
        return_ownership: signature.return_ownership.clone(),
        symbol: signature.symbol.clone(),
        is_static: signature.is_static,
    }
}

pub(super) fn explicit_generic_subst(
    signature: &FunctionSignature,
    generic_args: &[IrType],
) -> Result<HashMap<String, IrType>, String> {
    if signature.generic_params.len() != generic_args.len() {
        return Err(format!(
            "generic method expects {} type argument(s), found {}",
            signature.generic_params.len(),
            generic_args.len()
        ));
    }
    let mut subst = HashMap::new();
    for (name, arg) in signature
        .generic_params
        .iter()
        .cloned()
        .zip(generic_args.iter().cloned())
    {
        subst.insert(name, arg);
    }
    Ok(subst)
}

pub(super) fn specialize_signature_with_explicit_generic_args(
    signature: &FunctionSignature,
    generic_args: &[IrType],
    env: &TypeEnv,
    context: &str,
) -> Result<FunctionSignature, String> {
    let subst = explicit_generic_subst(signature, generic_args)?;
    validate_explicit_generic_constraints(signature, generic_args, env, context)?;
    let mut specialized = substitute_function_signature(signature, &subst);
    specialized.generic_params.clear();
    specialized.generic_constraints.clear();
    Ok(specialized)
}

pub(super) fn validate_explicit_generic_constraints(
    signature: &FunctionSignature,
    generic_args: &[IrType],
    env: &TypeEnv,
    context: &str,
) -> Result<(), String> {
    if signature.generic_params.len() != generic_args.len() {
        return Err(format!(
            "{context} expects {} type argument(s), found {}",
            signature.generic_params.len(),
            generic_args.len()
        ));
    }
    for ((param_name, constraints), actual) in signature
        .generic_params
        .iter()
        .zip(signature.generic_constraints.iter())
        .zip(generic_args.iter())
    {
        for constraint in constraints {
            if explicit_generic_constraint_satisfied(actual, constraint, env) {
                continue;
            }
            return Err(format!(
                "{context} type argument '{}' does not satisfy constraint '{}' for '{}'",
                render_monomorphized_ir_type(actual),
                constraint,
                param_name
            ));
        }
    }
    Ok(())
}

fn explicit_generic_constraint_satisfied(actual: &IrType, constraint: &str, env: &TypeEnv) -> bool {
    match constraint {
        "class" | "notnull" => !is_value_type_ir(actual) && !matches!(actual, IrType::Nullable(_)),
        "struct" | "unmanaged" => is_value_type_ir(actual),
        "new()" => has_parameterless_construction(actual, env),
        other => matches_named_generic_constraint(actual, other, env),
    }
}

fn has_parameterless_construction(actual: &IrType, env: &TypeEnv) -> bool {
    if is_value_type_ir(actual) {
        return true;
    }
    let Some(type_name) = named_ir_type(actual) else {
        return false;
    };
    env.resolve_constructor(type_name, &[]).is_some()
}

fn matches_named_generic_constraint(actual: &IrType, constraint: &str, env: &TypeEnv) -> bool {
    let Some(actual_name) = named_ir_type(actual) else {
        return false;
    };
    let actual_base = base_type_name(actual_name);
    let constraint_base = base_type_name(constraint);
    actual_name == constraint
        || actual_base == constraint
        || actual_name == constraint_base
        || actual_base == constraint_base
        || derives_from(env, actual_name, constraint)
        || derives_from(env, actual_name, constraint_base)
        || derives_from(env, actual_base, constraint)
        || derives_from(env, actual_base, constraint_base)
}

fn named_ir_type(actual: &IrType) -> Option<&str> {
    match actual {
        IrType::Class(name)
        | IrType::Struct(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => Some(name.as_str()),
        _ => None,
    }
}

fn is_value_type_ir(actual: &IrType) -> bool {
    matches!(
        actual,
        IrType::Bool
            | IrType::Byte
            | IrType::Short
            | IrType::Int
            | IrType::Long
            | IrType::UInt
            | IrType::Double
            | IrType::Decimal
            | IrType::Nullable(_)
            | IrType::Struct(_)
    )
}

pub(super) fn infer_generic_args_from_type(
    expected: &IrType,
    actual: &IrType,
    generic_params: &[String],
    inferred: &mut HashMap<String, IrType>,
) -> bool {
    match expected {
        IrType::Unknown(name) if generic_params.iter().any(|param| param == name) => {
            match inferred.get(name) {
                Some(existing) => existing == actual,
                None => {
                    inferred.insert(name.clone(), actual.clone());
                    true
                }
            }
        }
        IrType::Array(expected_inner)
        | IrType::Ref(expected_inner)
        | IrType::List(expected_inner)
        | IrType::Enumerable(expected_inner)
        | IrType::Task(expected_inner)
        | IrType::Weak(expected_inner) => match actual {
            IrType::Array(actual_inner)
            | IrType::Ref(actual_inner)
            | IrType::List(actual_inner)
            | IrType::Enumerable(actual_inner)
            | IrType::Task(actual_inner)
            | IrType::Weak(actual_inner) => infer_generic_args_from_type(
                expected_inner,
                actual_inner,
                generic_params,
                inferred,
            ),
            _ => false,
        },
        IrType::Dictionary(expected_key, expected_value) => match actual {
            IrType::Dictionary(actual_key, actual_value) => {
                infer_generic_args_from_type(expected_key, actual_key, generic_params, inferred)
                    && infer_generic_args_from_type(
                        expected_value,
                        actual_value,
                        generic_params,
                        inferred,
                    )
            }
            _ => false,
        },
        IrType::Function {
            params: expected_params,
            return_type: expected_return,
        } => match actual {
            IrType::Function {
                params: actual_params,
                return_type: actual_return,
            } => {
                expected_params.len() == actual_params.len()
                    && expected_params
                        .iter()
                        .zip(actual_params.iter())
                        .all(|(expected_param, actual_param)| {
                            infer_generic_args_from_type(
                                expected_param,
                                actual_param,
                                generic_params,
                                inferred,
                            )
                        })
                    && infer_generic_args_from_type(
                        expected_return,
                        actual_return,
                        generic_params,
                        inferred,
                    )
            }
            _ => false,
        },
        _ => expected == actual || matches!(expected, IrType::Unknown(_) if matches!(actual, IrType::Unknown(_))),
    }
}

pub(super) fn ir_conversion_rank(expected: &IrType, arg: &TypedExpr, env: &TypeEnv) -> Option<u16> {
    if expected == &arg.ty {
        return Some(
            if matches!(arg.kind, TypedExprKind::Int(_)) && expected == &IrType::Long {
                1
            } else {
                0
            },
        );
    }
    if matches!(
        expected,
        IrType::Unknown(name) if name == "object" || name == "System.Object"
    ) || matches!(expected, IrType::Class(name) if name == "object" || name == "System.Object") {
        return Some(20);
    }
    if let (
        IrType::Function {
            params: expected_params,
            return_type: expected_return,
        },
        IrType::Function {
            params: actual_params,
            return_type: actual_return,
        },
    ) = (expected, &arg.ty)
    {
        if expected_params.len() == actual_params.len()
            && expected_params
                .iter()
                .zip(actual_params.iter())
                .all(|(expected_param, actual_param)| {
                    expected_param == actual_param || matches!(actual_param, IrType::Unknown(_))
                })
        {
            if expected_return.as_ref() == actual_return.as_ref() {
                return Some(0);
            }
            if matches!(
                (expected_return.as_ref(), actual_return.as_ref()),
                (IrType::Void, IrType::Unknown(actual)) if actual == "null"
            ) {
                // Statement-bodied lambdas are still parsed through a placeholder
                // expression path, so treat that placeholder as compatible with
                // Action-style / void delegates during overload resolution.
                return Some(0);
            }
            if matches!(expected_return.as_ref(), IrType::Void)
                && lambda_body_is_void_compatible(arg)
            {
                return Some(0);
            }
            if matches!(
                (expected_return.as_ref(), actual_return.as_ref()),
                (IrType::Enumerable(expected_inner), IrType::List(actual_inner))
                    if expected_inner.as_ref() == actual_inner.as_ref()
            ) {
                return Some(1);
            }
            if matches!(
                (expected_return.as_ref(), actual_return.as_ref()),
                (IrType::Enumerable(expected_inner), IrType::Array(actual_inner))
                    if expected_inner.as_ref() == actual_inner.as_ref()
            ) {
                return Some(1);
            }
        }
    }
    if matches!(arg.kind, TypedExprKind::Int(_)) && expected == &IrType::Int {
        return Some(0);
    }
    if let Some(rank) = ir_numeric_conversion_rank(expected, &arg.ty, &arg.kind) {
        return Some(rank);
    }
    match (expected, &arg.ty) {
        (
            IrType::Class(_)
            | IrType::Interface(_)
            | IrType::Struct(_)
            | IrType::String
            | IrType::Array(_)
            | IrType::List(_)
            | IrType::Dictionary(_, _)
            | IrType::Enumerable(_)
            | IrType::Task(_),
            IrType::Unknown(actual),
        ) if actual == "null" => Some(50),
        (IrType::Class(expected), IrType::Class(actual))
        | (IrType::Interface(expected), IrType::Class(actual))
        | (IrType::Interface(expected), IrType::Struct(actual)) => {
            inheritance_distance(actual, expected, &env.bases).map(|distance| 10 + distance)
        }
        (_, IrType::Unknown(_)) | (IrType::Unknown(_), _) => Some(100),
        _ => None,
    }
}

fn lambda_body_is_void_compatible(arg: &TypedExpr) -> bool {
    match &arg.kind {
        TypedExprKind::Lambda { body, .. } => matches!(
            body.kind,
            TypedExprKind::Assign { .. }
                | TypedExprKind::Call(_)
                | TypedExprKind::IncDec { .. }
                | TypedExprKind::Await(_)
                | TypedExprKind::NewObject { .. }
                | TypedExprKind::Null
        ),
        _ => false,
    }
}

pub(super) fn ir_numeric_conversion_rank(
    expected: &IrType,
    actual: &IrType,
    actual_kind: &TypedExprKind,
) -> Option<u16> {
    if let TypedExprKind::Int(value) = actual_kind {
        return int_literal_conversion_rank(expected, *value);
    }
    match (actual, expected) {
        (IrType::Byte, IrType::Short) => Some(1),
        (IrType::Byte, IrType::Int) => Some(2),
        (IrType::Byte, IrType::UInt) => Some(2),
        (IrType::Byte, IrType::Long) => Some(3),
        (IrType::Byte, IrType::Double) | (IrType::Byte, IrType::Decimal) => Some(4),
        (IrType::Short, IrType::Int) => Some(1),
        (IrType::Short, IrType::Long) => Some(2),
        (IrType::Short, IrType::Double) | (IrType::Short, IrType::Decimal) => Some(3),
        (IrType::Int, IrType::Long) => Some(1),
        (IrType::Int, IrType::Double) | (IrType::Int, IrType::Decimal) => Some(2),
        (IrType::UInt, IrType::Long) => Some(1),
        (IrType::UInt, IrType::Double) | (IrType::UInt, IrType::Decimal) => Some(2),
        (IrType::Long, IrType::Double) | (IrType::Long, IrType::Decimal) => Some(1),
        _ => None,
    }
}

pub(super) fn int_literal_conversion_rank(expected: &IrType, value: i64) -> Option<u16> {
    match expected {
        IrType::Int if i32::try_from(value).is_ok() => Some(0),
        IrType::Long => Some(1),
        IrType::Short if i16::try_from(value).is_ok() => Some(2),
        IrType::Byte if u8::try_from(value).is_ok() => Some(3),
        IrType::UInt if u32::try_from(value).is_ok() => Some(4),
        IrType::Double | IrType::Decimal => Some(5),
        _ => None,
    }
}

pub(super) fn inheritance_distance(
    actual: &str,
    expected: &str,
    bases: &HashMap<String, Vec<String>>,
) -> Option<u16> {
    if actual == expected {
        return Some(0);
    }
    for base in bases.get(actual)? {
        if base == expected {
            return Some(1);
        }
        if let Some(distance) = inheritance_distance(base, expected, bases) {
            return Some(distance + 1);
        }
    }
    None
}
