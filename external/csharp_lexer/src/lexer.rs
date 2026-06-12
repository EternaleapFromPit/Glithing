// src/lexer.rs
// Simple C# lexer implementation for Glang.
// This lexer is a minimal prototype covering identifiers, integer literals,
// string literals, a small set of operators and punctuators, and EOF.

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum TokenKind {
    Identifier = 0,
    Keyword = 1,
    IntLiteral = 2,
    StringLiteral = 3,
    CharLiteral = 4,
    Operator = 5,
    Punctuator = 6,
    Comment = 7,
    RawString = 8,
    InterpolatedString = 9,
    VerbatimIdentifier = 10,
    EOF = 11,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_keyword(ident: &str) -> bool {
        matches!(ident, "int" | "string" | "char" | "bool" | "if" | "else" | "while" | "for" | "return" | "public" | "private" | "class" | "struct" | "enum" | "using" | "namespace" | "void")
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start_line = self.line;
        let start_col = self.column;
        match self.peek() {
            Some('@') => {
                // Verbatim identifier (e.g., @class)
                self.advance(); // consume '@'
                // Next characters follow identifier rules
                let mut ident = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                // Verbatim identifiers are always identifiers (keywords are not recognized after @)
                Token { kind: TokenKind::VerbatimIdentifier, lexeme: ident, line: start_line, column: start_col }
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                // Identifier or keyword
                let mut ident = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                let kind = if Self::is_keyword(&ident) {
                    TokenKind::Keyword
                } else {
                    TokenKind::Identifier
                };
                Token { kind, lexeme: ident, line: start_line, column: start_col }
            }
            // Single-line comment // ...
            Some('/') if matches!(self.source.get(self.pos + 1), Some('/') ) => {
                // Consume //
                self.advance(); // '/'
                self.advance(); // second '/'
                let mut comment = String::new();
                while let Some(ch) = self.peek() {
                    if ch == '\n' { break; }
                    comment.push(ch);
                    self.advance();
                }
                Token { kind: TokenKind::Comment, lexeme: comment, line: start_line, column: start_col }
            }
            // Multi-line comment /* ... */
            Some('/') if matches!(self.source.get(self.pos + 1), Some('*') ) => {
                // Consume /*
                self.advance(); // '/'
                self.advance(); // '*'
                let mut comment = String::new();
                while let Some(ch) = self.peek() {
                    if ch == '*' && matches!(self.source.get(self.pos + 1), Some('/') ) {
                        self.advance(); // '*'
                        self.advance(); // '/'
                        break;
                    }
                    comment.push(ch);
                    self.advance();
                }
                Token { kind: TokenKind::Comment, lexeme: comment, line: start_line, column: start_col }
            }
            // Raw string literal """
            Some('"') if matches!(self.source.get(self.pos + 1), Some('"')) && matches!(self.source.get(self.pos + 2), Some('"')) => {
                // Consume three quotes
                self.advance(); self.advance(); self.advance();
                let mut raw = String::new();
                while let Some(_) = self.peek() {
                    if matches!(self.source.get(self.pos), Some('"')) && matches!(self.source.get(self.pos + 1), Some('"')) && matches!(self.source.get(self.pos + 2), Some('"')) {
                        // End of raw string
                        self.advance(); self.advance(); self.advance();
                        break;
                    }
                    if let Some(ch) = self.peek() {
                        raw.push(ch);
                        self.advance();
                    }
                }
                Token { kind: TokenKind::RawString, lexeme: raw, line: start_line, column: start_col }
            }
            // Interpolated string literal starting with $"
            Some(c) if c == '$' => {
                // Look ahead for a '"'
                if let Some('"') = self.source.get(self.pos + 1).copied() {
                    // Consume $ and opening quote
                    self.advance(); // '$'
                    self.advance(); // '"'
                    let mut lit = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == '"' {
                            self.advance(); // closing quote
                            break;
                        } else {
                            lit.push(ch);
                            self.advance();
                        }
                    }
                    Token { kind: TokenKind::InterpolatedString, lexeme: lit, line: start_line, column: start_col }
                } else {
                    // Fallback to operator '$'
                    let op = c.to_string();
                    self.advance();
                    Token { kind: TokenKind::Operator, lexeme: op, line: start_line, column: start_col }
                }
            }
            Some(c) if c.is_ascii_digit() => {
                // Integer literal (decimal only)
                let mut num = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        num.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                Token { kind: TokenKind::IntLiteral, lexeme: num, line: start_line, column: start_col }
            }
            Some('"') => {
                // String literal
                self.advance(); // consume opening quote
                let mut lit = String::new();
                while let Some(ch) = self.peek() {
                    if ch == '"' {
                        self.advance(); // consume closing quote
                        break;
                    } else {
                        // handle escaped quotes minimally
                        if ch == '\\' {
                            self.advance();
                            if let Some(esc) = self.peek() {
                                lit.push('\\');
                                lit.push(esc);
                                self.advance();
                            }
                        } else {
                            lit.push(ch);
                            self.advance();
                        }
                    }
                }
                Token { kind: TokenKind::StringLiteral, lexeme: lit, line: start_line, column: start_col }
            }
            Some(c) if "(){}[];,.".contains(c) => {
                let punct = c.to_string();
                self.advance();
                Token { kind: TokenKind::Punctuator, lexeme: punct, line: start_line, column: start_col }
            }
            Some(c) => {
                // Try to match multi-char operators first
                let two = {
                    let mut s = String::new();
                    s.push(c);
                    if let Some(next) = self.source.get(self.pos + 1).copied() {
                        s.push(next);
                    }
                    s
                };
                let op = match two.as_str() {
                    "==" | "!=" | "<=" | ">=" | "&&" | "||" => {
                        self.advance();
                        self.advance();
                        Some(two)
                    }
                    _ => None,
                };
                if let Some(op_str) = op {
                    return Token { kind: TokenKind::Operator, lexeme: op_str, line: start_line, column: start_col };
                }
                // Single char operators
                let op_chars = "+-*/%=&|!<>";
                if op_chars.contains(c) {
                    let o = c.to_string();
                    self.advance();
                    return Token { kind: TokenKind::Operator, lexeme: o, line: start_line, column: start_col };
                }
                // Unknown character – treat as punctuator for safety
                let unk = c.to_string();
                self.advance();
                Token { kind: TokenKind::Punctuator, lexeme: unk, line: start_line, column: start_col }
            }
            None => Token { kind: TokenKind::EOF, lexeme: String::new(), line: start_line, column: start_col },
        }
    }
}

// Helper to convert Rust Token into the C‑compatible representation defined in System.CSharp.gl
#[repr(C)]
pub struct CToken {
    pub kind: c_int,
    pub lexeme: *mut c_char,
    pub line: c_int,
    pub column: c_int,
}

impl From<Token> for CToken {
    fn from(tok: Token) -> Self {
        let c_str = CString::new(tok.lexeme).unwrap_or_default();
        CToken {
            kind: tok.kind as c_int,
            lexeme: c_str.into_raw(),
            line: tok.line as c_int,
            column: tok.column as c_int,
        }
    }
}

// Exported functions will be defined in lib.rs.
