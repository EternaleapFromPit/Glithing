use super::*;

pub(super) fn augment_type_metadata_fields(
    mut fields: Vec<TypedFieldInit>,
    env: &TypeEnv,
) -> Result<Vec<TypedFieldInit>, String> {
    let has_properties = fields.iter().any(|field| field.name == "Properties");
    if has_properties {
        return Ok(fields);
    }
    let Some(full_name) = fields.iter().find_map(|field| match &field.expr.kind {
        TypedExprKind::String(value) if field.name == "FullName" => Some(value.clone()),
        _ => None,
    }) else {
        return Ok(fields);
    };
    let type_name = full_name.rsplit('.').next().unwrap_or(&full_name);
    let properties = env.all_field_infos(type_name);
    let property_values = properties
        .into_iter()
        .map(|(name, field)| {
            let property_type = type_object_expr_from_ir_shallow(&field.ty, env);
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: "PropertyInfo".to_string(),
                    constructor: None,
                    args: Vec::new(),
                    fields: vec![
                        TypedFieldInit {
                            name: "Name".to_string(),
                            expr: typed_expr(TypedExprKind::String(name), IrType::String),
                        },
                        TypedFieldInit {
                            name: "PropertyType".to_string(),
                            expr: property_type,
                        },
                        TypedFieldInit {
                            name: "CanRead".to_string(),
                            expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                        },
                        TypedFieldInit {
                            name: "CanWrite".to_string(),
                            expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                        },
                    ],
                },
                IrType::Class("PropertyInfo".to_string()),
                Ownership::Shared,
            )
        })
        .collect::<Vec<_>>();
    fields.push(TypedFieldInit {
        name: "Properties".to_string(),
        expr: typed_expr(
            TypedExprKind::ArrayLiteral(property_values),
            IrType::Array(Box::new(IrType::Class("PropertyInfo".to_string()))),
        ),
    });
    Ok(fields)
}

pub(super) fn type_object_expr_from_ir(ty: &IrType, env: &TypeEnv) -> TypedExpr {
    type_object_expr_from_ir_impl(ty, env, true)
}

pub(super) fn type_object_expr_from_ir_shallow(ty: &IrType, env: &TypeEnv) -> TypedExpr {
    type_object_expr_from_ir_impl(ty, env, false)
}

