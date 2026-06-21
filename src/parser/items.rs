use super::*;

impl Parser {
    pub(crate) fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program {
            package_id: None,
            native_c: Vec::new(),
            enums: Vec::new(),
            delegates: Vec::new(),
            types: Vec::new(),
            functions: Vec::new(),
        };
        let mut delegates = Vec::new();
        self.parse_items(&mut program, Vec::new(), false, &mut delegates)?;
        program.delegates = delegates;
        program.types = merge_type_declarations(program.types);
        synthesize_generated_regex_methods(&mut program);
        synthesize_xunit_fact_tests(self, &program);

        if !self.test_registrations.is_empty() {
            let main_fn = program.functions.iter_mut().find(|f| f.name == "main");
            if let Some(main_fn) = main_fn {
                let mut new_body = self.test_registrations.clone();
                new_body.extend(main_fn.body.clone());
                main_fn.body = new_body;
            } else {
                program.functions.push(Function {
                    package_id: None,
                    visibility: Visibility::Internal,
                    namespace: Vec::new(),
                    attributes: Vec::new(),
                    is_async: false,
                    is_extern: false,
                    is_static: false,
                    is_extension: false,
                    name: "main".to_string(),
                    generic_params: Vec::new(),
                    params: vec![Param {
                        attributes: Vec::new(),
                        name: "args".to_string(),
                        ty: TypeSyntax::Array(Box::new(TypeSyntax::String)),
                        modifier: ParamModifier::None,
                        is_params: false,
                        default: None,
                    }],
                    return_type: TypeSyntax::Void,
                    body: {
                        let mut body = self.test_registrations.clone();
                        body.push(Stmt::Expr(Expr::FunctionCall {
                            name: "XUnit_RunAllTests".to_string(),
                            generic_args: Vec::new(),
                            args: Vec::new(),
                        }));
                        body
                    },
                });
            }
        }

