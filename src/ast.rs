#![allow(dead_code)]

#[derive(Debug, Clone)]
pub(crate) struct Program {
    pub(crate) package_id: Option<String>,
    pub(crate) native_c: Vec<String>,
    pub(crate) enums: Vec<EnumDef>,
    pub(crate) delegates: Vec<DelegateDef>,
    pub(crate) types: Vec<TypeDef>,
    pub(crate) functions: Vec<Function>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Visibility {
    Public,
    Internal,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumDef {
    pub(crate) package_id: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) namespace: Vec<String>,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) name: String,
    pub(crate) variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumVariant {
    pub(crate) name: String,
    pub(crate) value: Option<i64>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypeDef {
    pub(crate) package_id: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) kind: TypeKind,
    pub(crate) is_extension: bool,
    pub(crate) namespace: Vec<String>,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) name: String,
    pub(crate) generic_params: Vec<GenericParam>,
    pub(crate) bases: Vec<String>,
    pub(crate) fields: Vec<FieldDef>,
    pub(crate) constructors: Vec<Constructor>,
    pub(crate) methods: Vec<Function>,
}

#[derive(Debug, Clone)]
pub(crate) struct DelegateDef {
    pub(crate) package_id: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) namespace: Vec<String>,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) name: String,
    pub(crate) generic_params: Vec<GenericParam>,
    pub(crate) return_type: TypeSyntax,
    pub(crate) params: Vec<Param>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TypeKind {
    Struct,
    RefStruct,
    Class,
    Interface,
    Enum,
}

#[derive(Debug, Clone)]
pub(crate) struct FieldDef {
    pub(crate) visibility: Visibility,
    pub(crate) name: String,
    pub(crate) ty: TypeSyntax,
    pub(crate) is_static: bool,
    pub(crate) initializer: Option<Expr>,
}

#[derive(Debug, Clone)]
pub(crate) struct Constructor {
    pub(crate) visibility: Visibility,
    pub(crate) namespace: Vec<String>,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) params: Vec<Param>,
    pub(crate) body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct Function {
    pub(crate) package_id: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) namespace: Vec<String>,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) is_async: bool,
    pub(crate) is_extern: bool,
    pub(crate) is_static: bool,
    pub(crate) is_extension: bool,
    pub(crate) name: String,
    pub(crate) generic_params: Vec<GenericParam>,
    pub(crate) params: Vec<Param>,
    pub(crate) return_type: TypeSyntax,
    pub(crate) body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct GenericParam {
    pub(crate) name: String,
    pub(crate) constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct Attribute {
    pub(crate) name: String,
    pub(crate) args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub(crate) struct Param {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) name: String,
    pub(crate) ty: TypeSyntax,
    pub(crate) modifier: ParamModifier,
    pub(crate) is_params: bool,
    pub(crate) default: Option<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParamModifier {
    None,
    Ref,
    Out,
    In,
    This,
}

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Let {
        name: String,
        mutable: bool,
        declared_type: Option<TypeSyntax>,
        expr: Expr,
    },
    Assign {
        name: String,
        expr: Expr,
    },
    AssignTarget {
        target: Expr,
        expr: Expr,
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    Try {
        try_body: Vec<Stmt>,
        catch: Option<CatchClause>,
        finally_body: Vec<Stmt>,
    },
    Switch {
        expr: Expr,
        cases: Vec<SwitchCase>,
        default: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Box<Stmt>>,
        body: Vec<Stmt>,
    },
    ForEach {
        item_type: TypeSyntax,
        item_name: String,
        collection: Expr,
        body: Vec<Stmt>,
    },
    Print(Expr),
    Expr(Expr),
    Return(Option<Expr>),
    Throw(Expr),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub(crate) struct CatchClause {
    pub(crate) exception_type: Option<TypeSyntax>,
    pub(crate) name: Option<String>,
    pub(crate) body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct SwitchCase {
    pub(crate) value: Expr,
    pub(crate) body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
    String(String),
    Var(String),
    Move(String),
    ArrayLiteral(Vec<Expr>),
    NewArray {
        element_type: TypeSyntax,
        length: Option<Box<Expr>>,
        values: Vec<Expr>,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    Field {
        target: Box<Expr>,
        name: String,
    },
    IsPattern {
        expr: Box<Expr>,
        ty: TypeSyntax,
        name: Option<String>,
    },
    MethodCall {
        target: Box<Expr>,
        name: String,
        generic_args: Vec<TypeSyntax>,
        args: Vec<Expr>,
    },
    FunctionCall {
        name: String,
        generic_args: Vec<TypeSyntax>,
        args: Vec<Expr>,
    },
    Throw(Box<Expr>),
    NewObject {
        type_name: String,
        args: Vec<Expr>,
        fields: Vec<FieldInit>,
    },
    NewCollection(TypeSyntax),
    NewThread(String),
    Borrow {
        mutable: bool,
        name: String,
    },
    Await(Box<Expr>),
    Lambda {
        params: Vec<String>,
        body: LambdaBody,
    },
    Conditional {
        condition: Box<Expr>,
        when_true: Box<Expr>,
        when_false: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    IncDec {
        name: String,
        delta: i32,
        prefix: bool,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
    },
    NamedArg {
        name: String,
        expr: Box<Expr>,
    },
    RefArg {
        modifier: ParamModifier,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum LambdaBody {
    Expr(Box<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub(crate) struct FieldInit {
    pub(crate) name: String,
    pub(crate) expr: Expr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Coalesce,
    And,
    Or,
    BitAnd,
    BitOr,
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

impl BinaryOp {
    pub(crate) fn c_operator(self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Coalesce => "??",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::Eq => "==",
            BinaryOp::NotEq => "!=",
            BinaryOp::Less => "<",
            BinaryOp::LessEq => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterEq => ">=",
        }
    }

    pub(crate) fn is_comparison(self) -> bool {
        !matches!(
            self,
            BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Mod
                | BinaryOp::Coalesce
                | BinaryOp::BitAnd
                | BinaryOp::BitOr
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScalarType {
    Bool,
    Byte,
    Short,
    I32,
    I64,
    U32,
    F64,
    Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TypeSyntax {
    Scalar(ScalarType),
    String,
    Array(Box<TypeSyntax>),
    Ref(Box<TypeSyntax>),
    Named(String),
    GenericNamed { name: String, args: Vec<TypeSyntax> },
    List(Box<TypeSyntax>),
    Dictionary(Box<TypeSyntax>, Box<TypeSyntax>),
    IEnumerable(Box<TypeSyntax>),
    Thread,
    Task(Box<TypeSyntax>),
    Nullable(Box<TypeSyntax>),
    Void,
}
