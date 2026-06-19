use super::*;
use super::helpers::*;

pub(super) fn find_generic_definition<'a>(program: &'a TypedProgram, symbol: &str) -> Option<&'a TypedFunction> {
    program
        .functions
        .iter()
        .find(|function| function.symbol == symbol && !function.generic_params.is_empty())
        .or_else(|| {
            for ty in &program.types {
                if let Some(method) = ty
                    .methods
                    .iter()
                    .find(|method| method.symbol == symbol && !method.generic_params.is_empty())
                {
                    return Some(method);
                }
                if let Some(constructor) = ty
                    .constructors
                    .iter()
                    .find(|constructor| constructor.symbol == symbol && !constructor.generic_params.is_empty())
                {
                    return Some(constructor);
                }
            }
            None
        })
}

pub(super) fn is_concrete_instantiation(args: &[IrType]) -> bool {
    args.iter().all(is_concrete_ir_type)
}

pub(super) fn is_safe_generic_wrapper_symbol(symbol: &str) -> bool {
    matches!(
        symbol,
        s if s.starts_with("make_view__g")
            || s.starts_with("make_owned__g")
            || s.starts_with("make_shared__g")
            || s.starts_with("make_borrow__g")
    )
}

pub(super) fn is_concrete_ir_type(ty: &IrType) -> bool {
    match ty {
        IrType::Unknown(_) => false,
        IrType::Array(inner)
        | IrType::Ref(inner)
        | IrType::Weak(inner)
        | IrType::List(inner)
        | IrType::Enumerable(inner)
        | IrType::Task(inner)
        | IrType::Nullable(inner) => is_concrete_ir_type(inner),
        IrType::Dictionary(key, value) => is_concrete_ir_type(key) && is_concrete_ir_type(value),
        IrType::Function { params, return_type } => {
            params.iter().all(is_concrete_ir_type) && is_concrete_ir_type(return_type)
        }
        _ => true,
    }
}

pub(super) fn collect_generic_instantiations_from_function(
    function: &TypedFunction,
    generic_symbols: &HashSet<String>,
    specialized_instance_symbols: &HashMap<(String, String), String>,
    output: &mut Vec<GenericInstantiation>,
) {
    collect_llvm_instantiation(&function.return_type, output);
    for param in &function.params {
        collect_llvm_instantiation(&param.ty, output);
    }
    for local in &function.locals {
        collect_llvm_instantiation(&local.ty, output);
    }
    collect_generic_call_instantiations_stmts(
        &function.body,
        generic_symbols,
        specialized_instance_symbols,
        output,
    );
}

