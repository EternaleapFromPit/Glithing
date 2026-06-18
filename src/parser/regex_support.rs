use super::*;
use super::helpers::*;

pub(super) fn synthesize_generated_regex_methods(program: &mut Program) {
    for function in &mut program.functions {
        synthesize_generated_regex_function(function);
    }
    for ty in &mut program.types {
        for method in &mut ty.methods {
            synthesize_generated_regex_function(method);
        }
    }
}

pub(super) fn synthesize_generated_regex_function(function: &mut Function) {
    if !function.body.is_empty() {
        return;
    }
    if !function
        .attributes
        .iter()
        .any(|attribute| attribute.name.contains("GeneratedRegex"))
    {
        return;
    }
    let Some(pattern) = function.attributes.iter().find_map(|attribute| {
        if !attribute.name.contains("GeneratedRegex") {
            return None;
        }
        attribute.args.first().and_then(|arg| match arg {
            Expr::String(value) => Some(value.clone()),
            _ => None,
        })
    }) else {
        return;
    };
    let return_type_name = type_syntax_name(&function.return_type);
    function.body = vec![Stmt::Return(Some(Expr::NewObject {
        type_name: return_type_name,
        args: vec![Expr::String(pattern)],
        fields: Vec::new(),
    }))];
}
