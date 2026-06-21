use super::*;

pub(super) fn validate_typed_function_visibility(
    function: &TypedFunction,
    env: &TypeEnv,
) -> Result<(), String> {
    let caller_package = function.package_id.as_deref();
    if caller_package.is_some() {
        return Ok(());
    }
    ensure_type_accessible(
        env,
        &function.return_type,
        caller_package,
        &format!("return type of '{}'", function.name),
    )?;
    for param in &function.params {
        ensure_type_accessible(
            env,
            &param.ty,
            caller_package,
            &format!("parameter '{}' of '{}'", param.name, function.name),
        )?;
    }
    for local in &function.locals {
        ensure_type_accessible(
            env,
            &local.ty,
            caller_package,
            &format!("local '{}' of '{}'", local.name, function.name),
        )?;
    }
    for stmt in &function.body {
        validate_typed_stmt_visibility(stmt, env, caller_package, &function.name)?;
    }
    Ok(())
}

pub(super) fn validate_typed_stmt_visibility(
    stmt: &TypedStmt,
    env: &TypeEnv,
    caller_package: Option<&str>,
    function_name: &str,
) -> Result<(), String> {
    match &stmt.kind {
        TypedStmtKind::Let { binding, expr } => {
            ensure_type_accessible(
                env,
                &binding.ty,
                caller_package,
                &format!("binding '{}' in '{}'", binding.name, function_name),
            )?;
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Throw(expr) => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedStmtKind::AssignTarget { target, expr } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedStmtKind::Block(body) => {
            for stmt in body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::If { condition, then_body, else_body } => {
            validate_typed_expr_visibility(condition, env, caller_package, function_name)?;
            for stmt in then_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
            for stmt in else_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::Try { try_body, catch_body, finally_body, .. } => {
            for stmt in try_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
            for stmt in catch_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
            for stmt in finally_body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::Switch { expr, cases, default } => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
            for case in cases {
                validate_typed_expr_visibility(&case.value, env, caller_package, function_name)?;
                for stmt in &case.body {
                    validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
                }
            }
            for stmt in default {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::While { condition, body } => {
            validate_typed_expr_visibility(condition, env, caller_package, function_name)?;
            for stmt in body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::For { init, condition, increment, body } => {
            if let Some(init) = init {
                validate_typed_stmt_visibility(init, env, caller_package, function_name)?;
            }
            if let Some(condition) = condition {
                validate_typed_expr_visibility(condition, env, caller_package, function_name)?;
            }
            if let Some(increment) = increment {
                validate_typed_stmt_visibility(increment, env, caller_package, function_name)?;
            }
            for stmt in body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::ForEach { item, collection, body } => {
            ensure_type_accessible(
                env,
                &item.ty,
                caller_package,
                &format!("foreach item '{}' in '{}'", item.name, function_name),
            )?;
            validate_typed_expr_visibility(collection, env, caller_package, function_name)?;
            for stmt in body {
                validate_typed_stmt_visibility(stmt, env, caller_package, function_name)?;
            }
        }
        TypedStmtKind::Return(Some(expr)) => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
    }
    Ok(())
}

pub(super) fn validate_typed_expr_visibility(
    expr: &TypedExpr,
    env: &TypeEnv,
    caller_package: Option<&str>,
    function_name: &str,
) -> Result<(), String> {
    ensure_type_accessible(env, &expr.ty, caller_package, &format!("expression in '{}'", function_name))?;
    match &expr.kind {
        TypedExprKind::Field { target, name } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
            if let Some(type_name) = object_type_name(&target.ty) {
                if let Some(field) = env.resolve_field(type_name, name) {
                    if !visibility_allows_access(field.visibility, field.package_id.as_deref(), caller_package) {
                        return Err(format!(
                            "field '{}.{}' is not public in package '{}' and cannot be accessed from '{}'",
                            type_name,
                            name,
                            field.package_id.as_deref().unwrap_or("<root>"),
                            caller_package.unwrap_or("<root>")
                        ));
                    }
                }
            }
        }
        TypedExprKind::Call(call) => {
            if let TypedCallKind::Method { target, symbol, .. } = &call.kind {
                validate_typed_expr_visibility(target, env, caller_package, function_name)?;
                if let Some(signature) = env.lookup_signature_by_symbol(symbol) {
                    if !visibility_allows_access(signature.visibility, signature.package_id.as_deref(), caller_package) {
                        return Err(format!(
                            "method '{}' is not public in package '{}' and cannot be called from '{}'",
                            symbol,
                            signature.package_id.as_deref().unwrap_or("<root>"),
                            caller_package.unwrap_or("<root>")
                        ));
                    }
                }
            } else if let TypedCallKind::Function { symbol, .. } = &call.kind {
                if let Some(signature) = env.lookup_signature_by_symbol(symbol) {
                    if !visibility_allows_access(signature.visibility, signature.package_id.as_deref(), caller_package) {
                        return Err(format!(
                            "function '{}' is not public in package '{}' and cannot be called from '{}'",
                            symbol,
                            signature.package_id.as_deref().unwrap_or("<root>"),
                            caller_package.unwrap_or("<root>")
                        ));
                    }
                }
            }
            for arg in &call.args {
                validate_typed_expr_visibility(arg, env, caller_package, function_name)?;
            }
        }
        TypedExprKind::NewObject { type_name, constructor, args, fields } => {
            if let Some((package_id, visibility)) =
                env.lookup_type_visibility(type_name, caller_package)
            {
                if !visibility_allows_access(visibility, package_id.as_deref(), caller_package) {
                    return Err(format!(
                        "type '{}' is not public in package '{}' and cannot be constructed from '{}'",
                        type_name,
                        package_id.as_deref().unwrap_or("<root>"),
                        caller_package.unwrap_or("<root>")
                    ));
                }
            }
            if let Some(symbol) = constructor {
                if let Some(signature) = env.lookup_signature_by_symbol(symbol) {
                    if !visibility_allows_access(signature.visibility, signature.package_id.as_deref(), caller_package) {
                        return Err(format!(
                            "constructor '{}' is not public in package '{}' and cannot be called from '{}'",
                            symbol,
                            signature.package_id.as_deref().unwrap_or("<root>"),
                            caller_package.unwrap_or("<root>")
                        ));
                    }
                }
            }
            for arg in args {
                validate_typed_expr_visibility(arg, env, caller_package, function_name)?;
            }
            for field in fields {
                validate_typed_expr_visibility(&field.expr, env, caller_package, function_name)?;
            }
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                validate_typed_expr_visibility(value, env, caller_package, function_name)?;
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                validate_typed_expr_visibility(length, env, caller_package, function_name)?;
            }
            for value in values {
                validate_typed_expr_visibility(value, env, caller_package, function_name)?;
            }
        }
        TypedExprKind::Index { target, index } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
            validate_typed_expr_visibility(index, env, caller_package, function_name)?;
        }
        TypedExprKind::IsPattern { expr, binding } => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
            if let Some(binding) = binding {
                ensure_type_accessible(env, &binding.ty, caller_package, &format!("pattern binding '{}' in '{}'", binding.name, function_name))?;
            }
        }
        TypedExprKind::Throw(expr)
        | TypedExprKind::Await(expr)
        | TypedExprKind::NullableSome(expr)
        | TypedExprKind::Unary { expr, .. } => {
            validate_typed_expr_visibility(expr, env, caller_package, function_name)?;
        }
        TypedExprKind::Assign { target, value } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
            validate_typed_expr_visibility(value, env, caller_package, function_name)?;
        }
        TypedExprKind::Conditional { condition, when_true, when_false } => {
            validate_typed_expr_visibility(condition, env, caller_package, function_name)?;
            validate_typed_expr_visibility(when_true, env, caller_package, function_name)?;
            validate_typed_expr_visibility(when_false, env, caller_package, function_name)?;
        }
        TypedExprKind::IncDec { target, .. } => {
            validate_typed_expr_visibility(target, env, caller_package, function_name)?;
        }
        TypedExprKind::Binary { left, right, .. } => {
            validate_typed_expr_visibility(left, env, caller_package, function_name)?;
            validate_typed_expr_visibility(right, env, caller_package, function_name)?;
        }
        TypedExprKind::Lambda { body, .. } => {
            validate_typed_expr_visibility(body, env, caller_package, function_name)?;
        }
        TypedExprKind::FunctionSymbol(symbol) => {
            if let Some(signature) = env.lookup_signature_by_symbol(symbol) {
                if !visibility_allows_access(signature.visibility, signature.package_id.as_deref(), caller_package) {
                    return Err(format!(
                        "function '{}' is not public in package '{}' and cannot be referenced from '{}'",
                        symbol,
                        signature.package_id.as_deref().unwrap_or("<root>"),
                        caller_package.unwrap_or("<root>")
                    ));
                }
            }
        }
        TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. }
        | TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_) => {}
    }
    Ok(())
}

