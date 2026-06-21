use super::*;

pub(super) fn type_syntax_to_ir(ty: &TypeSyntax, env: &TypeEnv) -> IrType {
    type_syntax_to_ir_with_subst(ty, env, &HashMap::new())
}

pub(super) fn type_syntax_to_ir_with_subst(
    ty: &TypeSyntax,
    env: &TypeEnv,
    subst: &HashMap<String, TypeSyntax>,
) -> IrType {
    match ty {
        TypeSyntax::GenericNamed { name, args }
            if name == "own"
                || name == "shared"
                || name == "borrow"
                || name == "view"
                || name == "System.Ownership.own"
                || name == "System.Ownership.shared"
                || name == "System.Ownership.borrow"
                || name == "System.Ownership.view" =>
        {
            if let Some(first_arg) = args.first() {
                type_syntax_to_ir_with_subst(first_arg, env, subst)
            } else {
                IrType::Unknown("T".to_string())
            }
        }
        TypeSyntax::Scalar(scalar) => scalar_to_ir(*scalar),
        TypeSyntax::String => IrType::String,
        TypeSyntax::Array(inner) => {
            IrType::Array(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Ref(inner) => {
            IrType::Ref(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Named(name) if name == "Exception" || name == "System.Exception" => {
            IrType::Exception
        }
        TypeSyntax::Named(name) if name == "Action" || name == "System.Action" => {
            IrType::Function {
                params: vec![],
                return_type: Box::new(IrType::Void),
            }
        }
        TypeSyntax::Named(name) => {
            if let Some(replacement) = subst.get(name) {
                if matches!(replacement, TypeSyntax::Named(replacement_name) if replacement_name == name) {
                    return IrType::Unknown(name.clone());
                }
                return type_syntax_to_ir_with_subst(replacement, env, subst);
            }
            if let Some(signature) = env.delegates.get(name) {
                return delegate_signature_to_ir(signature, &[], env, subst);
            }
            match env.kinds.get(name) {
                Some(TypeKind::Class) => IrType::Class(name.clone()),
                Some(TypeKind::Interface) => IrType::Interface(name.clone()),
                Some(TypeKind::Enum) => IrType::Int,
                Some(_) => IrType::Struct(name.clone()),
                None => IrType::Unknown(name.clone()),
            }
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "Rc" || name == "System.Ownership.Rc" =>
        {
            if let Some(first_arg) = args.first() {
                IrType::Struct(rc_runtime_type_name(&type_syntax_to_ir_with_subst(
                    first_arg, env, subst,
                )))
            } else {
                IrType::Struct("Rc_T".to_string())
            }
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "Weak"
                || name == "WeakReference"
                || name == "System.WeakReference"
                || name == "System.Ownership.weakref"
                || name == "weakref" =>
        {
            if let Some(first_arg) = args.first() {
                IrType::Weak(Box::new(type_syntax_to_ir_with_subst(first_arg, env, subst)))
            } else {
                IrType::Weak(Box::new(IrType::Unknown("T".to_string())))
            }
        }
        TypeSyntax::GenericNamed { name, args } if name == "Func" || name == "System.Func" => {
            if args.is_empty() {
                IrType::Function {
                    params: vec![],
                    return_type: Box::new(IrType::Void),
                }
            } else {
                let (params_args, ret_arg) = args.split_at(args.len() - 1);
                let params = params_args
                    .iter()
                    .map(|arg| type_syntax_to_ir_with_subst(arg, env, subst))
                    .collect();
                let return_type = Box::new(type_syntax_to_ir_with_subst(&ret_arg[0], env, subst));
                IrType::Function {
                    params,
                    return_type,
                }
            }
        }
        TypeSyntax::GenericNamed { name, args } if name == "Action" || name == "System.Action" => {
            let params = args
                .iter()
                .map(|arg| type_syntax_to_ir_with_subst(arg, env, subst))
                .collect();
            IrType::Function {
                params,
                return_type: Box::new(IrType::Void),
            }
        }
        TypeSyntax::GenericNamed { name, args } => {
            if let Some(replacement) = subst.get(name) {
                if matches!(replacement, TypeSyntax::Named(replacement_name) if replacement_name == name) {
                    return IrType::Unknown(format!("{name}<>"));
                }
                return type_syntax_to_ir_with_subst(replacement, env, subst);
            }
            if (name == "List" || name == "System.Collections.Generic.List") && args.len() == 1 {
                return IrType::List(Box::new(type_syntax_to_ir_with_subst(
                    &args[0],
                    env,
                    subst,
                )));
            }
            if (name == "Dictionary" || name == "System.Collections.Generic.Dictionary") && args.len() == 2 {
                return IrType::Dictionary(
                    Box::new(type_syntax_to_ir_with_subst(&args[0], env, subst)),
                    Box::new(type_syntax_to_ir_with_subst(&args[1], env, subst)),
                );
            }
            if let Some(signature) = env.delegates.get(name) {
                return delegate_signature_to_ir(signature, args, env, subst);
            }
            if is_collection_interface_name(name) {
                if args.len() == 1 {
                    return IrType::Enumerable(Box::new(type_syntax_to_ir_with_subst(
                        &args[0],
                        env,
                        subst,
                    )));
                }
                return IrType::Unknown(format!("{name}<>"));
            }
            if name == "IReadOnlyDictionary" || name == "System.Collections.Generic.IReadOnlyDictionary" {
                if args.len() == 2 {
                    return IrType::Dictionary(
                        Box::new(type_syntax_to_ir_with_subst(&args[0], env, subst)),
                        Box::new(type_syntax_to_ir_with_subst(&args[1], env, subst)),
                    );
                }
            }
            let concrete_args = args
                .iter()
                .map(|arg| type_syntax_to_ir_with_subst(arg, env, subst))
                .collect::<Vec<_>>();
            let concrete_name = monomorphized_type_name(name, &concrete_args);
            match env.kinds.get(name) {
                Some(TypeKind::Class) => IrType::Class(concrete_name),
                Some(TypeKind::Interface) => IrType::Interface(concrete_name),
                Some(TypeKind::Enum) => IrType::Int,
                Some(_) => IrType::Struct(concrete_name),
                None => IrType::Unknown(concrete_name),
            }
        }
        TypeSyntax::List(inner) => {
            IrType::List(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Dictionary(key, value) => IrType::Dictionary(
            Box::new(type_syntax_to_ir_with_subst(key, env, subst)),
            Box::new(type_syntax_to_ir_with_subst(value, env, subst)),
        ),
        TypeSyntax::IEnumerable(inner) => {
            IrType::Enumerable(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Thread => IrType::Thread,
        TypeSyntax::Task(inner) => {
            IrType::Task(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Nullable(inner) => {
            let inner = type_syntax_to_ir_with_subst(inner, env, subst);
            if is_nullable_value_type(&inner) {
                IrType::Nullable(Box::new(inner))
            } else {
                inner
            }
        }
        TypeSyntax::Void => IrType::Void,
    }
}

pub(super) fn is_nullable_value_type(ty: &IrType) -> bool {
    matches!(
        ty,
        IrType::Bool
            | IrType::Byte
            | IrType::Short
            | IrType::Int
            | IrType::Long
            | IrType::UInt
            | IrType::Double
            | IrType::Decimal
            | IrType::Struct(_)
            | IrType::Unknown(_)
    )
}

pub(super) fn ownership_for_declared_type_syntax(ty: &TypeSyntax, env: &TypeEnv) -> Ownership {
    match ty {
        TypeSyntax::GenericNamed { name, args }
            if name == "own" || name == "System.Ownership.own" =>
        {
            let _ = args;
            Ownership::Owned
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "shared" || name == "System.Ownership.shared" =>
        {
            let _ = args;
            Ownership::Shared
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "borrow" || name == "System.Ownership.borrow" =>
        {
            let _ = args;
            Ownership::Borrowed
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "view" || name == "System.Ownership.view" =>
        {
            let _ = args;
            Ownership::View
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "weakref" || name == "System.Ownership.weakref" =>
        {
            let _ = args;
            Ownership::Copy
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "IReadOnlyDictionary"
                || name == "System.Collections.Generic.IReadOnlyDictionary" =>
        {
            let _ = args;
            Ownership::View
        }
        TypeSyntax::GenericNamed { name, .. } if is_collection_interface_name(name) => {
            Ownership::View
        }
        _ => ownership_for_type(&type_syntax_to_ir(ty, env)),
    }
}

pub(super) fn is_collection_interface_name(name: &str) -> bool {
    matches!(
        name,
        "IEnumerable"
            | "System.Collections.Generic.IEnumerable"
            | "ICollection"
            | "System.Collections.Generic.ICollection"
    )
}

pub(super) fn delegate_signature_to_ir(
    signature: &DelegateSignature,
    args: &[TypeSyntax],
    env: &TypeEnv,
    outer_subst: &HashMap<String, TypeSyntax>,
) -> IrType {
    let mut subst = outer_subst.clone();
    for (index, generic_name) in signature.generic_params.iter().enumerate() {
        let replacement = args
            .get(index)
            .cloned()
            .unwrap_or_else(|| TypeSyntax::Named(format!("__open_generic_{generic_name}")));
        subst.insert(generic_name.clone(), replacement);
    }
    let params = signature
        .params
        .iter()
        .map(|param| type_syntax_to_ir_with_subst(param, env, &subst))
        .collect();
    let return_type = Box::new(type_syntax_to_ir_with_subst(&signature.return_type, env, &subst));
    IrType::Function {
        params,
        return_type,
    }
}

pub(super) fn scalar_to_ir(scalar: ScalarType) -> IrType {
    match scalar {
        ScalarType::Bool => IrType::Bool,
        ScalarType::Byte => IrType::Byte,
        ScalarType::Short => IrType::Short,
        ScalarType::I32 => IrType::Int,
        ScalarType::I64 => IrType::Long,
        ScalarType::U32 => IrType::UInt,
        ScalarType::F64 => IrType::Double,
        ScalarType::Decimal => IrType::Decimal,
    }
}

