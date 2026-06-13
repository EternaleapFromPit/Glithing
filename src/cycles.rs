use std::collections::{HashMap, HashSet};

use crate::ast::*;
use crate::diagnostics::{
    has_type_path, locate_source, render_diagnostic, type_syntax_display,
};

pub(crate) fn check_reference_cycles(source: &str, program: &Program) -> Vec<String> {
    let classes = program
        .types
        .iter()
        .filter(|ty| matches!(ty.kind, TypeKind::Class))
        .map(|ty| ty.name.clone())
        .collect::<HashSet<_>>();
    let mut graph = HashMap::<String, Vec<String>>::new();
    for ty in &program.types {
        for field in &ty.fields {
            for target in owned_referenced_classes(&field.ty, &classes) {
                graph
                    .entry(ty.name.clone())
                    .or_default()
                    .push(target.clone());
            }
        }
    }
    let mut warnings = Vec::new();
    for ty in &program.types {
        for field in &ty.fields {
            let targets = owned_referenced_classes(&field.ty, &classes);
            if targets.is_empty() {
                continue;
            }
            for target in targets {
                if !has_type_path(&graph, &target, &ty.name, &mut HashSet::new()) {
                    continue;
                }
                let needle = format!("{} {}", type_syntax_display(&field.ty), field.name);
                let (file, line, col, snippet) = locate_source(source, &needle);
                let message = format!(
                    "reference cycle detected: class '{}' field '{}' participates in a potential ownership cycle {} -> {}",
                    ty.name, field.name, ty.name, target
                );
                let help = if is_weakable_type(&field.ty) {
                    format!(
                        "replace '{}' with a weak or borrowed form to break the ownership chain and avoid memory leaks",
                        type_syntax_display(&field.ty)
                    )
                } else {
                    format!(
                        "replace '{} {}' with 'Weak<{}> {}' to break the cycle and avoid memory leaks",
                        type_syntax_display(&field.ty),
                        field.name,
                        target,
                        field.name
                    )
                };
                warnings.push(render_diagnostic(
                    "GL3007",
                    file.as_deref(),
                    line,
                    col,
                    snippet,
                    &message,
                    &help,
                ));
            }
        }
    }
    warnings
}

fn owned_referenced_classes(ty: &TypeSyntax, classes: &HashSet<String>) -> Vec<String> {
    let mut out = Vec::new();
    collect_owned_referenced_classes(ty, classes, &mut out, true);
    out
}

fn collect_owned_referenced_classes(
    ty: &TypeSyntax,
    classes: &HashSet<String>,
    out: &mut Vec<String>,
    owned: bool,
) {
    if !owned {
        return;
    }
    match ty {
        TypeSyntax::Named(name) => {
            if classes.contains(name) {
                out.push(name.clone());
            }
        }
        TypeSyntax::Array(inner)
        | TypeSyntax::List(inner)
        | TypeSyntax::Task(inner)
        | TypeSyntax::Nullable(inner) => collect_owned_referenced_classes(inner, classes, out, true),
        TypeSyntax::Dictionary(key, value) => {
            collect_owned_referenced_classes(key, classes, out, true);
            collect_owned_referenced_classes(value, classes, out, true);
        }
        TypeSyntax::Ref(_)
        | TypeSyntax::IEnumerable(_)
        | TypeSyntax::Thread
        | TypeSyntax::String
        | TypeSyntax::Scalar(_)
        | TypeSyntax::Void => {}
        TypeSyntax::GenericNamed { name, args } => {
            if is_ownership_bearing_generic(name) {
                for arg in args {
                    collect_owned_referenced_classes(arg, classes, out, true);
                }
            }
        }
    }
}

fn is_ownership_bearing_generic(name: &str) -> bool {
    matches!(
        name,
        "Rc"
            | "System.Ownership.Rc"
            | "own"
            | "System.Ownership.own"
            | "shared"
            | "System.Ownership.shared"
            | "List"
            | "System.Collections.Generic.List"
            | "Dictionary"
            | "System.Collections.Generic.Dictionary"
            | "Task"
            | "System.Threading.Tasks.Task"
            | "ValueTask"
            | "System.Threading.Tasks.ValueTask"
    )
}

fn is_weakable_type(ty: &TypeSyntax) -> bool {
    matches!(
        ty,
        TypeSyntax::Named(_) | TypeSyntax::GenericNamed { .. } | TypeSyntax::Array(_)
            | TypeSyntax::List(_)
            | TypeSyntax::Dictionary(_, _)
            | TypeSyntax::Task(_)
            | TypeSyntax::Nullable(_)
    )
}
