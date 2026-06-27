use super::*;

pub(super) fn allows_shared_reference_flow(_env: &TypeEnv, ty: &IrType) -> bool {
    matches!(ty, IrType::Class(_) | IrType::Interface(_))
}

pub(super) fn is_csharp_shared_handle_name(name: &str) -> bool {
    let simple = name.rsplit('.').next().unwrap_or(name);
    let looks_like_interface = simple
        .strip_prefix('I')
        .and_then(|rest| rest.chars().next())
        .is_some_and(|ch| ch.is_ascii_uppercase());
    looks_like_interface
        || matches!(
            simple,
            "IServiceCollection"
                | "ServiceCollection"
                | "ControllerBase"
                | "ActionResult"
                | "WebApplication"
                | "WebApplicationBuilder"
                | "HttpContext"
                | "HttpRequest"
                | "HttpResponse"
        )
        || simple.ends_with("Options")
        || simple.ends_with("Settings")
        || simple.ends_with("Configuration")
        || simple.ends_with("Manager")
        || simple.ends_with("Builder")
        || simple.ends_with("Collection")
        || simple.ends_with("Logger")
        || simple.ends_with("Provider")
        || simple.ends_with("Context")
        || simple.ends_with("Factory")
}

pub(super) fn derives_from(env: &TypeEnv, name: &str, expected_base: &str) -> bool {
    let Some(bases) = env.bases.get(name) else {
        return false;
    };
    bases.iter().any(|base| {
        base == expected_base
            || base.rsplit('.').next() == Some(expected_base)
            || derives_from(env, base, expected_base)
    })
}

