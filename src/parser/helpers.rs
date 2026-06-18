use super::*;

pub(super) fn token_to_string(tok: &Token) -> String {
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

pub(super) fn is_type_path(parts: &[String], expected: &[&str]) -> bool {
    parts.len() == expected.len() && parts.iter().zip(expected.iter()).all(|(a, b)| a == b)
}

pub(super) fn type_syntax_name(ty: &TypeSyntax) -> String {
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

pub(super) fn generic_type_definition_name(ty: &TypeSyntax) -> String {
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

pub(super) fn is_generic_type_syntax(ty: &TypeSyntax) -> bool {
    matches!(
        ty,
        TypeSyntax::GenericNamed { .. }
            | TypeSyntax::List(_)
            | TypeSyntax::Dictionary(_, _)
            | TypeSyntax::IEnumerable(_)
            | TypeSyntax::Task(_)
    )
}

pub(super) fn type_object_expr(ty: &TypeSyntax) -> Expr {
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

pub(super) fn generic_type_name_for_parser(name: &str, args: &[TypeSyntax]) -> String {
    format!(
        "{}<{}>",
        name,
        args.iter()
            .map(type_syntax_name)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(super) fn merge_type_declarations(types: Vec<TypeDef>) -> Vec<TypeDef> {
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

pub(super) fn is_task_like_type(ty: &TypeSyntax) -> bool {
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

pub(super) fn qualified_type_name(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", namespace.join("."), name)
    }
}

pub(super) fn default_expr_for_type(ty: &TypeSyntax) -> Expr {
    match ty {
        TypeSyntax::Scalar(ScalarType::Bool) => Expr::Bool(false),
        TypeSyntax::Scalar(_) => Expr::Int(0),
        TypeSyntax::Void => Expr::Null,
        _ => Expr::Null,
    }
}

pub(super) fn property_getter_name(name: &str) -> String {
    format!("get_{name}")
}

