use super::*;
use super::helpers::*;
use super::support::*;

fn is_service_provider_surface_type(ty: &IrType) -> bool {
    match ty {
        IrType::Class(name)
        | IrType::Struct(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => {
            matches!(
                base_type_name(name),
                "ServiceProvider"
                    | "IServiceProvider"
                    | "Microsoft.Extensions.DependencyInjection.ServiceProvider"
                    | "Microsoft.Extensions.DependencyInjection.IServiceProvider"
            )
        }
        _ => false,
    }
}

impl LlvmEmitter {
    pub(super) fn emit_endpoint_handler_result_value(
        &mut self,
        handler: &EndpointHandlerBinding,
        value: &str,
    ) -> Result<String, String> {
        match &handler.return_type {
            IrType::Task(inner) => {
                let inner = inner.as_ref();
                if !matches!(handler.response_type, IrType::String | IrType::Class(_)) {
                    return Err(format!(
                        "LLVM TIR backend: async endpoint result type {:?} is not supported yet",
                        handler.response_type
                    ));
                }
                let result = self.tmp();
                self.body.push_str(&format!(
                    "  {result} = call ptr @glitch_task_get_result_ptr(ptr {value})\n"
                ));
                match inner {
                    IrType::String => {
                        self.body.push_str(&format!(
                            "  call void @glitch_string_retain(ptr {result})\n"
                        ));
                    }
                    IrType::Class(type_name) => {
                        if self.object_types.contains_key(type_name) {
                            self.emit_retain(type_name, &result);
                        }
                    }
                    _ => {}
                }
                self.emit_task_drop_value(value, inner);
                Ok(result)
            }
            _ => Ok(value.to_string()),
        }
    }

    pub(super) fn emit_endpoint_dispatch(&mut self) -> Result<(), String> {
        let handlers = self.endpoint_handlers.clone();
        let mut thunk_names = Vec::with_capacity(handlers.len());
        for (index, handler) in handlers.iter().enumerate() {
            let thunk = format!("glitch_endpoint_handler_{index}");
            self.emit_endpoint_thunk(&thunk, handler)?;
            thunk_names.push(thunk);
        }

        let routes = handlers
            .iter()
            .map(|handler| {
                (
                    self.string_global(&handler.http_method),
                    self.string_global(&handler.path),
                )
            })
            .collect::<Vec<_>>();

        self.body.push_str(
            "define i1 @glitch_endpoint_handlers_contains(ptr %app, ptr %method, ptr %path) {\nentry:\n",
        );
        if routes.is_empty() {
            self.body.push_str("  ret i1 false\n}\n\n");
        } else {
            for (index, (method, path)) in routes.iter().enumerate() {
                let next = if index + 1 == routes.len() {
                    "endpoint_not_found".to_string()
                } else {
                    format!("endpoint_next_{index}")
                };
                self.body.push_str(&format!(
                    "  %contains_method_cmp_{index} = call i32 @strcmp(ptr %method, ptr {method})\n  %contains_method_match_{index} = icmp eq i32 %contains_method_cmp_{index}, 0\n  br i1 %contains_method_match_{index}, label %contains_path_{index}, label %{next}\ncontains_path_{index}:\n  %contains_path_match_{index} = call i1 @glitch_route_match(ptr {path}, ptr %path)\n  br i1 %contains_path_match_{index}, label %endpoint_found, label %{next}\n"
                ));
                if index + 1 != routes.len() {
                    self.body.push_str(&format!("{next}:\n"));
                }
            }
            self.body.push_str(
                "endpoint_found:\n  ret i1 true\nendpoint_not_found:\n  ret i1 false\n}\n\n",
            );
        }

        let not_found = self.string_global("404");
        self.body.push_str(
            "define ptr @glitch_endpoint_handlers_invoke(ptr %app, ptr %method, ptr %path, ptr %body) {\nentry:\n",
        );
        if routes.is_empty() {
            self.body
                .push_str(&format!("  ret ptr {not_found}\n}}\n\n"));
        } else {
            for (index, ((method, path), thunk)) in
                routes.iter().zip(thunk_names.iter()).enumerate()
            {
                let next = if index + 1 == routes.len() {
                    "invoke_not_found".to_string()
                } else {
                    format!("invoke_next_{index}")
                };
                self.body.push_str(&format!(
                    "  %invoke_method_cmp_{index} = call i32 @strcmp(ptr %method, ptr {method})\n  %invoke_method_match_{index} = icmp eq i32 %invoke_method_cmp_{index}, 0\n  br i1 %invoke_method_match_{index}, label %invoke_path_{index}, label %{next}\ninvoke_path_{index}:\n  %invoke_path_match_{index} = call i1 @glitch_route_match(ptr {path}, ptr %path)\n  br i1 %invoke_path_match_{index}, label %invoke_handler_{index}, label %{next}\ninvoke_handler_{index}:\n  %invoke_result_{index} = call ptr @{thunk}(ptr %app, ptr %path, ptr %body)\n  ret ptr %invoke_result_{index}\n"
                ));
                if index + 1 != routes.len() {
                    self.body.push_str(&format!("{next}:\n"));
                }
            }
            self.body
                .push_str(&format!("invoke_not_found:\n  ret ptr {not_found}\n}}\n\n"));
        }
        Ok(())
    }

    pub(super) fn emit_endpoint_thunk(
        &mut self,
        thunk: &str,
        handler: &EndpointHandlerBinding,
    ) -> Result<(), String> {
        let supported_return = self.endpoint_return_supported(&handler.response_type);
        let not_implemented = self.string_global("501 Not Implemented");
        self.body.push_str(&format!(
            "define ptr @{thunk}(ptr %app, ptr %path, ptr %body) {{\nentry:\n"
        ));

        if !supported_return
            || !handler
                .params
                .iter()
                .all(|param| self.endpoint_parameter_supported(param, &handler.path))
        {
            self.body
                .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
            return Ok(());
        }

        if let Some(controller_name) = &handler.controller {
            let Some(object) = self.object_types.get(controller_name).cloned() else {
                self.body
                    .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
                return Ok(());
            };
            let Some(method) = self.functions.get(&handler.function) else {
                self.body
                    .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
                return Ok(());
            };
            let mut expected_method_params = vec![LlType::Ptr];
            expected_method_params
                .extend(handler.params.iter().map(|param| llvm_ir_type(&param.ty)));
            if method.return_type != LlType::Ptr || method.params != expected_method_params {
                self.body
                    .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
                return Ok(());
            }
            if !handler
                .constructor_params
                .iter()
                .all(|ty| self.can_allocate_endpoint_dependency(ty))
            {
                self.body
                    .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
                return Ok(());
            }

            let llvm_name = llvm_object_name(&object.name);
            self.body.push_str(&format!(
                "  %size_ptr = getelementptr %{llvm_name}, ptr null, i32 1\n  %size = ptrtoint ptr %size_ptr to i64\n  %controller = call ptr @glitch_calloc(i64 1, i64 %size)\n"
            ));
            if matches!(object.kind, TypeKind::Class) {
                self.body.push_str(&format!(
                    "  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %controller, i32 0, i32 0\n  store i64 1, ptr %rc_ptr\n  %controller_drop_ptr = getelementptr inbounds %{llvm_name}, ptr %controller, i32 0, i32 1\n  store ptr @{}, ptr %controller_drop_ptr\n",
                    destroy_symbol(&object.name)
                ));
            }
            let mut dependencies = Vec::new();
            for (index, dependency_type) in handler.constructor_params.iter().enumerate() {
                let type_name = self
                    .resolve_endpoint_dependency_name(dependency_type)
                    .ok_or_else(|| {
                    format!(
                        "LLVM TIR backend: controller dependency has no object type: {dependency_type:?}"
                    )
                })?;
                let value = if is_service_provider_surface_type(dependency_type) {
                    self.emit_endpoint_app_service_provider("%app")?
                } else {
                    self.emit_endpoint_object_allocation(&type_name, &format!("dependency_{index}"))?
                };
                dependencies.push((type_name, value));
            }
            let controller_fields = object
                .fields
                .iter()
                .map(|(name, field)| (name.clone(), field.index, field.ty.clone()))
                .collect::<Vec<_>>();
            for (_field_name, field_index, field_ty) in controller_fields {
                if !is_service_provider_surface_type(&field_ty) {
                    continue;
                }
                let service_provider = self.emit_endpoint_app_service_provider("%app")?;
                let field_ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {field_ptr} = getelementptr inbounds %{llvm_name}, ptr %controller, i32 0, i32 {field_index}\n  store ptr {service_provider}, ptr {field_ptr}\n",
                    llvm_name = llvm_name
                ));
                if let Some(service_object) = self
                    .object_types
                    .values()
                    .find(|object| base_type_name(&object.name) == "ServiceProvider")
                    .cloned()
                {
                    self.body.push_str(&format!(
                        "  call void @{}(ptr {service_provider})\n",
                        retain_symbol(&service_object.name)
                    ));
                }
            }
            if let Some(constructor) = &handler.constructor {
                let mut args = vec!["ptr %controller".to_string()];
                args.extend(dependencies.iter().map(|(_, value)| format!("ptr {value}")));
                self.body.push_str(&format!(
                    "  call void @{}({})\n",
                    sanitize(constructor),
                    args.join(", ")
                ));
            }
            for (type_name, value) in &dependencies {
                let drop_name = self
                    .object_types
                    .get(type_name)
                    .map(|object| drop_symbol(&object.name))
                    .unwrap_or_else(|| drop_symbol(type_name));
                self.body.push_str(&format!(
                    "  call void @{}(ptr {value})\n",
                    drop_name
                ));
            }
            let (action_args, string_args, object_args) =
                self.emit_endpoint_action_arguments(handler, "%path", "%body")?;
            let mut rendered_action_args = vec!["ptr %controller".to_string()];
            rendered_action_args.extend(action_args);
            self.body.push_str(&format!(
                "  %result = call ptr @{}({})\n",
                sanitize(&handler.function),
                rendered_action_args.join(", ")
            ));
            for value in string_args {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {value})\n"
                ));
            }
            for (type_name, value) in object_args {
                let drop_name = self
                    .object_types
                    .get(&type_name)
                    .map(|object| drop_symbol(&object.name))
                    .unwrap_or_else(|| drop_symbol(&type_name));
                self.body.push_str(&format!(
                    "  call void @{}(ptr {value})\n",
                    drop_name
                ));
            }
            let response_value = self.emit_endpoint_handler_result_value(handler, "%result")?;
            let response = self.emit_endpoint_result(&handler.response_type, &response_value)?;
            self.body.push_str(&format!(
                "  call void @{}(ptr %controller)\n  ret ptr {response}\n}}\n\n",
                self
                    .object_types
                    .get(controller_name)
                    .map(|object| drop_symbol(&object.name))
                    .unwrap_or_else(|| drop_symbol(controller_name))
            ));
            return Ok(());
        }

        let Some(function) = self.functions.get(&handler.function) else {
            self.body
                .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
            return Ok(());
        };
        if function.return_type == LlType::Ptr && function.params.is_empty() {
            self.body.push_str(&format!("  %result = call ptr @{}()\n", sanitize(&handler.function)));
            let response_value = self.emit_endpoint_handler_result_value(handler, "%result")?;
            let response = self.emit_endpoint_result(&handler.response_type, &response_value)?;
            self.body.push_str(&format!("  ret ptr {response}\n}}\n\n"));
        } else {
            self.body
                .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
        }
        Ok(())
    }

    pub(super) fn emit_endpoint_action_arguments(
        &mut self,
        handler: &EndpointHandlerBinding,
        path: &str,
        body: &str,
    ) -> Result<(Vec<String>, Vec<String>, Vec<(String, String)>), String> {
        let mut args = Vec::new();
        let mut strings_to_release = Vec::new();
        let mut objects_to_release = Vec::new();
        for (index, param) in handler.params.iter().enumerate() {
            match (&param.source, &param.ty) {
                (EndpointParameterSource::Route, IrType::Int) => {
                    let segment =
                        route_parameter_segment(&handler.path, &param.name).ok_or_else(|| {
                            format!(
                                "LLVM TIR backend: route '{}' has no parameter '{}'",
                                handler.path, param.name
                            )
                        })?;
                    let raw = format!("%action_arg_{index}_i64");
                    let value = format!("%action_arg_{index}");
                    self.body.push_str(&format!(
                        "  {raw} = call i64 @glitch_path_segment_i64(ptr {path}, i64 {segment})\n  {value} = trunc i64 {raw} to i32\n"
                    ));
                    args.push(format!("i32 {value}"));
                }
                (EndpointParameterSource::Route, IrType::Long) => {
                    let segment =
                        route_parameter_segment(&handler.path, &param.name).ok_or_else(|| {
                            format!(
                                "LLVM TIR backend: route '{}' has no parameter '{}'",
                                handler.path, param.name
                            )
                        })?;
                    let value = format!("%action_arg_{index}");
                    self.body.push_str(&format!(
                        "  {value} = call i64 @glitch_path_segment_i64(ptr {path}, i64 {segment})\n"
                    ));
                    args.push(format!("i64 {value}"));
                }
                (EndpointParameterSource::Route, IrType::String) => {
                    let segment =
                        route_parameter_segment(&handler.path, &param.name).ok_or_else(|| {
                            format!(
                                "LLVM TIR backend: route '{}' has no parameter '{}'",
                                handler.path, param.name
                            )
                        })?;
                    let value = format!("%action_arg_{index}");
                    self.body.push_str(&format!(
                        "  {value} = call ptr @glitch_path_segment_string(ptr {path}, i64 {segment})\n"
                    ));
                    args.push(format!("ptr {value}"));
                    strings_to_release.push(value);
                }
                (EndpointParameterSource::Body, IrType::String) => {
                    let value = format!("%action_arg_{index}");
                    let empty = self.string_global("");
                    self.body.push_str(&format!(
                        "  {value} = call ptr @glitch_string_concat(ptr {body}, ptr {empty})\n"
                    ));
                    args.push(format!("ptr {value}"));
                    strings_to_release.push(value);
                }
                (EndpointParameterSource::Body, IrType::Class(type_name))
                | (EndpointParameterSource::Body, IrType::Struct(type_name)) => {
                    let value = self.emit_endpoint_body_object(
                        type_name,
                        body,
                        &format!("action_arg_{index}"),
                    )?;
                    args.push(format!("ptr {value}"));
                    objects_to_release.push((type_name.clone(), value));
                }
                (EndpointParameterSource::Query, IrType::String) => {
                    let value = format!("%action_arg_{index}");
                    let key = self.string_global(&param.name);
                    let key_length = param.name.len();
                    self.body.push_str(&format!(
                        "  {value} = call ptr @glitch_query_value_string(ptr {path}, ptr {key}, i64 {key_length})\n"
                    ));
                    args.push(format!("ptr {value}"));
                    strings_to_release.push(value);
                }
                (EndpointParameterSource::Query, IrType::Int) => {
                    let raw = format!("%action_arg_{index}_i64");
                    let value = format!("%action_arg_{index}");
                    let key = self.string_global(&param.name);
                    let key_length = param.name.len();
                    self.body.push_str(&format!(
                        "  {raw} = call i64 @glitch_query_value_i64(ptr {path}, ptr {key}, i64 {key_length})\n  {value} = trunc i64 {raw} to i32\n"
                    ));
                    args.push(format!("i32 {value}"));
                }
                (EndpointParameterSource::Query, IrType::Long) => {
                    let value = format!("%action_arg_{index}");
                    let key = self.string_global(&param.name);
                    let key_length = param.name.len();
                    self.body.push_str(&format!(
                        "  {value} = call i64 @glitch_query_value_i64(ptr {path}, ptr {key}, i64 {key_length})\n"
                    ));
                    args.push(format!("i64 {value}"));
                }
                (EndpointParameterSource::Query, IrType::Bool) => {
                    let text = format!("%action_arg_{index}_text");
                    let true_cmp = format!("%action_arg_{index}_true_cmp");
                    let one_cmp = format!("%action_arg_{index}_one_cmp");
                    let is_true = format!("%action_arg_{index}_true");
                    let is_one = format!("%action_arg_{index}_one");
                    let value = format!("%action_arg_{index}");
                    let key = self.string_global(&param.name);
                    let true_text = self.string_global("true");
                    let one_text = self.string_global("1");
                    let key_length = param.name.len();
                    self.body.push_str(&format!(
                        "  {text} = call ptr @glitch_query_value_string(ptr {path}, ptr {key}, i64 {key_length})\n  {true_cmp} = call i32 @strcmp(ptr {text}, ptr {true_text})\n  {one_cmp} = call i32 @strcmp(ptr {text}, ptr {one_text})\n  {is_true} = icmp eq i32 {true_cmp}, 0\n  {is_one} = icmp eq i32 {one_cmp}, 0\n  {value} = or i1 {is_true}, {is_one}\n  call void @glitch_string_release(ptr {text})\n"
                    ));
                    args.push(format!("i1 {value}"));
                }
                _ => {
                    return Err(format!(
                        "LLVM TIR backend: unsupported endpoint parameter {param:?}"
                    ));
                }
            }
        }
        Ok((args, strings_to_release, objects_to_release))
    }

    pub(super) fn endpoint_parameter_supported(&self, param: &EndpointParameterBinding, route: &str) -> bool {
        match (&param.source, &param.ty) {
            (EndpointParameterSource::Route, IrType::Int | IrType::Long | IrType::String) => {
                route_parameter_segment(route, &param.name).is_some()
            }
            (EndpointParameterSource::Body, IrType::String) => true,
            (EndpointParameterSource::Body, IrType::Class(name) | IrType::Struct(name)) => {
                self.object_types.get(name).is_some_and(|object| {
                    !matches!(object.kind, TypeKind::Interface)
                        && object.fields.values().all(|field| {
                            matches!(
                                field.ty,
                                IrType::Bool
                                    | IrType::Byte
                                    | IrType::Short
                                    | IrType::Int
                                    | IrType::Long
                                    | IrType::UInt
                                    | IrType::Double
                                    | IrType::String
                            )
                        })
                        && object.constructor.as_ref().is_none_or(|constructor| {
                            self.functions
                                .get(constructor)
                                .is_some_and(|signature| signature.params == vec![LlType::Ptr])
                        })
                })
            }
            (
                EndpointParameterSource::Query,
                IrType::Bool | IrType::Int | IrType::Long | IrType::String,
            ) => true,
            _ => false,
        }
    }

    pub(super) fn endpoint_return_supported(&self, ty: &IrType) -> bool {
        match ty {
            IrType::String => true,
            IrType::Class(name) | IrType::Struct(name) => {
                self.object_types.get(name).is_some_and(|object| {
                    !matches!(object.kind, TypeKind::Interface)
                        && object.fields.values().all(|field| {
                            matches!(
                                field.ty,
                                IrType::Bool
                                    | IrType::Byte
                                    | IrType::Short
                                    | IrType::Int
                                    | IrType::Long
                                    | IrType::UInt
                                    | IrType::Double
                                    | IrType::Decimal
                                    | IrType::String
                            )
                        })
                })
            }
            _ => false,
        }
    }

    pub(super) fn emit_endpoint_result(&mut self, ty: &IrType, value: &str) -> Result<String, String> {
        match ty {
            IrType::String => Ok(value.to_string()),
            IrType::Class(name) | IrType::Struct(name) => {
                let null_label = self.next_label("endpoint_result_null");
                let value_label = self.next_label("endpoint_result_value");
                let end_label = self.next_label("endpoint_result_end");
                let is_null = self.tmp();
                self.body.push_str(&format!(
                    "  {is_null} = icmp eq ptr {value}, null\n  br i1 {is_null}, label %{null_label}, label %{value_label}\n{value_label}:\n"
                ));
                let json = self.emit_json_serialize_object(name, value)?;
                self.body.push_str(&format!(
                    "  call void @{}(ptr {value})\n  br label %{end_label}\n{null_label}:\n  br label %{end_label}\n{end_label}:\n",
                    drop_symbol(name)
                ));
                let result = self.tmp();
                let null_text = self.string_global("null");
                self.body.push_str(&format!(
                    "  {result} = phi ptr [{json}, %{value_label}], [{null_text}, %{null_label}]\n"
                ));
                Ok(result)
            }
            _ => Err(format!(
                "LLVM TIR backend: unsupported endpoint result type {ty:?}"
            )),
        }
    }

    pub(super) fn emit_json_serialize_object(
        &mut self,
        type_name: &str,
        value: &str,
    ) -> Result<String, String> {
        let object = self.object_layout(type_name).cloned().ok_or_else(|| {
            format!("LLVM TIR backend: result type '{type_name}' has no object layout")
        })?;
        let llvm_name = llvm_object_name(type_name);
        let mut fields = object.fields.iter().collect::<Vec<_>>();
        fields.sort_by_key(|(_, field)| field.index);
        let mut current = self.string_global("{");
        let mut current_is_managed = false;
        for (position, (field_name, field)) in fields.into_iter().enumerate() {
            let key = self.string_global(&format!(
                "{}\"{}\":",
                if position == 0 { "" } else { "," },
                field_name
            ));
            let with_key = format!("%json_{}_{}_key", sanitize(type_name), field.index);
            self.body.push_str(&format!(
                "  {with_key} = call ptr @glitch_string_concat(ptr {current}, ptr {key})\n"
            ));
            if current_is_managed {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {current})\n"
                ));
            }
            let field_ptr = format!("%json_{}_{}_ptr", sanitize(type_name), field.index);
            let field_value = format!("%json_{}_{}", sanitize(type_name), field.index);
            let field_type = llvm_ir_type(&field.ty);
            self.body.push_str(&format!(
                "  {field_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 {}\n  {field_value} = load {}, ptr {field_ptr}\n",
                field.index,
                field_type.as_ir()
            ));
            let (rendered, rendered_is_managed) = match &field.ty {
                IrType::String => {
                    let quote = self.string_global("\"");
                    let quoted = format!("%json_{}_{}_quoted", sanitize(type_name), field.index);
                    let complete =
                        format!("%json_{}_{}_complete", sanitize(type_name), field.index);
                    self.body.push_str(&format!(
                        "  {quoted} = call ptr @glitch_string_concat(ptr {quote}, ptr {field_value})\n  {complete} = call ptr @glitch_string_concat(ptr {quoted}, ptr {quote})\n  call void @glitch_string_release(ptr {quoted})\n"
                    ));
                    (complete, true)
                }
                IrType::Bool => {
                    let rendered = format!("%json_{}_{}_bool", sanitize(type_name), field.index);
                    self.body.push_str(&format!(
                        "  {rendered} = select i1 {field_value}, ptr getelementptr inbounds ([5 x i8], ptr @.json_true, i64 0, i64 0), ptr getelementptr inbounds ([6 x i8], ptr @.json_false, i64 0, i64 0)\n"
                    ));
                    (rendered, false)
                }
                IrType::Double | IrType::Decimal => {
                    let rendered = format!("%json_{}_{}_double", sanitize(type_name), field.index);
                    self.body.push_str(&format!(
                        "  {rendered} = call ptr @glitch_double_to_string(double {field_value})\n"
                    ));
                    (rendered, true)
                }
                IrType::Byte | IrType::Short | IrType::Int | IrType::Long | IrType::UInt => {
                    let wide = if field_type == LlType::I64 {
                        field_value.clone()
                    } else {
                        let wide = format!("%json_{}_{}_wide", sanitize(type_name), field.index);
                        let extension = if matches!(field.ty, IrType::Byte | IrType::UInt) {
                            "zext"
                        } else {
                            "sext"
                        };
                        self.body.push_str(&format!(
                            "  {wide} = {extension} {} {field_value} to i64\n",
                            field_type.as_ir()
                        ));
                        wide
                    };
                    let rendered = format!("%json_{}_{}_integer", sanitize(type_name), field.index);
                    self.body.push_str(&format!(
                        "  {rendered} = call ptr @glitch_i64_to_string(i64 {wide})\n"
                    ));
                    (rendered, true)
                }
                _ => {
                    return Err(format!(
                        "LLVM TIR backend: JSON result field '{type_name}.{field_name}' has unsupported type {:?}",
                        field.ty
                    ));
                }
            };
            let next = format!("%json_{}_{}_value", sanitize(type_name), field.index);
            self.body.push_str(&format!(
                "  {next} = call ptr @glitch_string_concat(ptr {with_key}, ptr {rendered})\n  call void @glitch_string_release(ptr {with_key})\n"
            ));
            if rendered_is_managed {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {rendered})\n"
                ));
            }
            current = next;
            current_is_managed = true;
        }
        let close = self.string_global("}");
        let result = format!("%json_{}_result", sanitize(type_name));
        self.body.push_str(&format!(
            "  {result} = call ptr @glitch_string_concat(ptr {current}, ptr {close})\n"
        ));
        if current_is_managed {
            self.body.push_str(&format!(
                "  call void @glitch_string_release(ptr {current})\n"
            ));
        }
        Ok(result)
    }

    pub(super) fn emit_endpoint_body_object(
        &mut self,
        type_name: &str,
        body: &str,
        prefix: &str,
    ) -> Result<String, String> {
        let object = self.object_layout(type_name).cloned().ok_or_else(|| {
            format!("LLVM TIR backend: body type '{type_name}' has no object layout")
        })?;
        let value = self.emit_endpoint_object_allocation(type_name, prefix)?;
        let llvm_name = llvm_object_name(type_name);
        for (field_name, field) in &object.fields {
            let token = self.string_global(&format!("\"{field_name}\""));
            let token_length = field_name.len() + 2;
            let field_ptr = format!("%{prefix}_field_{}_ptr", sanitize(field_name));
            self.body.push_str(&format!(
                "  {field_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 {}\n",
                field.index
            ));
            match &field.ty {
                IrType::String => {
                    let field_value = format!("%{prefix}_field_{}", sanitize(field_name));
                    self.body.push_str(&format!(
                        "  {field_value} = call ptr @glitch_json_value_string(ptr {body}, ptr {token}, i64 {token_length})\n  store ptr {field_value}, ptr {field_ptr}\n"
                    ));
                }
                IrType::Bool => {
                    let field_value = format!("%{prefix}_field_{}", sanitize(field_name));
                    self.body.push_str(&format!(
                        "  {field_value} = call i1 @glitch_json_value_bool(ptr {body}, ptr {token}, i64 {token_length})\n  store i1 {field_value}, ptr {field_ptr}\n"
                    ));
                }
                IrType::Byte | IrType::Short | IrType::Int | IrType::Long | IrType::UInt => {
                    let raw = format!("%{prefix}_field_{}_i64", sanitize(field_name));
                    let field_type = llvm_ir_type(&field.ty);
                    let field_value = format!("%{prefix}_field_{}", sanitize(field_name));
                    self.body.push_str(&format!(
                        "  {raw} = call i64 @glitch_json_value_i64(ptr {body}, ptr {token}, i64 {token_length})\n"
                    ));
                    if field_type == LlType::I64 {
                        self.body
                            .push_str(&format!("  store i64 {raw}, ptr {field_ptr}\n"));
                    } else {
                        self.body.push_str(&format!(
                            "  {field_value} = trunc i64 {raw} to {}\n  store {} {field_value}, ptr {field_ptr}\n",
                            field_type.as_ir(),
                            field_type.as_ir()
                        ));
                    }
                }
                IrType::Double | IrType::Decimal => {
                    let field_value = format!("%{prefix}_field_{}", sanitize(field_name));
                    self.body.push_str(&format!(
                        "  {field_value} = call double @glitch_json_value_double(ptr {body}, ptr {token}, i64 {token_length})\n  store double {field_value}, ptr {field_ptr}\n"
                    ));
                }
                _ => {
                    return Err(format!(
                        "LLVM TIR backend: JSON field '{type_name}.{field_name}' has unsupported type {:?}",
                        field.ty
                    ));
                }
            }
        }
        Ok(value)
    }

    pub(super) fn can_allocate_endpoint_dependency(&self, ty: &IrType) -> bool {
        self.can_allocate_endpoint_dependency_inner(ty, &mut std::collections::HashSet::new())
    }

    pub(super) fn can_allocate_endpoint_dependency_inner(
        &self,
        ty: &IrType,
        visiting: &mut std::collections::HashSet<String>,
    ) -> bool {
        let Some(type_name) = self.resolve_endpoint_dependency_name(ty) else {
            return false;
        };
        if !visiting.insert(type_name.clone()) {
            return false;
        }
        let Some(object) = self.object_layout(&type_name) else {
            visiting.remove(&type_name);
            return false;
        };
        if matches!(object.kind, TypeKind::Interface) {
            visiting.remove(&type_name);
            return false;
        }
        let supported = object
            .constructor_params
            .iter()
            .all(|param| self.can_allocate_endpoint_dependency_inner(param, visiting));
        visiting.remove(&type_name);
        supported
    }

    pub(super) fn resolve_endpoint_dependency_name(&self, ty: &IrType) -> Option<String> {
        match ty {
            IrType::Class(name) | IrType::Struct(name) => {
                self.object_layout(name).map(|object| object.name.clone())
            }
            IrType::Interface(name) => self.resolve_interface_implementation(name),
            _ => None,
        }
    }

    pub(super) fn resolve_interface_implementation(&self, interface_name: &str) -> Option<String> {
        let mut exact = self
            .object_types
            .values()
            .filter(|object| {
                matches!(object.kind, TypeKind::Class)
                    && object.bases.iter().any(|base| base == interface_name)
            })
            .map(|object| object.name.clone())
            .collect::<Vec<_>>();
        exact.sort();
        exact.dedup();
        if exact.len() == 1 {
            return exact.pop();
        }

        let mut candidates = self
            .object_types
            .values()
            .filter(|object| {
                matches!(object.kind, TypeKind::Class)
                    && object.bases.iter().any(|base| {
                        base == interface_name
                            || base_type_name(base) == base_type_name(interface_name)
                    })
            })
            .map(|object| object.name.clone())
            .collect::<Vec<_>>();
        candidates.sort();
        candidates.dedup();
        (candidates.len() == 1).then(|| candidates.remove(0))
    }

    pub(super) fn resolve_interface_method_symbol(
        &self,
        interface_name: &str,
        method_name: &str,
        arg_count: usize,
    ) -> Option<String> {
        let implementation = self.resolve_interface_implementation(interface_name)?;
        let prefix = format!("{}_{}", sanitize(&implementation), sanitize(method_name));
        let mut candidates = self
            .functions
            .iter()
            .filter(|(symbol, signature)| {
                (symbol.as_str() == prefix || symbol.starts_with(&format!("{prefix}__")))
                    && signature.params.len() == arg_count + 1
            })
            .map(|(symbol, _)| symbol.clone())
            .collect::<Vec<_>>();
        candidates.sort();
        candidates.dedup();
        (candidates.len() == 1).then(|| candidates.remove(0))
    }

    pub(super) fn emit_endpoint_object_allocation(
        &mut self,
        type_name: &str,
        prefix: &str,
    ) -> Result<String, String> {
        self.emit_endpoint_object_allocation_inner(
            type_name,
            prefix,
            &mut std::collections::HashSet::new(),
        )
    }

    pub(super) fn emit_endpoint_object_allocation_inner(
        &mut self,
        type_name: &str,
        prefix: &str,
        visiting: &mut std::collections::HashSet<String>,
    ) -> Result<String, String> {
        if !visiting.insert(type_name.to_string()) {
            return Err(format!(
                "LLVM TIR backend: dependency cycle while activating '{type_name}'"
            ));
        }
        let object = self.object_layout(type_name).cloned().ok_or_else(|| {
            format!("LLVM TIR backend: dependency type '{type_name}' has no layout")
        })?;
        let llvm_name = llvm_object_name(type_name);
        let size_ptr = format!("%{prefix}_size_ptr");
        let size = format!("%{prefix}_size");
        let value = format!("%{prefix}");
        self.body.push_str(&format!(
            "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {value} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
        ));
        if matches!(object.kind, TypeKind::Class) {
            let rc_ptr = format!("%{prefix}_rc_ptr");
            let drop_ptr = format!("%{prefix}_drop_ptr");
            self.body.push_str(&format!(
                "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 1\n  store ptr @{}, ptr {drop_ptr}\n",
                destroy_symbol(type_name)
            ));
        }
        let mut dependencies = Vec::new();
        for (index, dependency_type) in object.constructor_params.iter().enumerate() {
            let dependency_name = self
                .resolve_endpoint_dependency_name(dependency_type)
                .ok_or_else(|| {
                    format!(
                        "LLVM TIR backend: cannot resolve dependency {dependency_type:?} for '{type_name}'"
                    )
                })?;
            let dependency_value = self.emit_endpoint_object_allocation_inner(
                &dependency_name,
                &format!("{prefix}_dependency_{index}"),
                visiting,
            )?;
            dependencies.push((dependency_name, dependency_value));
        }
        if let Some(constructor) = &object.constructor {
            let mut args = vec![format!("ptr {value}")];
            args.extend(
                dependencies
                    .iter()
                    .map(|(_, dependency)| format!("ptr {dependency}")),
            );
            self.body.push_str(&format!(
                "  call void @{}({})\n",
                sanitize(constructor),
                args.join(", ")
            ));
        }
        for (dependency_name, dependency_value) in dependencies {
            self.body.push_str(&format!(
                "  call void @{}(ptr {dependency_value})\n",
                drop_symbol(&dependency_name)
            ));
        }
        visiting.remove(type_name);
        Ok(value)
    }

    fn emit_endpoint_app_service_provider(&mut self, app_value: &str) -> Result<String, String> {
        let app = self
            .object_layout("WebApplication")
            .cloned()
            .ok_or_else(|| {
                "LLVM TIR backend: WebApplication layout is not available for controller activation"
                    .to_string()
            })?;
        let services_field = app.fields.get("Services").cloned().ok_or_else(|| {
            format!(
                "LLVM TIR backend: WebApplication layout '{}' has no Services field",
                app.name
            )
        })?;
        let service_provider = self.tmp();
        let service_provider_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {service_provider_ptr} = getelementptr inbounds %{llvm_name}, ptr {app_value}, i32 0, i32 {field_index}\n  {service_provider} = load ptr, ptr {service_provider_ptr}\n",
            llvm_name = llvm_object_name(&app.name),
            field_index = services_field.index
        ));
        let service_object = match &services_field.ty {
            IrType::Class(name)
            | IrType::Struct(name)
            | IrType::Interface(name)
            | IrType::Unknown(name) => self.object_layout(name).cloned(),
            _ => None,
        }
        .or_else(|| {
            self.object_types
                .values()
                .find(|object| base_type_name(&object.name) == "ServiceProvider")
                .cloned()
        });
        if let Some(service_object) = service_object {
            self.body.push_str(&format!(
                "  call void @{}(ptr {service_provider})\n",
                retain_symbol(&service_object.name)
            ));
        }
        Ok(service_provider)
    }

    fn object_layout(&self, type_name: &str) -> Option<&LlObjectType> {
        self.object_types.get(type_name).or_else(|| {
            self.object_types.values().find(|object| {
                object.name == type_name
                    || object.name.ends_with(type_name)
                    || type_name.ends_with(&object.name)
                    || base_type_name(&object.name) == base_type_name(type_name)
            })
        })
    }

    pub(super) fn emit_mediator_send(
        &mut self,
        mediator: &TypedExpr,
        request: &TypedExpr,
        response_ty: &IrType,
    ) -> Result<LlValue, String> {
        let mediator_value = self.emit_typed_expr(mediator)?;
        let mediator_iface = match &mediator.ty {
            IrType::Interface(name) => name.clone(),
            _ => {
                return Err("LLVM TIR backend: mediator dispatch expects an IMediator target".to_string())
            }
        };
        let get_provider_symbol = self
            .resolve_interface_method_symbol(&mediator_iface, "GetProvider", 0)
            .ok_or_else(|| {
                format!(
                    "LLVM TIR backend: no provider accessor found for mediator interface '{mediator_iface}'"
                )
            })?;
        let provider = self.tmp();
        self.body.push_str(&format!(
            "  {provider} = call ptr @{}(ptr {})\n",
            sanitize(&get_provider_symbol),
            mediator_value.value
        ));

        let request_name = object_type_name(&request.ty)
            .or_else(|| match &request.ty {
                IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => {
                    Some(name.as_str())
                }
                IrType::Unknown(name) => Some(name.as_str()),
                _ => None,
            })
            .ok_or_else(|| {
                format!(
                    "LLVM TIR backend: mediator request '{:?}' has no object layout",
                    request.ty
                )
            })?;

        let handler = self
            .object_types
            .values()
            .find(|object| {
                object.kind == TypeKind::Class
                    && object.bases.iter().any(|base| {
                        let base_name = base_type_name(base);
                        if base_name != "IRequestHandler" {
                            return false;
                        }
                        match split_monomorphized_type(base) {
                            Some((_, args)) if !args.is_empty() => {
                                request_type_matches(&args[0], request_name)
                            }
                            _ => base.contains(request_name),
                        }
                    })
            })
            .cloned()
            .ok_or_else(|| {
                format!(
                    "LLVM TIR backend: no IRequestHandler<{request_name}, _> implementation found"
                )
            })?;

        let inferred_response_ty = handler
            .bases
            .iter()
            .find_map(|base| {
                let base_name = base_type_name(base);
                if base_name != "IRequestHandler" {
                    return None;
                }
                let (_, args) = split_monomorphized_type(base)?;
                match args.as_slice() {
                    [request, response] if request_type_matches(request, request_name) => {
                        parse_monomorphized_rc_inner_type(response, &self.object_types)
                    }
                    [request] if request_type_matches(request, request_name) => Some(IrType::Void),
                    _ => None,
                }
            })
            .unwrap_or_else(|| response_ty.clone());

        let handler_ptr = self.tmp();
        let handler_size_ptr = self.tmp();
        let handler_size = self.tmp();
        self.body.push_str(&format!(
            "  {handler_size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {handler_size} = ptrtoint ptr {handler_size_ptr} to i64\n  {handler_ptr} = call ptr @glitch_calloc(i64 1, i64 {handler_size})\n",
            llvm_name = llvm_object_name(&handler.name)
        ));
        let handler_rc_ptr = self.tmp();
        let handler_drop_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {handler_rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {handler_ptr}, i32 0, i32 0\n  store i64 1, ptr {handler_rc_ptr}\n  {handler_drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {handler_ptr}, i32 0, i32 1\n  store ptr @{}, ptr {handler_drop_ptr}\n",
            destroy_symbol(&handler.name),
            llvm_name = llvm_object_name(&handler.name)
        ));

        let mut ctor_args = vec![format!("ptr {handler_ptr}")];
        for (index, dependency_ty) in handler.constructor_params.iter().enumerate() {
            let dependency_name = self.resolve_endpoint_dependency_name(dependency_ty).ok_or_else(|| {
                format!(
                    "LLVM TIR backend: mediator dependency {dependency_ty:?} for '{}' has no resolved implementation",
                    handler.name
                )
            })?;
            let dependency_value = if base_type_name(&dependency_name) == "ConduitContext" {
                let provider_call = self.tmp();
                let service_name = self.string_global(&dependency_name);
                let get_service_symbol = self
                    .resolve_interface_method_symbol(
                        "Microsoft.Extensions.DependencyInjection.IServiceProvider",
                        "GetRequiredService",
                        1,
                    )
                    .or_else(|| {
                        self.resolve_interface_method_symbol(
                            "IServiceProvider",
                            "GetRequiredService",
                            1,
                        )
                    })
                    .ok_or_else(|| {
                        "LLVM TIR backend: IServiceProvider.GetRequiredService(string) is not available"
                            .to_string()
                    })?;
                self.body.push_str(&format!(
                    "  {provider_call} = call ptr @{}(ptr {provider}, ptr {service_name})\n",
                    sanitize(&get_service_symbol)
                ));
                provider_call
            } else {
                self.emit_endpoint_object_allocation(
                    &dependency_name,
                    &format!("mediator_dependency_{index}"),
                )?
            };
            let _ = index;
            ctor_args.push(format!("ptr {dependency_value}"));
        }

        if let Some(constructor) = &handler.constructor {
            self.body.push_str(&format!(
                "  call void @{}({})\n",
                sanitize(constructor),
                ctor_args.join(", ")
            ));
        }

        let handle_symbol = self
            .functions
            .keys()
            .find(|symbol| {
                symbol.starts_with(&format!("{}_", sanitize(&handler.name)))
                    && symbol.contains("_Handle")
            })
            .cloned()
            .ok_or_else(|| {
                format!(
                    "LLVM TIR backend: no lowered Handle method found for mediator handler '{}'",
                    handler.name
                )
            })?;

        let request_value = self.emit_typed_expr(request)?;
        let token = "null".to_string();
        let result = self.tmp();
        self.body.push_str(&format!(
            "  {result} = call {} @{}(ptr {handler_ptr}, {} {}, ptr {token})\n",
            llvm_ir_type(&IrType::Task(Box::new(response_ty.clone()))).as_ir(),
            sanitize(&handle_symbol),
            request_value.ty.as_ir(),
            request_value.value
        ));
        Ok(LlValue {
            value: result,
            ty: llvm_ir_type(&IrType::Task(Box::new(inferred_response_ty))),
        })
    }


    pub(super) fn emit_mapper_map(
        &mut self,
        mapper: &TypedExpr,
        source: &TypedExpr,
        generic_args: &[IrType],
        dest_ty: &IrType,
    ) -> Result<LlValue, String> {
        let mapper_value = self.emit_typed_expr(mapper)?;
        self.emit_temporary_drop(mapper, &mapper_value);
        let source_value = self.emit_typed_expr(source)?;
        let source_hint = generic_args.first().unwrap_or(&source.ty);
        let dest_hint = generic_args.get(1).unwrap_or(dest_ty);
        let source_name = object_type_name(&source.ty)
            .or_else(|| match &source.ty {
                IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => {
                    Some(name.as_str())
                }
                IrType::Unknown(_) => object_type_name(source_hint).or_else(|| match source_hint {
                    IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => {
                        Some(name.as_str())
                    }
                    _ => None,
                }),
                _ => None,
            });
        let dest_name = object_type_name(dest_ty)
            .or_else(|| match dest_ty {
                IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => {
                    Some(name.as_str())
                }
                IrType::Unknown(_) => object_type_name(dest_hint).or_else(|| match dest_hint {
                    IrType::Class(name) | IrType::Struct(name) | IrType::Interface(name) => {
                        Some(name.as_str())
                    }
                    _ => None,
                }),
                _ => None,
            })
            .map(|name| name.to_string());
        let Some(dest_name) = dest_name else {
            return Ok(source_value);
        };
        let Some(source_name) = source_name else {
            return Ok(source_value);
        };
        let source_object = self.object_layout(source_name).cloned();
        let dest_object = self
            .object_layout(&dest_name)
            .cloned()
            .unwrap_or_else(|| LlObjectType {
                name: dest_name.clone(),
                kind: TypeKind::Class,
                bases: Vec::new(),
                fields: HashMap::new(),
                constructor: None,
                constructor_params: Vec::new(),
            });

        let mapped = self.emit_new_object(&dest_name, None, &[], &[])?;
        let Some(source_object) = source_object else {
            self.emit_temporary_drop(source, &source_value);
            return Ok(mapped);
        };
        for (field_name, dest_field) in &dest_object.fields {
            let Some(source_field) = source_object.fields.get(field_name) else {
                continue;
            };
            let source_expr = TypedExpr {
                kind: TypedExprKind::Field {
                    target: Box::new(source.clone()),
                    name: field_name.clone(),
                },
                ty: source_field.ty.clone(),
                ownership: Ownership::Owned,
                drop_kind: source_field.drop_kind.clone(),
            };
            let field_value = self.emit_typed_expr(&source_expr)?;
            let field_ty = llvm_ir_type(&dest_field.ty);
            let casted = self.cast_value(field_value, &field_ty)?;
            self.retain_for_store(&dest_field.ty, &source_expr, &casted.value);
            let ptr = self.tmp();
            self.body.push_str(&format!(
                "  {ptr} = getelementptr inbounds %{llvm_name}, ptr {}, i32 0, i32 {}\n  store {} {}, ptr {ptr}\n",
                mapped.value,
                dest_field.index,
                field_ty.as_ir(),
                casted.value,
                llvm_name = llvm_object_name(&dest_object.name)
            ));
        }
        self.emit_temporary_drop(source, &source_value);
        Ok(mapped)
    }
}

fn request_type_matches(object_name: &str, request_name: &str) -> bool {
    if object_name == request_name {
        return true;
    }
    let object_base = base_type_name(object_name);
    let request_base = base_type_name(request_name);
    object_name.ends_with(request_name)
        || request_name.ends_with(object_name)
        || object_base == request_base
        || object_base.ends_with(request_base)
        || request_base.ends_with(object_base)
}

