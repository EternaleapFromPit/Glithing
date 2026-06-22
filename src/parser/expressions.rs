use super::*;

impl Parser {
    pub(super) fn parse_expr(&mut self) -> Result<Expr, String> {
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
                    LambdaBody::Block(self.parse_stmt_body()?)
                } else {
                    LambdaBody::Expr(Box::new(self.parse_expr()?))
                };
                return Ok(Expr::Lambda {
                    params,
                    body,
                });
            }
            self.pos = checkpoint;
        }
        if let TokenKind::Ident(name) = self.current().kind.clone() {
            if self.peek_is(1, &TokenKind::Arrow) {
                self.advance();
                self.expect(TokenKind::Arrow)?;
                let body = if self.at(&TokenKind::LBrace) {
                    LambdaBody::Block(self.parse_stmt_body()?)
                } else {
                    LambdaBody::Expr(Box::new(self.parse_expr()?))
                };
                return Ok(Expr::Lambda {
                    params: vec![name],
                    body,
                });
            }
        }
        self.parse_assignment_expr()
    }

    pub(super) fn parse_assignment_expr(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_conditional(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_or(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_and(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_bit_and(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_coalesce(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_comparison(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_additive(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_multiplicative(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_unary(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_postfix(&mut self) -> Result<Expr, String> {
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
                        generic_args: Vec::new(),
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
                            generic_args: Vec::new(),
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
                let mut generic_args = Vec::new();
                if self.at(&TokenKind::Less) {
                    match self.parse_generic_type_args(&name) {
                        Ok(args) if self.at(&TokenKind::LParen) => generic_args = args,
                        _ => self.pos = generic_checkpoint,
                    }
                }
                if self.match_kind(&TokenKind::LParen) {
                    let args = self.parse_call_args_after_lparen()?;
                    expr = Expr::MethodCall {
                        target: Box::new(expr),
                        name,
                        generic_args,
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
                    match self.parse_generic_type_args(&name) {
                        Ok(generic_args) if self.at(&TokenKind::LParen) => {
                            self.expect(TokenKind::LParen)?;
                            let args = self.parse_call_args_after_lparen()?;
                            Expr::MethodCall {
                                target: Box::new(base.clone()),
                                name,
                                generic_args,
                                args,
                            }
                        }
                        _ => {
                            self.pos = generic_checkpoint;
                            Expr::Field {
                                target: Box::new(base.clone()),
                                name,
                            }
                        }
                    }
                } else if self.match_kind(&TokenKind::LParen) {
                    let args = self.parse_call_args_after_lparen()?;
                    Expr::MethodCall {
                        target: Box::new(base.clone()),
                        name,
                        generic_args: Vec::new(),
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

    pub(super) fn parse_primary(&mut self) -> Result<Expr, String> {
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
                let mut generic_args = Vec::new();
                if self.at(&TokenKind::Less) {
                    match self.parse_generic_type_args(&name) {
                        Ok(args) if self.at(&TokenKind::LParen) => generic_args = args,
                        _ => self.pos = generic_checkpoint,
                    }
                }
                if self.match_kind(&TokenKind::LParen) {
                    Ok(Expr::FunctionCall {
                        name,
                        generic_args,
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
                        generic_args: Vec::new(),
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

    pub(super) fn parse_nameof_argument(&mut self) -> Result<String, String> {
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

    pub(super) fn parse_opaque_parenthesized(&mut self) -> Result<(), String> {
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

    pub(super) fn parse_assignment_stmt(&mut self) -> Result<Option<Stmt>, String> {
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

    pub(super) fn parse_typed_decl_start(&mut self) -> Result<Option<(TypeSyntax, String)>, String> {
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

    pub(super) fn parse_type_syntax(&mut self) -> Result<Option<TypeSyntax>, String> {
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

    pub(super) fn parse_named_type_syntax(&mut self) -> Result<Option<TypeSyntax>, String> {
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

    pub(super) fn parse_generic_type_args(&mut self, type_name: &str) -> Result<Vec<TypeSyntax>, String> {
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

    pub(super) fn parse_qualified_name(&mut self) -> Result<Vec<String>, String> {
        let mut parts = vec![self.expect_ident()?];
        while self.match_kind(&TokenKind::Dot) {
            parts.push(self.expect_ident()?);
        }
        Ok(parts)
    }

    pub(super) fn resolve_type_path(&self, parts: Vec<String>) -> Vec<String> {
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

    pub(super) fn parse_array_initializer(&mut self) -> Result<Vec<Expr>, String> {
        self.expect(TokenKind::LBrace)?;
        self.parse_expr_list_until(TokenKind::RBrace)
    }

    pub(super) fn parse_object_initializer(&mut self) -> Result<Vec<FieldInit>, String> {
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

    pub(super) fn parse_opaque_brace_initializer(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_call_args_after_lparen(&mut self) -> Result<Vec<Expr>, String> {
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

    pub(super) fn parse_call_arg(&mut self) -> Result<Expr, String> {
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

    pub(super) fn parse_expr_list_until(&mut self, end: TokenKind) -> Result<Vec<Expr>, String> {
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

}

