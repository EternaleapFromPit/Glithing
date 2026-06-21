use super::*;

pub(super) fn collect_endpoint_handlers(
    program: &Program,
    env: &TypeEnv,
) -> Result<Vec<EndpointHandlerBinding>, String> {
    let mut collector = EndpointHandlerCollector {
        env,
        scopes: vec![HashMap::new()],
        handlers: Vec::new(),
    };
    for function in &program.functions {
        collector.scopes.clear();
        collector.scopes.push(HashMap::new());
        for param in &function.params {
            let ty = type_syntax_to_ir(&param.ty, env);
            collector.declare(param.name.clone(), ty);
        }
        collector.collect_stmts(&function.body)?;
    }
    for ty in &program.types {
        for constructor in &ty.constructors {
            collector.scopes.clear();
            collector.scopes.push(HashMap::new());
            collector.declare("this".to_string(), IrType::Class(ty.name.clone()));
            for param in &constructor.params {
                collector.declare(param.name.clone(), type_syntax_to_ir(&param.ty, env));
            }
            collector.collect_stmts(&constructor.body)?;
        }
        for method in &ty.methods {
            collector.scopes.clear();
            collector.scopes.push(HashMap::new());
            collector.declare("this".to_string(), IrType::Class(ty.name.clone()));
            for param in &method.params {
                collector.declare(param.name.clone(), type_syntax_to_ir(&param.ty, env));
            }
            collector.collect_stmts(&method.body)?;
        }
    }
    collect_controller_endpoint_handlers(program, env, &mut collector.handlers);
    Ok(collector.handlers)
}

pub(super) struct EndpointHandlerCollector<'a> {
    env: &'a TypeEnv,
    scopes: Vec<HashMap<String, IrType>>,
    handlers: Vec<EndpointHandlerBinding>,
}