pub(super) fn ensure_type_accessible(
    env: &TypeEnv,
    ty: &IrType,
    caller_package: Option<&str>,
    context: &str,
) -> Result<(), String> {
    match ty {
        IrType::Array(inner)
        | IrType::Ref(inner)
        | IrType::Nullable(inner)
        | IrType::Weak(inner)
        | IrType::Task(inner)
        | IrType::Enumerable(inner)
        | IrType::List(inner) => ensure_type_accessible(env, inner, caller_package, context),
        IrType::Dictionary(key, value) => {
            ensure_type_accessible(env, key, caller_package, context)?;
            ensure_type_accessible(env, value, caller_package, context)
        }
        IrType::Function { params, return_type } => {
            for param in params {
                ensure_type_accessible(env, param, caller_package, context)?;
            }
            ensure_type_accessible(env, return_type, caller_package, context)
        }
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) | IrType::Unknown(name) => {
            if let Some((package_id, visibility)) =
                env.lookup_type_visibility(name, caller_package)
            {
                if !visibility_allows_access(visibility, package_id.as_deref(), caller_package) {
                    return Err(format!(
                        "{context} uses non-public type '{}' from package '{}'",
                        name,
                        package_id.as_deref().unwrap_or("<root>")
                    ));
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

pub(super) fn visibility_allows_access(
    visibility: Visibility,
    declaration_package: Option<&str>,
    caller_package: Option<&str>,
) -> bool {
    visibility == Visibility::Public || declaration_package == caller_package
}

pub(super) fn object_type_name(ty: &IrType) -> Option<&str> {
    match ty {
        IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => Some(name),
        IrType::Ref(inner) | IrType::Nullable(inner) => object_type_name(inner),
        _ => None,
    }
}

pub(super) fn validate_async_function(function: &TypedFunction) -> Result<(), String> {
    if !function.is_async {
        return Ok(());
    }
    let IrType::Task(_) = &function.return_type else {
        return Err(format!(
            "async lowering: function '{}' must return Task, Task<T>, ValueTask, or ValueTask<T> in the current gate",
            function.name
        ));
    };
    for param in &function.params {
        if !async_state_capture_supported(param) {
            return Err(format!(
                "async lowering: parameter '{}' in '{}' cannot be captured into the async state safely yet; rewrite it to a copy/shared/class value or construct the owned value inside the async body",
                param.name, function.name
            ));
        }
    }
    let mut scope = HashMap::new();
    for param in &function.params {
        scope.insert(param.name.clone(), param.clone());
    }
    validate_async_stmts(&function.name, &function.body, &HashSet::new(), &mut scope)
}

pub(super) fn async_state_capture_supported(binding: &TypedBinding) -> bool {
    match binding.ownership {
        Ownership::Copy => true,
        Ownership::Borrowed => {
            binding.name == "this"
                && matches!(binding.ty, IrType::Class(_) | IrType::Interface(_))
        }
        Ownership::Shared => matches!(
            binding.ty,
            IrType::Class(_)
                | IrType::Interface(_)
                | IrType::Function { .. }
                | IrType::Nullable(_)
                | IrType::Unknown(_)
        ),
        Ownership::Owned => matches!(binding.ty, IrType::String | IrType::Exception)
            || matches!(&binding.ty, IrType::Struct(name) if name == "CancellationToken" || name.ends_with(".CancellationToken")),
        Ownership::View | Ownership::Moved => false,
    }
}

pub(super) fn validate_async_stmts(
    function_name: &str,
    stmts: &[TypedStmt],
    outer_after_uses: &HashSet<String>,
    scope: &mut HashMap<String, TypedBinding>,
) -> Result<(), String> {
    let mut suffix_uses = vec![HashSet::<String>::new(); stmts.len() + 1];
    suffix_uses[stmts.len()] = outer_after_uses.clone();
    for index in (0..stmts.len()).rev() {
        let mut uses = suffix_uses[index + 1].clone();
        collect_used_bindings_stmt(&stmts[index], &mut uses);
        suffix_uses[index] = uses;
    }

    for (index, stmt) in stmts.iter().enumerate() {
        let after_uses = &suffix_uses[index + 1];
        validate_async_stmt(function_name, stmt, after_uses, scope)?;
        if let TypedStmtKind::Let { binding, .. } = &stmt.kind {
            scope.insert(binding.name.clone(), binding.clone());
        }
    }
    Ok(())
}

pub(super) fn validate_async_stmt(
    function_name: &str,
    stmt: &TypedStmt,
    after_uses: &HashSet<String>,
    scope: &HashMap<String, TypedBinding>,
) -> Result<(), String> {
    match &stmt.kind {
        TypedStmtKind::Let { expr, .. }
        | TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Return(Some(expr))
        | TypedStmtKind::Throw(expr) => validate_async_expr(function_name, expr, after_uses, scope),
        TypedStmtKind::AssignTarget { target, expr } => {
            validate_async_expr(function_name, target, after_uses, scope)?;
            validate_async_expr(function_name, expr, after_uses, scope)
        }
        TypedStmtKind::Block(body) => {
            let mut nested_scope = scope.clone();
            validate_async_stmts(function_name, body, after_uses, &mut nested_scope)
        }
        TypedStmtKind::If {
            condition,
            then_body,
            else_body,
        } => {
            let mut condition_after = after_uses.clone();
            collect_used_bindings_stmts(then_body, &mut condition_after);
            collect_used_bindings_stmts(else_body, &mut condition_after);
            validate_async_expr(function_name, condition, &condition_after, scope)?;

            let mut then_scope = scope.clone();
            validate_async_stmts(function_name, then_body, after_uses, &mut then_scope)?;
            let mut else_scope = scope.clone();
            validate_async_stmts(function_name, else_body, after_uses, &mut else_scope)
        }
        TypedStmtKind::Try {
            try_body,
            catch_name,
            catch_body,
            finally_body,
            ..
        } => {
            let mut try_after = after_uses.clone();
            collect_used_bindings_stmts(catch_body, &mut try_after);
            collect_used_bindings_stmts(finally_body, &mut try_after);
            let mut try_scope = scope.clone();
            validate_async_stmts(function_name, try_body, &try_after, &mut try_scope)?;

            let mut catch_after = after_uses.clone();
            collect_used_bindings_stmts(finally_body, &mut catch_after);
            let mut catch_scope = scope.clone();
            if let Some(name) = catch_name {
                catch_scope.insert(
                    name.clone(),
                    TypedBinding {
                        name: name.clone(),
                        ty: IrType::Exception,
                        ownership: Ownership::Owned,
                    },
                );
            }
            validate_async_stmts(function_name, catch_body, &catch_after, &mut catch_scope)?;

            let mut finally_scope = scope.clone();
            validate_async_stmts(function_name, finally_body, after_uses, &mut finally_scope)
        }
        TypedStmtKind::Switch { expr, cases, default } => {
            let mut expr_after = after_uses.clone();
            for case in cases {
                collect_used_bindings_expr(&case.value, &mut expr_after);
                collect_used_bindings_stmts(&case.body, &mut expr_after);
            }
            collect_used_bindings_stmts(default, &mut expr_after);
            validate_async_expr(function_name, expr, &expr_after, scope)?;
            for case in cases {
                let mut case_after = after_uses.clone();
                collect_used_bindings_stmts(default, &mut case_after);
                validate_async_expr(function_name, &case.value, &case_after, scope)?;
                let mut case_scope = scope.clone();
                validate_async_stmts(function_name, &case.body, after_uses, &mut case_scope)?;
            }
            let mut default_scope = scope.clone();
            validate_async_stmts(function_name, default, after_uses, &mut default_scope)
        }
        TypedStmtKind::While { condition, body } => {
            let mut loop_after = after_uses.clone();
            collect_used_bindings_expr(condition, &mut loop_after);
            collect_used_bindings_stmts(body, &mut loop_after);
            validate_async_expr(function_name, condition, &loop_after, scope)?;
            let mut body_scope = scope.clone();
            validate_async_stmts(function_name, body, &loop_after, &mut body_scope)
        }
        TypedStmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            let mut loop_after = after_uses.clone();
            if let Some(init) = init.as_deref() {
                collect_used_bindings_stmt(init, &mut loop_after);
            }
            if let Some(condition) = condition {
                collect_used_bindings_expr(condition, &mut loop_after);
            }
            if let Some(increment) = increment.as_deref() {
                collect_used_bindings_stmt(increment, &mut loop_after);
            }
            collect_used_bindings_stmts(body, &mut loop_after);
            if let Some(init) = init.as_deref() {
                validate_async_stmt(function_name, init, &loop_after, scope)?;
            }
            if let Some(condition) = condition {
                validate_async_expr(function_name, condition, &loop_after, scope)?;
            }
            let mut body_scope = scope.clone();
            validate_async_stmts(function_name, body, &loop_after, &mut body_scope)?;
            if let Some(increment) = increment.as_deref() {
                validate_async_stmt(function_name, increment, &loop_after, scope)?;
            }
            Ok(())
        }
        TypedStmtKind::ForEach {
            item,
            collection,
            body,
        } => {
            let mut loop_after = after_uses.clone();
            collect_used_bindings_expr(collection, &mut loop_after);
            collect_used_bindings_stmts(body, &mut loop_after);
            validate_async_expr(function_name, collection, &loop_after, scope)?;
            let mut body_scope = scope.clone();
            body_scope.insert(item.name.clone(), item.clone());
            validate_async_stmts(function_name, body, &loop_after, &mut body_scope)
        }
        TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => Ok(()),
    }
}

pub(super) fn validate_async_expr(
    function_name: &str,
    expr: &TypedExpr,
    after_uses: &HashSet<String>,
    scope: &HashMap<String, TypedBinding>,
) -> Result<(), String> {
    match &expr.kind {
        TypedExprKind::Await(inner) => {
            for (name, binding) in scope {
                if !after_uses.contains(name) {
                    continue;
                }
                if matches!(binding.ownership, Ownership::Borrowed | Ownership::View)
                    && !(name == "this"
                        && matches!(binding.ty, IrType::Class(_) | IrType::Interface(_)))
                {
                    let rewrite = if matches!(binding.ownership, Ownership::View) {
                        "materialize the view into an owned collection or shorten the view scope before await"
                    } else {
                        "copy/move the value into an owned local or shorten the borrow before await"
                    };
                    return Err(format!(
                        "async lowering: '{}' stays {:?} across await in '{}'; {rewrite}",
                        name, binding.ownership, function_name
                    ));
                }
            }
            validate_async_expr(function_name, inner, after_uses, scope)
        }
        TypedExprKind::NullableSome(value)
        | TypedExprKind::Throw(value)
        | TypedExprKind::Unary { expr: value, .. } => {
            validate_async_expr(function_name, value, after_uses, scope)
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                validate_async_expr(function_name, value, after_uses, scope)?;
            }
            Ok(())
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                validate_async_expr(function_name, length, after_uses, scope)?;
            }
            for value in values {
                validate_async_expr(function_name, value, after_uses, scope)?;
            }
            Ok(())
        }
        TypedExprKind::Index { target, index } => {
            validate_async_expr(function_name, target, after_uses, scope)?;
            validate_async_expr(function_name, index, after_uses, scope)
        }
        TypedExprKind::Field { target, .. } => {
            validate_async_expr(function_name, target, after_uses, scope)
        }
        TypedExprKind::IsPattern { expr, .. } => {
            validate_async_expr(function_name, expr, after_uses, scope)
        }
        TypedExprKind::Assign { target, value } => {
            validate_async_expr(function_name, target, after_uses, scope)?;
            validate_async_expr(function_name, value, after_uses, scope)
        }
        TypedExprKind::Call(call) => {
            if let TypedCallKind::Method { target, .. } = &call.kind {
                validate_async_expr(function_name, target, after_uses, scope)?;
            }
            for arg in &call.args {
                validate_async_expr(function_name, arg, after_uses, scope)?;
            }
            Ok(())
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                validate_async_expr(function_name, arg, after_uses, scope)?;
            }
            for field in fields {
                validate_async_expr(function_name, &field.expr, after_uses, scope)?;
            }
            Ok(())
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            validate_async_expr(function_name, condition, after_uses, scope)?;
            validate_async_expr(function_name, when_true, after_uses, scope)?;
            validate_async_expr(function_name, when_false, after_uses, scope)
        }
        TypedExprKind::Binary { left, right, .. } => {
            validate_async_expr(function_name, left, after_uses, scope)?;
            validate_async_expr(function_name, right, after_uses, scope)
        }
        TypedExprKind::IncDec { target, .. } => {
            validate_async_expr(function_name, target, after_uses, scope)
        }
        TypedExprKind::Lambda { body, .. } => {
            validate_async_expr(function_name, body, after_uses, scope)
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
        | TypedExprKind::Borrow { .. } => Ok(()),
    }
}

pub(super) fn stmt_contains_await(stmt: &TypedStmt) -> bool {
    match &stmt.kind {
        TypedStmtKind::Let { expr, .. }
        | TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Return(Some(expr))
        | TypedStmtKind::Throw(expr) => expr_contains_await(expr),
        TypedStmtKind::AssignTarget { target, expr } => {
            expr_contains_await(target) || expr_contains_await(expr)
        }
        TypedStmtKind::Block(body) => body.iter().any(stmt_contains_await),
        TypedStmtKind::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_contains_await(condition)
                || then_body.iter().any(stmt_contains_await)
                || else_body.iter().any(stmt_contains_await)
        }
        TypedStmtKind::Try {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            try_body.iter().any(stmt_contains_await)
                || catch_body.iter().any(stmt_contains_await)
                || finally_body.iter().any(stmt_contains_await)
        }
        TypedStmtKind::Switch {
            expr,
            cases,
            default,
        } => {
            expr_contains_await(expr)
                || cases.iter().any(|case| {
                    expr_contains_await(&case.value) || case.body.iter().any(stmt_contains_await)
                })
                || default.iter().any(stmt_contains_await)
        }
        TypedStmtKind::While { condition, body } => {
            expr_contains_await(condition) || body.iter().any(stmt_contains_await)
        }
        TypedStmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            init.as_deref().is_some_and(stmt_contains_await)
                || condition.as_ref().is_some_and(expr_contains_await)
                || increment.as_deref().is_some_and(stmt_contains_await)
                || body.iter().any(stmt_contains_await)
        }
        TypedStmtKind::ForEach {
            collection, body, ..
        } => expr_contains_await(collection) || body.iter().any(stmt_contains_await),
        TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => false,
    }
}

