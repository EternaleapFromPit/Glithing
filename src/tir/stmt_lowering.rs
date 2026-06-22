use super::*;

pub(super) fn lower_stmts(
    stmts: &[Stmt],
    env: &TypeEnv,
    scopes: &mut Vec<HashMap<String, TypedBinding>>,
    locals: &mut Vec<TypedBinding>,
) -> Result<(), String> {
    for stmt in stmts {
        lower_stmt(stmt, env, scopes, locals)?;
    }
    Ok(())
}

pub(super) fn lower_typed_stmts(
    stmts: &[Stmt],
    env: &TypeEnv,
    scopes: &mut Vec<HashMap<String, TypedBinding>>,
    return_type: Option<&IrType>,
) -> Result<Vec<TypedStmt>, String> {
    stmts
        .iter()
        .map(|stmt| lower_typed_stmt(stmt, env, scopes, return_type))
        .collect()
}

fn lower_typed_stmt(
    stmt: &Stmt,
    env: &TypeEnv,
    scopes: &mut Vec<HashMap<String, TypedBinding>>,
    return_type: Option<&IrType>,
) -> Result<TypedStmt, String> {
    let kind = match stmt {
        Stmt::Let {
            name,
            declared_type,
            expr,
            ..
        } => {
            let expected_ty = declared_type.as_ref().map(|ty| type_syntax_to_ir(ty, env));
            let expr = lower_typed_expr_with_expected(expr, env, scopes, expected_ty.as_ref())?;
            let ty = declared_type
                .as_ref()
                .map(|ty| type_syntax_to_ir(ty, env))
                .unwrap_or_else(|| expr.ty.clone());
            let ownership = declared_type
                .as_ref()
                .map(|declared| ownership_for_declared_type_syntax(declared, env))
                .unwrap_or_else(|| expr.ownership.clone());
            let binding = TypedBinding {
                name: name.clone(),
                ty,
                ownership,
            };
            scopes
                .last_mut()
                .unwrap()
                .insert(name.clone(), binding.clone());
            TypedStmtKind::Let { binding, expr }
        }
        Stmt::Assign { name, expr } => {
            let expected_ty = lookup(scopes, name).map(|binding| binding.ty.clone());
            let expr = lower_typed_expr_with_expected(expr, env, scopes, expected_ty.as_ref())?;
            if lookup(scopes, name).is_none() {
                if let Some((this_type, field_type, field_ownership)) =
                    implicit_field(env, scopes, name)
                {
                    let this_expr = typed_expr_with_ownership(
                        TypedExprKind::Var("this".to_string()),
                        this_type,
                        Ownership::Borrowed,
                    );
                    TypedStmtKind::AssignTarget {
                        target: typed_expr_with_ownership(
                            TypedExprKind::Field {
                                target: Box::new(this_expr),
                                name: name.clone(),
                            },
                            field_type,
                            field_ownership,
                        ),
                        expr,
                    }
                } else {
                    TypedStmtKind::Assign {
                        name: name.clone(),
                        expr,
                    }
                }
            } else {
                TypedStmtKind::Assign {
                    name: name.clone(),
                    expr,
                }
            }
        }
        Stmt::AssignTarget { target, expr } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let expr = lower_typed_expr_with_expected(expr, env, scopes, Some(&target.ty))?;
            TypedStmtKind::AssignTarget { target, expr }
        }
        Stmt::Block(body) => {
            scopes.push(HashMap::new());
            let body = lower_typed_stmts(body, env, scopes, return_type)?;
            scopes.pop();
            TypedStmtKind::Block(body)
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let pattern_bindings = collect_condition_bindings(condition, env);
            scopes.push(
                pattern_bindings
                    .iter()
                    .map(|binding| (binding.name.clone(), binding.clone()))
                    .collect(),
            );
            let condition = lower_typed_expr(condition, env, scopes)?;
            let then_body = lower_typed_stmts(then_body, env, scopes, return_type)?;
            scopes.pop();
            scopes.push(HashMap::new());
            let else_body = lower_typed_stmts(else_body, env, scopes, return_type)?;
            scopes.pop();
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            }
        }
        Stmt::Try {
            try_body,
            catch,
            finally_body,
        } => {
            scopes.push(HashMap::new());
            let try_body = lower_typed_stmts(try_body, env, scopes, return_type)?;
            scopes.pop();
            let (catch_name, catch_body) = if let Some(catch) = catch {
                scopes.push(HashMap::new());
                if let Some(name) = &catch.name {
                    scopes.last_mut().unwrap().insert(
                        name.clone(),
                        TypedBinding {
                            name: name.clone(),
                            ty: IrType::Exception,
                            ownership: Ownership::Borrowed,
                        },
                    );
                }
                let body = lower_typed_stmts(&catch.body, env, scopes, return_type)?;
                scopes.pop();
                (catch.name.clone(), body)
            } else {
                (None, Vec::new())
            };
            scopes.push(HashMap::new());
            let finally_body = lower_typed_stmts(finally_body, env, scopes, return_type)?;
            scopes.pop();
            TypedStmtKind::Try {
                try_body,
                catch_name,
                catch_body,
                finally_body,
            }
        }
        Stmt::Switch {
            expr,
            cases,
            default,
        } => {
            let expr = lower_typed_expr(expr, env, scopes)?;
            let mut typed_cases = Vec::new();
            for case in cases {
                let value = lower_typed_expr(&case.value, env, scopes)?;
                scopes.push(HashMap::new());
                if let Expr::IsPattern {
                    ty,
                    name: Some(name),
                    ..
                } = &case.value
                {
                    scopes.last_mut().unwrap().insert(
                        name.clone(),
                        TypedBinding {
                            name: name.clone(),
                            ty: type_syntax_to_ir(ty, env),
                            ownership: Ownership::Borrowed,
                        },
                    );
                }
                let body = lower_typed_stmts(&case.body, env, scopes, return_type)?;
                scopes.pop();
                typed_cases.push(TypedSwitchCase { value, body });
            }
            scopes.push(HashMap::new());
            let default = lower_typed_stmts(default, env, scopes, return_type)?;
            scopes.pop();
            TypedStmtKind::Switch {
                expr,
                cases: typed_cases,
                default,
            }
        }
        Stmt::While { condition, body } => {
            let condition = lower_typed_expr(condition, env, scopes)?;
            scopes.push(HashMap::new());
            let body = lower_typed_stmts(body, env, scopes, return_type)?;
            scopes.pop();
            TypedStmtKind::While { condition, body }
        }
        Stmt::For {
            init,
            condition,
            increment,
            body,
        } => {
            scopes.push(HashMap::new());
            let init = init
                .as_ref()
                .map(|stmt| lower_typed_stmt(stmt, env, scopes, return_type).map(Box::new))
                .transpose()?;
            let condition = condition
                .as_ref()
                .map(|expr| lower_typed_expr(expr, env, scopes))
                .transpose()?;
            let body = lower_typed_stmts(body, env, scopes, return_type)?;
            let increment = increment
                .as_ref()
                .map(|stmt| lower_typed_stmt(stmt, env, scopes, return_type).map(Box::new))
                .transpose()?;
            scopes.pop();
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            }
        }
        Stmt::ForEach {
            item_type,
            item_name,
            collection,
            body,
        } => {
            let collection = lower_typed_expr(collection, env, scopes)?;
            let ty = type_syntax_to_ir(item_type, env);
            let item = TypedBinding {
                name: item_name.clone(),
                ty,
                ownership: ownership_for_declared_type_syntax(item_type, env),
            };
            scopes.push(HashMap::new());
            scopes
                .last_mut()
                .unwrap()
                .insert(item_name.clone(), item.clone());
            let body = lower_typed_stmts(body, env, scopes, return_type)?;
            scopes.pop();
            TypedStmtKind::ForEach {
                item,
                collection,
                body,
            }
        }
        Stmt::Print(expr) => TypedStmtKind::Print(lower_typed_expr(expr, env, scopes)?),
        Stmt::Expr(expr) => TypedStmtKind::Expr(lower_typed_expr(expr, env, scopes)?),
        Stmt::Return(Some(expr)) => TypedStmtKind::Return(Some(lower_typed_expr_with_expected(
            expr,
            env,
            scopes,
            return_type,
        )?)),
        Stmt::Return(None) => TypedStmtKind::Return(None),
        Stmt::Throw(expr) => TypedStmtKind::Throw(lower_typed_expr(expr, env, scopes)?),
        Stmt::Break => TypedStmtKind::Break,
        Stmt::Continue => TypedStmtKind::Continue,
    };
    Ok(TypedStmt { kind })
}