        Ok(program)
    }

    pub(super) fn parse_items(
        &mut self,
        program: &mut Program,
        base_namespace: Vec<String>,
        stop_at_rbrace: bool,
        delegates: &mut Vec<DelegateDef>,
    ) -> Result<(), String> {
        let mut current_namespace = base_namespace.clone();
        let mut current_package = program.package_id.clone();
        let mut top_level_stmts = Vec::new();
        while !self.at(&TokenKind::Eof) && !(stop_at_rbrace && self.at(&TokenKind::RBrace)) {
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
            let attributes = self.parse_attributes()?;
            let modifiers = self.parse_modifiers();
            if self.current_ident_is("__FILE_BOUNDARY__") {
                self.advance();
                self.expect(TokenKind::Semi)?;
                current_namespace = base_namespace.clone();
                current_package = None;
                self.using_aliases.clear();
                continue;
            }
            if self.at(&TokenKind::Hash) {
                self.skip_preprocessor_directive();
                continue;
            }
            if self.match_kind(&TokenKind::Semi) {
                continue;
            }
            if self.match_kind(&TokenKind::Package) {
                current_package = Some(self.parse_qualified_name()?.join("."));
                program.package_id = current_package.clone();
                self.expect(TokenKind::Semi)?;
            } else if self.match_kind(&TokenKind::Native) {
                let TokenKind::String(source) = self.current().kind.clone() else {
                    return Err(self.error_here("expected native C string"));
                };
                self.advance();
                self.match_kind(&TokenKind::Semi);
                program.native_c.push(source);
            } else if self.current_ident_is("macro") {
                self.parse_macro_def()?;
            } else if self.at(&TokenKind::Using)
                && !self.peek_is(1, &TokenKind::LParen)
                && !self.peek_is(1, &TokenKind::Var)
            {
                self.expect(TokenKind::Using)?;
                self.parse_using_directive()?;
            } else if self.current_ident_is("global") && self.peek_is(1, &TokenKind::Using) {
                self.advance();
                self.expect(TokenKind::Using)?;
                self.parse_using_directive()?;
            } else if self.match_kind(&TokenKind::Namespace) {
                let name = self.parse_qualified_name()?;
                if self.match_kind(&TokenKind::Semi) {
                    current_namespace = base_namespace.clone();
                    current_namespace.extend(name);
                } else {
                    let mut nested = current_namespace.clone();
                    nested.extend(name);
                    self.expect(TokenKind::LBrace)?;
                    self.parse_items(program, nested, true, delegates)?;
                    self.expect(TokenKind::RBrace)?;
                }
            } else if self.current_ident_is("type") {
                self.parse_type_alias()?; 
            } else if self.current_ident_is("extension") {
                program.types.push(self.parse_extension_def(
                    current_package.clone(),
                    current_namespace.clone(),
                    attributes,
                )?);
            } else if self.at(&TokenKind::Ref)
                || self.at(&TokenKind::Struct)
                || self.at(&TokenKind::Class)
                || self.at(&TokenKind::Record)
                || self.at(&TokenKind::Interface)
            {
                let mut nested = Vec::new();
                let type_def = self.parse_type_def(
                    current_package.clone(),
                    modifiers
                        .visibility
                        .unwrap_or_else(|| default_visibility_for_package(current_package.as_ref())),
                    current_namespace.clone(),
                    attributes,
                    &mut nested,
                    &mut program.native_c,
                    delegates,
                )?;
                program.types.push(type_def);
                program.types.extend(nested);
            } else if self.at(&TokenKind::Enum) {
                program
                    .enums
                    .push(self.parse_enum_def(
                        current_package.clone(),
                        modifiers
                            .visibility
                            .unwrap_or_else(|| default_visibility_for_package(current_package.as_ref())),
                        current_namespace.clone(),
                        attributes,
                    )?);
            } else if self.match_kind(&TokenKind::Delegate) {
                delegates.push(self.parse_delegate_decl(
                    current_package.clone(),
                    modifiers
                        .visibility
                        .unwrap_or_else(|| default_visibility_for_package(current_package.as_ref())),
                    current_namespace.clone(),
                    attributes,
                )?);
            } else {
                let checkpoint = self.pos;
                match self.parse_function(
                    current_package.clone(),
                    modifiers
                        .visibility
                        .unwrap_or_else(|| default_visibility_for_package(current_package.as_ref())),
                    current_namespace.clone(),
                    attributes.clone(),
                    modifiers.is_async,
                    modifiers.is_extern,
                    modifiers.is_static,
                ) {
                    Ok(function) => program.functions.push(function),
                    Err(_) => {
                        self.pos = checkpoint;
                        top_level_stmts.push(self.parse_stmt()?);
                    }
                }
            }
        }
        if !top_level_stmts.is_empty() {
            program.functions.push(Function {
                package_id: None,
                visibility: Visibility::Internal,
                namespace: base_namespace,
                attributes: Vec::new(),
                is_async: false,
                is_extern: false,
                is_static: false,
                is_extension: false,
                name: "main".to_string(),
                generic_params: Vec::new(),
                params: vec![Param {
                    attributes: Vec::new(),
                    name: "args".to_string(),
                    ty: TypeSyntax::Array(Box::new(TypeSyntax::String)),
                    modifier: ParamModifier::None,
                    is_params: false,
                    default: None,
                }],
                return_type: TypeSyntax::Void,
                body: top_level_stmts,
            });
        }
        Ok(())
    }

    pub(super) fn parse_using_directive(&mut self) -> Result<(), String> {
        if self.match_kind(&TokenKind::Static) {
            self.parse_qualified_name()?;
            self.expect(TokenKind::Semi)?;
            return Ok(());
        }
        let first = self.expect_ident()?;
        if self.match_kind(&TokenKind::Eq) {
            let target = self.parse_qualified_name()?;
            self.using_aliases.insert(first, target);
        } else {
            while self.match_kind(&TokenKind::Dot) {
                self.expect_ident()?;
            }
        }
        self.expect(TokenKind::Semi)
    }

    pub(super) fn parse_attributes(&mut self) -> Result<Vec<Attribute>, String> {
        let mut attributes = Vec::new();
        while self.match_kind(&TokenKind::LBracket) {
            loop {
                let raw_name = self.parse_qualified_name()?;
                let name = self.resolve_type_path(raw_name).join(".");
                let args = if self.match_kind(&TokenKind::LParen) {
                    self.parse_attribute_args_after_lparen()?
                } else {
                    Vec::new()
                };
                attributes.push(Attribute { name, args });
                if self.match_kind(&TokenKind::RBracket) {
                    break;
                }
                self.expect(TokenKind::Comma)?;
            }
        }
        Ok(attributes)
    }

    pub(super) fn parse_attribute_args_after_lparen(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        if self.match_kind(&TokenKind::RParen) {
            return Ok(args);
        }
        loop {
            if let TokenKind::Ident(name) = self.current().kind.clone() {
                if self.peek_is(1, &TokenKind::Eq) {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    args.push(Expr::NamedArg {
                        name,
                        expr: Box::new(self.parse_expr()?),
                    });
                } else {
                    args.push(self.parse_expr()?);
                }
            } else {
                args.push(self.parse_expr()?);
            }
            if self.match_kind(&TokenKind::RParen) {
                break;
            }
            self.expect(TokenKind::Comma)?;
        }
        Ok(args)
    }

    pub(super) fn parse_modifiers(&mut self) -> Modifiers {
        let mut modifiers = Modifiers::default();
        loop {
            if self.match_kind(&TokenKind::Async) {
                modifiers.is_async = true;
            } else if self.match_kind(&TokenKind::Internal) {
                modifiers.visibility = Some(Visibility::Internal);
            } else if self.match_kind(&TokenKind::Public)
                && {
                    modifiers.visibility = Some(Visibility::Public);
                    true
                }
                || {
                    if self.match_kind(&TokenKind::Static) {
                        modifiers.is_static = true;
                        true
                    } else {
                        false
                    }
                }
                || self.match_kind(&TokenKind::Const)
                || self.match_kind(&TokenKind::Readonly)
            {
            } else {
                let TokenKind::Ident(ref name) = self.current().kind else {
                    break;
                };
                if matches!(name.as_str(), "private" | "protected" | "internal" | "protected internal" | "private protected") {
                    modifiers.visibility = Some(Visibility::Internal);
                    self.advance();
                } else if name == "extern" {
                    modifiers.is_extern = true;
                    self.advance();
                } else if name == "static" {
                    modifiers.is_static = true;
                    self.advance();
                } else if matches!(
                    name.as_str(),
                    "partial" | "sealed" | "unsafe" | "required" | "volatile" | "async"
                ) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        modifiers
    }
}