pub(super) fn expr_contains_await(expr: &TypedExpr) -> bool {
    match &expr.kind {
        TypedExprKind::Await(_) => true,
        TypedExprKind::NullableSome(value)
        | TypedExprKind::Throw(value)
        | TypedExprKind::Unary { expr: value, .. } => expr_contains_await(value),
        TypedExprKind::ArrayLiteral(values) => values.iter().any(expr_contains_await),
        TypedExprKind::NewArray { length, values, .. } => {
            length.as_deref().is_some_and(expr_contains_await)
                || values.iter().any(expr_contains_await)
        }
        TypedExprKind::Index { target, index } => {
            expr_contains_await(target) || expr_contains_await(index)
        }
        TypedExprKind::Field { target, .. } | TypedExprKind::IsPattern { expr: target, .. } => {
            expr_contains_await(target)
        }
        TypedExprKind::Assign { target, value } => {
            expr_contains_await(target) || expr_contains_await(value)
        }
        TypedExprKind::Call(call) => {
            let target_await = match &call.kind {
                TypedCallKind::Method { target, .. } => expr_contains_await(target),
                TypedCallKind::Function { .. } => false,
            };
            target_await || call.args.iter().any(expr_contains_await)
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            args.iter().any(expr_contains_await)
                || fields.iter().any(|field| expr_contains_await(&field.expr))
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            expr_contains_await(condition)
                || expr_contains_await(when_true)
                || expr_contains_await(when_false)
        }
        TypedExprKind::Binary { left, right, .. } => {
            expr_contains_await(left) || expr_contains_await(right)
        }
        TypedExprKind::IncDec { target, .. } => expr_contains_await(target),
        TypedExprKind::Lambda { body, .. } => expr_contains_await(body),
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
        | TypedExprKind::Borrow { .. } => false,
    }
}

