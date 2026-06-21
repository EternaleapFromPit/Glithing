use super::*;

pub(super) fn resolve_typed_startup(
    program: &Program,
    types: &[TypedType],
    functions: &[TypedFunction],
    startup_object: Option<&str>,
) -> Result<Option<TypedStartup>, String> {
    if let Some(startup_object) = startup_object.map(str::trim).filter(|value| !value.is_empty()) {
        return resolve_explicit_startup(program, types, startup_object)
            .map(Some)
            .or_else(|err| Err(format!("invalid StartupObject '{}': {err}", startup_object)));
    }

    let mut top_level_candidates = Vec::new();
    for function in functions {
        if matches_startup_name(&function.name) {
            if let Some(startup) = typed_startup_from_function(function) {
                if function.symbol == "main" {
                    return Ok(Some(startup));
                }
                top_level_candidates.push(startup);
            }
        }
    }
    if top_level_candidates.len() == 1 {
        return Ok(top_level_candidates.into_iter().next());
    }
    if top_level_candidates.len() > 1 {
        let zero_arg = top_level_candidates
            .iter()
            .filter(|candidate| !candidate.accepts_args)
            .cloned()
            .collect::<Vec<_>>();
        if zero_arg.len() == 1 {
            return Ok(zero_arg.into_iter().next());
        }
        let with_args = top_level_candidates
            .iter()
            .filter(|candidate| candidate.accepts_args)
            .cloned()
            .collect::<Vec<_>>();
        if with_args.len() == 1 {
            return Ok(with_args.into_iter().next());
        }
        let names = top_level_candidates
            .iter()
            .map(|candidate| candidate.symbol.clone())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "multiple startup candidates found ({names}); set <StartupObject> in the project file"
        ));
    }

    let mut type_candidates = Vec::new();
    for (ast_type, typed_type) in program.types.iter().zip(types.iter()) {
        for (ast_method, typed_method) in ast_type.methods.iter().zip(typed_type.methods.iter()) {
            if ast_method.is_static && matches_startup_name(&ast_method.name) {
                if let Some(startup) = typed_startup_from_function(typed_method) {
                    type_candidates.push(startup);
                }
            }
        }
    }

    match type_candidates.len() {
        0 => Ok(None),
        1 => Ok(type_candidates.into_iter().next()),
        _ => {
            let zero_arg = type_candidates
                .iter()
                .filter(|candidate| !candidate.accepts_args)
                .cloned()
                .collect::<Vec<_>>();
            if zero_arg.len() == 1 {
                return Ok(zero_arg.into_iter().next());
            }
            let with_args = type_candidates
                .iter()
                .filter(|candidate| candidate.accepts_args)
                .cloned()
                .collect::<Vec<_>>();
            if with_args.len() == 1 {
                return Ok(with_args.into_iter().next());
            }
            Err(format!(
                "multiple startup candidates found ({}); set <StartupObject> in the project file",
                type_candidates
                    .iter()
                    .map(|candidate| candidate.symbol.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        }
    }
}

fn resolve_explicit_startup(
    program: &Program,
    types: &[TypedType],
    startup_object: &str,
) -> Result<TypedStartup, String> {
    let segments = startup_object
        .split('.')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.is_empty() {
        return Err("empty startup object".to_string());
    }
    let (type_name, method_candidates) = match segments.last().copied() {
        Some("Main") => (segments[..segments.len() - 1].join("."), vec!["Main", "main"]),
        Some("main") => (segments[..segments.len() - 1].join("."), vec!["main", "Main"]),
        _ => (segments.join("."), vec!["Main", "main"]),
    };
    if type_name.is_empty() {
        return Err("StartupObject must name a type or fully qualified type".to_string());
    }

    let mut matches = Vec::new();
    for (ast_type, typed_type) in program.types.iter().zip(types.iter()) {
        let full_name = if typed_type.namespace.is_empty() {
            typed_type.name.clone()
        } else {
            format!("{}.{}", typed_type.namespace.join("."), typed_type.name)
        };
        if full_name != type_name && typed_type.name != type_name {
            continue;
        }
        for candidate_name in &method_candidates {
            for (ast_method, typed_method) in ast_type.methods.iter().zip(typed_type.methods.iter()) {
                if ast_method.is_static && ast_method.name == *candidate_name {
                    if let Some(startup) = typed_startup_from_function(typed_method) {
                        matches.push(startup);
                    }
                }
            }
            if !matches.is_empty() {
                break;
            }
        }
    }

    match matches.len() {
        0 => Err(format!("could not find a static Main/main method on type '{}'", type_name)),
        1 => Ok(matches.remove(0)),
        _ => Err(format!(
            "multiple startup methods matched type '{}'; reduce overloads or choose a more specific StartupObject",
            type_name
        )),
    }
}

fn matches_startup_name(name: &str) -> bool {
    name == "main" || name == "Main"
}

fn typed_startup_from_function(function: &TypedFunction) -> Option<TypedStartup> {
    let accepts_args = function.params.is_empty()
        || (function.params.len() == 1
            && function.params[0].name == "args"
            && matches!(function.params[0].ty, IrType::Array(ref item) if item.as_ref() == &IrType::String));
    if !accepts_args {
        return None;
    }
    let returns_int = function.return_type == IrType::Int;
    if !returns_int && function.return_type != IrType::Void {
        return None;
    }
    Some(TypedStartup {
        symbol: function.symbol.clone(),
        accepts_args: function.params.len() == 1,
        returns_int,
    })
}