pub(super) fn collect_llvm_instantiation(ty: &IrType, output: &mut Vec<GenericInstantiation>) {
    let instantiation = match ty {
        IrType::List(inner) => {
            collect_llvm_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "List".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Dictionary(key, value) => {
            collect_llvm_instantiation(key, output);
            collect_llvm_instantiation(value, output);
            Some(GenericInstantiation {
                name: "Dictionary".to_string(),
                args: vec![key.as_ref().clone(), value.as_ref().clone()],
            })
        }
        IrType::Enumerable(inner) => {
            collect_llvm_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "IEnumerable".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Task(inner) => {
            collect_llvm_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "Task".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Array(inner) | IrType::Ref(inner) | IrType::Weak(inner) => {
            collect_llvm_instantiation(inner, output);
            None
        }
        IrType::Function {
            params,
            return_type,
        } => {
            for param in params {
                collect_llvm_instantiation(param, output);
            }
            collect_llvm_instantiation(return_type, output);
            None
        }
        _ => None,
    };
    if let Some(instantiation) = instantiation {
        if !output.contains(&instantiation) {
            output.push(instantiation);
        }
    }
}

pub(super) fn collect_generic_call_instantiations_stmts(
    stmts: &[TypedStmt],
    generic_symbols: &HashSet<String>,
    specialized_instance_symbols: &HashMap<(String, String), String>,
    output: &mut Vec<GenericInstantiation>,
) {
    for stmt in stmts {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                collect_llvm_instantiation(&binding.ty, output);
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            TypedStmtKind::Assign { expr, .. }
            | TypedStmtKind::Print(expr)
            | TypedStmtKind::Expr(expr)
            | TypedStmtKind::Return(Some(expr))
            | TypedStmtKind::Throw(expr) => {
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            TypedStmtKind::AssignTarget { target, expr } => {
                collect_generic_call_instantiations_expr(
                    target,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            TypedStmtKind::Block(body)
            | TypedStmtKind::While { body, .. }
            | TypedStmtKind::For { body, .. }
            | TypedStmtKind::ForEach { body, .. } => {
                collect_generic_call_instantiations_stmts(
                    body,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_generic_call_instantiations_expr(
                    condition,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    then_body,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    else_body,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                collect_generic_call_instantiations_stmts(
                    try_body,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    catch_body,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
                collect_generic_call_instantiations_stmts(
                    finally_body,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            TypedStmtKind::Switch { expr, cases, default } => {
                collect_generic_call_instantiations_expr(
                    expr,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
                for case in cases {
                    collect_generic_call_instantiations_expr(
                        &case.value,
                        generic_symbols,
                        specialized_instance_symbols,
                        output,
                    );
                    collect_generic_call_instantiations_stmts(
                        &case.body,
                        generic_symbols,
                        specialized_instance_symbols,
                        output,
                    );
                }
                collect_generic_call_instantiations_stmts(
                    default,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
        }
    }
}

pub(super) fn collect_generic_call_instantiations_expr(
    expr: &TypedExpr,
    generic_symbols: &HashSet<String>,
    specialized_instance_symbols: &HashMap<(String, String), String>,
    output: &mut Vec<GenericInstantiation>,
) {
    match &expr.kind {
        TypedExprKind::NullableSome(value) => {
            collect_generic_call_instantiations_expr(
                value,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                collect_generic_call_instantiations_expr(
                    value,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_generic_call_instantiations_expr(
                    length,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            for value in values {
                collect_generic_call_instantiations_expr(
                    value,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
        }
        TypedExprKind::Index { target, index } => {
            collect_generic_call_instantiations_expr(
                target,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
            collect_generic_call_instantiations_expr(
                index,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::Field { target, .. } => {
            collect_generic_call_instantiations_expr(
                target,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::IsPattern { expr, .. } => {
            collect_generic_call_instantiations_expr(
                expr,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::Throw(expr) | TypedExprKind::Await(expr) => {
            collect_generic_call_instantiations_expr(
                expr,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::Assign { target, value } => {
            collect_generic_call_instantiations_expr(
                target,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
            collect_generic_call_instantiations_expr(
                value,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_generic_call_instantiations_expr(
                condition,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
            collect_generic_call_instantiations_expr(
                when_true,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
            collect_generic_call_instantiations_expr(
                when_false,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_generic_call_instantiations_expr(
                left,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
            collect_generic_call_instantiations_expr(
                right,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::Unary { expr: inner, .. }
        | TypedExprKind::IncDec { target: inner, .. } => {
            collect_generic_call_instantiations_expr(
                inner,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::Lambda { body, .. } => {
            collect_generic_call_instantiations_expr(
                body,
                generic_symbols,
                specialized_instance_symbols,
                output,
            );
        }
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Function { symbol, .. } => {
                    if generic_symbols.contains(symbol) && !call.generic_args.is_empty() {
                        let instantiation = GenericInstantiation {
                            name: symbol.clone(),
                            args: call.generic_args.clone(),
                        };
                        if !output.contains(&instantiation) {
                            output.push(instantiation);
                        }
                    }
                }
                TypedCallKind::Method { target, symbol, .. } => {
                    collect_generic_call_instantiations_expr(
                        target,
                        generic_symbols,
                        specialized_instance_symbols,
                        output,
                    );
                    let resolved_symbol = resolve_generic_method_template_symbol(
                        symbol,
                        &target.ty,
                        specialized_instance_symbols,
                    );
                    if generic_symbols.contains(&resolved_symbol) && !call.generic_args.is_empty() {
                        let instantiation = GenericInstantiation {
                            name: resolved_symbol,
                            args: call.generic_args.clone(),
                        };
                        if !output.contains(&instantiation) {
                            output.push(instantiation);
                        }
                    }
                }
            }
            for arg in &call.args {
                collect_generic_call_instantiations_expr(
                    arg,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_generic_call_instantiations_expr(
                    arg,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
            for field in fields {
                collect_generic_call_instantiations_expr(
                    &field.expr,
                    generic_symbols,
                    specialized_instance_symbols,
                    output,
                );
            }
        }
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. } => {}
    }
}

pub(super) fn resolve_generic_method_template_symbol(
    base_symbol: &str,
    target_ty: &IrType,
    specialized_instance_symbols: &HashMap<(String, String), String>,
) -> String {
    match target_ty {
        IrType::Class(type_name) | IrType::Struct(type_name) | IrType::Interface(type_name) => {
            specialized_instance_symbols
                .get(&(base_symbol.to_string(), type_name.clone()))
                .cloned()
                .unwrap_or_else(|| base_symbol.to_string())
        }
        _ => base_symbol.to_string(),
    }
}

pub(super) fn specialize_typed_function(
    function: &TypedFunction,
    subst: &HashMap<String, IrType>,
    symbol: String,
) -> TypedFunction {
    let mut specialized = function.clone();
    specialized.symbol = symbol;
    specialized.generic_params.clear();
    specialized.return_type = substitute_ir_type(&specialized.return_type, subst);
    specialized.return_ownership = ownership_for_type(&specialized.return_type);
    specialized.params = specialized
        .params
        .into_iter()
        .map(|binding| specialize_typed_binding(binding, subst))
        .collect();
    specialized.locals = specialized
        .locals
        .into_iter()
        .map(|binding| specialize_typed_binding(binding, subst))
        .collect();
    specialized.body = specialized
        .body
        .into_iter()
        .map(|stmt| specialize_typed_stmt(stmt, subst))
        .collect();
    specialized
}

pub(super) fn specialize_typed_binding(binding: TypedBinding, subst: &HashMap<String, IrType>) -> TypedBinding {
    let ty = substitute_ir_type(&binding.ty, subst);
    TypedBinding {
        name: binding.name,
        ownership: ownership_for_type(&ty),
        ty,
    }
}

pub(super) fn specialize_typed_stmt(stmt: TypedStmt, subst: &HashMap<String, IrType>) -> TypedStmt {
    TypedStmt {
        kind: match stmt.kind {
            TypedStmtKind::Let { binding, expr } => TypedStmtKind::Let {
                binding: specialize_typed_binding(binding, subst),
                expr: specialize_typed_expr(expr, subst),
            },
            TypedStmtKind::Assign { name, expr } => TypedStmtKind::Assign {
                name,
                expr: specialize_typed_expr(expr, subst),
            },
            TypedStmtKind::AssignTarget { target, expr } => TypedStmtKind::AssignTarget {
                target: specialize_typed_expr(target, subst),
                expr: specialize_typed_expr(expr, subst),
            },
            TypedStmtKind::Block(body) => {
                TypedStmtKind::Block(body.into_iter().map(|stmt| specialize_typed_stmt(stmt, subst)).collect())
            }
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => TypedStmtKind::If {
                condition: specialize_typed_expr(condition, subst),
                then_body: then_body
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
                else_body: else_body
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
            },
            TypedStmtKind::Try {
                try_body,
                catch_name,
                catch_body,
                finally_body,
            } => TypedStmtKind::Try {
                try_body: try_body
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
                catch_name,
                catch_body: catch_body
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
                finally_body: finally_body
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
            },
            TypedStmtKind::Switch { expr, cases, default } => TypedStmtKind::Switch {
                expr: specialize_typed_expr(expr, subst),
                cases: cases
                    .into_iter()
                    .map(|case| TypedSwitchCase {
                        value: specialize_typed_expr(case.value, subst),
                        body: case
                            .body
                            .into_iter()
                            .map(|stmt| specialize_typed_stmt(stmt, subst))
                            .collect(),
                    })
                    .collect(),
                default: default
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
            },
            TypedStmtKind::While { condition, body } => TypedStmtKind::While {
                condition: specialize_typed_expr(condition, subst),
                body: body
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
            },
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => TypedStmtKind::For {
                init: init.map(|stmt| Box::new(specialize_typed_stmt(*stmt, subst))),
                condition: condition.map(|expr| specialize_typed_expr(expr, subst)),
                increment: increment.map(|stmt| Box::new(specialize_typed_stmt(*stmt, subst))),
                body: body
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
            },
            TypedStmtKind::ForEach {
                item,
                collection,
                body,
            } => TypedStmtKind::ForEach {
                item: specialize_typed_binding(item, subst),
                collection: specialize_typed_expr(collection, subst),
                body: body
                    .into_iter()
                    .map(|stmt| specialize_typed_stmt(stmt, subst))
                    .collect(),
            },
            TypedStmtKind::Print(expr) => TypedStmtKind::Print(specialize_typed_expr(expr, subst)),
            TypedStmtKind::Expr(expr) => TypedStmtKind::Expr(specialize_typed_expr(expr, subst)),
            TypedStmtKind::Return(expr) => TypedStmtKind::Return(expr.map(|expr| specialize_typed_expr(expr, subst))),
            TypedStmtKind::Throw(expr) => TypedStmtKind::Throw(specialize_typed_expr(expr, subst)),
            TypedStmtKind::Break => TypedStmtKind::Break,
            TypedStmtKind::Continue => TypedStmtKind::Continue,
        },
    }
}

pub(super) fn specialize_typed_field_init(field: TypedFieldInit, subst: &HashMap<String, IrType>) -> TypedFieldInit {
    TypedFieldInit {
        name: field.name,
        expr: specialize_typed_expr(field.expr, subst),
    }
}

pub(super) fn specialize_typed_expr(expr: TypedExpr, subst: &HashMap<String, IrType>) -> TypedExpr {
    let TypedExpr {
        kind,
        ty: original_ty,
        ..
    } = expr;
    let kind = match kind {
        TypedExprKind::ArrayLiteral(values) => TypedExprKind::ArrayLiteral(
            values
                .into_iter()
                .map(|value| specialize_typed_expr(value, subst))
                .collect(),
        ),
        TypedExprKind::NewArray {
            element_type,
            length,
            values,
        } => TypedExprKind::NewArray {
            element_type: substitute_ir_type(&element_type, subst),
            length: length.map(|expr| Box::new(specialize_typed_expr(*expr, subst))),
            values: values
                .into_iter()
                .map(|value| specialize_typed_expr(value, subst))
                .collect(),
        },
        TypedExprKind::Index { target, index } => TypedExprKind::Index {
            target: Box::new(specialize_typed_expr(*target, subst)),
            index: Box::new(specialize_typed_expr(*index, subst)),
        },
        TypedExprKind::Field { target, name } => TypedExprKind::Field {
            target: Box::new(specialize_typed_expr(*target, subst)),
            name,
        },
        TypedExprKind::IsPattern { expr, binding } => TypedExprKind::IsPattern {
            expr: Box::new(specialize_typed_expr(*expr, subst)),
            binding: binding.map(|binding| specialize_typed_binding(binding, subst)),
        },
        TypedExprKind::Throw(expr) => TypedExprKind::Throw(Box::new(specialize_typed_expr(*expr, subst))),
        TypedExprKind::Assign { target, value } => TypedExprKind::Assign {
            target: Box::new(specialize_typed_expr(*target, subst)),
            value: Box::new(specialize_typed_expr(*value, subst)),
        },
        TypedExprKind::Call(call) => TypedExprKind::Call(TypedCall {
            kind: match call.kind {
                TypedCallKind::Function { name, symbol } => {
                    TypedCallKind::Function { name, symbol }
                }
                TypedCallKind::Method {
                    target,
                    name,
                    symbol,
                    resolution,
                } => TypedCallKind::Method {
                    target: Box::new(specialize_typed_expr(*target, subst)),
                    name,
                    symbol,
                    resolution,
                },
            },
            args: call
                .args
                .into_iter()
                .map(|arg| specialize_typed_expr(arg, subst))
                .collect(),
            generic_args: call
                .generic_args
                .into_iter()
                .map(|arg| substitute_ir_type(&arg, subst))
                .collect(),
        }),
        TypedExprKind::NewObject {
            type_name,
            constructor,
            args,
            fields,
        } => TypedExprKind::NewObject {
            type_name,
            constructor,
            args: args
                .into_iter()
                .map(|arg| specialize_typed_expr(arg, subst))
                .collect(),
            fields: fields
                .into_iter()
                .map(|field| specialize_typed_field_init(field, subst))
                .collect(),
        },
        TypedExprKind::Lambda { params, body } => TypedExprKind::Lambda {
            params,
            body: Box::new(specialize_typed_expr(*body, subst)),
        },
        TypedExprKind::NullableSome(value) => {
            TypedExprKind::NullableSome(Box::new(specialize_typed_expr(*value, subst)))
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => TypedExprKind::Conditional {
            condition: Box::new(specialize_typed_expr(*condition, subst)),
            when_true: Box::new(specialize_typed_expr(*when_true, subst)),
            when_false: Box::new(specialize_typed_expr(*when_false, subst)),
        },
        TypedExprKind::Unary { op, expr } => TypedExprKind::Unary {
            op,
            expr: Box::new(specialize_typed_expr(*expr, subst)),
        },
        TypedExprKind::IncDec { target, delta, prefix } => TypedExprKind::IncDec {
            target: Box::new(specialize_typed_expr(*target, subst)),
            delta,
            prefix,
        },
        TypedExprKind::Binary { left, op, right } => TypedExprKind::Binary {
            left: Box::new(specialize_typed_expr(*left, subst)),
            op,
            right: Box::new(specialize_typed_expr(*right, subst)),
        },
        TypedExprKind::Await(expr) => TypedExprKind::Await(Box::new(specialize_typed_expr(*expr, subst))),
        TypedExprKind::Int(value) => TypedExprKind::Int(value),
        TypedExprKind::Float(value) => TypedExprKind::Float(value),
        TypedExprKind::Bool(value) => TypedExprKind::Bool(value),
        TypedExprKind::Null => TypedExprKind::Null,
        TypedExprKind::String(value) => TypedExprKind::String(value),
        TypedExprKind::Var(name) => TypedExprKind::Var(name),
        TypedExprKind::FunctionSymbol(name) => TypedExprKind::FunctionSymbol(name),
        TypedExprKind::Move(name) => TypedExprKind::Move(name),
        TypedExprKind::Borrow { mutable, name } => TypedExprKind::Borrow { mutable, name },
        TypedExprKind::NewCollection(ty) => TypedExprKind::NewCollection(substitute_ir_type(&ty, subst)),
        TypedExprKind::NewThread(name) => TypedExprKind::NewThread(name),
    };
    let ty = substitute_ir_type(&original_ty, subst);
    let ownership = match &kind {
        TypedExprKind::Borrow { .. } => Ownership::Borrowed,
        TypedExprKind::Move(_) => Ownership::Moved,
        TypedExprKind::Null => Ownership::Copy,
        _ => ownership_for_type(&ty),
    };
    let drop_kind = drop_kind_for_type(&ty, &ownership);
    TypedExpr {
        kind,
        ty,
        ownership,
        drop_kind,
    }
}

pub(super) fn substitute_ir_type(ty: &IrType, subst: &HashMap<String, IrType>) -> IrType {
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
        IrType::Array(inner) => IrType::Array(Box::new(substitute_ir_type(inner, subst))),
        IrType::Ref(inner) => IrType::Ref(Box::new(substitute_ir_type(inner, subst))),
        IrType::List(inner) => IrType::List(Box::new(substitute_ir_type(inner, subst))),
        IrType::Dictionary(key, value) => IrType::Dictionary(
            Box::new(substitute_ir_type(key, subst)),
            Box::new(substitute_ir_type(value, subst)),
        ),
        IrType::Enumerable(inner) => IrType::Enumerable(Box::new(substitute_ir_type(inner, subst))),
        IrType::Task(inner) => IrType::Task(Box::new(substitute_ir_type(inner, subst))),
        IrType::Nullable(inner) => IrType::Nullable(Box::new(substitute_ir_type(inner, subst))),
        IrType::Function { params, return_type } => IrType::Function {
            params: params.iter().map(|param| substitute_ir_type(param, subst)).collect(),
            return_type: Box::new(substitute_ir_type(return_type, subst)),
        },
        IrType::Weak(inner) => IrType::Weak(Box::new(substitute_ir_type(inner, subst))),
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
            let parsed = parse_substitutable_ir_type(&arg);
            render_substitutable_ir_type(&substitute_ir_type(&parsed, subst))
        })
        .collect::<Vec<_>>();
    Some(format!("{base}<{}>", rewritten.join(",")))
}

pub(super) fn parse_substitutable_ir_type(text: &str) -> IrType {
    let text = text.trim();
    if text.is_empty() {
        return IrType::Unknown("T".to_string());
    }
    match text {
        "bool" => IrType::Bool,
        "byte" => IrType::Byte,
        "short" => IrType::Short,
        "int" => IrType::Int,
        "long" => IrType::Long,
        "uint" => IrType::UInt,
        "double" => IrType::Double,
        "decimal" => IrType::Decimal,
        "string" => IrType::String,
        "void" => IrType::Void,
        "Exception" | "System.Exception" => IrType::Exception,
        _ => {
            if let Some(inner) = text.strip_suffix("[]") {
                return IrType::Array(Box::new(parse_substitutable_ir_type(inner)));
            }
            if let Some((base, args)) = split_monomorphized_type(text) {
                let base_name = base_type_name(base);
                return match base_name {
                    "List" if args.len() == 1 => {
                        IrType::List(Box::new(parse_substitutable_ir_type(&args[0])))
                    }
                    "Dictionary" if args.len() == 2 => IrType::Dictionary(
                        Box::new(parse_substitutable_ir_type(&args[0])),
                        Box::new(parse_substitutable_ir_type(&args[1])),
                    ),
                    "IEnumerable" | "ICollection" if args.len() == 1 => {
                        IrType::Enumerable(Box::new(parse_substitutable_ir_type(&args[0])))
                    }
                    "Task" | "ValueTask" if args.len() == 1 => {
                        IrType::Task(Box::new(parse_substitutable_ir_type(&args[0])))
                    }
                    "Weak" | "WeakReference" if args.len() == 1 => {
                        IrType::Weak(Box::new(parse_substitutable_ir_type(&args[0])))
                    }
                    "Nullable" if args.len() == 1 => {
                        IrType::Nullable(Box::new(parse_substitutable_ir_type(&args[0])))
                    }
                    _ => IrType::Unknown(text.to_string()),
                };
            }
            IrType::Unknown(text.to_string())
        }
    }
}

pub(super) fn render_substitutable_ir_type(ty: &IrType) -> String {
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
        IrType::Array(inner) => format!("{}[]", render_substitutable_ir_type(inner)),
        IrType::Ref(inner) => format!("ref {}", render_substitutable_ir_type(inner)),
        IrType::Struct(name)
        | IrType::Class(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => name.clone(),
        IrType::List(inner) => format!("List<{}>", render_substitutable_ir_type(inner)),
        IrType::Dictionary(key, value) => format!(
            "Dictionary<{},{}>",
            render_substitutable_ir_type(key),
            render_substitutable_ir_type(value)
        ),
        IrType::Enumerable(inner) => format!("IEnumerable<{}>", render_substitutable_ir_type(inner)),
        IrType::Thread => "Thread".to_string(),
        IrType::Task(inner) => format!("Task<{}>", render_substitutable_ir_type(inner)),
        IrType::Nullable(inner) => format!("Nullable<{}>", render_substitutable_ir_type(inner)),
        IrType::Function { params, return_type } => {
            let mut all = params
                .iter()
                .map(render_substitutable_ir_type)
                .collect::<Vec<_>>();
            all.push(render_substitutable_ir_type(return_type));
            format!("Func<{}>", all.join(","))
        }
        IrType::Exception => "Exception".to_string(),
        IrType::Weak(inner) => format!("Weak<{}>", render_substitutable_ir_type(inner)),
    }
}

pub(super) fn specialize_typed_type_owner(
    ty: &TypedType,
    concrete_name: &str,
    generic_args: &[IrType],
) -> TypedType {
    let mut subst = HashMap::new();
    for (name, arg) in ty.generic_params.iter().cloned().zip(generic_args.iter().cloned()) {
        subst.insert(name, arg);
    }
    let owner_suffix = generic_args
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    TypedType {
        package_id: ty.package_id.clone(),
        visibility: ty.visibility,
        name: concrete_name.to_string(),
        namespace: ty.namespace.clone(),
        generic_params: Vec::new(),
        symbol_id: ty.symbol_id,
        is_extension: ty.is_extension,
        kind: ty.kind,
        bases: ty.bases.clone(),
        fields: ty
            .fields
            .iter()
            .cloned()
            .map(|binding| specialize_typed_binding(binding, &subst))
            .collect(),
        constructors: ty
            .constructors
            .iter()
            .cloned()
            .map(|constructor| specialize_typed_function_owner(constructor, &subst, &owner_suffix))
            .collect(),
        methods: ty
            .methods
            .iter()
            .cloned()
            .map(|method| specialize_typed_function_owner(method, &subst, &owner_suffix))
            .collect(),
    }
}

pub(super) fn specialize_typed_function_owner(
    mut function: TypedFunction,
    subst: &HashMap<String, IrType>,
    owner_suffix: &str,
) -> TypedFunction {
    function.symbol = if owner_suffix.is_empty() {
        format!("{}__owner", function.symbol)
    } else {
        format!("{}__owner_{}", function.symbol, owner_suffix)
    };
    function.return_type = substitute_ir_type(&function.return_type, subst);
    function.return_ownership = ownership_for_type(&function.return_type);
    function.params = function
        .params
        .into_iter()
        .map(|binding| specialize_typed_binding(binding, subst))
        .collect();
    function.locals = function
        .locals
        .into_iter()
        .map(|binding| specialize_typed_binding(binding, subst))
        .collect();
    function.body = function
        .body
        .into_iter()
        .map(|stmt| specialize_typed_stmt(stmt, subst))
        .collect();
    function
}

pub(super) fn collect_generic_object_instantiations_program(
    program: &TypedProgram,
    out: &mut HashSet<String>,
) {
    for function in &program.functions {
        collect_generic_object_instantiations_function(function, out);
    }
    for ty in &program.types {
        for field in &ty.fields {
            collect_generic_object_instantiation_type(&field.ty, out);
        }
        for constructor in &ty.constructors {
            collect_generic_object_instantiations_function(constructor, out);
        }
        for method in &ty.methods {
            collect_generic_object_instantiations_function(method, out);
        }
    }
}

pub(super) fn collect_generic_object_instantiations_function(
    function: &TypedFunction,
    out: &mut HashSet<String>,
) {
    collect_generic_object_instantiation_type(&function.return_type, out);
    for param in &function.params {
        collect_generic_object_instantiation_type(&param.ty, out);
    }
    for local in &function.locals {
        collect_generic_object_instantiation_type(&local.ty, out);
    }
    collect_generic_object_instantiations_stmts(&function.body, out);
}

pub(super) fn collect_generic_object_instantiations_stmts(stmts: &[TypedStmt], out: &mut HashSet<String>) {
    for stmt in stmts {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                collect_generic_object_instantiation_type(&binding.ty, out);
                collect_generic_object_instantiations_expr(expr, out);
            }
            TypedStmtKind::Assign { expr, .. }
            | TypedStmtKind::Print(expr)
            | TypedStmtKind::Expr(expr)
            | TypedStmtKind::Return(Some(expr))
            | TypedStmtKind::Throw(expr) => collect_generic_object_instantiations_expr(expr, out),
            TypedStmtKind::AssignTarget { target, expr } => {
                collect_generic_object_instantiations_expr(target, out);
                collect_generic_object_instantiations_expr(expr, out);
            }
            TypedStmtKind::Block(body) | TypedStmtKind::While { body, .. } => {
                collect_generic_object_instantiations_stmts(body, out)
            }
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_generic_object_instantiations_expr(condition, out);
                collect_generic_object_instantiations_stmts(then_body, out);
                collect_generic_object_instantiations_stmts(else_body, out);
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                collect_generic_object_instantiations_stmts(try_body, out);
                collect_generic_object_instantiations_stmts(catch_body, out);
                collect_generic_object_instantiations_stmts(finally_body, out);
            }
            TypedStmtKind::Switch {
                expr,
                cases,
                default,
            } => {
                collect_generic_object_instantiations_expr(expr, out);
                for case in cases {
                    collect_generic_object_instantiations_expr(&case.value, out);
                    collect_generic_object_instantiations_stmts(&case.body, out);
                }
                collect_generic_object_instantiations_stmts(default, out);
            }
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                if let Some(init) = init {
                    collect_generic_object_instantiations_stmts(std::slice::from_ref(init), out);
                }
                if let Some(condition) = condition {
                    collect_generic_object_instantiations_expr(condition, out);
                }
                if let Some(increment) = increment {
                    collect_generic_object_instantiations_stmts(std::slice::from_ref(increment), out);
                }
                collect_generic_object_instantiations_stmts(body, out);
            }
            TypedStmtKind::ForEach {
                item,
                collection,
                body,
            } => {
                collect_generic_object_instantiation_type(&item.ty, out);
                collect_generic_object_instantiations_expr(collection, out);
                collect_generic_object_instantiations_stmts(body, out);
            }
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
        }
    }
}

pub(super) fn collect_generic_object_instantiations_expr(expr: &TypedExpr, out: &mut HashSet<String>) {
    collect_generic_object_instantiation_type(&expr.ty, out);
    match &expr.kind {
        TypedExprKind::NullableSome(value) | TypedExprKind::Await(value) | TypedExprKind::Throw(value) => {
            collect_generic_object_instantiations_expr(value, out);
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                collect_generic_object_instantiations_expr(value, out);
            }
        }
        TypedExprKind::NewArray {
            element_type,
            length,
            values,
        } => {
            collect_generic_object_instantiation_type(element_type, out);
            if let Some(length) = length {
                collect_generic_object_instantiations_expr(length, out);
            }
            for value in values {
                collect_generic_object_instantiations_expr(value, out);
            }
        }
        TypedExprKind::Index { target, index } => {
            collect_generic_object_instantiations_expr(target, out);
            collect_generic_object_instantiations_expr(index, out);
        }
        TypedExprKind::Field { target, .. } => collect_generic_object_instantiations_expr(target, out),
        TypedExprKind::IsPattern { expr, binding } => {
            collect_generic_object_instantiations_expr(expr, out);
            if let Some(binding) = binding {
                collect_generic_object_instantiation_type(&binding.ty, out);
            }
        }
        TypedExprKind::Assign { target, value } => {
            collect_generic_object_instantiations_expr(target, out);
            collect_generic_object_instantiations_expr(value, out);
        }
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Method { target, .. } => collect_generic_object_instantiations_expr(target, out),
                TypedCallKind::Function { .. } => {}
            }
            for arg in &call.args {
                collect_generic_object_instantiations_expr(arg, out);
            }
            for arg in &call.generic_args {
                collect_generic_object_instantiation_type(arg, out);
            }
        }
        TypedExprKind::NewObject {
            type_name,
            args,
            fields,
            ..
        } => {
            if type_name.contains('<') {
                out.insert(type_name.clone());
            }
            for arg in args {
                collect_generic_object_instantiations_expr(arg, out);
            }
            for field in fields {
                collect_generic_object_instantiations_expr(&field.expr, out);
            }
        }
        TypedExprKind::Borrow { .. }
        | TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_) => {}
        TypedExprKind::Lambda { body, .. } => {
            collect_generic_object_instantiations_expr(body, out);
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_generic_object_instantiations_expr(condition, out);
            collect_generic_object_instantiations_expr(when_true, out);
            collect_generic_object_instantiations_expr(when_false, out);
        }
        TypedExprKind::Unary { expr, .. } => collect_generic_object_instantiations_expr(expr, out),
        TypedExprKind::IncDec { target, .. } => collect_generic_object_instantiations_expr(target, out),
        TypedExprKind::Binary { left, right, .. } => {
            collect_generic_object_instantiations_expr(left, out);
            collect_generic_object_instantiations_expr(right, out);
        }
    }
}

pub(super) fn collect_generic_object_instantiation_type(ty: &IrType, out: &mut HashSet<String>) {
    match ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => {
            if name.contains('<') {
                out.insert(name.clone());
            }
        }
        IrType::Array(inner)
        | IrType::Ref(inner)
        | IrType::List(inner)
        | IrType::Enumerable(inner)
        | IrType::Task(inner)
        | IrType::Nullable(inner)
        | IrType::Weak(inner) => collect_generic_object_instantiation_type(inner, out),
        IrType::Dictionary(key, value) => {
            collect_generic_object_instantiation_type(key, out);
            collect_generic_object_instantiation_type(value, out);
        }
        IrType::Function { params, return_type } => {
            for param in params {
                collect_generic_object_instantiation_type(param, out);
            }
            collect_generic_object_instantiation_type(return_type, out);
        }
        _ => {}
    }
}

pub(super) fn collect_rc_instantiations_program(program: &TypedProgram, out: &mut HashSet<String>) {
    for function in &program.functions {
        collect_rc_instantiations_function(function, out);
    }
    for ty in &program.types {
        for field in &ty.fields {
            collect_rc_type(&field.ty, out);
        }
        for constructor in &ty.constructors {
            collect_rc_instantiations_function(constructor, out);
        }
        for method in &ty.methods {
            collect_rc_instantiations_function(method, out);
        }
    }
    for endpoint in &program.endpoint_handlers {
        collect_rc_type(&endpoint.return_type, out);
        collect_rc_type(&endpoint.response_type, out);
        for param in &endpoint.params {
            collect_rc_type(&param.ty, out);
        }
    }
}

pub(super) fn collect_rc_instantiations_function(function: &TypedFunction, out: &mut HashSet<String>) {
    collect_rc_type(&function.return_type, out);
    for param in &function.params {
        collect_rc_type(&param.ty, out);
    }
    for local in &function.locals {
        collect_rc_type(&local.ty, out);
    }
    collect_rc_stmts(&function.body, out);
}

pub(super) fn collect_rc_stmts(stmts: &[TypedStmt], out: &mut HashSet<String>) {
    for stmt in stmts {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                collect_rc_type(&binding.ty, out);
                collect_rc_expr(expr, out);
            }
            TypedStmtKind::Assign { expr, .. } | TypedStmtKind::Print(expr) | TypedStmtKind::Expr(expr) => {
                collect_rc_expr(expr, out);
            }
            TypedStmtKind::AssignTarget { target, expr } => {
                collect_rc_expr(target, out);
                collect_rc_expr(expr, out);
            }
            TypedStmtKind::Block(body) | TypedStmtKind::While { body, .. } => {
                collect_rc_stmts(body, out);
            }
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_rc_expr(condition, out);
                collect_rc_stmts(then_body, out);
                collect_rc_stmts(else_body, out);
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                collect_rc_stmts(try_body, out);
                collect_rc_stmts(catch_body, out);
                collect_rc_stmts(finally_body, out);
            }
            TypedStmtKind::Switch {
                expr,
                cases,
                default,
            } => {
                collect_rc_expr(expr, out);
                for case in cases {
                    collect_rc_expr(&case.value, out);
                    collect_rc_stmts(&case.body, out);
                }
                collect_rc_stmts(default, out);
            }
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                if let Some(init) = init {
                    collect_rc_stmts(std::slice::from_ref(init), out);
                }
                if let Some(condition) = condition {
                    collect_rc_expr(condition, out);
                }
                if let Some(increment) = increment {
                    collect_rc_stmts(std::slice::from_ref(increment), out);
                }
                collect_rc_stmts(body, out);
            }
            TypedStmtKind::ForEach { collection, body, .. } => {
                collect_rc_expr(collection, out);
                collect_rc_stmts(body, out);
            }
            TypedStmtKind::Return(Some(expr)) | TypedStmtKind::Throw(expr) => {
                collect_rc_expr(expr, out);
            }
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
        }
    }
}

pub(super) fn collect_rc_expr(expr: &TypedExpr, out: &mut HashSet<String>) {
    collect_rc_type(&expr.ty, out);
    if let TypedExprKind::NewObject { type_name, .. } = &expr.kind {
        if type_name.starts_with("Rc_") {
            out.insert(type_name.clone());
        }
    }
    match &expr.kind {
        TypedExprKind::NullableSome(value) => collect_rc_expr(value, out),
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                collect_rc_expr(value, out);
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_rc_expr(length, out);
            }
            for value in values {
                collect_rc_expr(value, out);
            }
        }
        TypedExprKind::Index { target, index } => {
            collect_rc_expr(target, out);
            collect_rc_expr(index, out);
        }
        TypedExprKind::Field { target, .. } => collect_rc_expr(target, out),
        TypedExprKind::IsPattern { expr, .. } => collect_rc_expr(expr, out),
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Method { target, .. } => collect_rc_expr(target, out),
                TypedCallKind::Function { .. } => {}
            }
            for arg in &call.args {
                collect_rc_expr(arg, out);
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_rc_expr(arg, out);
            }
            for field in fields {
                collect_rc_expr(&field.expr, out);
            }
        }
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. } => {}
        TypedExprKind::Await(inner)
        | TypedExprKind::Unary { expr: inner, .. }
        | TypedExprKind::IncDec { target: inner, .. } => collect_rc_expr(inner, out),
        TypedExprKind::Throw(expr) => collect_rc_expr(expr, out),
        TypedExprKind::Assign { target, value } => {
            collect_rc_expr(target, out);
            collect_rc_expr(value, out);
        }
        TypedExprKind::Lambda { body, .. } => collect_rc_expr(body, out),
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_rc_expr(condition, out);
            collect_rc_expr(when_true, out);
            collect_rc_expr(when_false, out);
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_rc_expr(left, out);
            collect_rc_expr(right, out);
        }
    }
}

pub(super) fn collect_rc_type(ty: &IrType, out: &mut HashSet<String>) {
    match ty {
        IrType::Struct(name) if name.starts_with("Rc_") => {
            out.insert(name.clone());
        }
        IrType::Array(inner)
        | IrType::Ref(inner)
        | IrType::List(inner)
        | IrType::Enumerable(inner)
        | IrType::Task(inner)
        | IrType::Nullable(inner)
        | IrType::Weak(inner) => collect_rc_type(inner, out),
        IrType::Dictionary(key, value) => {
            collect_rc_type(key, out);
            collect_rc_type(value, out);
        }
        IrType::Function { params, return_type } => {
            for ty in params {
                collect_rc_type(ty, out);
            }
            collect_rc_type(return_type, out);
        }
        _ => {}
    }
}

pub(super) fn parse_monomorphized_rc_inner_type(
    text: &str,
    known_objects: &HashMap<String, LlObjectType>,
) -> Option<IrType> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    match text {
        "bool" => Some(IrType::Bool),
        "byte" => Some(IrType::Byte),
        "short" => Some(IrType::Short),
        "int" => Some(IrType::Int),
        "long" => Some(IrType::Long),
        "uint" => Some(IrType::UInt),
        "double" => Some(IrType::Double),
        "decimal" => Some(IrType::Decimal),
        "string" => Some(IrType::String),
        "void" => Some(IrType::Void),
        "Exception" | "System.Exception" => Some(IrType::Exception),
        _ => {
            if let Some((base, args)) = split_monomorphized_type(text) {
                let base_name = base_type_name(base);
                match base_name {
                    "List" if args.len() == 1 => Some(IrType::List(Box::new(
                        parse_monomorphized_rc_inner_type(&args[0], known_objects)?,
                    ))),
                    "Dictionary" if args.len() == 2 => Some(IrType::Dictionary(
                        Box::new(parse_monomorphized_rc_inner_type(&args[0], known_objects)?),
                        Box::new(parse_monomorphized_rc_inner_type(&args[1], known_objects)?),
                    )),
                    "IEnumerable" | "ICollection" if args.len() == 1 => Some(IrType::Enumerable(
                        Box::new(parse_monomorphized_rc_inner_type(&args[0], known_objects)?),
                    )),
                    "Task" | "ValueTask" if args.len() == 1 => Some(IrType::Task(Box::new(
                        parse_monomorphized_rc_inner_type(&args[0], known_objects)?,
                    ))),
                    "Weak" | "WeakReference" if args.len() == 1 => Some(IrType::Weak(Box::new(
                        parse_monomorphized_rc_inner_type(&args[0], known_objects)?,
                    ))),
                    _ => resolve_known_object_type(base_name, known_objects)
                        .or_else(|| Some(IrType::Unknown(text.to_string()))),
                }
            } else {
                resolve_known_object_type(base_type_name(text), known_objects)
                    .or_else(|| Some(IrType::Unknown(text.to_string())))
            }
        }
    }
}

pub(super) fn resolve_known_object_type(
    name: &str,
    known_objects: &HashMap<String, LlObjectType>,
) -> Option<IrType> {
    known_objects
        .get(name)
        .or_else(|| known_objects.get(name.rsplit('.').next().unwrap_or(name)))
        .map(|object| match object.kind {
            TypeKind::Class => IrType::Class(object.name.clone()),
            TypeKind::Interface => IrType::Interface(object.name.clone()),
            TypeKind::Enum => IrType::Int,
            TypeKind::Struct | TypeKind::RefStruct => IrType::Struct(object.name.clone()),
        })
}