pub(super) fn collect_used_bindings_stmts(stmts: &[TypedStmt], out: &mut HashSet<String>) {
    for stmt in stmts {
        collect_used_bindings_stmt(stmt, out);
    }
}

pub(super) fn collect_used_bindings_stmt(stmt: &TypedStmt, out: &mut HashSet<String>) {
    match &stmt.kind {
        TypedStmtKind::Let { expr, .. }
        | TypedStmtKind::Assign { expr, .. }
        | TypedStmtKind::Print(expr)
        | TypedStmtKind::Expr(expr)
        | TypedStmtKind::Return(Some(expr))
        | TypedStmtKind::Throw(expr) => collect_used_bindings_expr(expr, out),
        TypedStmtKind::AssignTarget { target, expr } => {
            collect_used_bindings_expr(target, out);
            collect_used_bindings_expr(expr, out);
        }
        TypedStmtKind::Block(body) => collect_used_bindings_stmts(body, out),
        TypedStmtKind::If {
            condition,
            then_body,
            else_body,
        } => {
            collect_used_bindings_expr(condition, out);
            collect_used_bindings_stmts(then_body, out);
            collect_used_bindings_stmts(else_body, out);
        }
        TypedStmtKind::Try {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            collect_used_bindings_stmts(try_body, out);
            collect_used_bindings_stmts(catch_body, out);
            collect_used_bindings_stmts(finally_body, out);
        }
        TypedStmtKind::Switch {
            expr,
            cases,
            default,
        } => {
            collect_used_bindings_expr(expr, out);
            for case in cases {
                collect_used_bindings_expr(&case.value, out);
                collect_used_bindings_stmts(&case.body, out);
            }
            collect_used_bindings_stmts(default, out);
        }
        TypedStmtKind::While { condition, body } => {
            collect_used_bindings_expr(condition, out);
            collect_used_bindings_stmts(body, out);
        }
        TypedStmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            if let Some(init) = init.as_deref() {
                collect_used_bindings_stmt(init, out);
            }
            if let Some(condition) = condition {
                collect_used_bindings_expr(condition, out);
            }
            if let Some(increment) = increment.as_deref() {
                collect_used_bindings_stmt(increment, out);
            }
            collect_used_bindings_stmts(body, out);
        }
        TypedStmtKind::ForEach {
            collection, body, ..
        } => {
            collect_used_bindings_expr(collection, out);
            collect_used_bindings_stmts(body, out);
        }
        TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
    }
}

