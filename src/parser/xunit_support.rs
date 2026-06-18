use super::*;
use super::helpers::*;

pub(super) fn synthesize_xunit_fact_tests(parser: &mut Parser, program: &Program) {
    for ty in &program.types {
        let class_name = qualified_type_name(&ty.namespace, &ty.name);
        for method in &ty.methods {
            if method.is_extern {
                continue;
            }
            if is_skip_attribute(&method.attributes) {
                continue;
            }
            if is_fact_attribute(&method.attributes)
                && method.generic_params.is_empty()
                && method.params.is_empty()
            {
                let invoke = Expr::MethodCall {
                    target: Box::new(Expr::NewObject {
                        type_name: ty.name.clone(),
                        args: Vec::new(),
                        fields: Vec::new(),
                    }),
                    name: method.name.clone(),
                    args: Vec::new(),
                };
                let invoke = if is_task_like_type(&method.return_type) {
                    Expr::MethodCall {
                        target: Box::new(invoke),
                        name: "Wait".to_string(),
                        args: Vec::new(),
                    }
                } else {
                    invoke
                };
                parser.test_registrations.push(Stmt::Expr(Expr::FunctionCall {
                    name: "XUnit_AddTest".to_string(),
                    args: vec![
                        Expr::String(class_name.clone()),
                        Expr::String(method.name.clone()),
                        Expr::Lambda {
                            params: Vec::new(),
                            body: Box::new(invoke),
                        },
                    ],
                }));
            }
            if !is_theory_attribute(&method.attributes) || method.params.is_empty() {
                continue;
            }
            let inline_data = method
                .attributes
                .iter()
                .filter(|attribute| is_inline_data_attribute(&attribute.name))
                .collect::<Vec<_>>();
            for (index, attribute) in inline_data.iter().enumerate() {
                register_xunit_theory_case(
                    parser,
                    &class_name,
                    ty,
                    method,
                    &format!("{}[{index}]", method.name),
                    attribute.args.clone(),
                );
            }
            for attribute in method
                .attributes
                .iter()
                .filter(|attribute| is_member_data_attribute(&attribute.name))
            {
                let Some(member_name) = attribute.args.first().and_then(xunit_string_value) else {
                    continue;
                };
                let rows = xunit_rows_from_member(program, ty, &member_name);
                for (index, row) in rows.into_iter().enumerate() {
                    register_xunit_theory_case(
                        parser,
                        &class_name,
                        ty,
                        method,
                        &format!("{}[member:{member_name}:{index}]", method.name),
                        row,
                    );
                }
            }
            for attribute in method
                .attributes
                .iter()
                .filter(|attribute| is_class_data_attribute(&attribute.name))
            {
                let Some(type_name) = attribute.args.first().and_then(xunit_type_name_value) else {
                    continue;
                };
                let rows = xunit_rows_from_type(program, &type_name);
                for (index, row) in rows.into_iter().enumerate() {
                    register_xunit_theory_case(
                        parser,
                        &class_name,
                        ty,
                        method,
                        &format!("{}[class:{type_name}:{index}]", method.name),
                        row,
                    );
                }
            }
        }
    }
}

pub(super) fn register_xunit_theory_case(
    parser: &mut Parser,
    class_name: &str,
    ty: &TypeDef,
    method: &Function,
    test_name: &str,
    args: Vec<Expr>,
) {
    if args.len() != method.params.len() {
        return;
    }
    let invoke = Expr::MethodCall {
        target: Box::new(Expr::NewObject {
            type_name: ty.name.clone(),
            args: Vec::new(),
            fields: Vec::new(),
        }),
        name: method.name.clone(),
        args,
    };
    let invoke = if is_task_like_type(&method.return_type) {
        Expr::MethodCall {
            target: Box::new(invoke),
            name: "Wait".to_string(),
            args: Vec::new(),
        }
    } else {
        invoke
    };
    parser.test_registrations.push(Stmt::Expr(Expr::FunctionCall {
        name: "XUnit_AddTest".to_string(),
        args: vec![
            Expr::String(class_name.to_string()),
            Expr::String(test_name.to_string()),
            Expr::Lambda {
                params: Vec::new(),
                body: Box::new(invoke),
            },
        ],
    }));
}

