use std::collections::HashMap;

use crate::ast::*;
mod helpers;
mod expressions;
mod regex_support;
mod statements;
mod xunit_support;
mod items;
mod declarations;
mod core;

use self::helpers::*;
use self::regex_support::*;
use self::xunit_support::*;

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
    visibility: Option<Visibility>,
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
}

fn default_visibility_for_package(package_id: Option<&String>) -> Visibility {
    if package_id.is_some() {
        Visibility::Public
    } else {
        Visibility::Internal
    }
}