fn lower_stmt(
    stmt: &Stmt,
    env: &TypeEnv,
    scopes: &mut Vec<HashMap<String, TypedBinding>>,
    locals: &mut Vec<TypedBinding>,
) -> Result<(), String> {
    match stmt {
        Stmt::Let {
            name,
            declared_type,
            expr,
            ..
        } => {
            let inferred = lower_expr(expr, env, scopes)?;
            let ty = declared_type
                .as_ref()
                .map(|ty| type_syntax_to_ir(ty, env))
                .unwrap_or(inferred.ty);
            let ownership = declared_type
                .as_ref()
                .map(|declared| ownership_for_declared_type_syntax(declared, env))
                .unwrap_or(inferred.ownership);
            let binding = TypedBinding {
                name: name.clone(),
                ty,
                ownership,
            };
            scopes
                .last_mut()
                .unwrap()
                .insert(name.clone(), binding.clone());
            locals.push(binding);
        }
        Stmt::Assign { expr, .. } | Stmt::AssignTarget { expr, .. } => {
            lower_expr(expr, env, scopes)?;
        }
        Stmt::Block(body) => {
            scopes.push(HashMap::new());
            lower_stmts(body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            lower_expr(condition, env, scopes)?;
            scopes.push(HashMap::new());
            lower_stmts(then_body, env, scopes, locals)?;
            scopes.pop();
            scopes.push(HashMap::new());
            lower_stmts(else_body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::Try {
            try_body,
            catch,
            finally_body,
        } => {
            scopes.push(HashMap::new());
            lower_stmts(try_body, env, scopes, locals)?;
            scopes.pop();
            if let Some(catch) = catch {
                scopes.push(HashMap::new());
                if let Some(name) = &catch.name {
                    scopes.last_mut().unwrap().insert(
                        name.clone(),
                        TypedBinding {
                            name: name.clone(),
                            ty: IrType::Exception,
                            ownership: Ownership::Borrowed,
                        },
                    );
                }
                lower_stmts(&catch.body, env, scopes, locals)?;
                scopes.pop();
            }
            scopes.push(HashMap::new());
            lower_stmts(finally_body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::Switch {
            expr,
            cases,
            default,
        } => {
            lower_expr(expr, env, scopes)?;
            for case in cases {
                lower_expr(&case.value, env, scopes)?;
                scopes.push(HashMap::new());
                if let Expr::IsPattern {
                    ty,
                    name: Some(name),
                    ..
                } = &case.value
                {
                    scopes.last_mut().unwrap().insert(
                        name.clone(),
                        TypedBinding {
                            name: name.clone(),
                            ty: type_syntax_to_ir(ty, env),
                            ownership: Ownership::Borrowed,
                        },
                    );
                }
                lower_stmts(&case.body, env, scopes, locals)?;
                scopes.pop();
            }
            scopes.push(HashMap::new());
            lower_stmts(default, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::While { condition, body } => {
            lower_expr(condition, env, scopes)?;
            scopes.push(HashMap::new());
            lower_stmts(body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::For {
            init,
            condition,
            increment,
            body,
        } => {
            scopes.push(HashMap::new());
            if let Some(init) = init {
                lower_stmt(init, env, scopes, locals)?;
            }
            if let Some(condition) = condition {
                lower_expr(condition, env, scopes)?;
            }
            lower_stmts(body, env, scopes, locals)?;
            if let Some(increment) = increment {
                lower_stmt(increment, env, scopes, locals)?;
            }
            scopes.pop();
        }
        Stmt::ForEach {
            item_type,
            item_name,
            collection,
            body,
        } => {
            lower_expr(collection, env, scopes)?;
            let ty = type_syntax_to_ir(item_type, env);
            let binding = TypedBinding {
                name: item_name.clone(),
                ownership: ownership_for_declared_type_syntax(item_type, env),
                ty,
            };
            scopes.push(HashMap::new());
            scopes
                .last_mut()
                .unwrap()
                .insert(item_name.clone(), binding.clone());
            locals.push(binding);
            lower_stmts(body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::Print(expr) | Stmt::Expr(expr) | Stmt::Return(Some(expr)) | Stmt::Throw(expr) => {
            lower_expr(expr, env, scopes)?;
        }
        Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
    }
    Ok(())
}