pub(super) fn type_object_expr_from_ir_impl(ty: &IrType, env: &TypeEnv, include_members: bool) -> TypedExpr {
    let full_name = type_object_full_name_from_ir(ty);
    let simple_name = full_name.rsplit('.').next().unwrap_or(&full_name).to_string();
    let namespace = if let Some((ns, _)) = full_name.rsplit_once('.') {
        ns.to_string()
    } else {
        String::new()
    };
    let generic_definition_name = type_object_generic_definition_name_from_ir(ty);
    let is_generic = matches!(
        ty,
        IrType::List(_)
            | IrType::Dictionary(_, _)
            | IrType::Enumerable(_)
            | IrType::Task(_)
            | IrType::Nullable(_)
            | IrType::Function { .. }
    );
    let generic_arguments = type_object_generic_arguments_from_ir(ty, env);
    let base_type = type_object_base_type_from_ir(ty, env);
    let element_type = match ty {
        IrType::Array(inner)
        | IrType::List(inner)
        | IrType::Enumerable(inner)
        | IrType::Task(inner)
        | IrType::Nullable(inner) => {
            Some(type_object_expr_from_ir_shallow(inner, env))
        }
        _ => None,
    };
    let properties = if include_members {
        type_object_properties_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    let methods = if include_members {
        type_object_methods_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    let fields = if include_members {
        type_object_fields_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    let constructors = if include_members {
        type_object_constructors_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    let interfaces = if include_members {
        type_object_interfaces_from_ir(ty, env, include_members)
    } else {
        Vec::new()
    };
    typed_expr_with_ownership(
        TypedExprKind::NewObject {
            type_name: "Type".to_string(),
            constructor: None,
            args: Vec::new(),
            fields: vec![
                TypedFieldInit {
                    name: "Name".to_string(),
                    expr: typed_expr(TypedExprKind::String(simple_name), IrType::String),
                },
                TypedFieldInit {
                    name: "Namespace".to_string(),
                    expr: typed_expr(TypedExprKind::String(namespace), IrType::String),
                },
                TypedFieldInit {
                    name: "FullName".to_string(),
                    expr: typed_expr(TypedExprKind::String(full_name), IrType::String),
                },
                TypedFieldInit {
                    name: "GenericTypeDefinitionName".to_string(),
                    expr: typed_expr(
                        TypedExprKind::String(generic_definition_name),
                        IrType::String,
                    ),
                },
                TypedFieldInit {
                    name: "IsGenericType".to_string(),
                    expr: typed_expr(TypedExprKind::Bool(is_generic), IrType::Bool),
                },
                TypedFieldInit {
                    name: "IsClass".to_string(),
                    expr: typed_expr(
                        TypedExprKind::Bool(matches!(ty, IrType::Class(_))),
                        IrType::Bool,
                    ),
                },
                TypedFieldInit {
                    name: "IsInterface".to_string(),
                    expr: typed_expr(
                        TypedExprKind::Bool(matches!(ty, IrType::Interface(_))),
                        IrType::Bool,
                    ),
                },
                TypedFieldInit {
                    name: "IsValueType".to_string(),
                    expr: typed_expr(
                        TypedExprKind::Bool(matches!(
                            ty,
                            IrType::Struct(_)
                                | IrType::Nullable(_)
                                | IrType::Bool
                                | IrType::Byte
                                | IrType::Short
                                | IrType::Int
                                | IrType::Long
                                | IrType::UInt
                                | IrType::Double
                                | IrType::Decimal
                        )),
                        IrType::Bool,
                    ),
                },
                TypedFieldInit {
                    name: "IsEnum".to_string(),
                    expr: typed_expr(TypedExprKind::Bool(false), IrType::Bool),
                },
                TypedFieldInit {
                    name: "IsArray".to_string(),
                    expr: typed_expr(TypedExprKind::Bool(matches!(ty, IrType::Array(_))), IrType::Bool),
                },
                TypedFieldInit {
                    name: "BaseType".to_string(),
                    expr: base_type.unwrap_or_else(|| {
                        typed_expr_with_ownership(
                            TypedExprKind::Null,
                            IrType::Unknown("null".to_string()),
                            Ownership::Copy,
                        )
                    }),
                },
                TypedFieldInit {
                    name: "ElementType".to_string(),
                    expr: element_type.unwrap_or_else(|| {
                        typed_expr_with_ownership(
                            TypedExprKind::Null,
                            IrType::Unknown("null".to_string()),
                            Ownership::Copy,
                        )
                    }),
                },
                TypedFieldInit {
                    name: "GenericArguments".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(generic_arguments),
                        IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Properties".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(properties),
                        IrType::Array(Box::new(IrType::Class("PropertyInfo".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Methods".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(methods),
                        IrType::Array(Box::new(IrType::Class("MethodInfo".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Fields".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(fields),
                        IrType::Array(Box::new(IrType::Class("FieldInfo".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Constructors".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(constructors),
                        IrType::Array(Box::new(IrType::Class("ConstructorInfo".to_string()))),
                    ),
                },
                TypedFieldInit {
                    name: "Interfaces".to_string(),
                    expr: typed_expr(
                        TypedExprKind::ArrayLiteral(interfaces),
                        IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                    ),
                },
            ],
        },
        IrType::Class("Type".to_string()),
        Ownership::Shared,
    )
}

pub(super) fn type_object_base_type_from_ir(ty: &IrType, env: &TypeEnv) -> Option<TypedExpr> {
    let base_name = match ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) | IrType::Unknown(name) => {
            let simple = name.rsplit('.').next().unwrap_or(name);
            env.bases.get(simple).and_then(|bases| bases.first()).cloned()
        }
        IrType::String => Some("System.Object".to_string()),
        _ => None,
    }?;
    Some(type_object_expr_from_ir_impl(&IrType::Unknown(base_name), env, false))
}

pub(super) fn type_object_properties_from_ir(ty: &IrType, env: &TypeEnv, include_members: bool) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let type_name = type_object_lookup_name(ty);
    let Some(type_name) = type_name else {
        return Vec::new();
    };
    env.all_field_infos(type_name)
        .into_iter()
        .map(|(name, field)| {
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: "PropertyInfo".to_string(),
                    constructor: None,
                    args: Vec::new(),
                    fields: vec![
                        TypedFieldInit {
                            name: "Name".to_string(),
                            expr: typed_expr(TypedExprKind::String(name), IrType::String),
                        },
                        TypedFieldInit {
                            name: "PropertyType".to_string(),
                            expr: type_object_expr_from_ir_shallow(&field.ty, env),
                        },
                        TypedFieldInit {
                            name: "CanRead".to_string(),
                            expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                        },
                        TypedFieldInit {
                            name: "CanWrite".to_string(),
                            expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                        },
                    ],
                },
                IrType::Class("PropertyInfo".to_string()),
                Ownership::Shared,
            )
        })
        .collect()
}

pub(super) fn type_object_methods_from_ir(ty: &IrType, env: &TypeEnv, include_members: bool) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let mut methods = builtin_methods_from_ir(ty, env);
    if let Some(type_name) = type_object_lookup_name(ty) {
        for ((owner, method_name), signatures) in &env.methods {
            if owner != type_name {
                continue;
            }
            for signature in signatures {
                let parameters = signature
                    .params
                    .iter()
                    .map(|param| {
                        type_object_expr_from_ir_shallow(param, env)
                    })
                    .collect::<Vec<_>>();
                methods.push(typed_expr_with_ownership(
                    TypedExprKind::NewObject {
                        type_name: "MethodInfo".to_string(),
                        constructor: None,
                        args: Vec::new(),
                        fields: vec![
                            TypedFieldInit {
                                name: "Name".to_string(),
                                expr: typed_expr(
                                    TypedExprKind::String(method_name.clone()),
                                    IrType::String,
                                ),
                            },
                            TypedFieldInit {
                                name: "ReturnType".to_string(),
                                expr: type_object_expr_from_ir_shallow(&signature.return_type, env),
                            },
                            TypedFieldInit {
                                name: "ParameterTypes".to_string(),
                                expr: typed_expr(
                                    TypedExprKind::ArrayLiteral(parameters),
                                    IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                                ),
                            },
                            TypedFieldInit {
                                name: "IsStatic".to_string(),
                                expr: typed_expr(TypedExprKind::Bool(signature.is_static), IrType::Bool),
                            },
                        ],
                    },
                    IrType::Class("MethodInfo".to_string()),
                    Ownership::Shared,
                ));
            }
        }
    }
    methods
}

pub(super) fn type_object_fields_from_ir(ty: &IrType, env: &TypeEnv, include_members: bool) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let Some(type_name) = type_object_lookup_name(ty) else {
        return Vec::new();
    };
    env.all_field_infos(type_name)
        .into_iter()
        .map(|(name, field)| {
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: "FieldInfo".to_string(),
                    constructor: None,
                    args: Vec::new(),
                    fields: vec![
                        TypedFieldInit {
                            name: "Name".to_string(),
                            expr: typed_expr(TypedExprKind::String(name), IrType::String),
                        },
                        TypedFieldInit {
                            name: "FieldType".to_string(),
                            expr: type_object_expr_from_ir_shallow(&field.ty, env),
                        },
                    ],
                },
                IrType::Class("FieldInfo".to_string()),
                Ownership::Shared,
            )
        })
        .collect()
}

pub(super) fn type_object_constructors_from_ir(
    ty: &IrType,
    env: &TypeEnv,
    include_members: bool,
) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let Some(type_name) = type_object_lookup_name(ty) else {
        return Vec::new();
    };
    env.constructors
        .get(type_name)
        .into_iter()
        .flat_map(|signatures| signatures.iter())
        .map(|signature| {
            let parameters = signature
                .params
                .iter()
                .map(|param| type_object_expr_from_ir_shallow(param, env))
                .collect::<Vec<_>>();
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: "ConstructorInfo".to_string(),
                    constructor: None,
                    args: Vec::new(),
                    fields: vec![
                        TypedFieldInit {
                            name: "Name".to_string(),
                            expr: typed_expr(
                                TypedExprKind::String(type_name.to_string()),
                                IrType::String,
                            ),
                        },
                        TypedFieldInit {
                            name: "ParameterTypes".to_string(),
                            expr: typed_expr(
                                TypedExprKind::ArrayLiteral(parameters),
                                IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                            ),
                        },
                    ],
                },
                IrType::Class("ConstructorInfo".to_string()),
                Ownership::Shared,
            )
        })
        .collect()
}

