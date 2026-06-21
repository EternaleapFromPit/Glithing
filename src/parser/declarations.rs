use super::*;

impl Parser {
    pub(super) fn parse_enum_def(
        &mut self,
        package_id: Option<String>,
        visibility: Visibility,
        namespace: Vec<String>,
        attributes: Vec<Attribute>,
    ) -> Result<EnumDef, String> {
        self.expect(TokenKind::Enum)?;
        let name = self.expect_ident()?;
        if self.match_kind(&TokenKind::Colon) {
            self.parse_type_syntax()?
                .ok_or_else(|| self.error_here("expected enum underlying type"))?;
        }
        self.expect(TokenKind::LBrace)?;
        let mut variants = Vec::new();
        while !self.at(&TokenKind::RBrace) {
            let variant_name = self.expect_ident()?;
            let value = if self.match_kind(&TokenKind::Eq) {
                Some(self.parse_enum_value()?)
            } else {
                None
            };
            variants.push(EnumVariant {
                name: variant_name,
                value,
            });
            if !self.match_kind(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(EnumDef {
            package_id,
            visibility,
            namespace,
            attributes,
            name,
            variants,
        })
    }

    pub(super) fn parse_enum_value(&mut self) -> Result<i64, String> {
        let TokenKind::Int(mut value) = self.current().kind.clone() else {
            return Err(self.error_here("expected enum integer value"));
        };
        self.advance();
        if self.match_kind(&TokenKind::Less) {
            self.expect(TokenKind::Less)?;
            let TokenKind::Int(shift) = self.current().kind.clone() else {
                return Err(self.error_here("expected enum shift amount"));
            };
            self.advance();
            if shift < 0 || shift > 62 {
                return Err(self.error_here("enum shift amount must be between 0 and 62"));
            }
            value = value
                .checked_shl(shift as u32)
                .ok_or_else(|| self.error_here("enum shift value overflowed"))?;
        }
        Ok(value)
    }

    pub(super) fn parse_type_def(
        &mut self,
        package_id: Option<String>,
        visibility: Visibility,
        namespace: Vec<String>,
        attributes: Vec<Attribute>,
        nested_types: &mut Vec<TypeDef>,
        native_c: &mut Vec<String>,
        delegates: &mut Vec<DelegateDef>,
    ) -> Result<TypeDef, String> {
        let is_record = self.match_kind(&TokenKind::Record);
        let kind = if self.match_kind(&TokenKind::Ref) {
            self.expect(TokenKind::Struct)?;
            TypeKind::RefStruct
        } else if self.match_kind(&TokenKind::Struct) {
            TypeKind::Struct
        } else if is_record {
            TypeKind::Class
        } else if self.match_kind(&TokenKind::Interface) {
            TypeKind::Interface
        } else {
            self.expect(TokenKind::Class)?;
            TypeKind::Class
        };
        let name = self.expect_ident()?;
        let generic_params = self.parse_generic_params()?;
        let primary_params = if matches!(kind, TypeKind::Class) && self.match_kind(&TokenKind::LParen)
        {
            let params = self.parse_params()?;
            self.expect(TokenKind::RParen)?;
            Some(params)
        } else {
            None
        };
        let bases = if self.match_kind(&TokenKind::Colon) {
            let mut bases = Vec::new();
            loop {
                let base_ty = self
                    .parse_type_syntax()?
                    .ok_or_else(|| self.error_here("expected base type"))?;
                if self.at(&TokenKind::LParen) {
                    self.expect(TokenKind::LParen)?;
                    let _ = self.parse_expr_list_until(TokenKind::RParen)?;
                }
                bases.push(type_syntax_name(&base_ty));
                if !self.match_kind(&TokenKind::Comma) {
                    break;
                }
            }
            bases
        } else {
            Vec::new()
        };
        let generic_params = self.parse_generic_constraints(generic_params)?;
        let mut fields = Vec::new();
        let mut constructors = Vec::new();
        if let Some(params) = &primary_params {
            for param in params {
                fields.push(FieldDef {
                    visibility,
                    name: param.name.clone(),
                    ty: param.ty.clone(),
                    is_static: false,
                    initializer: None,
                });
            }
            constructors.push(Constructor {
                visibility,
                namespace: namespace.clone(),
                attributes: attributes.clone(),
                params: params.clone(),
                body: params
                    .iter()
                    .map(|param| Stmt::AssignTarget {
                        target: Expr::Field {
                            target: Box::new(Expr::Var("this".to_string())),
                            name: param.name.clone(),
                        },
                        expr: Expr::Var(param.name.clone()),
                    })
                    .collect(),
            });
        }
        if is_record && self.match_kind(&TokenKind::Semi) {
            return Ok(TypeDef {
                package_id,
                visibility,
                kind,
                is_extension: false,
                namespace,
                attributes,
                name,
                generic_params,
                bases,
                fields,
                constructors,
                methods: Vec::new(),
            });
        }
        self.expect(TokenKind::LBrace)?;
        let mut methods = Vec::new();
        while !self.at(&TokenKind::RBrace) {
            if self.at(&TokenKind::Hash) {
                self.skip_preprocessor_directive();
                continue;
            }
            if let TokenKind::Ident(ref name) = self.current().kind {
                if self.macros.contains_key(name) {
                    let macro_name = name.clone();
                    self.advance();
                    let args = self.parse_macro_args()?;
                    let macro_def = self.macros.get(&macro_name).unwrap().clone();
                    if args.len() != macro_def.params.len() {
                        return Err(format!(
                            "macro {} expected {} arguments, found {}",
                            macro_name,
                            macro_def.params.len(),
                            args.len()
                        ));
                    }
                    let expanded = self.expand_macro(&macro_def, &args);
                    self.expect(TokenKind::Semi)?;
                    if !expanded.is_empty() {
                        let mut temp_parser = Parser {
                            tokens: expanded,
                            pos: 0,
                            using_aliases: self.using_aliases.clone(),
                            macros: self.macros.clone(),
                            test_registrations: Vec::new(),
                        };
                        let stmt = temp_parser.parse_stmt()?;
                        self.test_registrations.push(stmt);
                        self.test_registrations
                            .extend(temp_parser.test_registrations);
                    }
                    continue;
                }
            }
            let member_attributes = self.parse_attributes()?;
            let member_modifiers = self.parse_modifiers();
            if self.match_kind(&TokenKind::Native) {
                let TokenKind::String(source) = self.current().kind.clone() else {
                    return Err(self.error_here("expected native C string"));
                };
                self.advance();
                self.match_kind(&TokenKind::Semi);
                native_c.push(source);
                continue;
            }
            if self.at(&TokenKind::Ref)
                || self.at(&TokenKind::Struct)
                || self.at(&TokenKind::Class)
                || self.at(&TokenKind::Record)
                || self.at(&TokenKind::Interface)
            {
                let nested = self.parse_type_def(
                    package_id.clone(),
                    member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    namespace.clone(),
                    member_attributes,
                    nested_types,
                    native_c,
                    delegates,
                )?;
                nested_types.push(nested);
                continue;
            }
            if self.at(&TokenKind::Enum) {
                let _nested = self.parse_enum_def(
                    package_id.clone(),
                    member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    namespace.clone(),
                    member_attributes,
                )?;
                continue;
            }
            if self.match_kind(&TokenKind::Delegate) {
                delegates.push(self.parse_delegate_decl(
                    package_id.clone(),
                    member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    namespace.clone(),
                    member_attributes,
                )?);
                continue;
            }
            if self.current_ident_is(&name) && self.peek_is(1, &TokenKind::LParen) {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let params = self.parse_params()?;
                self.expect(TokenKind::RParen)?;
                if self.match_kind(&TokenKind::Colon) {
                    let initializer = self.expect_ident()?;
                    if initializer != "base" && initializer != "this" {
                        return Err(
                            self.error_here("expected base or this constructor initializer")
                        );
                    }
                    self.expect(TokenKind::LParen)?;
                    let _args = self.parse_call_args_after_lparen()?;
                }
                let body = if self.match_kind(&TokenKind::Arrow) {
                    vec![self.parse_stmt()?]
                } else {
                    self.parse_stmt_body()?
                };
                constructors.push(Constructor {
                    visibility: member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    namespace: namespace.clone(),
                    attributes: member_attributes,
                    params,
                    body,
                });
                continue;
            }
            if self.at(&TokenKind::Fn) {
                methods.push(self.parse_function(
                    package_id.clone(),
                    member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    namespace.clone(),
                    member_attributes,
                    member_modifiers.is_async,
                    member_modifiers.is_extern,
                    member_modifiers.is_static,
                )?);
                continue;
            }
            let ty = self
                .parse_type_syntax()?
                .ok_or_else(|| self.error_here("expected member type"))?;
            let name = self.expect_ident()?;
            let generic_params = self.parse_generic_params()?;
            if self.match_kind(&TokenKind::LParen) {
                let params = self.parse_params()?;
                self.expect(TokenKind::RParen)?;
                let generic_params = self.parse_generic_constraints(generic_params)?;
                let body = if self.match_kind(&TokenKind::Arrow) {
                    let expr = self.parse_expr()?;
                    self.expect(TokenKind::Semi)?;
                    vec![Stmt::Return(Some(expr))]
                } else if self.match_kind(&TokenKind::Semi) {
                    Vec::new()
                } else {
                    self.parse_stmt_body()?
                };
                methods.push(Function {
                    package_id: package_id.clone(),
                    visibility: member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    namespace: namespace.clone(),
                    attributes: member_attributes,
                    is_async: member_modifiers.is_async,
                    is_extern: member_modifiers.is_extern,
                    is_static: member_modifiers.is_static,
                    is_extension: false,
                    name,
                    generic_params,
                    params,
                    return_type: ty,
                    body,
                });
            } else if self.match_kind(&TokenKind::Arrow) {
                let expr = self.parse_expr()?;
                self.expect(TokenKind::Semi)?;
                methods.push(Function {
                    package_id: package_id.clone(),
                    visibility: member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    namespace: namespace.clone(),
                    attributes: member_attributes,
                    is_async: false,
                    is_extern: false,
                    is_static: false,
                    is_extension: false,
                    name: property_getter_name(&name),
                    generic_params: Vec::new(),
                    params: Vec::new(),
                    return_type: ty,
                    body: vec![Stmt::Return(Some(expr))],
                });
            } else if self.at(&TokenKind::LBrace) {
                self.parse_auto_property_body()?;
                let mut initializer = None;
                if self.match_kind(&TokenKind::Eq) {
                    initializer = Some(self.parse_expr()?);
                    self.expect(TokenKind::Semi)?;
                }
                fields.push(FieldDef {
                    visibility: member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    name,
                    ty,
                    is_static: member_modifiers.is_static,
                    initializer,
                });
            } else {
                let mut initializer = None;
                if self.match_kind(&TokenKind::Eq) {
                    initializer = Some(self.parse_expr()?);
                }
                self.expect(TokenKind::Semi)?;
                fields.push(FieldDef {
                    visibility: member_modifiers.visibility.unwrap_or_else(|| {
                        default_visibility_for_package(package_id.as_ref())
                    }),
                    name,
                    ty,
                    is_static: member_modifiers.is_static,
                    initializer,
                });
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(TypeDef {
            package_id,
            visibility,
            kind,
            is_extension: false,
            namespace,
            attributes,
            name,
            generic_params,
            bases,
            fields,
            constructors,
            methods,
        })
    }

    pub(super) fn parse_extension_def(
        &mut self,
        package_id: Option<String>,
        current_namespace: Vec<String>,
        attributes: Vec<Attribute>,
    ) -> Result<TypeDef, String> {
        self.expect_ident()?;
        let target_parts = {
            let raw_parts = self.parse_qualified_name()?;
            self.resolve_type_path(raw_parts)
        };
        if target_parts.is_empty() {
            return Err(self.error_here("expected extension target type"));
        }
        let (namespace, name) = if target_parts.len() == 1 {
            (current_namespace, target_parts[0].clone())
        } else {
            (
                target_parts[..target_parts.len() - 1].to_vec(),
                target_parts[target_parts.len() - 1].clone(),
            )
        };
        self.expect(TokenKind::LBrace)?;
        let mut methods = Vec::new();
        let qualified_target = if namespace.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", namespace.join("."), name)
        };
        while !self.at(&TokenKind::RBrace) {
            if self.at(&TokenKind::Hash) {
                self.skip_preprocessor_directive();
                continue;
            }
            let member_attributes = self.parse_attributes()?;
            let member_modifiers = self.parse_modifiers();
            let mut method = self.parse_function(
                package_id.clone(),
                member_modifiers.visibility.unwrap_or_else(|| {
                    default_visibility_for_package(package_id.as_ref())
                }),
                namespace.clone(),
                member_attributes,
                member_modifiers.is_async,
                member_modifiers.is_extern,
                member_modifiers.is_static,
            )?;
            let Some(first_param) = method.params.first() else {
                return Err(self.error_here(
                    "extension methods must declare an explicit `this` receiver parameter",
                ));
            };
            if first_param.modifier != ParamModifier::This {
                return Err(self.error_here(
                    "extension methods must declare an explicit `this` receiver parameter",
                ));
            }
            let receiver_name = type_syntax_name(&first_param.ty);
            if receiver_name != name && receiver_name != qualified_target {
                return Err(self.error_here(&format!(
                    "extension receiver type '{}' does not match target '{}'",
                    receiver_name, qualified_target
                )));
            }
            method.is_extension = true;
            methods.push(method);
        }
        self.expect(TokenKind::RBrace)?;
        let extension_visibility = default_visibility_for_package(package_id.as_ref());
        Ok(TypeDef {
            package_id,
            visibility: extension_visibility,
            kind: TypeKind::Class,
            is_extension: true,
            namespace,
            attributes,
            name,
            generic_params: Vec::new(),
            bases: Vec::new(),
            fields: Vec::new(),
            constructors: Vec::new(),
            methods,
        })
    }

    pub(super) fn parse_auto_property_body(&mut self) -> Result<(), String> {
        self.expect(TokenKind::LBrace)?;
        while !self.at(&TokenKind::RBrace) {
            self.parse_modifiers();
            self.expect_ident()?;
            if self.match_kind(&TokenKind::Semi) {
                // simple get; or set;
            } else if self.match_kind(&TokenKind::LBrace) {
                // getter/setter body: get { ... }
                self.parse_block_after_lbrace()?;
            } else if self.match_kind(&TokenKind::Arrow) {
                // arrow body: get => expr;
                self.parse_expr()?;
                self.expect(TokenKind::Semi)?;
            } else {
                return Err(
                    self.error_here("expected semicolon, arrow, or body for property accessor")
                );
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(())
    }

    pub(super) fn parse_function(
        &mut self,
        package_id: Option<String>,
        visibility: Visibility,
        namespace: Vec<String>,
        attributes: Vec<Attribute>,
        is_async: bool,
        is_extern: bool,
        is_static: bool,
    ) -> Result<Function, String> {
        let (return_type, name) = if self.match_kind(&TokenKind::Fn) {
            let checkpoint = self.pos;
            if let Some(return_type) = self.parse_type_syntax()? {
                if let TokenKind::Ident(name) = self.current().kind.clone() {
                    self.advance();
                    if self.at(&TokenKind::LParen) {
                        (return_type, name)
                    } else {
                        self.pos = checkpoint;
                        (TypeSyntax::Void, self.expect_ident()?)
                    }
                } else {
                    self.pos = checkpoint;
                    (TypeSyntax::Void, self.expect_ident()?)
                }
            } else {
                (TypeSyntax::Void, self.expect_ident()?)
            }
        } else {
            let return_type = self
                .parse_type_syntax()?
                .ok_or_else(|| self.error_here("expected function return type"))?;
            let name = self.expect_ident()?;
            (return_type, name)
        };
        let generic_params = self.parse_generic_params()?;
        self.expect(TokenKind::LParen)?;
        let params = self.parse_params()?;
        self.expect(TokenKind::RParen)?;
        let generic_params = self.parse_generic_constraints(generic_params)?;
        let body = if self.match_kind(&TokenKind::Arrow) {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            vec![Stmt::Return(Some(expr))]
        } else if is_extern && self.match_kind(&TokenKind::Semi) {
            Vec::new()
        } else {
            self.parse_stmt_body()?
        };
        Ok(Function {
            package_id,
            visibility,
            namespace,
            attributes,
            is_async,
            is_extern,
            is_static,
            is_extension: false,
            name,
            generic_params,
            params,
            return_type,
            body,
        })
    }

    pub(super) fn parse_delegate_decl(
        &mut self,
        package_id: Option<String>,
        visibility: Visibility,
        namespace: Vec<String>,
        attributes: Vec<Attribute>,
    ) -> Result<DelegateDef, String> {
        let return_type = self
            .parse_type_syntax()?
            .ok_or_else(|| self.error_here("expected delegate return type"))?;
        let name = self.expect_ident()?;
        let parsed_generic_params = self.parse_generic_params()?;
        let generic_params = self.parse_generic_constraints(parsed_generic_params)?;
        self.expect(TokenKind::LParen)?;
        let params = self.parse_params()?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Semi)?;
        Ok(DelegateDef {
            package_id,
            visibility,
            namespace,
            attributes,
            name,
            generic_params,
            return_type,
            params,
        })
    }

    pub(super) fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();
        if self.at(&TokenKind::RParen) {
            return Ok(params);
        }
        loop {
            let attributes = self.parse_attributes()?;
            let mut modifier = ParamModifier::None;
            let mut is_params = false;
            loop {
                if self.match_kind(&TokenKind::Ref) {
                    modifier = ParamModifier::Ref;
                } else if self.match_kind(&TokenKind::In) {
                    modifier = ParamModifier::In;
                } else if self.current_ident_is("out") {
                    self.advance();
                    modifier = ParamModifier::Out;
                } else if self.current_ident_is("params") {
                    self.advance();
                    is_params = true;
                } else if self.current_ident_is("this") {
                    self.advance();
                    modifier = ParamModifier::This;
                } else {
                    break;
                }
            }
            let ty = self
                .parse_type_syntax()?
                .ok_or_else(|| self.error_here("expected parameter type"))?;
            let name = self.expect_ident()?;
            let default = if self.match_kind(&TokenKind::Eq) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            params.push(Param {
                attributes,
                name,
                ty,
                modifier,
                is_params,
                default,
            });
            if !self.match_kind(&TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    pub(super) fn parse_generic_params(&mut self) -> Result<Vec<GenericParam>, String> {
        let mut generic_params = Vec::new();
        if !self.match_kind(&TokenKind::Less) {
            return Ok(generic_params);
        }
        loop {
            let name = self.expect_ident()?;
            generic_params.push(GenericParam {
                name,
                constraints: Vec::new(),
            });
            if self.match_kind(&TokenKind::Greater) {
                break;
            }
            self.expect(TokenKind::Comma)?;
        }
        Ok(generic_params)
    }

    pub(super) fn parse_generic_constraints(
        &mut self,
        mut generic_params: Vec<GenericParam>,
    ) -> Result<Vec<GenericParam>, String> {
        while self.current_ident_is("where") {
            self.advance();
            let name = self.expect_ident()?;
            self.expect(TokenKind::Colon)?;
            let mut constraints = Vec::new();
            loop {
                let constraint = if self.current_ident_is("class")
                    || self.current_ident_is("struct")
                    || self.current_ident_is("notnull")
                    || self.current_ident_is("unmanaged")
                {
                    self.expect_ident()?
                } else if self.at(&TokenKind::Class) {
                    self.advance();
                    "class".to_string()
                } else if self.at(&TokenKind::Struct) {
                    self.advance();
                    "struct".to_string()
                } else if self.at(&TokenKind::New) {
                    self.advance();
                    self.expect(TokenKind::LParen)?;
                    self.expect(TokenKind::RParen)?;
                    "new()".to_string()
                } else {
                    self.parse_qualified_name()?.join(".")
                };
                constraints.push(constraint);
                if !self.match_kind(&TokenKind::Comma) {
                    break;
                }
            }
            if let Some(param) = generic_params.iter_mut().find(|param| param.name == name) {
                param.constraints = constraints;
            }
        }
        Ok(generic_params)
    }
}