impl<'a> EndpointHandlerCollector<'a> {
    fn declare(&mut self, name: String, ty: IrType) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    fn lookup(&self, name: &str) -> Option<IrType> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    fn collect_stmts(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.collect_stmt(stmt)?;
        }
        Ok(())
    }

    fn collect_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let {
                name,
                declared_type,
                expr,
                ..
            } => {
                self.collect_expr(expr)?;
                let ty = declared_type
                    .as_ref()
                    .map(|ty| type_syntax_to_ir(ty, self.env))
                    .unwrap_or_else(|| self.expr_type(expr));
                self.declare(name.clone(), ty);
            }
            Stmt::Assign { expr, .. }
            | Stmt::AssignTarget { expr, .. }
            | Stmt::Print(expr)
            | Stmt::Expr(expr)
            | Stmt::Return(Some(expr))
            | Stmt::Throw(expr) => self.collect_expr(expr)?,
            Stmt::Block(body) => {
                self.scopes.push(HashMap::new());
                self.collect_stmts(body)?;
                self.scopes.pop();
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_expr(condition)?;
                self.scopes.push(HashMap::new());
                self.collect_stmts(then_body)?;
                self.scopes.pop();
                self.scopes.push(HashMap::new());
                self.collect_stmts(else_body)?;
                self.scopes.pop();
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                self.scopes.push(HashMap::new());
                self.collect_stmts(try_body)?;
                self.scopes.pop();
                if let Some(catch) = catch {
                    self.scopes.push(HashMap::new());
                    if let Some(name) = &catch.name {
                        self.declare(name.clone(), IrType::Exception);
                    }
                    self.collect_stmts(&catch.body)?;
                    self.scopes.pop();
                }
                self.scopes.push(HashMap::new());
                self.collect_stmts(finally_body)?;
                self.scopes.pop();
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                self.collect_expr(expr)?;
                for case in cases {
                    self.collect_expr(&case.value)?;
                    self.scopes.push(HashMap::new());
                    self.collect_stmts(&case.body)?;
                    self.scopes.pop();
                }
                self.scopes.push(HashMap::new());
                self.collect_stmts(default)?;
                self.scopes.pop();
            }
            Stmt::While { condition, body } => {
                self.collect_expr(condition)?;
                self.scopes.push(HashMap::new());
                self.collect_stmts(body)?;
                self.scopes.pop();
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                self.scopes.push(HashMap::new());
                if let Some(init) = init {
                    self.collect_stmt(init)?;
                }
                if let Some(condition) = condition {
                    self.collect_expr(condition)?;
                }
                self.collect_stmts(body)?;
                if let Some(increment) = increment {
                    self.collect_stmt(increment)?;
                }
                self.scopes.pop();
            }
            Stmt::ForEach {
                item_type,
                item_name,
                collection,
                body,
            } => {
                self.collect_expr(collection)?;
                self.scopes.push(HashMap::new());
                self.declare(item_name.clone(), type_syntax_to_ir(item_type, self.env));
                self.collect_stmts(body)?;
                self.scopes.pop();
            }
            Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
        }
        Ok(())
    }

    fn collect_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::MethodCall { target, name, args, .. } => {
                self.collect_endpoint_handler(target, name, args)?;
                self.collect_expr(target)?;
                for arg in args {
                    self.collect_expr(arg)?;
                }
            }
            Expr::ArrayLiteral(values) => {
                for value in values {
                    self.collect_expr(value)?;
                }
            }
            Expr::NewArray { length, values, .. } => {
                if let Some(length) = length {
                    self.collect_expr(length)?;
                }
                for value in values {
                    self.collect_expr(value)?;
                }
            }
            Expr::Index { target, index }
            | Expr::Binary {
                left: target,
                right: index,
                ..
            } => {
                self.collect_expr(target)?;
                self.collect_expr(index)?;
            }
            Expr::Unary { expr, .. } => self.collect_expr(expr)?,
            Expr::IncDec { .. } => {}
            Expr::Lambda { body, .. } => self.collect_expr(body)?,
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.collect_expr(condition)?;
                self.collect_expr(when_true)?;
                self.collect_expr(when_false)?;
            }
            Expr::Field { target, .. }
            | Expr::IsPattern { expr: target, .. }
            | Expr::Await(target) => self.collect_expr(target)?,
            Expr::Throw(expr) => self.collect_expr(expr)?,
            Expr::Assign { target, value } => {
                self.collect_expr(target)?;
                self.collect_expr(value)?;
            }
            Expr::FunctionCall { args, .. } => {
                for arg in args {
                    self.collect_expr(arg)?;
                }
            }
            Expr::NewObject { args, fields, .. } => {
                for arg in args {
                    self.collect_expr(arg)?;
                }
                for field in fields {
                    self.collect_expr(&field.expr)?;
                }
            }
            Expr::Int(_)
            | Expr::Float(_)
            | Expr::Bool(_)
            | Expr::Null
            | Expr::String(_)
            | Expr::Var(_)
            | Expr::Move(_)
            | Expr::NewCollection(_)
            | Expr::NewThread(_)
            | Expr::Borrow { .. } => {}
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                self.collect_expr(expr)?;
            }
        }
        Ok(())
    }

    fn collect_endpoint_handler(
        &mut self,
        target: &Expr,
        name: &str,
        args: &[Expr],
    ) -> Result<(), String> {
        if !matches!(name, "MapGet" | "MapPost")
            || self.expr_type(target) != IrType::Class("WebApplication".to_string())
        {
            return Ok(());
        }
        let [Expr::String(path), Expr::Var(function)] = args else {
            return Ok(());
        };
        let Some(signature) = self.env.single_function(function) else {
            return Ok(());
        };
        let response_type = endpoint_response_ir_type(&signature.return_type);
        let response_supported = matches!(response_type, IrType::String | IrType::Class(_));
        if !signature.params.is_empty() || !response_supported {
            return Err(format!(
                "typed IR: endpoint handler '{function}' must be string/class {function}() or Task<string>/Task<class> {function}()"
            ));
        }
        self.handlers.push(EndpointHandlerBinding {
            http_method: if name == "MapGet" { "GET" } else { "POST" }.to_string(),
            path: path.clone(),
            function: signature.symbol.clone(),
            return_type: signature.return_type.clone(),
            response_type: response_type.clone(),
            ownership: ownership_for_type(&response_type),
            controller: None,
            constructor: None,
            constructor_params: Vec::new(),
            params: Vec::new(),
        });
        Ok(())
    }

    fn expr_type(&self, expr: &Expr) -> IrType {
        match expr {
            Expr::Var(name) => self
                .lookup(name)
                .or_else(|| {
                    self.env
                        .single_function(name)
                        .map(|signature| IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        })
                })
                .unwrap_or_else(|| IrType::Unknown(name.clone())),
            Expr::NewObject { type_name, .. } => match self.env.kinds.get(type_name) {
                Some(TypeKind::Class) | Some(TypeKind::Interface) => {
                    IrType::Class(type_name.clone())
                }
                _ => IrType::Struct(type_name.clone()),
            },
            Expr::FunctionCall { name, args, generic_args } => self
                .env
                .resolve_function_call_with_generic_args(
                    name,
                    &args
                        .iter()
                        .map(|arg| TypedExpr {
                            kind: TypedExprKind::Var("<endpoint>".to_string()),
                            ty: self.expr_type(arg),
                            ownership: Ownership::Copy,
                            drop_kind: DropKind::None,
                        })
                        .collect::<Vec<_>>(),
                    &generic_args
                        .iter()
                        .map(|arg| type_syntax_to_ir(arg, &self.env))
                        .collect::<Vec<_>>(),
                )
                .ok()
                .flatten()
                .map(|signature| signature.return_type.clone())
                .unwrap_or_else(|| IrType::Unknown(name.clone())),
            Expr::String(_) => IrType::String,
            Expr::Int(_) => IrType::Long,
            Expr::Float(_) => IrType::Double,
            Expr::Bool(_) => IrType::Bool,
            Expr::Null => IrType::Unknown("null".to_string()),
            _ => IrType::Unknown("<expr>".to_string()),
        }
    }
}

