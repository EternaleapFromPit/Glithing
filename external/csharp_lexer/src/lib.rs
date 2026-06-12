// src/lib.rs

use std::ffi::{CStr, CString};
mod parser;
use std::os::raw::{c_char, c_int, c_void};

mod lexer;
use lexer::{Lexer, Token, TokenKind, CToken};

#[no_mangle]
pub unsafe extern "C" fn csharp_lexer_new(source: *const c_char) -> *mut c_void {
    if source.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = CStr::from_ptr(source);
    let src_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => "",
    };
    let lexer = Lexer::new(src_str);
    // Box it to allocate on heap and leak pointer to C
    let boxed = Box::new(lexer);
    Box::into_raw(boxed) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn csharp_lexer_free(handle: *mut c_void) {
    if handle.is_null() {
        return;
    }
    // Reclaim ownership and drop
    let _boxed: Box<Lexer> = Box::from_raw(handle as *mut Lexer);
    // Dropped here
}

#[no_mangle]
pub unsafe extern "C" fn csharp_lexer_next(handle: *mut c_void) -> CToken {
    if handle.is_null() {
        // Return EOF token
        return CToken {
            kind: TokenKind::EOF as c_int,
            lexeme: std::ptr::null_mut(),
            line: 0,
            column: 0,
        };
    }
    let lexer: &mut Lexer = &mut *(handle as *mut Lexer);
    let token = lexer.next_token();
    CToken::from(token)
}

// Note: The caller is responsible for freeing the `lexeme` string returned in CToken.
// A helper function could be added, but omitted for brevity.