pub(super) fn type_object_interfaces_from_ir(ty: &IrType, env: &TypeEnv, include_members: bool) -> Vec<TypedExpr> {
    if !include_members {
        return Vec::new();
    }
    let Some(type_name) = type_object_lookup_name(ty) else {
        return Vec::new();
    };
    env.bases
        .get(type_name)
        .into_iter()
        .flat_map(|bases| bases.iter())
        .map(|base| type_object_expr_from_ir_shallow(&IrType::Unknown(base.clone()), env))
        .collect()
}

pub(super) fn builtin_methods_from_ir(ty: &IrType, env: &TypeEnv) -> Vec<TypedExpr> {
    let mut methods = Vec::new();
    let entries: &[(&str, &[&str])] = match ty {
        IrType::String => &[
            ("ToLower", &[]),
            ("ToLowerInvariant", &[]),
            ("Substring", &["int"]),
            ("Split", &["string"]),
            ("Replace", &["string", "string"]),
            ("Trim", &[]),
            ("StartsWith", &["string", "StringComparison"]),
            ("Equals", &["string", "StringComparison"]),
            ("Contains", &["string"]),
        ],
        IrType::Class(name) | IrType::Struct(name) if name == "DateTime" || name == "System.DateTime" => {
            &[("CompareTo", &["DateTime"])]
        }
        IrType::Class(name) | IrType::Struct(name) if name == "Type" || name == "System.Type" => {
            &[("GetGenericTypeDefinition", &[]), ("GetGenericArguments", &[]), ("GetMethod", &["string", "Type[]"]), ("GetProperty", &["string", "object"]), ("GetProperties", &[])]
        }
        _ => &[],
    };
    for (name, params) in entries {
        let parameter_types = params
            .iter()
            .map(|param| match *param {
                "int" => type_object_expr_from_ir_shallow(&IrType::Int, env),
                "string" => type_object_expr_from_ir_shallow(&IrType::String, env),
                "DateTime" => type_object_expr_from_ir_shallow(&IrType::Unknown("DateTime".to_string()), env),
                "StringComparison" => type_object_expr_from_ir_shallow(&IrType::Unknown("StringComparison".to_string()), env),
                "object" => type_object_expr_from_ir_shallow(&IrType::Unknown("object".to_string()), env),
                "Type[]" => type_object_expr_from_ir_shallow(&IrType::Array(Box::new(IrType::Class("Type".to_string()))), env),
                _ => type_object_expr_from_ir_shallow(&IrType::Unknown((*param).to_string()), env),
            })
            .collect::<Vec<_>>();
        methods.push(typed_expr_with_ownership(
            TypedExprKind::NewObject {
                type_name: "MethodInfo".to_string(),
                constructor: None,
                args: Vec::new(),
                fields: vec![
                    TypedFieldInit {
                        name: "Name".to_string(),
                        expr: typed_expr(TypedExprKind::String((*name).to_string()), IrType::String),
                    },
                    TypedFieldInit {
                        name: "ReturnType".to_string(),
                        expr: typed_expr(
                            TypedExprKind::String(if *name == "GetGenericArguments" {
                                "Type[]".to_string()
                            } else if *name == "GetProperties" {
                                "PropertyInfo[]".to_string()
                            } else if *name == "GetMethod" {
                                "MethodInfo".to_string()
                            } else if *name == "GetProperty" {
                                "PropertyInfo".to_string()
                            } else if *name == "GetGenericTypeDefinition" {
                                "Type".to_string()
                            } else if *name == "CompareTo" {
                                "int".to_string()
                            } else {
                                "void".to_string()
                            }),
                            IrType::String,
                        ),
                    },
                    TypedFieldInit {
                        name: "ParameterTypes".to_string(),
                        expr: typed_expr(
                            TypedExprKind::ArrayLiteral(parameter_types),
                            IrType::Array(Box::new(IrType::Class("Type".to_string()))),
                        ),
                    },
                    TypedFieldInit {
                        name: "IsStatic".to_string(),
                        expr: typed_expr(TypedExprKind::Bool(true), IrType::Bool),
                    },
                ],
            },
            IrType::Class("MethodInfo".to_string()),
            Ownership::Shared,
        ));
    }
    methods
}