fn endpoint_response_ir_type(ty: &IrType) -> IrType {
    match ty {
        IrType::Task(inner) => endpoint_response_ir_type(inner),
        _ => ty.clone(),
    }
}

pub(super) fn collect_controller_endpoint_handlers(
    program: &Program,
    env: &TypeEnv,
    handlers: &mut Vec<EndpointHandlerBinding>,
) {
    let types = program
        .types
        .iter()
        .map(|ty| (ty.name.as_str(), ty))
        .collect::<HashMap<_, _>>();
    for ty in &program.types {
        // Open generic controller bases provide metadata and actions, but are not
        // themselves activatable endpoints.
        if !ty.generic_params.is_empty() {
            continue;
        }
        let Some(metadata_type) = controller_metadata_type(ty, &types) else {
            continue;
        };
        let controller_name = ty.name.strip_suffix("Controller").unwrap_or(&ty.name);
        let versions = attribute_string_args(&metadata_type.attributes, "ApiVersion");
        let mut prefixes = attribute_string_args(&metadata_type.attributes, "Route");
        if prefixes.is_empty() {
            prefixes.push(String::new());
        }
        let constructor_signature = env.constructors.get(&ty.name).and_then(|constructors| {
            constructors
                .iter()
                .find(|constructor| constructor.params.is_empty())
                .or_else(|| constructors.first())
        });
        let constructor = constructor_signature.map(|constructor| constructor.symbol.clone());
        let constructor_params = constructor_signature
            .map(|constructor| constructor.params.clone())
            .unwrap_or_default();
        let can_activate = ty.constructors.is_empty() || constructor.is_some();

        for (method_owner, method) in controller_methods(ty, &types) {
            let Some((http_method, inline_path)) = http_method_attribute(&method.attributes) else {
                continue;
            };
            let mut action_paths = attribute_string_args(&method.attributes, "Route");
            if action_paths.is_empty() {
                action_paths.push(inline_path.unwrap_or_default());
            }
            let parameter_types = method
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let Some(signature) =
                env.resolve_method(&method_owner.name, &method.name, &parameter_types)
            else {
                continue;
            };

            for prefix in &prefixes {
                let expanded_prefixes = expand_controller_route(prefix, controller_name, &versions);
                for expanded_prefix in expanded_prefixes {
                    for action_path in &action_paths {
                        let path = join_route_path(&expanded_prefix, action_path);
                        if handlers.iter().any(|handler| {
                            handler.http_method == http_method && handler.path == path
                        }) {
                            continue;
                        }
                        let params = method
                            .params
                            .iter()
                            .map(|param| EndpointParameterBinding {
                                name: param.name.clone(),
                                ty: type_syntax_to_ir(&param.ty, env),
                                source: endpoint_parameter_source(param, &path, http_method),
                            })
                            .collect();
                        handlers.push(EndpointHandlerBinding {
                            http_method: http_method.to_string(),
                            path,
                            function: signature.symbol.clone(),
                            return_type: signature.return_type.clone(),
                            response_type: endpoint_response_type(&method.return_type, env),
                            ownership: ownership_for_type(&signature.return_type),
                            controller: can_activate.then(|| ty.name.clone()),
                            constructor: constructor.clone(),
                            constructor_params: constructor_params.clone(),
                            params,
                        });
                    }
                }
            }
        }
    }
}

