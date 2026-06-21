use super::*;

impl Parser {
    pub(super) fn expect_ident(&mut self) -> Result<String, String> {
        let name = match self.current().kind.clone() {
            TokenKind::Ident(name) => name,
            TokenKind::Borrow => "borrow".to_string(),
            TokenKind::Move => "move".to_string(),
            _ => return Err(self.error_here("expected identifier")),
        };
        self.advance();
        Ok(name)
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.match_kind(&kind) {
            Ok(())
        } else {
            Err(self.error_here(&format!("expected {:?}", kind)))
        }
    }

    pub(super) fn match_kind(&mut self, kind: &TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(super) fn at(&self, kind: &TokenKind) -> bool {
        self.current().kind == *kind
    }

    pub(super) fn current_ident_is(&self, name: &str) -> bool {
        matches!(&self.current().kind, TokenKind::Ident(current) if current == name)
    }

    pub(super) fn current_starts_expr(&self) -> bool {
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

    pub(super) fn peek_is(&self, offset: usize, kind: &TokenKind) -> bool {
        self.tokens
            .get(self.pos + offset)
            .is_some_and(|token| token.kind == *kind)
    }

    pub(super) fn previous_is(&self, kind: &TokenKind) -> bool {
        self.pos
            .checked_sub(1)
            .and_then(|index| self.tokens.get(index))
            .is_some_and(|token| token.kind == *kind)
    }

    pub(super) fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub(super) fn advance(&mut self) {
        if !self.at(&TokenKind::Eof) {
            self.pos += 1;
        }
    }

    pub(super) fn parse_macro_def(&mut self) -> Result<(), String> {
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

    pub(super) fn parse_macro_args(&mut self) -> Result<Vec<Vec<Token>>, String> {
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

    pub(super) fn expand_macro(&self, def: &MacroDef, args: &[Vec<Token>]) -> Vec<Token> {
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

    pub(super) fn parse_type_alias(&mut self) -> Result<(), String> {
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

    pub(super) fn error_here(&self, message: &str) -> String {
        let token = self.current();
        format!("{}:{}: {message}", token.line, token.col)
    }

    pub(super) fn skip_preprocessor_directive(&mut self) {
        let line = self.current().line;
        while !self.at(&TokenKind::Eof) && self.current().line == line {
            self.advance();
        }
    }
}