pub(super) fn type_object_lookup_name(ty: &IrType) -> Option<&str> {
    match ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) | IrType::Unknown(name) => {
            Some(name.rsplit('.').next().unwrap_or(name))
        }
        IrType::String => Some("string"),
        IrType::Bool => Some("bool"),
        IrType::Int => Some("int"),
        IrType::Long => Some("long"),
        IrType::Double => Some("double"),
        IrType::UInt => Some("uint"),
        IrType::Byte => Some("byte"),
        IrType::Short => Some("short"),
        IrType::Decimal => Some("decimal"),
        _ => None,
    }
}

pub(super) fn type_object_full_name_from_ir(ty: &IrType) -> String {
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
        IrType::Array(inner) => format!("{}[]", type_object_full_name_from_ir(inner)),
        IrType::Ref(inner) => format!("ref {}", type_object_full_name_from_ir(inner)),
        IrType::Struct(name)
        | IrType::Class(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => name.clone(),
        IrType::Exception => "Exception".to_string(),
        IrType::List(inner) => format!("List<{}>", type_object_full_name_from_ir(inner)),
        IrType::Dictionary(key, value) => format!(
            "Dictionary<{},{}>",
            type_object_full_name_from_ir(key),
            type_object_full_name_from_ir(value)
        ),
        IrType::Enumerable(inner) => format!("ICollection<{}>", type_object_full_name_from_ir(inner)),
        IrType::Thread => "Thread".to_string(),
        IrType::Task(inner) => format!("Task<{}>", type_object_full_name_from_ir(inner)),
        IrType::Function { params, return_type } => format!(
            "Func<{}{}>",
            params
                .iter()
                .map(type_object_full_name_from_ir)
                .collect::<Vec<_>>()
                .join(","),
            if matches!(return_type.as_ref(), IrType::Void) {
                String::new()
            } else {
                format!(
                    "{}{}",
                    if params.is_empty() { "" } else { "," },
                    type_object_full_name_from_ir(return_type)
                )
            }
        ),
        IrType::Weak(inner) => format!("weakref<{}>", type_object_full_name_from_ir(inner)),
        IrType::Nullable(inner) => format!("{}?", type_object_full_name_from_ir(inner)),
    }
}