pub(super) fn resolve_method_call(
    env: &TypeEnv,
    target: &TypedExpr,
    name: &str,
    generic_args: &[IrType],
    args: &[TypedExpr],
) -> Result<(IrType, Ownership, String, CallResolution), String> {
    if matches!(
        &target.ty,
        IrType::Unknown(target_name) | IrType::Class(target_name)
            if matches!(base_type_name(target_name), "object" | "Object")
    ) && name == "Equals"
    {
        let resolution = if args.len() == 2 {
            CallResolution::StaticFunction
        } else {
            CallResolution::InstanceMethod
        };
        return Ok((
            IrType::Bool,
            Ownership::Copy,
            "object.Equals".to_string(),
            resolution,
        ));
    }
    match (&target.ty, name) {
        (IrType::String, _) => {
            let has_string_method_candidates = ["string", "String", "System.String"]
                .iter()
                .any(|string_type| !env.method_candidate_signatures(string_type, name).is_empty());
            for string_type in ["string", "String", "System.String"] {
                let resolved = if generic_args.is_empty() {
                    env.resolve_method_call(string_type, name, args)?
                } else {
                    env.resolve_method_call_with_generic_args(string_type, name, args, generic_args)?
                };
                if let Some(signature) = resolved {
                    if signature.is_static {
                        return Ok((
                            signature.return_type.clone(),
                            signature.return_ownership.clone(),
                            signature.symbol.clone(),
                            CallResolution::StaticFunction,
                        ));
                    }
                    return Ok((
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                        signature.symbol.clone(),
                        CallResolution::InstanceMethod,
                    ));
                }
            }
            if generic_args.is_empty() || !has_string_method_candidates {
                let resolved = if generic_args.is_empty() {
                    env.resolve_function_call(name, args)?.cloned()
                } else {
                    env.resolve_function_call_with_generic_args(name, args, generic_args)?
                };
                if let Some(signature) = resolved {
                    return Ok((
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                        signature.symbol.clone(),
                        CallResolution::StaticFunction,
                    ));
                }
            }
        }
        (IrType::Unknown(target_name) | IrType::Class(target_name), _) => {
            let has_method_candidates = !env.method_candidate_signatures(target_name, name).is_empty();
            let resolved = if generic_args.is_empty() {
                env.resolve_method_call(target_name, name, args)?
            } else {
                env.resolve_method_call_with_generic_args(target_name, name, args, generic_args)?
            };
            if let Some(signature) = resolved {
                if signature.is_static {
                    return Ok((
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                        signature.symbol.clone(),
                        CallResolution::StaticFunction,
                    ));
                }
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            if generic_args.is_empty() || !has_method_candidates {
                let resolved = if generic_args.is_empty() {
                    env.resolve_function_call(name, args)?.cloned()
                } else {
                    env.resolve_function_call_with_generic_args(name, args, generic_args)?
                };
                if let Some(signature) = resolved {
                    return Ok((
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                        signature.symbol.clone(),
                        CallResolution::StaticFunction,
                    ));
                }
            }
        }
        _ => {}
    }
    match (&target.ty, name) {
        (IrType::Class(type_name), "MapGet" | "MapPost") if type_name == "WebApplication" => {
            if let [TypedExpr {
                kind: TypedExprKind::String(path),
                ..
            }, TypedExpr {
                kind: TypedExprKind::FunctionSymbol(handler),
                ty:
                    IrType::Function {
                        params,
                        return_type,
                    },
                ..
            }] = args
            {
                if params.is_empty() && return_type.as_ref() == &IrType::String {
                    return Ok((
                        IrType::Void,
                        Ownership::Copy,
                        name.to_string(),
                        CallResolution::EndpointHandlerRegistration {
                            http_method: if name == "MapGet" { "GET" } else { "POST" }.to_string(),
                            path: path.clone(),
                            handler: handler.clone(),
                        },
                    ));
                }
            }
            let resolved_method = if generic_args.is_empty() {
                env.resolve_method_call(type_name, name, args)?
            } else {
                env.resolve_method_call_with_generic_args(type_name, name, args, generic_args)?
            };
            if let Some(signature) = resolved_method {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else if let Some(signature) = if generic_args.is_empty() {
                env.resolve_extension_method_call(type_name, name, target, args)?
            } else {
                env.resolve_extension_method_call_with_generic_args(
                    type_name,
                    name,
                    target,
                    args,
                    generic_args,
                )?
            } {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else {
                Ok((
                    IrType::Unknown(name.to_string()),
                    Ownership::Shared,
                    name.to_string(),
                    CallResolution::InstanceMethod,
                ))
            }
        }
        (IrType::Unknown(target) | IrType::Class(target), "Run") if target == "Task" => {
            let result = args
                .first()
                .map(|arg| arg.ty.clone())
                .unwrap_or(IrType::Void);
            Ok((
                IrType::Task(Box::new(result)),
                Ownership::Owned,
                name.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "FromResult")
            if target == "Task"
                || target == "ValueTask"
                || target == "System.Threading.Tasks.Task"
                || target == "System.Threading.Tasks.ValueTask" =>
        {
            let result = args
                .first()
                .map(|arg| arg.ty.clone())
                .unwrap_or(IrType::Void);
            Ok((
                IrType::Task(Box::new(result)),
                Ownership::Owned,
                name.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "WhenAll")
            if target == "Task"
                || target == "ValueTask"
                || target == "System.Threading.Tasks.Task"
                || target == "System.Threading.Tasks.ValueTask" =>
        {
            Ok((
                IrType::Task(Box::new(IrType::Void)),
                Ownership::Owned,
                name.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "ReadAllText")
            if target == "File" || target == "System.IO.File" =>
        {
            Ok((
                IrType::String,
                Ownership::Owned,
                "System_IO_File_ReadAllText".to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "WriteAllText")
            if target == "File" || target == "System.IO.File" =>
        {
            Ok((
                IrType::Void,
                Ownership::Copy,
                "System_IO_File_WriteAllText".to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "WriteLine")
            if target == "Console" || target == "System.Console" =>
        {
            let arg_ty = args.first().map(|arg| &arg.ty).unwrap_or(&IrType::Void);
            let symbol = match arg_ty {
                IrType::String => "System_Console_WriteLine_String",
                IrType::Byte | IrType::Short | IrType::Int | IrType::Long | IrType::UInt => {
                    "System_Console_WriteLine_I64"
                }
                IrType::Double => "System_Console_WriteLine_Double",
                IrType::Bool => "System_Console_WriteLine_Bool",
                _ => "System_Console_WriteLine_String",
            };
            Ok((
                IrType::Void,
                Ownership::Copy,
                symbol.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::List(element), method_name) => {
            let collection_resolution = match method_name {
                "ToArray" => Some((
                    IrType::Array(element.clone()),
                    Ownership::Owned,
                    name.to_string(),
                    CallResolution::CollectionMethod,
                )),
                "Contains" => Some((
                    IrType::Bool,
                    Ownership::Copy,
                    name.to_string(),
                    CallResolution::CollectionMethod,
                )),
                "Add" | "Clear" => Some((
                    IrType::Void,
                    Ownership::Copy,
                    name.to_string(),
                    CallResolution::CollectionMethod,
                )),
                _ => None,
            };
            if let Some(result) = collection_resolution {
                return Ok(result);
            }
            if let Some(signature) = env.resolve_method_call("List", method_name, args)? {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            if let Some(signature) =
                env.resolve_extension_method_call("List", method_name, target, args)?
            {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            Ok((
                IrType::Unknown(method_name.to_string()),
                Ownership::Shared,
                method_name.to_string(),
                CallResolution::Unknown,
            ))
        }
        (IrType::Enumerable(_), method_name) => {
            if let Some(signature) = env.resolve_method_call("IEnumerable", method_name, args)? {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            if let Some(signature) =
                env.resolve_extension_method_call("IEnumerable", method_name, target, args)?
            {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            Ok((
                IrType::Unknown(method_name.to_string()),
                Ownership::Shared,
                method_name.to_string(),
                CallResolution::Unknown,
            ))
        }
        (
            IrType::Class(type_name) | IrType::Struct(type_name) | IrType::Interface(type_name),
            method_name,
        ) => {
            if let Some(signature) = env.resolve_method_call(type_name, method_name, args)? {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else if let Some(signature) =
                env.resolve_extension_method_call(type_name, method_name, target, args)?
            {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else {
                Ok((
                    IrType::Unknown(name.to_string()),
                    Ownership::Shared,
                    name.to_string(),
                    CallResolution::InstanceMethod,
                ))
            }
        }
        (IrType::Task(inner), "GetResult") => Ok((
            inner.as_ref().clone(),
            ownership_for_type(inner),
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Task(inner), "GetAwaiter") => Ok((
            IrType::Task(inner.clone()),
            Ownership::Owned,
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Task(inner), "AsTask") => Ok((
            IrType::Task(inner.clone()),
            Ownership::Owned,
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Weak(_), "TryGetTarget") => {
            Ok((IrType::Bool, Ownership::Copy, name.to_string(), CallResolution::WeakMethod))
        }
        (IrType::Dictionary(_, _), "ContainsKey" | "Remove" | "TryGetValue") => {
            Ok((
                IrType::Bool,
                Ownership::Copy,
                name.to_string(),
                CallResolution::CollectionMethod,
            ))
        }
        (IrType::Dictionary(_, _), method_name) => {
            let collection_resolution = match method_name {
                "Add" | "Clear" => Some((
                    IrType::Void,
                    Ownership::Copy,
                    name.to_string(),
                    CallResolution::CollectionMethod,
                )),
                "GetEnumerator" => Some(
                    env.resolve_method_call("Dictionary", method_name, args)?
                        .map(|signature| {
                            (
                                signature.return_type.clone(),
                                signature.return_ownership.clone(),
                                name.to_string(),
                                CallResolution::CollectionMethod,
                            )
                        })
                        .unwrap_or((
                            IrType::Unknown(method_name.to_string()),
                            Ownership::Owned,
                            name.to_string(),
                            CallResolution::CollectionMethod,
                        )),
                ),
                _ => None,
            };
            if let Some(result) = collection_resolution {
                return Ok(result);
            }
            if let Some(signature) = env.resolve_method_call("Dictionary", method_name, args)? {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            if let Some(signature) =
                env.resolve_extension_method_call("Dictionary", method_name, target, args)?
            {
                return Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ));
            }
            Ok((
                IrType::Unknown(method_name.to_string()),
                Ownership::Shared,
                method_name.to_string(),
                CallResolution::Unknown,
            ))
        }
        (_, "Wait") => Ok((IrType::Void, Ownership::Copy, name.to_string(), CallResolution::TaskMethod)),
        _ => Ok((
            IrType::Unknown(name.to_string()),
            Ownership::Shared,
            name.to_string(),
            CallResolution::Unknown,
        )),
    }
}

