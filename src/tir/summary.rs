use super::*;

pub(super) fn summarize_typed_stmts(stmts: &[TypedStmt], indent: &str, out: &mut String) {
    for stmt in stmts {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                out.push_str(&format!(
                    "{indent}tir let {}: {:?} {:?}\n",
                    binding.name, binding.ownership, binding.ty
                ));
                summarize_typed_expr(expr, indent, out);
            }
            TypedStmtKind::Assign { expr, .. }
            | TypedStmtKind::AssignTarget { expr, .. }
            | TypedStmtKind::Print(expr)
            | TypedStmtKind::Expr(expr)
            | TypedStmtKind::Throw(expr) => summarize_typed_expr(expr, indent, out),
            TypedStmtKind::Return(Some(expr)) => summarize_typed_expr(expr, indent, out),
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
            TypedStmtKind::Block(body) => summarize_typed_stmts(body, indent, out),
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                summarize_typed_expr(condition, indent, out);
                summarize_typed_stmts(then_body, indent, out);
                summarize_typed_stmts(else_body, indent, out);
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                summarize_typed_stmts(try_body, indent, out);
                summarize_typed_stmts(catch_body, indent, out);
                summarize_typed_stmts(finally_body, indent, out);
            }
            TypedStmtKind::Switch {
                expr,
                cases,
                default,
            } => {
                summarize_typed_expr(expr, indent, out);
                for case in cases {
                    summarize_typed_expr(&case.value, indent, out);
                    summarize_typed_stmts(&case.body, indent, out);
                }
                summarize_typed_stmts(default, indent, out);
            }
            TypedStmtKind::While { condition, body } => {
                summarize_typed_expr(condition, indent, out);
                summarize_typed_stmts(body, indent, out);
            }
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                if let Some(init) = init {
                    summarize_typed_stmts(std::slice::from_ref(init.as_ref()), indent, out);
                }
                if let Some(condition) = condition {
                    summarize_typed_expr(condition, indent, out);
                }
                if let Some(increment) = increment {
                    summarize_typed_stmts(std::slice::from_ref(increment.as_ref()), indent, out);
                }
                summarize_typed_stmts(body, indent, out);
            }
            TypedStmtKind::ForEach {
                collection, body, ..
            } => {
                summarize_typed_expr(collection, indent, out);
                summarize_typed_stmts(body, indent, out);
            }
        }
    }
}

pub(super) fn summarize_typed_expr(expr: &TypedExpr, indent: &str, out: &mut String) {
    match &expr.kind {
        TypedExprKind::NullableSome(value) => {
            summarize_typed_expr(value, indent, out);
        }
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Function { name, symbol } => out.push_str(&format!(
                    "{indent}tir call function {} symbol={} -> {:?} {:?} drop={:?}\n",
                    name, symbol, expr.ownership, expr.ty, expr.drop_kind
                )),
                TypedCallKind::Method {
                    name,
                    symbol,
                    resolution,
                    ..
                } => out.push_str(&format!(
                    "{indent}tir call method {} symbol={} resolution={:?} -> {:?} {:?} drop={:?}\n",
                    name, symbol, resolution, expr.ownership, expr.ty, expr.drop_kind
                )),
            }
            for arg in &call.args {
                summarize_typed_expr(arg, indent, out);
            }
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                summarize_typed_expr(value, indent, out);
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                summarize_typed_expr(length, indent, out);
            }
            for value in values {
                summarize_typed_expr(value, indent, out);
            }
        }
        TypedExprKind::Index { target, index } => {
            summarize_typed_expr(target, indent, out);
            summarize_typed_expr(index, indent, out);
        }
        TypedExprKind::Field { target, .. }
        | TypedExprKind::IsPattern { expr: target, .. }
        | TypedExprKind::Await(target) => {
            summarize_typed_expr(target, indent, out);
        }
        TypedExprKind::Throw(expr) => {
            summarize_typed_expr(expr, indent, out);
        }
        TypedExprKind::Assign { target, value } => {
            summarize_typed_expr(target, indent, out);
            summarize_typed_expr(value, indent, out);
        }
        TypedExprKind::Lambda { body, .. } => {
            summarize_typed_expr(body, indent, out);
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            summarize_typed_expr(condition, indent, out);
            summarize_typed_expr(when_true, indent, out);
            summarize_typed_expr(when_false, indent, out);
        }
        TypedExprKind::Unary { expr, .. } => {
            summarize_typed_expr(expr, indent, out);
        }
        TypedExprKind::IncDec { .. } => {}
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                summarize_typed_expr(arg, indent, out);
            }
            for field in fields {
                summarize_typed_expr(&field.expr, indent, out);
            }
        }
        TypedExprKind::Binary { left, right, .. } => {
            summarize_typed_expr(left, indent, out);
            summarize_typed_expr(right, indent, out);
        }
        TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Borrow { .. } => {}
    }
}

