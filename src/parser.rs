use std::collections::HashMap;

use crate::ast::*;
use crate::lexer::{Token, TokenKind};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct MacroDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Vec<Token>,
}

pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    using_aliases: HashMap<String, Vec<String>>,
    macros: HashMap<String, MacroDef>,
    test_registrations: Vec<Stmt>,
}

#[derive(Default, Clone, Copy)]
struct Modifiers {
    is_async: bool,
    is_extern: bool,
    is_static: bool,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            using_aliases: HashMap::new(),
            macros: HashMap::new(),
            test_registrations: Vec::new(),
        }
    }

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
                            args: Vec::new(),
                        }));
                        body
                    },
                });
            }
        }

        Ok(program)
    }

    fn parse_items(
        &mut self,
        program: &mut Program,
        base_namespace: Vec<String>,
        stop_at_rbrace: bool,
        delegates: &mut Vec<DelegateDef>,
    ) -> Result<(), String> {
        let mut current_namespace = base_namespace.clone();
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
                program.package_id = Some(self.parse_qualified_name()?.join("."));
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
                    .push(self.parse_enum_def(current_namespace.clone(), attributes)?);
            } else if self.match_kind(&TokenKind::Delegate) {
                delegates.push(self.parse_delegate_decl(current_namespace.clone(), attributes)?);
            } else {
                let checkpoint = self.pos;
                match self.parse_function(
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

    fn parse_using_directive(&mut self) -> Result<(), String> {
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

    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, String> {
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

    fn parse_attribute_args_after_lparen(&mut self) -> Result<Vec<Expr>, String> {
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

    fn parse_modifiers(&mut self) -> Modifiers {
        let mut modifiers = Modifiers::default();
        loop {
            if self.match_kind(&TokenKind::Async) {
                modifiers.is_async = true;
            } else if self.match_kind(&TokenKind::Public)
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

    fn parse_enum_def(
        &mut self,
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
            namespace,
            attributes,
            name,
            variants,
        })
    }

    fn parse_enum_value(&mut self) -> Result<i64, String> {
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

    fn parse_type_def(
        &mut self,
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
                    name: param.name.clone(),
                    ty: param.ty.clone(),
                    is_static: false,
                    initializer: None,
                });
            }
            constructors.push(Constructor {
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
                let _nested = self.parse_enum_def(namespace.clone(), member_attributes)?;
                continue;
            }
            if self.match_kind(&TokenKind::Delegate) {
                delegates.push(self.parse_delegate_decl(namespace.clone(), member_attributes)?);
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
                    namespace: namespace.clone(),
                    attributes: member_attributes,
                    params,
                    body,
                });
                continue;
            }
            if self.at(&TokenKind::Fn) {
                methods.push(self.parse_function(
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
                    name,
                    ty,
                    is_static: member_modifiers.is_static,
                    initializer,
                });
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(TypeDef {
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

    fn parse_extension_def(
        &mut self,
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
        Ok(TypeDef {
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

    fn parse_auto_property_body(&mut self) -> Result<(), String> {
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

    fn parse_function(
        &mut self,
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

    fn parse_delegate_decl(
        &mut self,
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
            namespace,
            attributes,
            name,
            generic_params,
            return_type,
            params,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
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

    fn parse_generic_params(&mut self) -> Result<Vec<GenericParam>, String> {
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

    fn parse_generic_constraints(
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

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        if self.match_kind(&TokenKind::LBrace) {
            return Ok(Stmt::Block(self.parse_block_after_lbrace()?));
        }
        if self.match_kind(&TokenKind::If) {
            self.expect(TokenKind::LParen)?;
            let condition = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            let then_body = self.parse_stmt_or_block_body()?;
            let else_body = if self.match_kind(&TokenKind::Else) {
                self.parse_stmt_or_block_body()?
            } else {
                Vec::new()
            };
            return Ok(Stmt::If {
                condition,
                then_body,
                else_body,
            });
        }
        if self.match_kind(&TokenKind::Try) {
            let try_body = self.parse_stmt_body()?;
            let mut catch: Option<CatchClause> = None;
            let mut finally_body = Vec::new();
            while self.match_kind(&TokenKind::Catch) {
                let mut exception_type = None;
                let mut name = None;
                if self.match_kind(&TokenKind::LParen) {
                    if !self.at(&TokenKind::RParen) {
                        exception_type = Some(
                            self.parse_type_syntax()?
                                .ok_or_else(|| self.error_here("expected catch exception type"))?,
                        );
                        if matches!(self.current().kind, TokenKind::Ident(_)) {
                            name = Some(self.expect_ident()?);
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                }
                let body = self.parse_stmt_body()?;
                if let Some(existing) = &mut catch {
                    existing.body.extend(body);
                } else {
                    catch = Some(CatchClause {
                        exception_type,
                        name,
                        body,
                    });
                }
            }
            if self.match_kind(&TokenKind::Finally) {
                finally_body = self.parse_stmt_body()?;
            }
            return Ok(Stmt::Try {
                try_body,
                catch,
                finally_body,
            });
        }
        if self.match_kind(&TokenKind::Switch) {
            self.expect(TokenKind::LParen)?;
            let expr = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            self.expect(TokenKind::LBrace)?;
            let mut cases = Vec::new();
            let mut default = Vec::new();
            while !self.at(&TokenKind::RBrace) {
                if self.match_kind(&TokenKind::Case) {
                    let case_expr = expr.clone();
                    let checkpoint = self.pos;
                    let value = if let Some(ty) = self.parse_type_syntax()? {
                        if let TokenKind::Ident(name) = self.current().kind.clone() {
                            if self.peek_is(1, &TokenKind::Colon) {
                                self.advance();
                                self.expect(TokenKind::Colon)?;
                                Expr::IsPattern {
                                    expr: Box::new(case_expr),
                                    ty,
                                    name: Some(name),
                                }
                            } else {
                                self.pos = checkpoint;
                                let value = self.parse_expr()?;
                                self.expect(TokenKind::Colon)?;
                                value
                            }
                        } else {
                            self.pos = checkpoint;
                            let value = self.parse_expr()?;
                            self.expect(TokenKind::Colon)?;
                            value
                        }
                    } else {
                        let value = self.parse_expr()?;
                        self.expect(TokenKind::Colon)?;
                        value
                    };
                    let body = self.parse_switch_section_body()?;
                    cases.push(SwitchCase { value, body });
                } else if self.match_kind(&TokenKind::Default) {
                    self.expect(TokenKind::Colon)?;
                    default = self.parse_switch_section_body()?;
                } else {
                    return Err(self.error_here("expected case or default"));
                }
            }
            self.expect(TokenKind::RBrace)?;
            return Ok(Stmt::Switch {
                expr,
                cases,
                default,
            });
        }
        if self.match_kind(&TokenKind::While) {
            self.expect(TokenKind::LParen)?;
            let condition = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            let body = self.parse_stmt_or_block_body()?;
            return Ok(Stmt::While { condition, body });
        }
        if self.match_kind(&TokenKind::For) {
            self.expect(TokenKind::LParen)?;
            let init = if self.match_kind(&TokenKind::Semi) {
                None
            } else {
                let stmt = self.parse_for_component()?;
                self.expect(TokenKind::Semi)?;
                Some(Box::new(stmt))
            };
            let condition = if self.match_kind(&TokenKind::Semi) {
                None
            } else {
                let expr = self.parse_expr()?;
                self.expect(TokenKind::Semi)?;
                Some(expr)
            };
            let increment = if self.at(&TokenKind::RParen) {
                None
            } else {
                Some(Box::new(self.parse_for_component()?))
            };
            self.expect(TokenKind::RParen)?;
            let body = self.parse_stmt_or_block_body()?;
            return Ok(Stmt::For {
                init,
                condition,
                increment,
                body,
            });
        }
        if self.match_kind(&TokenKind::Foreach) {
            self.expect(TokenKind::LParen)?;
            let item_type = if self.match_kind(&TokenKind::Var) {
                TypeSyntax::Named("var".to_string())
            } else {
                self.parse_type_syntax()?
                    .ok_or_else(|| self.error_here("expected foreach item type"))?
            };
            let item_name = self.expect_ident()?;
            self.expect(TokenKind::In)?;
            let collection = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            let body = self.parse_stmt_or_block_body()?;
            return Ok(Stmt::ForEach {
                item_type,
                item_name,
                collection,
                body,
            });
        }
        if self.match_kind(&TokenKind::Using) {
            if self.match_kind(&TokenKind::LParen) {
                let init = if self.match_kind(&TokenKind::Var) {
                    let name = self.expect_ident()?;
                    self.expect(TokenKind::Eq)?;
                    let expr = self.parse_expr()?;
                    Stmt::Let {
                        name,
                        mutable: true,
                        declared_type: None,
                        expr,
                    }
                } else if let Some((declared_type, name)) = self.parse_typed_decl_start()? {
                    self.expect(TokenKind::Eq)?;
                    let expr = self.parse_expr()?;
                    Stmt::Let {
                        name,
                        mutable: true,
                        declared_type: Some(declared_type),
                        expr,
                    }
                } else {
                    return Err(self.error_here("expected using declaration"));
                };
                self.expect(TokenKind::RParen)?;
                let mut body = vec![init];
                body.push(Stmt::Block(self.parse_stmt_or_block_body()?));
                return Ok(Stmt::Block(body));
            }
            if self.match_kind(&TokenKind::Var) {
                let name = self.expect_ident()?;
                self.expect(TokenKind::Eq)?;
                let expr = self.parse_expr()?;
                self.expect(TokenKind::Semi)?;
                return Ok(Stmt::Let {
                    name,
                    mutable: true,
                    declared_type: None,
                    expr,
                });
            }
            if let Some((declared_type, name)) = self.parse_typed_decl_start()? {
                self.expect(TokenKind::Eq)?;
                let expr = self.parse_expr()?;
                self.expect(TokenKind::Semi)?;
                return Ok(Stmt::Let {
                    name,
                    mutable: true,
                    declared_type: Some(declared_type),
                    expr,
                });
            }
            return Err(self.error_here("expected using local declaration"));
        }
        if self.match_kind(&TokenKind::Let) {
            let mutable = self.match_kind(&TokenKind::Mut);
            let name = self.expect_ident()?;
            self.expect(TokenKind::Eq)?;
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            return Ok(Stmt::Let {
                name,
                mutable,
                declared_type: None,
                expr,
            });
        }
        if self.match_kind(&TokenKind::Var) {
            let name = self.expect_ident()?;
            self.expect(TokenKind::Eq)?;
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            return Ok(Stmt::Let {
                name,
                mutable: true,
                declared_type: None,
                expr,
            });
        }
        self.parse_modifiers();
        if let Some(stmt) = self.parse_local_function_stmt()? {
            return Ok(stmt);
        }
        if let Some((declared_type, name)) = self.parse_typed_decl_no_initializer()? {
            self.expect(TokenKind::Semi)?;
            let expr = default_expr_for_type(&declared_type);
            return Ok(Stmt::Let {
                name,
                mutable: true,
                declared_type: Some(declared_type),
                expr,
            });
        }
        if let Some((declared_type, name)) = self.parse_typed_decl_start()? {
            self.expect(TokenKind::Eq)?;
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            return Ok(Stmt::Let {
                name,
                mutable: true,
                declared_type: Some(declared_type),
                expr,
            });
        }
        if matches!(self.current().kind, TokenKind::Ident(_)) {
            if let TokenKind::Ident(name) = self.current().kind.clone() {
                if self.peek_is(1, &TokenKind::PlusPlus) {
                    self.advance();
                    self.advance();
                    self.expect(TokenKind::Semi)?;
                    return Ok(Stmt::Assign {
                        name: name.clone(),
                        expr: Expr::Binary {
                            left: Box::new(Expr::Var(name)),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Int(1)),
                        },
                    });
                }
                if self.peek_is(1, &TokenKind::MinusMinus) {
                    self.advance();
                    self.advance();
                    self.expect(TokenKind::Semi)?;
                    return Ok(Stmt::Assign {
                        name: name.clone(),
                        expr: Expr::Binary {
                            left: Box::new(Expr::Var(name)),
                            op: BinaryOp::Sub,
                            right: Box::new(Expr::Int(1)),
                        },
                    });
                }
            }
            if let Some(assign) = self.parse_assignment_stmt()? {
                return Ok(assign);
            }
        }
        if self.match_kind(&TokenKind::Print) {
            self.expect(TokenKind::LParen)?;
            let expr = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            self.expect(TokenKind::Semi)?;
            return Ok(Stmt::Print(expr));
        }
        if self.match_kind(&TokenKind::Return) {
            if self.match_kind(&TokenKind::Semi) {
                return Ok(Stmt::Return(None));
            }
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            return Ok(Stmt::Return(Some(expr)));
        }
        if self.match_kind(&TokenKind::Throw) {
            if self.match_kind(&TokenKind::Semi) {
                return Ok(Stmt::Throw(Expr::String("rethrow".to_string())));
            }
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            return Ok(Stmt::Throw(expr));
        }
        if self.match_kind(&TokenKind::Break) {
            self.expect(TokenKind::Semi)?;
            return Ok(Stmt::Break);
        }
        if self.match_kind(&TokenKind::Continue) {
            self.expect(TokenKind::Semi)?;
            return Ok(Stmt::Continue);
        }
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semi)?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_typed_decl_no_initializer(&mut self) -> Result<Option<(TypeSyntax, String)>, String> {
        let checkpoint = self.pos;
        let Some(ty) = self.parse_type_syntax()? else {
            return Ok(None);
        };
        let TokenKind::Ident(name) = self.current().kind.clone() else {
            self.pos = checkpoint;
            return Ok(None);
        };
        self.advance();
        if self.at(&TokenKind::Semi) {
            Ok(Some((ty, name)))
        } else {
            self.pos = checkpoint;
            Ok(None)
        }
    }

    fn parse_switch_section_body(&mut self) -> Result<Vec<Stmt>, String> {
        let mut body = Vec::new();
        while !self.at(&TokenKind::Case)
            && !self.at(&TokenKind::Default)
            && !self.at(&TokenKind::RBrace)
        {
            body.push(self.parse_stmt()?);
        }
        Ok(body)
    }

    fn parse_for_component(&mut self) -> Result<Stmt, String> {
        if let Some((declared_type, name)) = self.parse_typed_decl_start()? {
            self.expect(TokenKind::Eq)?;
            let expr = self.parse_expr()?;
            return Ok(Stmt::Let {
                name,
                mutable: true,
                declared_type: Some(declared_type),
                expr,
            });
        }
        if let TokenKind::Ident(name) = self.current().kind.clone() {
            if self.peek_is(1, &TokenKind::PlusPlus) {
                self.advance();
                self.advance();
                return Ok(Stmt::Assign {
                    name: name.clone(),
                    expr: Expr::Binary {
                        left: Box::new(Expr::Var(name)),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Int(1)),
                    },
                });
            }
            if self.peek_is(1, &TokenKind::MinusMinus) {
                self.advance();
                self.advance();
                return Ok(Stmt::Assign {
                    name: name.clone(),
                    expr: Expr::Binary {
                        left: Box::new(Expr::Var(name)),
                        op: BinaryOp::Sub,
                        right: Box::new(Expr::Int(1)),
                    },
                });
            }
            if self
                .tokens
                .get(self.pos + 1)
                .is_some_and(|t| t.kind == TokenKind::Eq)
            {
                self.advance();
                self.expect(TokenKind::Eq)?;
                let expr = self.parse_expr()?;
                return Ok(Stmt::Assign { name, expr });
            }
        }
        Ok(Stmt::Expr(self.parse_expr()?))
    }

    fn parse_local_function_stmt(&mut self) -> Result<Option<Stmt>, String> {
        let checkpoint = self.pos;
        let Some(return_type) = self.parse_type_syntax()? else {
            self.pos = checkpoint;
            return Ok(None);
        };
        let TokenKind::Ident(_) = self.current().kind.clone() else {
            self.pos = checkpoint;
            return Ok(None);
        };
        self.advance();
        let generic_params = self.parse_generic_params()?;
        if !self.match_kind(&TokenKind::LParen) {
            self.pos = checkpoint;
            return Ok(None);
        }
        let _params = self.parse_params()?;
        self.expect(TokenKind::RParen)?;
        let _generic_params = self.parse_generic_constraints(generic_params)?;
        let _body = self.parse_stmt_body()?;
        let _ = return_type;
        Ok(Some(Stmt::Expr(Expr::Null)))
    }

    fn parse_stmt_body(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(TokenKind::LBrace)?;
        self.parse_block_after_lbrace()
    }

    fn parse_stmt_or_block_body(&mut self) -> Result<Vec<Stmt>, String> {
        if self.match_kind(&TokenKind::LBrace) {
            self.parse_block_after_lbrace()
        } else {
            Ok(vec![self.parse_stmt()?])
        }
    }

    fn parse_block_after_lbrace(&mut self) -> Result<Vec<Stmt>, String> {
        let mut body = Vec::new();
        while !self.at(&TokenKind::RBrace) {
            body.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(body)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let checkpoint = self.pos;
        if self.match_kind(&TokenKind::LParen) {
            let mut params = Vec::new();
            let mut valid_params = true;
            if !self.at(&TokenKind::RParen) {
                loop {
                    if let TokenKind::Ident(name) = self.current().kind.clone() {
                        self.advance();
                        params.push(name);
                    } else {
                        valid_params = false;
                        break;
                    }
                    if !self.match_kind(&TokenKind::Comma) {
                        break;
                    }
                }
            }
            if valid_params
                && self.match_kind(&TokenKind::RParen)
                && self.match_kind(&TokenKind::Arrow)
            {
                let body = if self.at(&TokenKind::LBrace) {
                    let _stmts = self.parse_stmt_body()?;
                    Expr::Null
                } else {
                    self.parse_expr()?
                };
                return Ok(Expr::Lambda {
                    params,
                    body: Box::new(body),
                });
            }
            self.pos = checkpoint;
        }
        if let TokenKind::Ident(name) = self.current().kind.clone() {
            if self.peek_is(1, &TokenKind::Arrow) {
                self.advance();
                self.expect(TokenKind::Arrow)?;
                let body = if self.at(&TokenKind::LBrace) {
                    let _stmts = self.parse_stmt_body()?;
                    Expr::Null
                } else {
                    self.parse_expr()?
                };
                return Ok(Expr::Lambda {
                    params: vec![name],
                    body: Box::new(body),
                });
            }
        }
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Result<Expr, String> {
        let left = self.parse_conditional()?;
        if self.at(&TokenKind::Eq) && !self.peek_is(1, &TokenKind::Eq) {
            self.advance();
            let right = self.parse_assignment_expr()?;
            return Ok(Expr::Assign {
                target: Box::new(left),
                value: Box::new(right),
            });
        }
        Ok(left)
    }

    fn parse_conditional(&mut self) -> Result<Expr, String> {
        let condition = self.parse_or()?;
        if self.at(&TokenKind::Question) && !self.peek_is(1, &TokenKind::Question) {
            self.advance();
            let when_true = self.parse_expr()?;
            self.expect(TokenKind::Colon)?;
            let when_false = self.parse_expr()?;
            Ok(Expr::Conditional {
                condition: Box::new(condition),
                when_true: Box::new(when_true),
                when_false: Box::new(when_false),
            })
        } else {
            Ok(condition)
        }
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_and()?;
        while self.match_kind(&TokenKind::PipePipe) {
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        while self.match_kind(&TokenKind::Pipe) {
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::BitOr,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_bit_and()?;
        while self.match_kind(&TokenKind::AmpAmp) {
            let right = self.parse_bit_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_bit_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_coalesce()?;
        while self.match_kind(&TokenKind::Amp) {
            let right = self.parse_coalesce()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::BitAnd,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_coalesce(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;
        while self.at(&TokenKind::Question) && self.peek_is(1, &TokenKind::Question) {
            self.advance();
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Coalesce,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_additive()?;
        loop {
            let op = if self.match_kind(&TokenKind::EqEq) {
                BinaryOp::Eq
            } else if self.match_kind(&TokenKind::BangEq) {
                BinaryOp::NotEq
            } else if self.match_kind(&TokenKind::LessEq) {
                BinaryOp::LessEq
            } else if self.match_kind(&TokenKind::GreaterEq) {
                BinaryOp::GreaterEq
            } else if self.match_kind(&TokenKind::Less) {
                BinaryOp::Less
            } else if self.match_kind(&TokenKind::Greater) {
                BinaryOp::Greater
            } else {
                if self.current_ident_is("is") {
                    self.advance();
                    if self.current_ident_is("not") {
                        self.advance();
                        if self.at(&TokenKind::Null) {
                            self.advance();
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                op: BinaryOp::NotEq,
                                right: Box::new(Expr::Null),
                            };
                            continue;
                        }
                    }
                    if self.at(&TokenKind::Null) {
                        self.advance();
                        expr = Expr::Binary {
                            left: Box::new(expr),
                            op: BinaryOp::Eq,
                            right: Box::new(Expr::Null),
                        };
                        continue;
                    }
                    let ty = self
                        .parse_type_syntax()?
                        .ok_or_else(|| self.error_here("expected type after 'is'"))?;
                    let name = if matches!(self.current().kind, TokenKind::Ident(_)) {
                        Some(self.expect_ident()?)
                    } else {
                        None
                    };
                    expr = Expr::IsPattern {
                        expr: Box::new(expr),
                        ty,
                        name,
                    };
                    continue;
                }
                if self.current_ident_is("as") {
                    self.advance();
                    self.parse_type_syntax()?
                        .ok_or_else(|| self.error_here("expected type after 'as'"))?;
                    continue;
                }
                break;
            };
            let right = self.parse_additive()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplicative()?;
        loop {
            let op = if self.match_kind(&TokenKind::Plus) {
                BinaryOp::Add
            } else if self.match_kind(&TokenKind::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };
            let right = self.parse_multiplicative()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;
        loop {
            let op = if self.match_kind(&TokenKind::Star) {
                BinaryOp::Mul
            } else if self.match_kind(&TokenKind::Slash) {
                BinaryOp::Div
            } else if self.match_kind(&TokenKind::Percent) {
                BinaryOp::Mod
            } else {
                break;
            };
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.match_kind(&TokenKind::PlusPlus) || self.match_kind(&TokenKind::MinusMinus) {
            let delta = if self.previous_is(&TokenKind::PlusPlus) {
                1
            } else {
                -1
            };
            let name = self.expect_ident()?;
            return Ok(Expr::IncDec {
                name,
                delta,
                prefix: true,
            });
        }
        if self.match_kind(&TokenKind::Bang) {
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(self.parse_unary()?),
            });
        }
        if self.match_kind(&TokenKind::Minus) {
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(self.parse_unary()?),
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_kind(&TokenKind::LBracket) {
                if self.at(&TokenKind::Dot) && self.peek_is(1, &TokenKind::Dot) {
                    self.advance();
                    self.advance();
                    let end = self.parse_expr()?;
                    self.expect(TokenKind::RBracket)?;
                    expr = Expr::MethodCall {
                        target: Box::new(expr),
                        name: "Substring".to_string(),
                        args: vec![Expr::Int(0), end],
                    };
                } else {
                    let start = self.parse_expr()?;
                    if self.at(&TokenKind::Dot) && self.peek_is(1, &TokenKind::Dot) {
                        self.advance();
                        self.advance();
                        let end = if self.at(&TokenKind::RBracket) {
                            None
                        } else {
                            Some(self.parse_expr()?)
                        };
                        self.expect(TokenKind::RBracket)?;
                        let args = if let Some(end) = end {
                            vec![start, end]
                        } else {
                            vec![start]
                        };
                        expr = Expr::MethodCall {
                            target: Box::new(expr),
                            name: "Substring".to_string(),
                            args,
                        };
                    } else {
                        self.expect(TokenKind::RBracket)?;
                        expr = Expr::Index {
                            target: Box::new(expr),
                            index: Box::new(start),
                        };
                    }
                }
            } else if self.match_kind(&TokenKind::Dot) {
                if self.at(&TokenKind::Dot) {
                    self.pos -= 1;
                    break;
                }
                let name = self.expect_ident()?;
                let generic_checkpoint = self.pos;
                if self.at(&TokenKind::Less) {
                    if self.parse_generic_type_args(&name).is_err() || !self.at(&TokenKind::LParen)
                    {
                        self.pos = generic_checkpoint;
                    }
                }
                if self.match_kind(&TokenKind::LParen) {
                    let args = self.parse_call_args_after_lparen()?;
                    expr = Expr::MethodCall {
                        target: Box::new(expr),
                        name,
                        args,
                    };
                } else {
                    expr = Expr::Field {
                        target: Box::new(expr),
                        name,
                    };
                }
            } else if self.at(&TokenKind::Question) && self.peek_is(1, &TokenKind::Dot) {
                self.advance();
                self.advance();
                let base = expr.clone();
                let name = self.expect_ident()?;
                let generic_checkpoint = self.pos;
                let access = if self.at(&TokenKind::Less) {
                    if self.parse_generic_type_args(&name).is_err() || !self.at(&TokenKind::LParen)
                    {
                        self.pos = generic_checkpoint;
                        Expr::Field {
                            target: Box::new(base.clone()),
                            name,
                        }
                    } else {
                        self.expect(TokenKind::LParen)?;
                        let args = self.parse_call_args_after_lparen()?;
                        Expr::MethodCall {
                            target: Box::new(base.clone()),
                            name,
                            args,
                        }
                    }
                } else if self.match_kind(&TokenKind::LParen) {
                    let args = self.parse_call_args_after_lparen()?;
                    Expr::MethodCall {
                        target: Box::new(base.clone()),
                        name,
                        args,
                    }
                } else {
                    Expr::Field {
                        target: Box::new(base.clone()),
                        name,
                    }
                };
                expr = Expr::Conditional {
                    condition: Box::new(Expr::Binary {
                        left: Box::new(base),
                        op: BinaryOp::Eq,
                        right: Box::new(Expr::Null),
                    }),
                    when_true: Box::new(Expr::Null),
                    when_false: Box::new(access),
                };
            } else if self.at(&TokenKind::Question) {
                break;
            } else if self.match_kind(&TokenKind::Bang) {
                // C# null-forgiving operator is compile-time only.
            } else if self.match_kind(&TokenKind::PlusPlus)
                || self.match_kind(&TokenKind::MinusMinus)
            {
                let delta = if self.previous_is(&TokenKind::PlusPlus) {
                    1
                } else {
                    -1
                };
                let Expr::Var(name) = expr else {
                    return Err(self.error_here("++ and -- currently require a simple variable"));
                };
                expr = Expr::IncDec {
                    name,
                    delta,
                    prefix: false,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let token = self.current().clone();
        match token.kind {
            TokenKind::Int(value) => {
                self.advance();
                Ok(Expr::Int(value))
            }
            TokenKind::Float(value) => {
                self.advance();
                Ok(Expr::Float(value))
            }
            TokenKind::Bool(value) => {
                self.advance();
                Ok(Expr::Bool(value))
            }
            TokenKind::Null => {
                self.advance();
                Ok(Expr::Null)
            }
            TokenKind::Default => {
                self.advance();
                if self.match_kind(&TokenKind::LParen) {
                    self.parse_type_syntax()?
                        .ok_or_else(|| self.error_here("expected type after default("))?;
                    self.expect(TokenKind::RParen)?;
                }
                Ok(Expr::Null)
            }
            TokenKind::String(value) => {
                self.advance();
                if self.current_ident_is("u8") {
                    self.advance();
                    if self.match_kind(&TokenKind::Dot) {
                        let method = self.expect_ident()?;
                        if method == "ToArray" && self.at(&TokenKind::LParen) {
                            let _ = self.parse_opaque_parenthesized()?;
                        }
                    }
                    let values = value
                        .as_bytes()
                        .iter()
                        .map(|byte| Expr::Int(*byte as i64))
                        .collect();
                    return Ok(Expr::NewArray {
                        element_type: TypeSyntax::Scalar(ScalarType::Byte),
                        length: None,
                        values,
                    });
                }
                Ok(Expr::String(value))
            }
            TokenKind::Ident(name) => {
                self.advance();
                if name == "nameof" && self.at(&TokenKind::LParen) {
                    return Ok(Expr::String(self.parse_nameof_argument()?));
                }
                if name == "typeof" && self.at(&TokenKind::LParen) {
                    self.expect(TokenKind::LParen)?;
                    let ty = self
                        .parse_type_syntax()?
                        .ok_or_else(|| self.error_here("expected type after typeof("))?;
                    self.expect(TokenKind::RParen)?;
                    return Ok(type_object_expr(&ty));
                }
                let generic_checkpoint = self.pos;
                if self.at(&TokenKind::Less) {
                    if self.parse_generic_type_args(&name).is_err() || !self.at(&TokenKind::LParen)
                    {
                        self.pos = generic_checkpoint;
                    }
                }
                if self.match_kind(&TokenKind::LParen) {
                    Ok(Expr::FunctionCall {
                        name,
                        args: self.parse_call_args_after_lparen()?,
                    })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            TokenKind::Move => {
                self.advance();
                Ok(Expr::Move(self.expect_ident()?))
            }
            TokenKind::Borrow => {
                self.advance();
                let mutable = self.match_kind(&TokenKind::Mut);
                Ok(Expr::Borrow {
                    mutable,
                    name: self.expect_ident()?,
                })
            }
            TokenKind::Throw => {
                self.advance();
                Ok(Expr::Throw(Box::new(self.parse_expr()?)))
            }
            TokenKind::Await => {
                self.advance();
                Ok(Expr::Await(Box::new(self.parse_postfix()?)))
            }
            TokenKind::New => {
                self.advance();
                if self.at(&TokenKind::LBrace) {
                    return Ok(Expr::NewObject {
                        type_name: "__anonymous".to_string(),
                        args: Vec::new(),
                        fields: self.parse_object_initializer()?,
                    });
                }
                if self.at(&TokenKind::LParen) {
                    self.expect(TokenKind::LParen)?;
                    let args = self.parse_call_args_after_lparen()?;
                    let fields = if self.at(&TokenKind::LBrace) {
                        if self.peek_is(1, &TokenKind::LBrace)
                            || !matches!(
                                self.tokens.get(self.pos + 1).map(|t| &t.kind),
                                Some(TokenKind::Ident(_))
                            )
                        {
                            let _ = self.parse_opaque_brace_initializer()?;
                            Vec::new()
                        } else {
                            self.parse_object_initializer()?
                        }
                    } else {
                        Vec::new()
                    };
                    return Ok(Expr::NewObject {
                        type_name: "__target".to_string(),
                        args,
                        fields,
                    });
                }
                if self.at(&TokenKind::LBracket) && self.peek_is(1, &TokenKind::RBracket) {
                    self.expect(TokenKind::LBracket)?;
                    self.expect(TokenKind::RBracket)?;
                    let values = self.parse_array_initializer()?;
                    return Ok(Expr::NewArray {
                        element_type: TypeSyntax::Named("var".to_string()),
                        length: None,
                        values,
                    });
                }
                let ty = self
                    .parse_type_syntax()?
                    .ok_or_else(|| self.error_here("expected type after new"))?;
                match ty {
                    TypeSyntax::Scalar(scalar) => {
                        if self.at(&TokenKind::LBracket) {
                            self.expect(TokenKind::LBracket)?;
                            let length = if self.at(&TokenKind::RBracket) {
                                None
                            } else {
                                Some(Box::new(self.parse_expr()?))
                            };
                            self.expect(TokenKind::RBracket)?;
                            let values = if self.at(&TokenKind::LBrace) {
                                self.parse_array_initializer()?
                            } else {
                                Vec::new()
                            };
                            return Ok(Expr::NewArray {
                                element_type: TypeSyntax::Scalar(scalar),
                                length,
                                values,
                            });
                        }
                        if self.match_kind(&TokenKind::LParen) {
                            let _ = self.parse_call_args_after_lparen()?;
                        }
                        Ok(match scalar {
                            ScalarType::Bool => Expr::Bool(false),
                            ScalarType::F64 | ScalarType::Decimal => Expr::Float(0.0),
                            _ => Expr::Int(0),
                        })
                    }
                    TypeSyntax::String => {
                        if self.match_kind(&TokenKind::LParen) {
                            let _ = self.parse_call_args_after_lparen()?;
                        }
                        Ok(Expr::String(String::new()))
                    }
                    TypeSyntax::Array(element) => {
                        if self.at(&TokenKind::LBracket) {
                            self.expect(TokenKind::LBracket)?;
                            let length = if self.at(&TokenKind::RBracket) {
                                None
                            } else {
                                Some(Box::new(self.parse_expr()?))
                            };
                            self.expect(TokenKind::RBracket)?;
                            let values = if self.at(&TokenKind::LBrace) {
                                self.parse_array_initializer()?
                            } else {
                                Vec::new()
                            };
                            Ok(Expr::NewArray {
                                element_type: *element,
                                length,
                                values,
                            })
                        } else {
                            let values = self.parse_array_initializer()?;
                            Ok(Expr::NewArray {
                                element_type: *element,
                                length: None,
                                values,
                            })
                        }
                    }
                    TypeSyntax::List(_) | TypeSyntax::Dictionary(_, _) => {
                        if self.match_kind(&TokenKind::LParen) {
                            self.expect(TokenKind::RParen)?;
                        }
                        if self.at(&TokenKind::LBrace) {
                            let _ = self.parse_opaque_brace_initializer()?;
                        }
                        Ok(Expr::NewCollection(ty))
                    }
                    TypeSyntax::Thread => {
                        self.expect(TokenKind::LParen)?;
                        let entry = self.expect_ident()?;
                        self.expect(TokenKind::RParen)?;
                        Ok(Expr::NewThread(entry))
                    }
                    TypeSyntax::Named(type_name) => {
                        let args = if self.at(&TokenKind::LParen) {
                            self.expect(TokenKind::LParen)?;
                            self.parse_call_args_after_lparen()?
                        } else {
                            Vec::new()
                        };
                        let fields = if self.at(&TokenKind::LBrace) {
                            if self.peek_is(1, &TokenKind::LBrace)
                                || !matches!(
                                    self.tokens.get(self.pos + 1).map(|t| &t.kind),
                                    Some(TokenKind::Ident(_))
                                )
                            {
                                let _ = self.parse_opaque_brace_initializer()?;
                                Vec::new()
                            } else {
                                self.parse_object_initializer()?
                            }
                        } else {
                            Vec::new()
                        };
                        Ok(Expr::NewObject {
                            type_name,
                            args,
                            fields,
                        })
                    }
                    TypeSyntax::GenericNamed {
                        name,
                        args: ty_args,
                    } => {
                        let args = if self.at(&TokenKind::LParen) {
                            self.expect(TokenKind::LParen)?;
                            self.parse_call_args_after_lparen()?
                        } else {
                            Vec::new()
                        };
                        let fields = if self.at(&TokenKind::LBrace) {
                            if self.peek_is(1, &TokenKind::LBrace)
                                || !matches!(
                                    self.tokens.get(self.pos + 1).map(|t| &t.kind),
                                    Some(TokenKind::Ident(_))
                                )
                            {
                                let _ = self.parse_opaque_brace_initializer()?;
                                Vec::new()
                            } else {
                                self.parse_object_initializer()?
                            }
                        } else {
                            Vec::new()
                        };
                        Ok(Expr::NewObject {
                            type_name: generic_type_name_for_parser(&name, &ty_args),
                            args,
                            fields,
                        })
                    }
                    other_ty => {
                        let type_name = type_syntax_name(&other_ty);
                        let args = if self.at(&TokenKind::LParen) {
                            self.expect(TokenKind::LParen)?;
                            self.parse_call_args_after_lparen()?
                        } else {
                            Vec::new()
                        };
                        let fields = if self.at(&TokenKind::LBrace) {
                            if self.peek_is(1, &TokenKind::LBrace)
                                || !matches!(
                                    self.tokens.get(self.pos + 1).map(|t| &t.kind),
                                    Some(TokenKind::Ident(_))
                                )
                            {
                                let _ = self.parse_opaque_brace_initializer()?;
                                Vec::new()
                            } else {
                                self.parse_object_initializer()?
                            }
                        } else {
                            Vec::new()
                        };
                        Ok(Expr::NewObject {
                            type_name,
                            args,
                            fields,
                        })
                    }
                }
            }
            TokenKind::LParen => {
                self.advance();
                let cast_checkpoint = self.pos;
                if self.parse_type_syntax()?.is_some() && self.match_kind(&TokenKind::RParen) {
                    if self.current_starts_expr() {
                        return self.parse_unary();
                    }
                }
                self.pos = cast_checkpoint;
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                self.advance();
                if self.at(&TokenKind::Dot) && self.peek_is(1, &TokenKind::Dot) {
                    self.advance();
                    self.advance();
                    let source = self.parse_expr()?;
                    self.expect(TokenKind::RBracket)?;
                    Ok(Expr::MethodCall {
                        target: Box::new(Expr::Var("Enumerable".to_string())),
                        name: "ToList".to_string(),
                        args: vec![source],
                    })
                } else {
                    Ok(Expr::ArrayLiteral(
                        self.parse_expr_list_until(TokenKind::RBracket)?,
                    ))
                }
            }
            TokenKind::LBrace => Ok(Expr::ArrayLiteral(self.parse_array_initializer()?)),
            _ => Err(self.error_here("expected expression")),
        }
    }

    fn parse_nameof_argument(&mut self) -> Result<String, String> {
        self.expect(TokenKind::LParen)?;
        let mut depth = 1usize;
        let mut last_ident: Option<String> = None;
        while depth > 0 {
            match self.current().kind.clone() {
                TokenKind::LParen => {
                    depth += 1;
                    self.advance();
                }
                TokenKind::RParen => {
                    depth -= 1;
                    self.advance();
                }
                TokenKind::Ident(name) => {
                    last_ident = Some(name);
                    self.advance();
                }
                TokenKind::Eof => {
                    return Err(self.error_here("unterminated nameof expression"));
                }
                _ => self.advance(),
            }
        }
        Ok(last_ident.unwrap_or_default())
    }

    fn parse_opaque_parenthesized(&mut self) -> Result<(), String> {
        self.expect(TokenKind::LParen)?;
        let mut depth = 1usize;
        while depth > 0 {
            match self.current().kind {
                TokenKind::LParen => {
                    depth += 1;
                    self.advance();
                }
                TokenKind::RParen => {
                    depth -= 1;
                    self.advance();
                }
                TokenKind::Eof => {
                    return Err(self.error_here("unterminated parenthesized expression"));
                }
                _ => self.advance(),
            }
        }
        Ok(())
    }

    fn parse_assignment_stmt(&mut self) -> Result<Option<Stmt>, String> {
        let checkpoint = self.pos;
        let target = self.parse_postfix()?;
        if self.at(&TokenKind::Question)
            && self.peek_is(1, &TokenKind::Question)
            && self.peek_is(2, &TokenKind::Eq)
        {
            self.advance();
            self.advance();
            self.advance();
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            let condition = Expr::Binary {
                left: Box::new(target.clone()),
                op: BinaryOp::Eq,
                right: Box::new(Expr::Null),
            };
            let assign = match target {
                Expr::Var(name) => Stmt::Assign { name, expr },
                target => Stmt::AssignTarget { target, expr },
            };
            return Ok(Some(Stmt::If {
                condition,
                then_body: vec![assign],
                else_body: Vec::new(),
            }));
        }
        if self.match_kind(&TokenKind::Eq) {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            return match target {
                Expr::Var(name) => Ok(Some(Stmt::Assign { name, expr })),
                target => Ok(Some(Stmt::AssignTarget { target, expr })),
            };
        }
        let compound_op = if self.at(&TokenKind::Plus) && self.peek_is(1, &TokenKind::Eq) {
            self.advance();
            self.advance();
            Some(BinaryOp::Add)
        } else if self.at(&TokenKind::Minus) && self.peek_is(1, &TokenKind::Eq) {
            self.advance();
            self.advance();
            Some(BinaryOp::Sub)
        } else {
            None
        };
        if let Some(op) = compound_op {
            let right = self.parse_expr()?;
            self.expect(TokenKind::Semi)?;
            let expr = Expr::Binary {
                left: Box::new(target.clone()),
                op,
                right: Box::new(right),
            };
            return match target {
                Expr::Var(name) => Ok(Some(Stmt::Assign { name, expr })),
                target => Ok(Some(Stmt::AssignTarget { target, expr })),
            };
        }
        self.pos = checkpoint;
        Ok(None)
    }

    fn parse_typed_decl_start(&mut self) -> Result<Option<(TypeSyntax, String)>, String> {
        let checkpoint = self.pos;
        let Some(ty) = self.parse_type_syntax()? else {
            return Ok(None);
        };
        let TokenKind::Ident(name) = self.current().kind.clone() else {
            self.pos = checkpoint;
            return Ok(None);
        };
        self.advance();
        if self.at(&TokenKind::Eq) {
            Ok(Some((ty, name)))
        } else {
            self.pos = checkpoint;
            Ok(None)
        }
    }

    fn parse_type_syntax(&mut self) -> Result<Option<TypeSyntax>, String> {
        let mut ty = match self.current().kind.clone() {
            TokenKind::Ref => {
                self.advance();
                Some(TypeSyntax::Ref(Box::new(
                    self.parse_type_syntax()?
                        .ok_or_else(|| self.error_here("expected type after ref"))?,
                )))
            }
            TokenKind::Ident(_) | TokenKind::Borrow | TokenKind::Move => self.parse_named_type_syntax()?,
            _ => None,
        };
        loop {
            if ty.is_some() && self.at(&TokenKind::LBracket) && self.peek_is(1, &TokenKind::RBracket) {
                self.expect(TokenKind::LBracket)?;
                self.expect(TokenKind::RBracket)?;
                ty = Some(TypeSyntax::Array(Box::new(ty.expect("present"))));
            } else if ty.is_some() && self.match_kind(&TokenKind::Star) {
                ty = Some(TypeSyntax::Ref(Box::new(ty.expect("present"))));
            } else {
                break;
            }
        }
        if ty.is_some() && self.match_kind(&TokenKind::Question) {
            ty = Some(TypeSyntax::Nullable(Box::new(ty.expect("present"))));
        }
        Ok(ty)
    }

    fn parse_named_type_syntax(&mut self) -> Result<Option<TypeSyntax>, String> {
        let raw_parts = self.parse_qualified_name()?;
        let parts = self.resolve_type_path(raw_parts);
        if is_type_path(&parts, &["bool"]) || is_type_path(&parts, &["Boolean"]) {
            return Ok(Some(TypeSyntax::Scalar(ScalarType::Bool)));
        }
        if is_type_path(&parts, &["byte"]) {
            return Ok(Some(TypeSyntax::Scalar(ScalarType::Byte)));
        }
        if is_type_path(&parts, &["short"]) {
            return Ok(Some(TypeSyntax::Scalar(ScalarType::Short)));
        }
        if is_type_path(&parts, &["int"]) {
            return Ok(Some(TypeSyntax::Scalar(ScalarType::I32)));
        }
        if is_type_path(&parts, &["long"]) {
            return Ok(Some(TypeSyntax::Scalar(ScalarType::I64)));
        }
        if is_type_path(&parts, &["uint"]) {
            return Ok(Some(TypeSyntax::Scalar(ScalarType::U32)));
        }
        if is_type_path(&parts, &["double"]) {
            return Ok(Some(TypeSyntax::Scalar(ScalarType::F64)));
        }
        if is_type_path(&parts, &["decimal"]) {
            return Ok(Some(TypeSyntax::Scalar(ScalarType::Decimal)));
        }
        if is_type_path(&parts, &["string"]) || is_type_path(&parts, &["String"]) {
            return Ok(Some(TypeSyntax::String));
        }
        if is_type_path(&parts, &["DateTime"]) || is_type_path(&parts, &["System", "DateTime"]) {
            return Ok(Some(TypeSyntax::String));
        }
        if is_type_path(&parts, &["Guid"]) || is_type_path(&parts, &["System", "Guid"]) {
            return Ok(Some(TypeSyntax::String));
        }
        if is_type_path(&parts, &["void"]) {
            return Ok(Some(TypeSyntax::Void));
        }
        if is_type_path(&parts, &["Thread"])
            || is_type_path(&parts, &["System", "Threading", "Thread"])
        {
            return Ok(Some(TypeSyntax::Thread));
        }
        if is_type_path(&parts, &["Task"])
            || is_type_path(&parts, &["System", "Threading", "Tasks", "Task"])
            || is_type_path(&parts, &["ValueTask"])
            || is_type_path(&parts, &["System", "Threading", "Tasks", "ValueTask"])
        {
            let result = if self.at(&TokenKind::Less) {
                let mut args = self.parse_generic_type_args("Task")?;
                if args.len() != 1 {
                    return Err(self.error_here("Task/ValueTask expects one result type"));
                }
                args.remove(0)
            } else {
                TypeSyntax::Void
            };
            return Ok(Some(TypeSyntax::Task(Box::new(result))));
        }
        if is_type_path(&parts, &["List"])
            || is_type_path(&parts, &["System", "Collections", "Generic", "List"])
        {
            let mut args = self.parse_generic_type_args("List")?;
            if args.len() != 1 {
                return Err(self.error_here("List expects one element type"));
            }
            let element = args.remove(0);
            return Ok(Some(TypeSyntax::List(Box::new(element))));
        }
        if is_type_path(&parts, &["Dictionary"])
            || is_type_path(&parts, &["System", "Collections", "Generic", "Dictionary"])
        {
            let mut args = self.parse_generic_type_args("Dictionary")?;
            if args.len() != 2 {
                return Err(self.error_here("Dictionary expects key and value types"));
            }
            let key = args.remove(0);
            let value = args.remove(0);
            return Ok(Some(TypeSyntax::Dictionary(Box::new(key), Box::new(value))));
        }
        if is_type_path(&parts, &["IReadOnlyDictionary"])
            || is_type_path(&parts, &["System", "Collections", "Generic", "IReadOnlyDictionary"])
        {
            let args = self.parse_generic_type_args("IReadOnlyDictionary")?;
            if args.len() != 2 {
                return Err(self.error_here("IReadOnlyDictionary expects key and value types"));
            }
            return Ok(Some(TypeSyntax::GenericNamed {
                name: parts.join("."),
                args,
            }));
        }
        if is_type_path(&parts, &["IEnumerable"])
            || is_type_path(&parts, &["System", "Collections", "Generic", "IEnumerable"])
            || is_type_path(&parts, &["ICollection"])
            || is_type_path(&parts, &["System", "Collections", "Generic", "ICollection"])
        {
            let args = self.parse_generic_type_args("collection interface")?;
            if args.len() > 1 {
                return Err(self.error_here("collection interface expects one element type"));
            }
            return Ok(Some(TypeSyntax::GenericNamed {
                name: parts.join("."),
                args,
            }));
        }
        if self.at(&TokenKind::Less) {
            let args = self.parse_generic_type_args(&parts.join("."))?;
            return Ok(Some(TypeSyntax::GenericNamed {
                name: parts.join("."),
                args,
            }));
        }
        Ok(Some(TypeSyntax::Named(parts.join("."))))
    }

    fn parse_generic_type_args(&mut self, type_name: &str) -> Result<Vec<TypeSyntax>, String> {
        self.expect(TokenKind::Less)?;
        let mut args = Vec::new();
        if self.match_kind(&TokenKind::Greater) {
            return Ok(args);
        }
        loop {
            if self.at(&TokenKind::Comma) {
                args.push(TypeSyntax::Void);
                self.advance();
                if self.at(&TokenKind::Greater) {
                    args.push(TypeSyntax::Void);
                    self.advance();
                    break;
                }
                continue;
            }
            let ty = self.parse_type_syntax()?.ok_or_else(|| {
                self.error_here(&format!("expected type argument for {type_name}"))
            })?;
            args.push(ty);
            if self.match_kind(&TokenKind::Comma) {
                if self.at(&TokenKind::Greater) {
                    args.push(TypeSyntax::Void);
                    self.advance();
                    break;
                }
                continue;
            }
            self.expect(TokenKind::Greater)?;
            break;
        }
        Ok(args)
    }

    fn parse_qualified_name(&mut self) -> Result<Vec<String>, String> {
        let mut parts = vec![self.expect_ident()?];
        while self.match_kind(&TokenKind::Dot) {
            parts.push(self.expect_ident()?);
        }
        Ok(parts)
    }

    fn resolve_type_path(&self, parts: Vec<String>) -> Vec<String> {
        let Some((first, rest)) = parts.split_first() else {
            return parts;
        };
        if let Some(alias) = self.using_aliases.get(first) {
            let mut resolved = alias.clone();
            resolved.extend(rest.iter().cloned());
            resolved
        } else {
            parts
        }
    }

    fn parse_array_initializer(&mut self) -> Result<Vec<Expr>, String> {
        self.expect(TokenKind::LBrace)?;
        self.parse_expr_list_until(TokenKind::RBrace)
    }

    fn parse_object_initializer(&mut self) -> Result<Vec<FieldInit>, String> {
        self.expect(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        if self.match_kind(&TokenKind::RBrace) {
            return Ok(fields);
        }
        loop {
            let name = self.expect_ident()?;
            let (name, expr) = if self.match_kind(&TokenKind::Eq) {
                let expr = if self.at(&TokenKind::LBrace) {
                    self.parse_opaque_brace_initializer()?
                } else {
                    self.parse_expr()?
                };
                (name, expr)
            } else {
                let mut expr = Expr::Var(name.clone());
                let mut field_name = name;
                loop {
                    if self.match_kind(&TokenKind::Dot) {
                        field_name = self.expect_ident()?;
                        expr = Expr::Field {
                            target: Box::new(expr),
                            name: field_name.clone(),
                        };
                    } else if self.match_kind(&TokenKind::LBracket) {
                        let index = self.parse_expr()?;
                        self.expect(TokenKind::RBracket)?;
                        expr = Expr::Index {
                            target: Box::new(expr),
                            index: Box::new(index),
                        };
                    } else {
                        break;
                    }
                }
                (field_name, expr)
            };
            fields.push(FieldInit { name, expr });
            if self.match_kind(&TokenKind::RBrace) {
                break;
            }
            self.expect(TokenKind::Comma)?;
            if self.match_kind(&TokenKind::RBrace) {
                break;
            }
        }
        Ok(fields)
    }

    fn parse_opaque_brace_initializer(&mut self) -> Result<Expr, String> {
        self.expect(TokenKind::LBrace)?;
        let mut depth = 1usize;
        while depth > 0 {
            match self.current().kind {
                TokenKind::LBrace => {
                    depth += 1;
                    self.advance();
                }
                TokenKind::RBrace => {
                    depth -= 1;
                    self.advance();
                }
                TokenKind::Eof => return Err(self.error_here("unterminated initializer")),
                _ => self.advance(),
            }
        }
        Ok(Expr::Null)
    }

    fn parse_call_args_after_lparen(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        if self.match_kind(&TokenKind::RParen) {
            return Ok(args);
        }
        loop {
            args.push(self.parse_call_arg()?);
            if self.match_kind(&TokenKind::RParen) {
                break;
            }
            self.expect(TokenKind::Comma)?;
        }
        Ok(args)
    }

    fn parse_call_arg(&mut self) -> Result<Expr, String> {
        let modifier = if self.match_kind(&TokenKind::Ref) {
            Some(ParamModifier::Ref)
        } else if self.match_kind(&TokenKind::In) {
            Some(ParamModifier::In)
        } else if self.current_ident_is("out") {
            self.advance();
            Some(ParamModifier::Out)
        } else {
            None
        };
        if let Some(modifier) = modifier {
            if self.match_kind(&TokenKind::Var) {
                let name = self.expect_ident()?;
                return Ok(Expr::RefArg {
                    modifier,
                    expr: Box::new(Expr::Var(name)),
                });
            }
            let checkpoint = self.pos;
            if self.parse_type_syntax()?.is_some() {
                if let TokenKind::Ident(name) = self.current().kind.clone() {
                    self.advance();
                    return Ok(Expr::RefArg {
                        modifier,
                        expr: Box::new(Expr::Var(name)),
                    });
                }
                self.pos = checkpoint;
            }
            return Ok(Expr::RefArg {
                modifier,
                expr: Box::new(self.parse_expr()?),
            });
        }
        if let TokenKind::Ident(name) = self.current().kind.clone() {
            if self.peek_is(1, &TokenKind::Colon) {
                self.advance();
                self.expect(TokenKind::Colon)?;
                return Ok(Expr::NamedArg {
                    name,
                    expr: Box::new(self.parse_expr()?),
                });
            }
        }
        self.parse_expr()
    }

    fn parse_expr_list_until(&mut self, end: TokenKind) -> Result<Vec<Expr>, String> {
        let mut values = Vec::new();
        if self.match_kind(&end) {
            return Ok(values);
        }
        loop {
            values.push(self.parse_expr()?);
            if self.match_kind(&end) {
                break;
            }
            self.expect(TokenKind::Comma)?;
            if self.match_kind(&end) {
                break;
            }
        }
        Ok(values)
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        let name = match self.current().kind.clone() {
            TokenKind::Ident(name) => name,
            TokenKind::Borrow => "borrow".to_string(),
            TokenKind::Move => "move".to_string(),
            _ => return Err(self.error_here("expected identifier")),
        };
        self.advance();
        Ok(name)
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.match_kind(&kind) {
            Ok(())
        } else {
            Err(self.error_here(&format!("expected {:?}", kind)))
        }
    }

    fn match_kind(&mut self, kind: &TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn at(&self, kind: &TokenKind) -> bool {
        self.current().kind == *kind
    }

    fn current_ident_is(&self, name: &str) -> bool {
        matches!(&self.current().kind, TokenKind::Ident(current) if current == name)
    }

    fn current_starts_expr(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Int(_)
                | TokenKind::Float(_)
                | TokenKind::Bool(_)
                | TokenKind::Null
                | TokenKind::Throw
                | TokenKind::Default
                | TokenKind::String(_)
                | TokenKind::Ident(_)
                | TokenKind::Move
                | TokenKind::Borrow
                | TokenKind::Await
                | TokenKind::New
                | TokenKind::LParen
                | TokenKind::LBracket
                | TokenKind::LBrace
                | TokenKind::Bang
                | TokenKind::Minus
                | TokenKind::PlusPlus
                | TokenKind::MinusMinus
        )
    }

    fn peek_is(&self, offset: usize, kind: &TokenKind) -> bool {
        self.tokens
            .get(self.pos + offset)
            .is_some_and(|token| token.kind == *kind)
    }

    fn previous_is(&self, kind: &TokenKind) -> bool {
        self.pos
            .checked_sub(1)
            .and_then(|index| self.tokens.get(index))
            .is_some_and(|token| token.kind == *kind)
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if !self.at(&TokenKind::Eof) {
            self.pos += 1;
        }
    }

    fn parse_macro_def(&mut self) -> Result<(), String> {
        self.expect_ident()?; // consume "macro"
        let name = self.expect_ident()?;
        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        if !self.at(&TokenKind::RParen) {
            loop {
                params.push(self.expect_ident()?);
                if !self.match_kind(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Arrow)?;

        let mut body = Vec::new();
        let mut paren_level = 0;
        let mut brace_level = 0;
        let mut bracket_level = 0;
        loop {
            let tok = self.current().clone();
            match tok.kind {
                TokenKind::LParen => paren_level += 1,
                TokenKind::RParen => {
                    if paren_level > 0 {
                        paren_level -= 1;
                    }
                }
                TokenKind::LBrace => brace_level += 1,
                TokenKind::RBrace => {
                    if brace_level > 0 {
                        brace_level -= 1;
                    }
                }
                TokenKind::LBracket => bracket_level += 1,
                TokenKind::RBracket => {
                    if bracket_level > 0 {
                        bracket_level -= 1;
                    }
                }
                TokenKind::Semi => {
                    if paren_level == 0 && brace_level == 0 && bracket_level == 0 {
                        self.advance(); // consume the Semi
                        break;
                    }
                }
                TokenKind::Eof => {
                    return Err(self.error_here("unexpected EOF in macro body"));
                }
                _ => {}
            }
            body.push(tok);
            self.advance();
        }

        self.macros
            .insert(name.clone(), MacroDef { name, params, body });
        Ok(())
    }

    fn parse_macro_args(&mut self) -> Result<Vec<Vec<Token>>, String> {
        self.expect(TokenKind::LParen)?;
        let mut args = Vec::new();
        if self.match_kind(&TokenKind::RParen) {
            return Ok(args);
        }
        loop {
            let mut arg_tokens = Vec::new();
            let mut paren_level = 0;
            let mut brace_level = 0;
            let mut bracket_level = 0;
            loop {
                let tok = self.current().clone();
                match tok.kind {
                    TokenKind::LParen => paren_level += 1,
                    TokenKind::RParen => {
                        if paren_level == 0 && brace_level == 0 && bracket_level == 0 {
                            break;
                        }
                        paren_level -= 1;
                    }
                    TokenKind::LBrace => brace_level += 1,
                    TokenKind::RBrace => {
                        if brace_level == 0 && paren_level == 0 && bracket_level == 0 {
                            break;
                        }
                        brace_level -= 1;
                    }
                    TokenKind::LBracket => bracket_level += 1,
                    TokenKind::RBracket => {
                        if bracket_level == 0 && paren_level == 0 && brace_level == 0 {
                            break;
                        }
                        bracket_level -= 1;
                    }
                    TokenKind::Comma => {
                        if paren_level == 0 && brace_level == 0 && bracket_level == 0 {
                            break;
                        }
                    }
                    TokenKind::Eof => {
                        return Err(self.error_here("unexpected EOF in macro arguments"));
                    }
                    _ => {}
                }
                arg_tokens.push(tok);
                self.advance();
            }
            args.push(arg_tokens);
            if self.match_kind(&TokenKind::RParen) {
                break;
            }
            self.expect(TokenKind::Comma)?;
        }
        Ok(args)
    }

    fn expand_macro(&self, def: &MacroDef, args: &[Vec<Token>]) -> Vec<Token> {
        let mut expanded = Vec::new();
        let mut i = 0;
        while i < def.body.len() {
            if def.body[i].kind == TokenKind::Hash {
                if i + 1 < def.body.len() {
                    if let TokenKind::Ident(param_name) = &def.body[i + 1].kind {
                        if let Some(param_idx) = def.params.iter().position(|p| p == param_name) {
                            let arg_tokens = &args[param_idx];
                            let mut arg_str = String::new();
                            for (t_idx, tok) in arg_tokens.iter().enumerate() {
                                if t_idx > 0 {
                                    arg_str.push(' ');
                                }
                                arg_str.push_str(&token_to_string(tok));
                            }
                            expanded.push(Token {
                                kind: TokenKind::String(arg_str),
                                line: def.body[i].line,
                                col: def.body[i].col,
                            });
                            i += 2;
                            continue;
                        }
                    }
                }
            }

            if let TokenKind::Ident(ref name) = def.body[i].kind {
                if let Some(param_idx) = def.params.iter().position(|p| p == name) {
                    let arg_tokens = &args[param_idx];
                    for tok in arg_tokens {
                        let mut replaced_tok = tok.clone();
                        replaced_tok.line = def.body[i].line;
                        replaced_tok.col = def.body[i].col;
                        expanded.push(replaced_tok);
                    }
                    i += 1;
                    continue;
                }
            }

            expanded.push(def.body[i].clone());
            i += 1;
        }

        if !expanded.is_empty() && expanded.last().unwrap().kind != TokenKind::Semi {
            let last = expanded.last().unwrap();
            expanded.push(Token {
                kind: TokenKind::Semi,
                line: last.line,
                col: last.col + 1,
            });
        }

        expanded
    }

    fn parse_type_alias(&mut self) -> Result<(), String> {
        self.expect_ident()?; // consume "type"
        let _name = if self.match_kind(&TokenKind::Borrow) {
            "borrow".to_string()
        } else {
            self.expect_ident()?
        };
        let _generic_params = self.parse_generic_params()?;
        self.expect(TokenKind::Eq)?;
        let _target_ty = self.parse_type_syntax()?;
        self.expect(TokenKind::Semi)?;
        Ok(())
    }

    fn error_here(&self, message: &str) -> String {
        let token = self.current();
        format!("{}:{}: {message}", token.line, token.col)
    }

    fn skip_preprocessor_directive(&mut self) {
        let line = self.current().line;
        while !self.at(&TokenKind::Eof) && self.current().line == line {
            self.advance();
        }
    }
}

fn token_to_string(tok: &Token) -> String {
    match &tok.kind {
        TokenKind::Fn => "fn".to_string(),
        TokenKind::Let => "let".to_string(),
        TokenKind::Var => "var".to_string(),
        TokenKind::Package => "package".to_string(),
        TokenKind::Native => "native".to_string(),
        TokenKind::Namespace => "namespace".to_string(),
        TokenKind::Using => "using".to_string(),
        TokenKind::Async => "async".to_string(),
        TokenKind::Static => "static".to_string(),
        TokenKind::Const => "const".to_string(),
        TokenKind::Readonly => "readonly".to_string(),
        TokenKind::Mut => "mut".to_string(),
        TokenKind::New => "new".to_string(),
        TokenKind::Ref => "ref".to_string(),
        TokenKind::Struct => "struct".to_string(),
        TokenKind::Class => "class".to_string(),
        TokenKind::Record => "record".to_string(),
        TokenKind::Interface => "interface".to_string(),
        TokenKind::Delegate => "delegate".to_string(),
        TokenKind::Enum => "enum".to_string(),
        TokenKind::Public => "public".to_string(),
        TokenKind::Borrow => "borrow".to_string(),
        TokenKind::Move => "move".to_string(),
        TokenKind::Print => "print".to_string(),
        TokenKind::Return => "return".to_string(),
        TokenKind::Throw => "throw".to_string(),
        TokenKind::If => "if".to_string(),
        TokenKind::Else => "else".to_string(),
        TokenKind::Try => "try".to_string(),
        TokenKind::Catch => "catch".to_string(),
        TokenKind::Finally => "finally".to_string(),
        TokenKind::Switch => "switch".to_string(),
        TokenKind::Case => "case".to_string(),
        TokenKind::Default => "default".to_string(),
        TokenKind::Break => "break".to_string(),
        TokenKind::Continue => "continue".to_string(),
        TokenKind::While => "while".to_string(),
        TokenKind::For => "for".to_string(),
        TokenKind::Foreach => "foreach".to_string(),
        TokenKind::In => "in".to_string(),
        TokenKind::Await => "await".to_string(),
        TokenKind::Bool(b) => b.to_string(),
        TokenKind::Null => "null".to_string(),
        TokenKind::Ident(s) => s.clone(),
        TokenKind::Int(i) => i.to_string(),
        TokenKind::Float(f) => f.to_string(),
        TokenKind::String(s) => format!("\"{}\"", s),
        TokenKind::LParen => "(".to_string(),
        TokenKind::RParen => ")".to_string(),
        TokenKind::LBrace => "{".to_string(),
        TokenKind::RBrace => "}".to_string(),
        TokenKind::LBracket => "[".to_string(),
        TokenKind::RBracket => "]".to_string(),
        TokenKind::Question => "?".to_string(),
        TokenKind::Eq => "=".to_string(),
        TokenKind::EqEq => "==".to_string(),
        TokenKind::Bang => "!".to_string(),
        TokenKind::BangEq => "!=".to_string(),
        TokenKind::Arrow => "=>".to_string(),
        TokenKind::Semi => ";".to_string(),
        TokenKind::Colon => ":".to_string(),
        TokenKind::Plus => "+".to_string(),
        TokenKind::PlusPlus => "++".to_string(),
        TokenKind::Minus => "-".to_string(),
        TokenKind::MinusMinus => "--".to_string(),
        TokenKind::Star => "*".to_string(),
        TokenKind::Slash => "/".to_string(),
        TokenKind::Percent => "%".to_string(),
        TokenKind::Amp => "&".to_string(),
        TokenKind::AmpAmp => "&&".to_string(),
        TokenKind::Pipe => "|".to_string(),
        TokenKind::PipePipe => "||".to_string(),
        TokenKind::Comma => ",".to_string(),
        TokenKind::Dot => ".".to_string(),
        TokenKind::Less => "<".to_string(),
        TokenKind::LessEq => "<=".to_string(),
        TokenKind::Greater => ">".to_string(),
        TokenKind::GreaterEq => ">=".to_string(),
        TokenKind::Eof => "".to_string(),
        TokenKind::Hash => "#".to_string(),
    }
}

fn is_type_path(parts: &[String], expected: &[&str]) -> bool {
    parts.len() == expected.len() && parts.iter().zip(expected.iter()).all(|(a, b)| a == b)
}

fn type_syntax_name(ty: &TypeSyntax) -> String {
    match ty {
        TypeSyntax::Scalar(ScalarType::Bool) => "bool".to_string(),
        TypeSyntax::Scalar(ScalarType::Byte) => "byte".to_string(),
        TypeSyntax::Scalar(ScalarType::Short) => "short".to_string(),
        TypeSyntax::Scalar(ScalarType::I32) => "int".to_string(),
        TypeSyntax::Scalar(ScalarType::I64) => "long".to_string(),
        TypeSyntax::Scalar(ScalarType::U32) => "uint".to_string(),
        TypeSyntax::Scalar(ScalarType::F64) => "double".to_string(),
        TypeSyntax::Scalar(ScalarType::Decimal) => "decimal".to_string(),
        TypeSyntax::String => "string".to_string(),
        TypeSyntax::Array(inner) => format!("{}[]", type_syntax_name(inner)),
        TypeSyntax::Ref(inner) => format!("ref {}", type_syntax_name(inner)),
        TypeSyntax::Named(name) => name.clone(),
        TypeSyntax::GenericNamed { name, args } => format!(
            "{}<{}>",
            name,
            args.iter()
                .map(type_syntax_name)
                .collect::<Vec<_>>()
                .join(",")
        ),
        TypeSyntax::List(inner) => format!("List<{}>", type_syntax_name(inner)),
        TypeSyntax::Dictionary(key, value) => {
            format!(
                "Dictionary<{},{}>",
                type_syntax_name(key),
                type_syntax_name(value)
            )
        }
        TypeSyntax::IEnumerable(inner) => format!("IEnumerable<{}>", type_syntax_name(inner)),
        TypeSyntax::Thread => "Thread".to_string(),
        TypeSyntax::Task(inner) => format!("Task<{}>", type_syntax_name(inner)),
        TypeSyntax::Nullable(inner) => format!("{}?", type_syntax_name(inner)),
        TypeSyntax::Void => "void".to_string(),
    }
}

fn generic_type_definition_name(ty: &TypeSyntax) -> String {
    match ty {
        TypeSyntax::GenericNamed { name, args } => {
            if args.is_empty() {
                format!("{name}<>")
            } else {
                let placeholders = std::iter::repeat("")
                    .take(args.len())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{name}<{placeholders}>")
            }
        }
        TypeSyntax::List(_) => "List<>".to_string(),
        TypeSyntax::Dictionary(_, _) => "Dictionary<,>".to_string(),
        TypeSyntax::IEnumerable(_) => "IEnumerable<>".to_string(),
        TypeSyntax::Task(_) => "Task<>".to_string(),
        _ => type_syntax_name(ty),
    }
}

fn is_generic_type_syntax(ty: &TypeSyntax) -> bool {
    matches!(
        ty,
        TypeSyntax::GenericNamed { .. }
            | TypeSyntax::List(_)
            | TypeSyntax::Dictionary(_, _)
            | TypeSyntax::IEnumerable(_)
            | TypeSyntax::Task(_)
    )
}

fn type_object_expr(ty: &TypeSyntax) -> Expr {
    let generic_args = match ty {
        TypeSyntax::GenericNamed { args, .. } => args.iter().map(type_object_expr).collect(),
        TypeSyntax::List(inner)
        | TypeSyntax::IEnumerable(inner)
        | TypeSyntax::Task(inner) => vec![type_object_expr(inner)],
        TypeSyntax::Dictionary(key, value) => vec![type_object_expr(key), type_object_expr(value)],
        TypeSyntax::Nullable(inner) => vec![type_object_expr(inner)],
        _ => Vec::new(),
    };
    Expr::NewObject {
        type_name: "Type".to_string(),
        args: Vec::new(),
        fields: vec![
            FieldInit {
                name: "FullName".to_string(),
                expr: Expr::String(type_syntax_name(ty)),
            },
            FieldInit {
                name: "GenericTypeDefinitionName".to_string(),
                expr: Expr::String(generic_type_definition_name(ty)),
            },
            FieldInit {
                name: "IsGenericType".to_string(),
                expr: Expr::Bool(is_generic_type_syntax(ty)),
            },
            FieldInit {
                name: "GenericArguments".to_string(),
                expr: Expr::ArrayLiteral(generic_args),
            },
        ],
    }
}

fn generic_type_name_for_parser(name: &str, args: &[TypeSyntax]) -> String {
    format!(
        "{}_{}",
        name,
        args.iter()
            .map(type_syntax_name)
            .collect::<Vec<_>>()
            .join("_")
    )
}

fn merge_type_declarations(types: Vec<TypeDef>) -> Vec<TypeDef> {
    let mut merged = Vec::<TypeDef>::new();
    let mut indices = HashMap::<(Vec<String>, String), usize>::new();
    for mut ty in types {
        let key = (ty.namespace.clone(), ty.name.clone());
        if let Some(index) = indices.get(&key).copied() {
            let existing = &mut merged[index];
            let can_merge = existing.kind == ty.kind || existing.is_extension || ty.is_extension;
            if can_merge {
                if existing.is_extension && !ty.is_extension {
                    existing.kind = ty.kind;
                    existing.is_extension = false;
                }
                existing.attributes.append(&mut ty.attributes);
                for base in ty.bases {
                    if !existing.bases.contains(&base) {
                        existing.bases.push(base);
                    }
                }
                existing.fields.append(&mut ty.fields);
                existing.constructors.append(&mut ty.constructors);
                existing.methods.append(&mut ty.methods);
                continue;
            }
        }
        indices.insert(key, merged.len());
        merged.push(ty);
    }
    merged
}

fn synthesize_generated_regex_methods(program: &mut Program) {
    for function in &mut program.functions {
        synthesize_generated_regex_function(function);
    }
    for ty in &mut program.types {
        for method in &mut ty.methods {
            synthesize_generated_regex_function(method);
        }
    }
}

fn synthesize_generated_regex_function(function: &mut Function) {
    if !function.body.is_empty() {
        return;
    }
    if !function
        .attributes
        .iter()
        .any(|attribute| attribute.name.contains("GeneratedRegex"))
    {
        return;
    }
    let Some(pattern) = function.attributes.iter().find_map(|attribute| {
        if !attribute.name.contains("GeneratedRegex") {
            return None;
        }
        attribute.args.first().and_then(|arg| match arg {
            Expr::String(value) => Some(value.clone()),
            _ => None,
        })
    }) else {
        return;
    };
    let return_type_name = type_syntax_name(&function.return_type);
    function.body = vec![Stmt::Return(Some(Expr::NewObject {
        type_name: return_type_name,
        args: vec![Expr::String(pattern)],
        fields: Vec::new(),
    }))];
}

fn synthesize_xunit_fact_tests(parser: &mut Parser, program: &Program) {
    for ty in &program.types {
        let class_name = qualified_type_name(&ty.namespace, &ty.name);
        for method in &ty.methods {
            if method.is_extern {
                continue;
            }
            if is_skip_attribute(&method.attributes) {
                continue;
            }
            if is_fact_attribute(&method.attributes)
                && method.generic_params.is_empty()
                && method.params.is_empty()
            {
                let invoke = Expr::MethodCall {
                    target: Box::new(Expr::NewObject {
                        type_name: ty.name.clone(),
                        args: Vec::new(),
                        fields: Vec::new(),
                    }),
                    name: method.name.clone(),
                    args: Vec::new(),
                };
                let invoke = if is_task_like_type(&method.return_type) {
                    Expr::MethodCall {
                        target: Box::new(invoke),
                        name: "Wait".to_string(),
                        args: Vec::new(),
                    }
                } else {
                    invoke
                };
                parser.test_registrations.push(Stmt::Expr(Expr::FunctionCall {
                    name: "XUnit_AddTest".to_string(),
                    args: vec![
                        Expr::String(class_name.clone()),
                        Expr::String(method.name.clone()),
                        Expr::Lambda {
                            params: Vec::new(),
                            body: Box::new(invoke),
                        },
                    ],
                }));
            }
            if !is_theory_attribute(&method.attributes) || method.params.is_empty() {
                continue;
            }
            let inline_data = method
                .attributes
                .iter()
                .filter(|attribute| is_inline_data_attribute(&attribute.name))
                .collect::<Vec<_>>();
            for (index, attribute) in inline_data.iter().enumerate() {
                register_xunit_theory_case(
                    parser,
                    &class_name,
                    ty,
                    method,
                    &format!("{}[{index}]", method.name),
                    attribute.args.clone(),
                );
            }
            for attribute in method
                .attributes
                .iter()
                .filter(|attribute| is_member_data_attribute(&attribute.name))
            {
                let Some(member_name) = attribute.args.first().and_then(xunit_string_value) else {
                    continue;
                };
                let rows = xunit_rows_from_member(program, ty, &member_name);
                for (index, row) in rows.into_iter().enumerate() {
                    register_xunit_theory_case(
                        parser,
                        &class_name,
                        ty,
                        method,
                        &format!("{}[member:{member_name}:{index}]", method.name),
                        row,
                    );
                }
            }
            for attribute in method
                .attributes
                .iter()
                .filter(|attribute| is_class_data_attribute(&attribute.name))
            {
                let Some(type_name) = attribute.args.first().and_then(xunit_type_name_value) else {
                    continue;
                };
                let rows = xunit_rows_from_type(program, &type_name);
                for (index, row) in rows.into_iter().enumerate() {
                    register_xunit_theory_case(
                        parser,
                        &class_name,
                        ty,
                        method,
                        &format!("{}[class:{type_name}:{index}]", method.name),
                        row,
                    );
                }
            }
        }
    }
}

fn register_xunit_theory_case(
    parser: &mut Parser,
    class_name: &str,
    ty: &TypeDef,
    method: &Function,
    test_name: &str,
    args: Vec<Expr>,
) {
    if args.len() != method.params.len() {
        return;
    }
    let invoke = Expr::MethodCall {
        target: Box::new(Expr::NewObject {
            type_name: ty.name.clone(),
            args: Vec::new(),
            fields: Vec::new(),
        }),
        name: method.name.clone(),
        args,
    };
    let invoke = if is_task_like_type(&method.return_type) {
        Expr::MethodCall {
            target: Box::new(invoke),
            name: "Wait".to_string(),
            args: Vec::new(),
        }
    } else {
        invoke
    };
    parser.test_registrations.push(Stmt::Expr(Expr::FunctionCall {
        name: "XUnit_AddTest".to_string(),
        args: vec![
            Expr::String(class_name.to_string()),
            Expr::String(test_name.to_string()),
            Expr::Lambda {
                params: Vec::new(),
                body: Box::new(invoke),
            },
        ],
    }));
}

fn is_fact_attribute(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        matches!(
            attribute.name.rsplit('.').next().unwrap_or(&attribute.name),
            "Fact" | "FactAttribute"
        )
    })
}

fn is_theory_attribute(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        matches!(
            attribute.name.rsplit('.').next().unwrap_or(&attribute.name),
            "Theory" | "TheoryAttribute"
        )
    })
}

fn is_inline_data_attribute(name: &str) -> bool {
    matches!(name.rsplit('.').next().unwrap_or(name), "InlineData" | "InlineDataAttribute")
}

fn is_skip_attribute(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        matches!(
            attribute.name.rsplit('.').next().unwrap_or(&attribute.name),
            "Skip" | "SkipAttribute"
        )
    })
}

fn is_member_data_attribute(name: &str) -> bool {
    matches!(name.rsplit('.').next().unwrap_or(name), "MemberData" | "MemberDataAttribute")
}

fn is_class_data_attribute(name: &str) -> bool {
    matches!(name.rsplit('.').next().unwrap_or(name), "ClassData" | "ClassDataAttribute")
}

fn xunit_string_value(expr: &Expr) -> Option<String> {
    match expr {
        Expr::String(value) => Some(value.clone()),
        _ => None,
    }
}

fn xunit_type_name_value(expr: &Expr) -> Option<String> {
    match expr {
        Expr::String(value) => Some(value.clone()),
        Expr::NewObject {
            type_name,
            fields,
            ..
        } if type_name == "Type" => fields.iter().find_map(|field| {
            if field.name == "FullName" {
                xunit_string_value(&field.expr)
            } else {
                None
            }
        }),
        _ => None,
    }
}

fn xunit_rows_from_member(_program: &Program, ty: &TypeDef, member_name: &str) -> Vec<Vec<Expr>> {
    let Some(method) = ty.methods.iter().find(|method| {
        method.is_static
            && method.generic_params.is_empty()
            && method.params.is_empty()
            && (method.name == member_name || method.name == property_getter_name(member_name))
    }) else {
        return Vec::new();
    };
    xunit_rows_from_data_function(method).unwrap_or_default()
}

fn xunit_rows_from_type(program: &Program, type_name: &str) -> Vec<Vec<Expr>> {
    let Some(ty) = program
        .types
        .iter()
        .find(|ty| qualified_type_name(&ty.namespace, &ty.name) == type_name)
    else {
        return Vec::new();
    };
    for candidate in ty.methods.iter().filter(|method| {
        method.is_static && method.generic_params.is_empty() && method.params.is_empty()
    }) {
        if candidate.name == "GetData" || candidate.name == "get_Data" {
            if let Some(rows) = xunit_rows_from_data_function(candidate) {
                return rows;
            }
        }
    }
    Vec::new()
}

fn xunit_rows_from_data_function(method: &Function) -> Option<Vec<Vec<Expr>>> {
    let [Stmt::Return(Some(expr))] = method.body.as_slice() else {
        return None;
    };
    xunit_rows_from_data_expr(expr)
}

fn xunit_rows_from_data_expr(expr: &Expr) -> Option<Vec<Vec<Expr>>> {
    let values = match expr {
        Expr::ArrayLiteral(values) => values,
        Expr::NewArray { values, .. } => values,
        _ => return None,
    };
    if values.is_empty() {
        return Some(Vec::new());
    }
    if values
        .iter()
        .all(|value| matches!(value, Expr::ArrayLiteral(_) | Expr::NewArray { .. }))
    {
        let mut rows = Vec::new();
        for value in values {
            let row = match value {
                Expr::ArrayLiteral(row) => row.clone(),
                Expr::NewArray { values, .. } => values.clone(),
                _ => unreachable!(),
            };
            rows.push(row);
        }
        return Some(rows);
    }
    Some(vec![values.clone()])
}

fn is_task_like_type(ty: &TypeSyntax) -> bool {
    match ty {
        TypeSyntax::Task(_) => true,
        TypeSyntax::Named(name) => matches!(name.as_str(), "Task" | "ValueTask" | "System.Threading.Tasks.Task" | "System.Threading.Tasks.ValueTask"),
        TypeSyntax::GenericNamed { name, .. } => matches!(
            name.as_str(),
            "Task" | "ValueTask" | "System.Threading.Tasks.Task" | "System.Threading.Tasks.ValueTask"
        ),
        _ => false,
    }
}

fn qualified_type_name(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", namespace.join("."), name)
    }
}

fn default_expr_for_type(ty: &TypeSyntax) -> Expr {
    match ty {
        TypeSyntax::Scalar(ScalarType::Bool) => Expr::Bool(false),
        TypeSyntax::Scalar(_) => Expr::Int(0),
        TypeSyntax::Void => Expr::Null,
        _ => Expr::Null,
    }
}

fn property_getter_name(name: &str) -> String {
    format!("get_{name}")
}