pub(super) fn is_fact_attribute(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        matches!(
            attribute.name.rsplit('.').next().unwrap_or(&attribute.name),
            "Fact" | "FactAttribute"
        )
    })
}

pub(super) fn is_theory_attribute(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        matches!(
            attribute.name.rsplit('.').next().unwrap_or(&attribute.name),
            "Theory" | "TheoryAttribute"
        )
    })
}

pub(super) fn is_inline_data_attribute(name: &str) -> bool {
    matches!(name.rsplit('.').next().unwrap_or(name), "InlineData" | "InlineDataAttribute")
}

pub(super) fn is_skip_attribute(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        matches!(
            attribute.name.rsplit('.').next().unwrap_or(&attribute.name),
            "Skip" | "SkipAttribute"
        )
    })
}

pub(super) fn is_member_data_attribute(name: &str) -> bool {
    matches!(name.rsplit('.').next().unwrap_or(name), "MemberData" | "MemberDataAttribute")
}

pub(super) fn is_class_data_attribute(name: &str) -> bool {
    matches!(name.rsplit('.').next().unwrap_or(name), "ClassData" | "ClassDataAttribute")
}

pub(super) fn xunit_string_value(expr: &Expr) -> Option<String> {
    match expr {
        Expr::String(value) => Some(value.clone()),
        _ => None,
    }
}

pub(super) fn xunit_type_name_value(expr: &Expr) -> Option<String> {
    match expr {
        Expr::String(value) => Some(value.clone()),
        Expr::NewObject {
            type_name,
            fields,
            ..
        } if type_name == "Type" => fields.iter().find_map(|field| {
            if field.name == "FullName" {
                xunit_string_value(&field.expr)
            } else {
                None
            }
        }),
        _ => None,
    }
}

pub(super) fn xunit_rows_from_member(_program: &Program, ty: &TypeDef, member_name: &str) -> Vec<Vec<Expr>> {
    let Some(method) = ty.methods.iter().find(|method| {
        method.is_static
            && method.generic_params.is_empty()
            && method.params.is_empty()
            && (method.name == member_name || method.name == property_getter_name(member_name))
    }) else {
        return Vec::new();
    };
    xunit_rows_from_data_function(method).unwrap_or_default()
}

pub(super) fn xunit_rows_from_type(program: &Program, type_name: &str) -> Vec<Vec<Expr>> {
    let Some(ty) = program
        .types
        .iter()
        .find(|ty| qualified_type_name(&ty.namespace, &ty.name) == type_name)
    else {
        return Vec::new();
    };
    for candidate in ty.methods.iter().filter(|method| {
        method.is_static && method.generic_params.is_empty() && method.params.is_empty()
    }) {
        if candidate.name == "GetData" || candidate.name == "get_Data" {
            if let Some(rows) = xunit_rows_from_data_function(candidate) {
                return rows;
            }
        }
    }
    Vec::new()
}

pub(super) fn xunit_rows_from_data_function(method: &Function) -> Option<Vec<Vec<Expr>>> {
    let [Stmt::Return(Some(expr))] = method.body.as_slice() else {
        return None;
    };
    xunit_rows_from_data_expr(expr)
}

pub(super) fn xunit_rows_from_data_expr(expr: &Expr) -> Option<Vec<Vec<Expr>>> {
    let values = match expr {
        Expr::ArrayLiteral(values) => values,
        Expr::NewArray { values, .. } => values,
        _ => return None,
    };
    if values.is_empty() {
        return Some(Vec::new());
    }
    if values
        .iter()
        .all(|value| matches!(value, Expr::ArrayLiteral(_) | Expr::NewArray { .. }))
    {
        let mut rows = Vec::new();
        for value in values {
            let row = match value {
                Expr::ArrayLiteral(row) => row.clone(),
                Expr::NewArray { values, .. } => values.clone(),
                _ => unreachable!(),
            };
            rows.push(row);
        }
        return Some(rows);
    }
    Some(vec![values.clone()])
}

