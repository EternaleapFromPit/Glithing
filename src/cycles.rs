use std::collections::{HashMap, HashSet};

use crate::ast::*;
use crate::diagnostics::{
    has_type_path, locate_source, referenced_class, render_diagnostic, type_syntax_display,
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
            if let Some(target) = referenced_class(&field.ty, &classes) {
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
            let Some(target) = referenced_class(&field.ty, &classes) else {
                continue;
            };
            if !has_type_path(&graph, target, &ty.name, &mut HashSet::new()) {
                continue;
            }
            let needle = format!("{} {}", type_syntax_display(&field.ty), field.name);
            let (line, col, snippet) = locate_source(source, &needle);
            let message = format!(
                "reference cycle detected: class '{}' field '{}' participates in a potential ownership cycle {} -> {}",
                ty.name, field.name, ty.name, target
            );
            let help = format!(
                "replace '{} {}' with 'Weak<{}> {}' to break the cycle and avoid memory leaks",
                type_syntax_display(&field.ty),
                field.name,
                target,
                field.name
            );
            warnings.push(render_diagnostic(
                "GL3007", line, col, snippet, &message, &help,
            ));
        }
    }
    warnings
}
