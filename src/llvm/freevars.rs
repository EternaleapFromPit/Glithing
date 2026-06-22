use super::*;

pub(super) fn collect_free_vars_expr(
    expr: &TypedExpr,
    lambda_params: &[String],
    scope: &HashMap<String, LlVar>,
    seen: &mut std::collections::HashSet<String>,
    out: &mut Vec<(String, LlVar)>,
) {
    match &expr.kind {
        TypedExprKind::Var(name)
        | TypedExprKind::Move(name)
        | TypedExprKind::Borrow { name, .. } => {
            if !lambda_params.contains(name) {
                if let Some(var) = scope.get(name) {
                    if seen.insert(name.clone()) {
                        out.push((name.clone(), var.clone()));
                    }
                }
            }
        }
        TypedExprKind::IncDec { target, .. } => {
            collect_free_vars_expr(target, lambda_params, scope, seen, out);
        }
        TypedExprKind::Lambda {
            params: inner_params,
            body,
        } => {
            // Don't descend into nested lambdas -- their parameters shadow the
            // outer scope but their free vars are resolved when they are lifted.
            let merged_params: Vec<String> = lambda_params
                .iter()
                .chain(inner_params.iter())
                .cloned()
                .collect();
            match body {
                TypedLambdaBody::Expr(body) => {
                    collect_free_vars_expr(body, &merged_params, scope, seen, out);
                }
                TypedLambdaBody::Block(stmts) => {
                    for stmt in stmts {
                        collect_free_vars_stmt(stmt, &merged_params, scope, seen, out);
                    }
                }
            }
        }
        TypedExprKind::Field { target, .. } | TypedExprKind::Await(target) => {
            collect_free_vars_expr(target, lambda_params, scope, seen, out);
        }
        TypedExprKind::NullableSome(value) => {
            collect_free_vars_expr(value, lambda_params, scope, seen, out);
        }
        TypedExprKind::IsPattern { expr, .. } => {
            collect_free_vars_expr(expr, lambda_params, scope, seen, out);
        }
        TypedExprKind::Throw(expr) => {
            collect_free_vars_expr(expr, lambda_params, scope, seen, out);
        }
        TypedExprKind::Assign { target, value } => {
            collect_free_vars_expr(target, lambda_params, scope, seen, out);
            collect_free_vars_expr(value, lambda_params, scope, seen, out);
        }
        TypedExprKind::Unary { expr, .. } => {
            collect_free_vars_expr(expr, lambda_params, scope, seen, out);
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_free_vars_expr(left, lambda_params, scope, seen, out);
            collect_free_vars_expr(right, lambda_params, scope, seen, out);
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_free_vars_expr(condition, lambda_params, scope, seen, out);
            collect_free_vars_expr(when_true, lambda_params, scope, seen, out);
            collect_free_vars_expr(when_false, lambda_params, scope, seen, out);
        }
        TypedExprKind::Index { target, index } => {
            collect_free_vars_expr(target, lambda_params, scope, seen, out);
            collect_free_vars_expr(index, lambda_params, scope, seen, out);
        }
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Method { target, .. } => {
                    collect_free_vars_expr(target, lambda_params, scope, seen, out);
                }
                TypedCallKind::Function { .. } => {}
            }
            for arg in &call.args {
                collect_free_vars_expr(arg, lambda_params, scope, seen, out);
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_free_vars_expr(arg, lambda_params, scope, seen, out);
            }
            for f in fields {
                collect_free_vars_expr(&f.expr, lambda_params, scope, seen, out);
            }
        }
        TypedExprKind::ArrayLiteral(elements) => {
            for e in elements {
                collect_free_vars_expr(e, lambda_params, scope, seen, out);
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_free_vars_expr(length, lambda_params, scope, seen, out);
            }
            for v in values {
                collect_free_vars_expr(v, lambda_params, scope, seen, out);
            }
        }
        // Leaf nodes with no sub-expressions.
        TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_) => {}
    }
}

pub(super) fn collect_free_vars_stmt(
    stmt: &TypedStmt,
    lambda_params: &[String],
    scope: &HashMap<String, LlVar>,
    seen: &mut std::collections::HashSet<String>,
    out: &mut Vec<(String, LlVar)>,
) {
    match &stmt.kind {
        TypedStmtKind::Let { expr, .. }
        | TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Return(Some(expr))
        | TypedStmtKind::Throw(expr) => collect_free_vars_expr(expr, lambda_params, scope, seen, out),
        TypedStmtKind::AssignTarget { target, expr } => {
            collect_free_vars_expr(target, lambda_params, scope, seen, out);
            collect_free_vars_expr(expr, lambda_params, scope, seen, out);
        }
        TypedStmtKind::Block(body) => {
            for stmt in body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
        }
        TypedStmtKind::If {
            condition,
            then_body,
            else_body,
        } => {
            collect_free_vars_expr(condition, lambda_params, scope, seen, out);
            for stmt in then_body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
            for stmt in else_body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
        }
        TypedStmtKind::Try {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            for stmt in try_body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
            for stmt in catch_body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
            for stmt in finally_body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
        }
        TypedStmtKind::Switch { expr, cases, default } => {
            collect_free_vars_expr(expr, lambda_params, scope, seen, out);
            for case in cases {
                collect_free_vars_expr(&case.value, lambda_params, scope, seen, out);
                for stmt in &case.body {
                    collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
                }
            }
            for stmt in default {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
        }
        TypedStmtKind::While { condition, body } => {
            collect_free_vars_expr(condition, lambda_params, scope, seen, out);
            for stmt in body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
        }
        TypedStmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            if let Some(init) = init {
                collect_free_vars_stmt(init, lambda_params, scope, seen, out);
            }
            if let Some(condition) = condition {
                collect_free_vars_expr(condition, lambda_params, scope, seen, out);
            }
            if let Some(increment) = increment {
                collect_free_vars_stmt(increment, lambda_params, scope, seen, out);
            }
            for stmt in body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
        }
        TypedStmtKind::ForEach { collection, body, .. } => {
            collect_free_vars_expr(collection, lambda_params, scope, seen, out);
            for stmt in body {
                collect_free_vars_stmt(stmt, lambda_params, scope, seen, out);
            }
        }
        TypedStmtKind::Return(None)
        | TypedStmtKind::Break
        | TypedStmtKind::Continue => {}
    }
}