pub(super) fn type_object_generic_definition_name_from_ir(ty: &IrType) -> String {
    match ty {
        IrType::List(_) => "List<>".to_string(),
        IrType::Dictionary(_, _) => "Dictionary<,>".to_string(),
        IrType::Enumerable(_) => "ICollection<>".to_string(),
        IrType::Task(_) => "Task<>".to_string(),
        IrType::Nullable(_) => "Nullable<>".to_string(),
        IrType::Function { params, return_type } => {
            let placeholders = std::iter::repeat("")
                .take(params.len() + if matches!(return_type.as_ref(), IrType::Void) { 0 } else { 1 })
                .collect::<Vec<_>>()
                .join(",");
            format!("Func<{}>", placeholders)
        }
        _ => type_object_full_name_from_ir(ty),
    }
}

pub(super) fn type_object_generic_arguments_from_ir(ty: &IrType, env: &TypeEnv) -> Vec<TypedExpr> {
    match ty {
        IrType::List(inner)
        | IrType::Enumerable(inner)
        | IrType::Task(inner)
        | IrType::Nullable(inner)
        | IrType::Weak(inner) => vec![type_object_expr_from_ir_shallow(inner, env)],
        IrType::Dictionary(key, value) => vec![
            type_object_expr_from_ir_shallow(key, env),
            type_object_expr_from_ir_shallow(value, env),
        ],
        IrType::Function { params, return_type } => {
            let mut args = params
                .iter()
                .map(|param| type_object_expr_from_ir_shallow(param, env))
                .collect::<Vec<_>>();
            if !matches!(return_type.as_ref(), IrType::Void) {
                args.push(type_object_expr_from_ir_shallow(return_type, env));
            }
            args
        }
        _ => Vec::new(),
    }
}