pub(super) fn endpoint_response_type(ty: &TypeSyntax, env: &TypeEnv) -> IrType {
    match ty {
        TypeSyntax::Task(inner) => endpoint_response_type(inner, env),
        TypeSyntax::GenericNamed { name, args }
            if matches!(
                name.rsplit('.').next().unwrap_or(name),
                "Task" | "ValueTask" | "ActionResult"
            ) =>
        {
            args.first()
                .map(|arg| endpoint_response_type(arg, env))
                .unwrap_or(IrType::String)
        }
        _ => type_syntax_to_ir(ty, env),
    }
}

pub(super) fn endpoint_parameter_source(
    param: &Param,
    route: &str,
    http_method: &str,
) -> EndpointParameterSource {
    for (attribute, source) in [
        ("FromRoute", EndpointParameterSource::Route),
        ("FromQuery", EndpointParameterSource::Query),
        ("FromBody", EndpointParameterSource::Body),
        ("FromForm", EndpointParameterSource::Form),
        ("FromHeader", EndpointParameterSource::Header),
    ] {
        if has_attribute(&param.attributes, attribute) {
            return source;
        }
    }
    if route.contains(&format!("{{{}", param.name)) {
        return EndpointParameterSource::Route;
    }
    if matches!(http_method, "GET" | "DELETE") {
        EndpointParameterSource::Query
    } else if matches!(
        param.ty,
        TypeSyntax::Named(_) | TypeSyntax::GenericNamed { .. }
    ) {
        EndpointParameterSource::Body
    } else {
        EndpointParameterSource::Auto
    }
}

pub(super) fn controller_metadata_type<'a>(
    ty: &'a TypeDef,
    types: &HashMap<&str, &'a TypeDef>,
) -> Option<&'a TypeDef> {
    if has_attribute(&ty.attributes, "ApiController")
        || has_attribute(&ty.attributes, "Route")
        || has_http_annotated_method(ty)
        || derives_from_controller_runtime_base(ty, types)
    {
        return Some(ty);
    }
    ty.bases.iter().find_map(|base| {
        controller_base_type(base, types).and_then(|base| controller_metadata_type(base, types))
    })
}

fn has_http_annotated_method(ty: &TypeDef) -> bool {
    ty.methods
        .iter()
        .any(|method| http_method_attribute(&method.attributes).is_some())
}

