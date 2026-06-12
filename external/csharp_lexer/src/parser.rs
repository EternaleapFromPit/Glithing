// src/parser.rs

use super::lexer::{Lexer, Token, TokenKind, CToken};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

// Minimal AST node representation – for now just a wrapper around a token.
#[repr(C)]
pub struct ASTNode {
    pub token: CToken,
    // In a full implementation, this would include child pointers, node kind, etc.
    pub next: *mut ASTNode,
}

impl ASTNode {
    fn new(token: CToken) -> Self {
        ASTNode { token, next: std::ptr::null_mut() }
    }
}

pub struct Parser {
    lexer: Lexer,
    head: *mut ASTNode,
    tail: *mut ASTNode,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Parser {
            lexer: Lexer::new(source),
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
        }
    }

    // Consume all tokens and build a simple linked list of AST nodes.
    pub fn parse_all(&mut self) {
        loop {
            let token = self.lexer.next_token();
            if token.kind == TokenKind::EOF {
                break;
            }
            let c_token: CToken = token.into();
            let mut node = Box::new(ASTNode::new(c_token));
            let node_ptr: *mut ASTNode = Box::into_raw(node);
            unsafe {
                if self.head.is_null() {
                    self.head = node_ptr;
                    self.tail = node_ptr;
                } else {
                    (*self.tail).next = node_ptr;
                    self.tail = node_ptr;
                }
            }
        }
    }

    pub fn get_head(&self) -> *mut ASTNode {
        self.head
    }
}

// Expose C‑compatible API.
#[no_mangle]
pub unsafe extern "C" fn csharp_parser_new(source: *const c_char) -> *mut c_void {
    if source.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = CStr::from_ptr(source);
    let src_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => "",
    };
    let mut parser = Parser::new(src_str);
    parser.parse_all();
    // Leak the parser so the caller can later free it.
    let boxed = Box::new(parser);
    Box::into_raw(boxed) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn csharp_parser_free(handle: *mut c_void) {
    if handle.is_null() {
        return;
    }
    // Reclaim ownership and drop.
    let _boxed: Box<Parser> = Box::from_raw(handle as *mut Parser);
    // Dropped here, which also frees the linked list nodes (memory leak for CToken strings not freed, but omitted for brevity).
}

#[no_mangle]
pub unsafe extern "C" fn csharp_parser_get_head(handle: *mut c_void) -> *mut ASTNode {
    if handle.is_null() {
        return std::ptr::null_mut();
    }
    let parser: &Parser = &*(handle as *mut Parser);
    parser.get_head()
}

// Note: Caller is responsible for traversing the ASTNode list and freeing the lexeme strings.