pub(super) fn collect_used_bindings_expr(expr: &TypedExpr, out: &mut HashSet<String>) {
    match &expr.kind {
        TypedExprKind::Var(name) | TypedExprKind::Move(name) | TypedExprKind::Borrow { name, .. } => {
            out.insert(name.clone());
        }
        TypedExprKind::NullableSome(value)
        | TypedExprKind::Throw(value)
        | TypedExprKind::Unary { expr: value, .. }
        | TypedExprKind::Await(value) => collect_used_bindings_expr(value, out),
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                collect_used_bindings_expr(value, out);
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_used_bindings_expr(length, out);
            }
            for value in values {
                collect_used_bindings_expr(value, out);
            }
        }
        TypedExprKind::Index { target, index } => {
            collect_used_bindings_expr(target, out);
            collect_used_bindings_expr(index, out);
        }
        TypedExprKind::Field { target, .. } | TypedExprKind::IsPattern { expr: target, .. } => {
            collect_used_bindings_expr(target, out)
        }
        TypedExprKind::Assign { target, value } => {
            collect_used_bindings_expr(target, out);
            collect_used_bindings_expr(value, out);
        }
        TypedExprKind::Call(call) => {
            if let TypedCallKind::Method { target, .. } = &call.kind {
                collect_used_bindings_expr(target, out);
            }
            for arg in &call.args {
                collect_used_bindings_expr(arg, out);
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_used_bindings_expr(arg, out);
            }
            for field in fields {
                collect_used_bindings_expr(&field.expr, out);
            }
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_used_bindings_expr(condition, out);
            collect_used_bindings_expr(when_true, out);
            collect_used_bindings_expr(when_false, out);
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_used_bindings_expr(left, out);
            collect_used_bindings_expr(right, out);
        }
        TypedExprKind::IncDec { target, .. } => collect_used_bindings_expr(target, out),
        TypedExprKind::Lambda { body, .. } => collect_used_bindings_expr(body, out),
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::FunctionSymbol(_) => {}
    }
}

