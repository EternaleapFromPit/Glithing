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
            collect_free_vars_expr(body, &merged_params, scope, seen, out);
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

