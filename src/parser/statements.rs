use super::*;

impl Parser {
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt, String> {
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

    pub(super) fn parse_typed_decl_no_initializer(&mut self) -> Result<Option<(TypeSyntax, String)>, String> {
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

    pub(super) fn parse_switch_section_body(&mut self) -> Result<Vec<Stmt>, String> {
        let mut body = Vec::new();
        while !self.at(&TokenKind::Case)
            && !self.at(&TokenKind::Default)
            && !self.at(&TokenKind::RBrace)
        {
            body.push(self.parse_stmt()?);
        }
        Ok(body)
    }

    pub(super) fn parse_for_component(&mut self) -> Result<Stmt, String> {
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

    pub(super) fn parse_local_function_stmt(&mut self) -> Result<Option<Stmt>, String> {
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

    pub(super) fn parse_stmt_body(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(TokenKind::LBrace)?;
        self.parse_block_after_lbrace()
    }

    pub(super) fn parse_stmt_or_block_body(&mut self) -> Result<Vec<Stmt>, String> {
        if self.match_kind(&TokenKind::LBrace) {
            self.parse_block_after_lbrace()
        } else {
            Ok(vec![self.parse_stmt()?])
        }
    }

    pub(super) fn parse_block_after_lbrace(&mut self) -> Result<Vec<Stmt>, String> {
        let mut body = Vec::new();
        while !self.at(&TokenKind::RBrace) {
            body.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(body)
    }

}