fn derives_from_controller_runtime_base(
    ty: &TypeDef,
    types: &HashMap<&str, &TypeDef>,
) -> bool {
    ty.bases.iter().any(|base| {
        let short = base.rsplit('.').next().unwrap_or(base);
        if matches!(short, "Controller" | "ControllerBase") {
            return true;
        }
        controller_base_type(base, types)
            .map(|resolved| derives_from_controller_runtime_base(resolved, types))
            .unwrap_or(false)
    })
}

pub(super) fn controller_methods<'a>(
    ty: &'a TypeDef,
    types: &HashMap<&str, &'a TypeDef>,
) -> Vec<(&'a TypeDef, &'a Function)> {
    fn collect<'a>(
        ty: &'a TypeDef,
        types: &HashMap<&str, &'a TypeDef>,
        seen: &mut std::collections::HashSet<(String, usize)>,
        output: &mut Vec<(&'a TypeDef, &'a Function)>,
    ) {
        for method in &ty.methods {
            let key = (method.name.clone(), method.params.len());
            if seen.insert(key) {
                output.push((ty, method));
            }
        }
        for base in &ty.bases {
            if let Some(base) = controller_base_type(base, types) {
                collect(base, types, seen, output);
            }
        }
    }

    let mut output = Vec::new();
    let mut seen = std::collections::HashSet::new();
    collect(ty, types, &mut seen, &mut output);
    output
}

pub(super) fn controller_base_type<'a>(base: &str, types: &HashMap<&str, &'a TypeDef>) -> Option<&'a TypeDef> {
    types
        .get(base)
        .copied()
        .or_else(|| types.get(base.split('<').next().unwrap_or(base)).copied())
}

pub(super) fn has_attribute(attributes: &[Attribute], expected: &str) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute_name_matches(&attribute.name, expected))
}

pub(super) fn attribute_string_args(attributes: &[Attribute], expected: &str) -> Vec<String> {
    attributes
        .iter()
        .filter(|attribute| attribute_name_matches(&attribute.name, expected))
        .filter_map(|attribute| match attribute.args.first() {
            Some(Expr::String(value)) => Some(value.clone()),
            None => Some(String::new()),
            _ => None,
        })
        .collect()
}

pub(super) fn attribute_name_matches(actual: &str, expected: &str) -> bool {
    let short = actual.rsplit('.').next().unwrap_or(actual);
    short == expected || short.strip_suffix("Attribute") == Some(expected)
}

pub(super) fn http_method_attribute(attributes: &[Attribute]) -> Option<(&'static str, Option<String>)> {
    [
        ("HttpGet", "GET"),
        ("HttpPost", "POST"),
        ("HttpPut", "PUT"),
        ("HttpDelete", "DELETE"),
        ("HttpPatch", "PATCH"),
    ]
    .into_iter()
    .find_map(|(attribute_name, method)| {
        attributes
            .iter()
            .find(|attribute| attribute_name_matches(&attribute.name, attribute_name))
            .map(|attribute| {
                let path = match attribute.args.first() {
                    Some(Expr::String(value)) => Some(value.clone()),
                    _ => None,
                };
                (method, path)
            })
    })
}

pub(super) fn expand_controller_route(prefix: &str, controller: &str, versions: &[String]) -> Vec<String> {
    let controller_path = prefix
        .replace("[controller]", controller)
        .replace("[action]", "");
    if !controller_path.contains("{version:apiVersion}") {
        return vec![controller_path];
    }
    if versions.is_empty() {
        return vec![controller_path.replace("{version:apiVersion}", "1.0")];
    }
    versions
        .iter()
        .map(|version| controller_path.replace("{version:apiVersion}", version))
        .collect()
}

pub(super) fn join_route_path(prefix: &str, path: &str) -> String {
    let left = prefix.trim().trim_matches('/');
    let right = path.trim().trim_matches('/');
    match (left.is_empty(), right.is_empty()) {
        (true, true) => "/".to_string(),
        (true, false) => format!("/{right}"),
        (false, true) => format!("/{left}"),
        (false, false) => format!("/{left}/{right}"),
    }
}

