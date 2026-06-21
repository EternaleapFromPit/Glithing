#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::*;
#[path = "tir/generics.rs"]
mod generics;
#[path = "tir/startup.rs"]
mod startup;
#[path = "tir/stmt_lowering.rs"]
mod stmt_lowering;
#[path = "tir/summary.rs"]
mod summary;
#[path = "tir/endpoints.rs"]
mod endpoints;
#[path = "tir/validation.rs"]
mod validation;
#[path = "tir/ownership.rs"]
mod ownership;
#[path = "tir/reflection.rs"]
mod reflection;
#[path = "tir/signatures.rs"]
mod signatures;
#[path = "tir/lowering.rs"]
mod lowering;
#[path = "tir/call_resolution.rs"]
mod call_resolution;
#[path = "tir/typesystem.rs"]
mod typesystem;

use self::call_resolution::*;
use self::lowering::*;
use self::endpoints::*;
use self::generics::*;
use self::ownership::*;
use self::reflection::*;
use self::typesystem::*;
use self::startup::*;
use self::stmt_lowering::*;
use self::summary::*;
use self::validation::*;
pub(crate) use self::signatures::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ownership {
    Copy,
    Owned,
    Borrowed,
    Shared,
    View,
    Moved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DropKind {
    None,
    Free,
    DropStruct,
    DropClass,
    DropDelegate,
    DropCollection,
    DropTask,
    DropException,
    ViewOnly,
    BorrowOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum IrType {
    Bool,
    Byte,
    Short,
    Int,
    Long,
    UInt,
    Double,
    Decimal,
    String,
    Void,
    Array(Box<IrType>),
    Ref(Box<IrType>),
    Struct(String),
    Class(String),
    Interface(String),
    List(Box<IrType>),
    Dictionary(Box<IrType>, Box<IrType>),
    Enumerable(Box<IrType>),
    Thread,
    Task(Box<IrType>),
    Nullable(Box<IrType>),
    Function {
        params: Vec<IrType>,
        return_type: Box<IrType>,
    },
    Exception,
    Weak(Box<IrType>),
    Unknown(String),
}

#[derive(Debug, Clone)]
pub(crate) struct TypedProgram {
    pub(crate) functions: Vec<TypedFunction>,
    pub(crate) types: Vec<TypedType>,
    pub(crate) generic_instantiations: Vec<GenericInstantiation>,
    pub(crate) endpoint_handlers: Vec<EndpointHandlerBinding>,
    pub(crate) startup: Option<TypedStartup>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedStartup {
    pub(crate) symbol: String,
    pub(crate) accepts_args: bool,
    pub(crate) returns_int: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedType {
    pub(crate) package_id: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) name: String,
    pub(crate) namespace: Vec<String>,
    pub(crate) generic_params: Vec<String>,
    pub(crate) symbol_id: usize,
    pub(crate) is_extension: bool,
    pub(crate) kind: TypeKind,
    pub(crate) bases: Vec<String>,
    pub(crate) fields: Vec<TypedBinding>,
    pub(crate) constructors: Vec<TypedFunction>,
    pub(crate) methods: Vec<TypedFunction>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedFunction {
    pub(crate) package_id: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) name: String,
    pub(crate) symbol: String,
    pub(crate) is_async: bool,
    pub(crate) generic_params: Vec<String>,
    pub(crate) is_extern: bool,
    pub(crate) required_params: usize,
    pub(crate) return_type: IrType,
    pub(crate) return_ownership: Ownership,
    pub(crate) params: Vec<TypedBinding>,
    pub(crate) locals: Vec<TypedBinding>,
    pub(crate) body: Vec<TypedStmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedBinding {
    pub(crate) name: String,
    pub(crate) ty: IrType,
    pub(crate) ownership: Ownership,
}

impl TypedBinding {
    pub(crate) fn drop_kind(&self) -> DropKind {
        drop_kind_for_type(&self.ty, &self.ownership)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TypedStmt {
    pub(crate) kind: TypedStmtKind,
}

#[derive(Debug, Clone)]
pub(crate) enum TypedStmtKind {
    Let {
        binding: TypedBinding,
        expr: TypedExpr,
    },
    Assign {
        name: String,
        expr: TypedExpr,
    },
    AssignTarget {
        target: TypedExpr,
        expr: TypedExpr,
    },
    Block(Vec<TypedStmt>),
    If {
        condition: TypedExpr,
        then_body: Vec<TypedStmt>,
        else_body: Vec<TypedStmt>,
    },
    Try {
        try_body: Vec<TypedStmt>,
        catch_name: Option<String>,
        catch_body: Vec<TypedStmt>,
        finally_body: Vec<TypedStmt>,
    },
    Switch {
        expr: TypedExpr,
        cases: Vec<TypedSwitchCase>,
        default: Vec<TypedStmt>,
    },
    While {
        condition: TypedExpr,
        body: Vec<TypedStmt>,
    },
    For {
        init: Option<Box<TypedStmt>>,
        condition: Option<TypedExpr>,
        increment: Option<Box<TypedStmt>>,
        body: Vec<TypedStmt>,
    },
    ForEach {
        item: TypedBinding,
        collection: TypedExpr,
        body: Vec<TypedStmt>,
    },
    Print(TypedExpr),
    Expr(TypedExpr),
    Return(Option<TypedExpr>),
    Throw(TypedExpr),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedSwitchCase {
    pub(crate) value: TypedExpr,
    pub(crate) body: Vec<TypedStmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedExpr {
    pub(crate) kind: TypedExprKind,
    pub(crate) ty: IrType,
    pub(crate) ownership: Ownership,
    pub(crate) drop_kind: DropKind,
}

#[derive(Debug, Clone)]
pub(crate) enum TypedExprKind {
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
    String(String),
    Var(String),
    FunctionSymbol(String),
    Move(String),
    NullableSome(Box<TypedExpr>),
    ArrayLiteral(Vec<TypedExpr>),
    NewArray {
        element_type: IrType,
        length: Option<Box<TypedExpr>>,
        values: Vec<TypedExpr>,
    },
    Index {
        target: Box<TypedExpr>,
        index: Box<TypedExpr>,
    },
    Field {
        target: Box<TypedExpr>,
        name: String,
    },
    IsPattern {
        expr: Box<TypedExpr>,
        binding: Option<TypedBinding>,
    },
    Throw(Box<TypedExpr>),
    Assign {
        target: Box<TypedExpr>,
        value: Box<TypedExpr>,
    },
    Call(TypedCall),
    NewObject {
        type_name: String,
        constructor: Option<String>,
        args: Vec<TypedExpr>,
        fields: Vec<TypedFieldInit>,
    },
    NewCollection(IrType),
    NewThread(String),
    Borrow {
        mutable: bool,
        name: String,
    },
    Await(Box<TypedExpr>),
    Lambda {
        params: Vec<String>,
        body: Box<TypedExpr>,
    },
    Conditional {
        condition: Box<TypedExpr>,
        when_true: Box<TypedExpr>,
        when_false: Box<TypedExpr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<TypedExpr>,
    },
    IncDec {
        target: Box<TypedExpr>,
        delta: i32,
        prefix: bool,
    },
    Binary {
        left: Box<TypedExpr>,
        op: BinaryOp,
        right: Box<TypedExpr>,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct TypedFieldInit {
    pub(crate) name: String,
    pub(crate) expr: TypedExpr,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedCall {
    pub(crate) kind: TypedCallKind,
    pub(crate) args: Vec<TypedExpr>,
    pub(crate) generic_args: Vec<IrType>,
}

#[derive(Debug, Clone)]
pub(crate) enum TypedCallKind {
    Function {
        name: String,
        symbol: String,
    },
    Method {
        target: Box<TypedExpr>,
        name: String,
        symbol: String,
        resolution: CallResolution,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallResolution {
    StaticFunction,
    InstanceMethod,
    CollectionMethod,
    TaskMethod,
    WeakMethod,
    EndpointHandlerRegistration {
        http_method: String,
        path: String,
        handler: String,
    },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenericInstantiation {
    pub(crate) name: String,
    pub(crate) args: Vec<IrType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EndpointHandlerBinding {
    pub(crate) http_method: String,
    pub(crate) path: String,
    pub(crate) function: String,
    pub(crate) return_type: IrType,
    pub(crate) response_type: IrType,
    pub(crate) ownership: Ownership,
    pub(crate) controller: Option<String>,
    pub(crate) constructor: Option<String>,
    pub(crate) constructor_params: Vec<IrType>,
    pub(crate) params: Vec<EndpointParameterBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EndpointParameterSource {
    Route,
    Query,
    Body,
    Form,
    Header,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EndpointParameterBinding {
    pub(crate) name: String,
    pub(crate) ty: IrType,
    pub(crate) source: EndpointParameterSource,
}

#[derive(Debug, Clone)]
struct FunctionSignature {
    package_id: Option<String>,
    visibility: Visibility,
    generic_params: Vec<String>,
    params: Vec<IrType>,
    param_ownerships: Vec<Ownership>,
    required_params: usize,
    params_element_type: Option<IrType>,
    return_type: IrType,
    return_ownership: Ownership,
    symbol: String,
    is_static: bool,
}

#[derive(Debug, Clone)]
struct FieldSignature {
    package_id: Option<String>,
    visibility: Visibility,
    ty: IrType,
    ownership: Ownership,
}

#[derive(Debug, Clone)]
struct DelegateSignature {
    full_name: String,
    generic_params: Vec<String>,
    params: Vec<TypeSyntax>,
    return_type: TypeSyntax,
}

#[derive(Default)]
struct TypeEnv {
    kinds: HashMap<String, TypeKind>,
    type_packages: HashMap<String, Option<String>>,
    type_visibilities: HashMap<String, Visibility>,
    bases: HashMap<String, Vec<String>>,
    type_generic_params: HashMap<String, Vec<String>>,
    static_fields: HashMap<(String, String), Expr>,
    enum_values: HashMap<(String, String), i64>,
    delegates: HashMap<String, DelegateSignature>,
    functions: HashMap<String, Vec<FunctionSignature>>,
    methods: HashMap<(String, String), Vec<FunctionSignature>>,
    extension_methods: HashMap<(String, String), Vec<FunctionSignature>>,
    constructors: HashMap<String, Vec<FunctionSignature>>,
    fields: HashMap<(String, String), FieldSignature>,
    field_order: HashMap<String, Vec<(String, FieldSignature)>>,
}

impl TypedProgram {
    pub(crate) fn lower(program: &Program, startup_object: Option<&str>) -> Result<Self, String> {
        let mut env = TypeEnv::default();
        for enum_def in &program.enums {
            env.kinds.insert(enum_def.name.clone(), TypeKind::Enum);
            env.type_packages
                .insert(enum_def.name.clone(), enum_def.package_id.clone());
            env.type_visibilities
                .insert(enum_def.name.clone(), enum_def.visibility);
            for (index, variant) in enum_def.variants.iter().enumerate() {
                env.enum_values.insert(
                    (enum_def.name.clone(), variant.name.clone()),
                    variant.value.unwrap_or(index as i64),
                );
            }
        }
        for ty in &program.types {
            env.kinds.insert(ty.name.clone(), ty.kind);
            env.type_packages
                .insert(ty.name.clone(), ty.package_id.clone());
            env.type_visibilities
                .insert(ty.name.clone(), ty.visibility);
            env.bases.insert(ty.name.clone(), ty.bases.clone());
            env.type_generic_params.insert(
                ty.name.clone(),
                ty.generic_params
                    .iter()
                    .map(|param| param.name.clone())
                    .collect(),
            );
        }
        populate_delegate_signatures(program, &mut env);
        populate_function_signatures(program, &mut env);
        populate_method_signatures(program, &mut env);
        populate_constructor_signatures(program, &mut env);
        for ty in &program.types {
            for field in &ty.fields {
                if field.is_static {
                    if let Some(initializer) = &field.initializer {
                        env.static_fields
                            .insert((ty.name.clone(), field.name.clone()), initializer.clone());
                    }
                    continue;
                }
                let field_type = type_syntax_to_ir(&field.ty, &env);
                let field_signature = FieldSignature {
                    package_id: ty.package_id.clone(),
                    visibility: field.visibility,
                    ty: field_type.clone(),
                    ownership: ownership_for_declared_type_syntax(&field.ty, &env),
                };
                env.fields
                    .insert((ty.name.clone(), field.name.clone()), field_signature.clone());
                env.field_order
                    .entry(ty.name.clone())
                    .or_default()
                    .push((field.name.clone(), field_signature));
            }
        }

        let types = program
            .types
            .iter()
            .enumerate()
            .map(|(index, ty)| lower_type(ty, index, &env))
            .collect::<Result<Vec<_>, _>>()?;
        let functions = program
            .functions
            .iter()
            .map(|function| lower_function(function, &env, &[], None, None))
            .collect::<Result<Vec<_>, _>>()?;

        let mut generic_instantiations = Vec::new();
        for function in &functions {
            collect_instantiation(&function.return_type, &mut generic_instantiations);
            for param in &function.params {
                collect_instantiation(&param.ty, &mut generic_instantiations);
            }
            for local in &function.locals {
                collect_instantiation(&local.ty, &mut generic_instantiations);
            }
        }
        for ty in &types {
            for field in &ty.fields {
                collect_instantiation(&field.ty, &mut generic_instantiations);
            }
            for constructor in &ty.constructors {
                for param in &constructor.params {
                    collect_instantiation(&param.ty, &mut generic_instantiations);
                }
            }
            for method in &ty.methods {
                collect_instantiation(&method.return_type, &mut generic_instantiations);
            }
        }
        collect_generic_call_instantiations(
            &functions,
            &types,
            &mut generic_instantiations,
        );
        let endpoint_handlers = collect_endpoint_handlers(program, &env)?;

        let startup = resolve_typed_startup(program, &types, &functions, startup_object)?;
        let typed = Self {
            functions,
            types,
            generic_instantiations,
            endpoint_handlers,
            startup,
        };
        typed.validate_visibility(&env)?;
        typed.validate_async_lowering()?;
        Ok(typed)
    }

    pub(crate) fn check_ownership(program: &Program) -> Result<(), String> {
        let env = TypeEnv::from_program(program);
        for function in &program.functions {
            OwnershipChecker::check_function(function, &env, Vec::new())?;
        }
        for ty in &program.types {
            for constructor in &ty.constructors {
                let mut params = vec![TypedBinding {
                    name: "this".to_string(),
                    ty: collection_this_type(ty),
                    ownership: Ownership::Borrowed,
                }];
                for param in &constructor.params {
                    let ty = type_syntax_to_ir(&param.ty, &env);
                    params.push(TypedBinding {
                        name: param.name.clone(),
                        ty,
                        ownership: ownership_for_declared_type_syntax(&param.ty, &env),
                    });
                }
                OwnershipChecker::check_body(
                    &format!("constructor {}", ty.name),
                    &constructor.body,
                    &env,
                    params,
                    &IrType::Void,
                    Ownership::Copy,
                )?;
            }
            for method in &ty.methods {
                OwnershipChecker::check_function(
                    method,
                    &env,
                    vec![TypedBinding {
                        name: "this".to_string(),
                        ty: collection_this_type(ty),
                        ownership: Ownership::Borrowed,
                    }],
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn ownership_summary(&self) -> String {
        let mut out = String::new();
        for function in &self.functions {
            out.push_str(&format!(
                "function {} returns {:?} {:?}\n",
                function.name, function.return_ownership, function.return_type
            ));
            if !function.generic_params.is_empty() {
                out.push_str(&format!(
                    "  generic params {:?}\n",
                    function.generic_params
                ));
            }
            for param in &function.params {
                out.push_str(&format!(
                    "  param {}: {:?} {:?} drop={:?}\n",
                    param.name,
                    param.ownership,
                    param.ty,
                    param.drop_kind()
                ));
            }
            for local in &function.locals {
                out.push_str(&format!(
                    "  local {}: {:?} {:?} drop={:?}\n",
                    local.name,
                    local.ownership,
                    local.ty,
                    local.drop_kind()
                ));
            }
            out.push_str(&format!("  typed body stmts={}\n", function.body.len()));
            summarize_typed_stmts(&function.body, "  ", &mut out);
        }
        for ty in &self.types {
            out.push_str(&format!("type {} {:?}\n", ty.name, ty.kind));
            for field in &ty.fields {
                out.push_str(&format!(
                    "  field {}: {:?} {:?} drop={:?}\n",
                    field.name,
                    field.ownership,
                    field.ty,
                    field.drop_kind()
                ));
            }
            for constructor in &ty.constructors {
                out.push_str(&format!(
                    "  constructor {} params={}\n",
                    constructor.symbol,
                    constructor.params.len().saturating_sub(1)
                ));
            }
            for method in &ty.methods {
                out.push_str(&format!(
                    "  method {} returns {:?} {:?}\n",
                    method.name, method.return_ownership, method.return_type
                ));
            }
        }
        for instantiation in &self.generic_instantiations {
            out.push_str(&format!(
                "generic {}<{:?}>\n",
                instantiation.name, instantiation.args
            ));
        }
        for endpoint in &self.endpoint_handlers {
            out.push_str(&format!(
                "endpoint {} {} -> {} returns {:?} {:?}\n",
                endpoint.http_method,
                endpoint.path,
                endpoint.function,
                endpoint.ownership,
                endpoint.return_type
            ));
        }
        out
    }

    fn validate_visibility(&self, env: &TypeEnv) -> Result<(), String> {
        for function in &self.functions {
            validate_typed_function_visibility(function, env)?;
        }
        for ty in &self.types {
            for field in &ty.fields {
                ensure_type_accessible(
                    env,
                    &field.ty,
                    ty.package_id.as_deref(),
                    &format!("type '{}.{}'", ty.name, field.name),
                )?;
            }
            for base in &ty.bases {
                if let Some((package_id, visibility)) =
                    env.lookup_type_visibility(base, ty.package_id.as_deref())
                {
                    if !visibility_allows_access(
                        visibility,
                        package_id.as_deref(),
                        ty.package_id.as_deref(),
                    ) {
                        return Err(format!(
                            "type '{}' is not public in package '{}' and cannot be used from '{}'",
                            base,
                            package_id.as_deref().unwrap_or("<root>"),
                            ty.package_id.as_deref().unwrap_or("<root>")
                        ));
                    }
                }
            }
            for constructor in &ty.constructors {
                validate_typed_function_visibility(constructor, env)?;
            }
            for method in &ty.methods {
                validate_typed_function_visibility(method, env)?;
            }
        }
        Ok(())
    }

    pub(crate) fn validate_async_lowering(&self) -> Result<(), String> {
        for function in &self.functions {
            validate_async_function(function)?;
        }
        for ty in &self.types {
            for constructor in &ty.constructors {
                validate_async_function(constructor)?;
            }
            for method in &ty.methods {
                validate_async_function(method)?;
            }
        }
        Ok(())
    }
}

pub(crate) fn ownership_for_type(ty: &IrType) -> Ownership {
    match ty {
        IrType::Bool
        | IrType::Byte
        | IrType::Short
        | IrType::Int
        | IrType::Long
        | IrType::UInt
        | IrType::Double
        | IrType::Decimal
        | IrType::Void
        | IrType::Weak(_) => Ownership::Copy,
        IrType::Function { .. } | IrType::Nullable(_) => Ownership::Shared,
        IrType::Ref(_) => Ownership::Borrowed,
        IrType::Enumerable(_) => Ownership::View,
        IrType::Class(_) | IrType::Interface(_) => Ownership::Shared,
        IrType::String
        | IrType::Array(_)
        | IrType::Struct(_)
        | IrType::List(_)
        | IrType::Dictionary(_, _)
        | IrType::Thread
        | IrType::Task(_)
        | IrType::Exception => Ownership::Owned,
        IrType::Unknown(_) => Ownership::Shared,
    }
}

pub(crate) fn drop_kind_for_type(ty: &IrType, ownership: &Ownership) -> DropKind {
    match ownership {
        Ownership::Copy => DropKind::None,
        Ownership::Borrowed => DropKind::BorrowOnly,
        Ownership::View => DropKind::ViewOnly,
        Ownership::Moved => DropKind::None,
        Ownership::Shared => match ty {
            IrType::Class(_) | IrType::Interface(_) | IrType::Nullable(_) => DropKind::DropClass,
            IrType::Function { .. } => DropKind::DropDelegate,
            _ => DropKind::None,
        },
        Ownership::Owned => match ty {
            IrType::String | IrType::Array(_) => DropKind::Free,
            IrType::Struct(_) => DropKind::DropStruct,
            IrType::Class(_) | IrType::Interface(_) => DropKind::DropClass,
            IrType::List(_) | IrType::Dictionary(_, _) => DropKind::DropCollection,
            IrType::Task(_) => DropKind::DropTask,
            IrType::Exception => DropKind::DropException,
            _ => DropKind::None,
        },
    }
}
