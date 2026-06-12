#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenKind {
    Fn,
    Let,
    Var,
    Package,
    Native,
    Namespace,
    Using,
    Async,
    Static,
    Const,
    Readonly,
    Mut,
    New,
    Ref,
    Struct,
    Class,
    Interface,
    Enum,
    Public,
    Borrow,
    Move,
    Print,
    Return,
    Throw,
    If,
    Else,
    Try,
    Catch,
    Finally,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    While,
    For,
    Foreach,
    In,
    Await,
    Bool(bool),
    Null,
    Ident(String),
    Int(i64),
    Float(f64),
    String(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Question,
    Eq,
    EqEq,
    Bang,
    BangEq,
    Arrow,
    Semi,
    Colon,
    Plus,
    Minus,
    Star,
    AmpAmp,
    Pipe,
    PipePipe,
    Comma,
    Dot,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Eof,
    Hash,
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

pub(crate) struct Lexer<'a> {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    _source: &'a str,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            _source: source,
        }
    }

    pub(crate) fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                '/' if self.peek_next() == Some('/') => {
                    while self.peek().is_some() && self.peek() != Some('\n') {
                        self.advance();
                    }
                }
                '/' if self.peek_next() == Some('*') => {
                    let start_line = self.line;
                    let start_col = self.col;
                    self.advance(); // consume '/'
                    self.advance(); // consume '*'
                    let mut terminated = false;
                    while let Some(c) = self.peek() {
                        if c == '*' && self.peek_next() == Some('/') {
                            self.advance(); // consume '*'
                            self.advance(); // consume '/'
                            terminated = true;
                            break;
                        }
                        self.advance();
                    }
                    if !terminated {
                        return Err(format!("{start_line}:{start_col}: unterminated block comment"));
                    }
                }
                '#' => {
                    let next = self.peek_next();
                    if next.is_none() || next.is_some_and(|c| c == ' ' || c == '\t' || c == '\r' || c == '\n') {
                        while self.peek().is_some() && self.peek() != Some('\n') {
                            self.advance();
                        }
                    } else {
                        tokens.push(self.single(TokenKind::Hash));
                    }
                }
                '0'..='9' => tokens.push(self.lex_number()?),
                '$' if self.peek_next() == Some('"') => {
                    self.advance();
                    tokens.push(self.lex_string()?);
                }
                '$' if self.peek_next() == Some('@') && self.peek_n(2) == Some('"') => {
                    self.advance();
                    self.advance();
                    tokens.push(self.lex_string()?);
                }
                '@' if self.peek_next() == Some('"') => {
                    self.advance();
                    tokens.push(self.lex_string()?);
                }
                '@' if self.peek_next() == Some('$') && self.peek_n(2) == Some('"') => {
                    self.advance();
                    self.advance();
                    tokens.push(self.lex_string()?);
                }
                '"' => tokens.push(self.lex_string()?),
                '\'' => tokens.push(self.lex_char_as_string()?),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.lex_ident()),
                '(' => tokens.push(self.single(TokenKind::LParen)),
                ')' => tokens.push(self.single(TokenKind::RParen)),
                '{' => tokens.push(self.single(TokenKind::LBrace)),
                '}' => tokens.push(self.single(TokenKind::RBrace)),
                '[' => tokens.push(self.single(TokenKind::LBracket)),
                ']' => tokens.push(self.single(TokenKind::RBracket)),
                '?' => tokens.push(self.single(TokenKind::Question)),
                '=' if self.peek_next() == Some('=') => tokens.push(self.double(TokenKind::EqEq)),
                '=' if self.peek_next() == Some('>') => tokens.push(self.double(TokenKind::Arrow)),
                '=' => tokens.push(self.single(TokenKind::Eq)),
                '!' if self.peek_next() == Some('=') => tokens.push(self.double(TokenKind::BangEq)),
                '!' => tokens.push(self.single(TokenKind::Bang)),
                ';' => tokens.push(self.single(TokenKind::Semi)),
                ':' => tokens.push(self.single(TokenKind::Colon)),
                '+' => tokens.push(self.single(TokenKind::Plus)),
                '-' => tokens.push(self.single(TokenKind::Minus)),
                '*' => tokens.push(self.single(TokenKind::Star)),
                '&' if self.peek_next() == Some('&') => tokens.push(self.double(TokenKind::AmpAmp)),
                '|' if self.peek_next() == Some('|') => {
                    tokens.push(self.double(TokenKind::PipePipe))
                }
                '|' => tokens.push(self.single(TokenKind::Pipe)),
                ',' => tokens.push(self.single(TokenKind::Comma)),
                '.' => tokens.push(self.single(TokenKind::Dot)),
                '<' if self.peek_next() == Some('=') => tokens.push(self.double(TokenKind::LessEq)),
                '<' => tokens.push(self.single(TokenKind::Less)),
                '>' if self.peek_next() == Some('=') => {
                    tokens.push(self.double(TokenKind::GreaterEq))
                }
                '>' => tokens.push(self.single(TokenKind::Greater)),
                _ => {
                    return Err(format!(
                        "{}:{}: unexpected character '{}'",
                        self.line, self.col, ch
                    ));
                }
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            line: self.line,
            col: self.col,
        });
        Ok(tokens)
    }

    fn lex_number(&mut self) -> Result<Token, String> {
        let line = self.line;
        let col = self.col;
        let mut value = String::new();
        while let Some(ch @ '0'..='9') = self.peek() {
            value.push(ch);
            self.advance();
        }
        if self.peek() == Some('.') && self.peek_next().is_some_and(|ch| ch.is_ascii_digit()) {
            value.push('.');
            self.advance();
            while let Some(ch @ '0'..='9') = self.peek() {
                value.push(ch);
                self.advance();
            }
            let value = value
                .parse()
                .map_err(|e| format!("{line}:{col}: invalid floating-point literal: {e}"))?;
            return Ok(Token {
                kind: TokenKind::Float(value),
                line,
                col,
            });
        }
        let value = value
            .parse()
            .map_err(|e| format!("{line}:{col}: invalid integer literal: {e}"))?;
        if matches!(self.peek(), Some('L' | 'l')) {
            self.advance();
        }
        Ok(Token {
            kind: TokenKind::Int(value),
            line,
            col,
        })
    }

    fn lex_string(&mut self) -> Result<Token, String> {
        let line = self.line;
        let col = self.col;
        self.advance();
        let mut value = String::new();
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance();
                    return Ok(Token {
                        kind: TokenKind::String(value),
                        line,
                        col,
                    });
                }
                '\\' => {
                    self.advance();
                    let escaped = self
                        .advance()
                        .ok_or_else(|| format!("{line}:{col}: unterminated string literal"))?;
                    value.push(match escaped {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        other => other,
                    });
                }
                _ => {
                    value.push(ch);
                    self.advance();
                }
            }
        }
        Err(format!("{line}:{col}: unterminated string literal"))
    }

    fn lex_char_as_string(&mut self) -> Result<Token, String> {
        let line = self.line;
        let col = self.col;
        self.advance();
        let ch = if self.peek() == Some('\\') {
            self.advance();
            let escaped = self
                .advance()
                .ok_or_else(|| format!("{line}:{col}: unterminated char literal"))?;
            match escaped {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\'' => '\'',
                '\\' => '\\',
                other => other,
            }
        } else {
            self.advance()
                .ok_or_else(|| format!("{line}:{col}: unterminated char literal"))?
        };
        if self.advance() != Some('\'') {
            return Err(format!("{line}:{col}: unterminated char literal"));
        }
        Ok(Token {
            kind: TokenKind::String(ch.to_string()),
            line,
            col,
        })
    }

    fn lex_ident(&mut self) -> Token {
        let line = self.line;
        let col = self.col;
        let mut value = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        let kind = match value.as_str() {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "var" => TokenKind::Var,
            "package" => TokenKind::Package,
            "native" => TokenKind::Native,
            "namespace" => TokenKind::Namespace,
            "using" => TokenKind::Using,
            "import" => TokenKind::Using,
            "async" => TokenKind::Async,
            "static" => TokenKind::Static,
            "const" => TokenKind::Const,
            "readonly" => TokenKind::Readonly,
            "virtual" | "override" | "abstract" | "private" | "protected" | "internal" => {
                TokenKind::Public
            }
            "mut" => TokenKind::Mut,
            "new" => TokenKind::New,
            "ref" => TokenKind::Ref,
            "struct" => TokenKind::Struct,
            "class" => TokenKind::Class,
            "interface" => TokenKind::Interface,
            "enum" => TokenKind::Enum,
            "public" => TokenKind::Public,
            "borrow" => TokenKind::Borrow,
            "move" => TokenKind::Move,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "throw" => TokenKind::Throw,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            "finally" => TokenKind::Finally,
            "switch" => TokenKind::Switch,
            "case" => TokenKind::Case,
            "default" => TokenKind::Default,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "foreach" => TokenKind::Foreach,
            "in" => TokenKind::In,
            "await" => TokenKind::Await,
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            "null" => TokenKind::Null,
            _ => TokenKind::Ident(value),
        };
        Token { kind, line, col }
    }

    fn single(&mut self, kind: TokenKind) -> Token {
        let token = Token {
            kind,
            line: self.line,
            col: self.col,
        };
        self.advance();
        token
    }

    fn double(&mut self, kind: TokenKind) -> Token {
        let token = Token {
            kind,
            line: self.line,
            col: self.col,
        };
        self.advance();
        self.advance();
        token
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn peek_n(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }
}
