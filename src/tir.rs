#![allow(dead_code)]

use std::collections::HashMap;

use crate::ast::*;

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

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

#[derive(Debug, Clone)]
pub(crate) struct TypedType {
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
    pub(crate) name: String,
    pub(crate) symbol: String,
    pub(crate) is_extern: bool,
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
    params: Vec<IrType>,
    param_ownerships: Vec<Ownership>,
    params_element_type: Option<IrType>,
    return_type: IrType,
    return_ownership: Ownership,
    symbol: String,
}

#[derive(Debug, Clone)]
struct FieldSignature {
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
    bases: HashMap<String, Vec<String>>,
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
    pub(crate) fn lower(program: &Program) -> Result<Self, String> {
        let mut env = TypeEnv::default();
        for enum_def in &program.enums {
            env.kinds.insert(enum_def.name.clone(), TypeKind::Enum);
            for (index, variant) in enum_def.variants.iter().enumerate() {
                env.enum_values.insert(
                    (enum_def.name.clone(), variant.name.clone()),
                    variant.value.unwrap_or(index as i64),
                );
            }
        }
        for ty in &program.types {
            env.kinds.insert(ty.name.clone(), ty.kind);
            env.bases.insert(ty.name.clone(), ty.bases.clone());
        }
        populate_delegate_signatures(program, &mut env);
        populate_function_signatures(program, &mut env);
        populate_method_signatures(program, &mut env);
        populate_constructor_signatures(program, &mut env);
        for ty in &program.types {
            for field in &ty.fields {
                let field_type = type_syntax_to_ir(&field.ty, &env);
                let field_signature = FieldSignature {
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
            .map(|function| lower_function(function, &env, &[], None))
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
        let endpoint_handlers = collect_endpoint_handlers(program, &env)?;

        Ok(Self {
            functions,
            types,
            generic_instantiations,
            endpoint_handlers,
        })
    }

    pub(crate) fn check_ownership(program: &Program) -> Result<(), String> {
        let env = TypeEnv::from_program(program);
        for function in &program.functions {
            OwnershipChecker::check_function(function, &env, Vec::new())?;
        }
        for ty in &program.types {
            for constructor in &ty.constructors {
                let this_type = match ty.kind {
                    TypeKind::Class | TypeKind::Interface => IrType::Class(ty.name.clone()),
                    _ => IrType::Ref(Box::new(IrType::Struct(ty.name.clone()))),
                };
                let mut params = vec![TypedBinding {
                    name: "this".to_string(),
                    ty: this_type,
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
                let this_type = match ty.kind {
                    TypeKind::Class | TypeKind::Interface => IrType::Class(ty.name.clone()),
                    _ => IrType::Ref(Box::new(IrType::Struct(ty.name.clone()))),
                };
                OwnershipChecker::check_function(
                    method,
                    &env,
                    vec![TypedBinding {
                        name: "this".to_string(),
                        ty: this_type,
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
}

fn populate_function_signatures(program: &Program, env: &mut TypeEnv) {
    let mut counts = HashMap::<String, usize>::new();
    for function in &program.functions {
        *counts.entry(function.name.clone()).or_default() += 1;
    }
    for function in &program.functions {
        let overloaded = counts.get(&function.name).copied().unwrap_or_default() > 1;
        let signature = function_signature(function, env, overloaded);
        env.functions
            .entry(function.name.clone())
            .or_default()
            .push(signature);
    }
}

fn populate_delegate_signatures(program: &Program, env: &mut TypeEnv) {
    for delegate in &program.delegates {
        let signature = DelegateSignature {
            full_name: qualified_delegate_name(&delegate.namespace, &delegate.name),
            generic_params: delegate
                .generic_params
                .iter()
                .map(|param| param.name.clone())
                .collect(),
            params: delegate.params.iter().map(|param| param.ty.clone()).collect(),
            return_type: delegate.return_type.clone(),
        };
        env.delegates
            .entry(signature.full_name.clone())
            .or_insert_with(|| signature.clone());
        env.delegates
            .entry(delegate.name.clone())
            .or_insert(signature);
    }
}

fn populate_method_signatures(program: &Program, env: &mut TypeEnv) {
    for (type_index, ty) in program.types.iter().enumerate() {
        for method in &ty.methods {
            let overloaded = ty
                .methods
                .iter()
                .filter(|candidate| candidate.name == method.name)
                .count()
                > 1;
            let params = method
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let param_ownerships = method
                .params
                .iter()
                .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
                .collect::<Vec<_>>();
            let suffix = params
                .iter()
                .map(ir_symbol_suffix)
                .collect::<Vec<_>>()
                .join("_");
            let base = format!(
                "{}_{}__g{}",
                qualified_type_symbol_name(
                    &ty.namespace,
                    &ty.name,
                    ty.generic_params.len(),
                    type_index,
                ),
                sanitize_ir_symbol(&method.name),
                method.generic_params.len()
            );
            // Preserve the C# source method name for lookup, but give each overload
            // a stable lowered symbol that later IR/codegen stages can call directly.
            let symbol = if overloaded {
                if suffix.is_empty() {
                    format!("{base}__overload")
                } else {
                    format!("{base}__{suffix}")
                }
            } else {
                base
            };
            let return_type = type_syntax_to_ir(&method.return_type, env);
            let return_ownership = ownership_for_declared_type_syntax(&method.return_type, env);
            let params_element_type = params_element_type(&method.params, env);
            let key = (ty.name.clone(), method.name.clone());
            let signature = FunctionSignature {
                params,
                param_ownerships,
                params_element_type,
                return_type,
                return_ownership,
                symbol,
            };
            if method.is_extension || ty.is_extension {
                env.extension_methods.entry(key).or_default().push(signature);
            } else {
                env.methods.entry(key).or_default().push(signature);
            }
        }
    }
}

fn populate_constructor_signatures(program: &Program, env: &mut TypeEnv) {
    for (type_index, ty) in program.types.iter().enumerate() {
        let overloaded = ty.constructors.len() > 1;
        for constructor in &ty.constructors {
            let params = constructor
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let param_ownerships = constructor
                .params
                .iter()
                .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
                .collect::<Vec<_>>();
            let suffix = params
                .iter()
                .map(ir_symbol_suffix)
                .collect::<Vec<_>>()
                .join("_");
            let base = format!(
                "{}_ctor",
                qualified_type_symbol_name(
                    &ty.namespace,
                    &ty.name,
                    ty.generic_params.len(),
                    type_index,
                )
            );
            let symbol = if overloaded {
                if suffix.is_empty() {
                    format!("{base}__empty")
                } else {
                    format!("{base}__{suffix}")
                }
            } else {
                base
            };
            let params_element_type = params_element_type(&constructor.params, env);
            env.constructors
                .entry(ty.name.clone())
                .or_default()
                .push(FunctionSignature {
                    params,
                    param_ownerships,
                    params_element_type,
                    return_type: IrType::Void,
                    return_ownership: Ownership::Copy,
                    symbol,
                });
        }
    }
}

impl TypeEnv {
    fn from_program(program: &Program) -> Self {
        let mut env = TypeEnv::default();
        for enum_def in &program.enums {
            env.kinds.insert(enum_def.name.clone(), TypeKind::Enum);
        }
        for ty in &program.types {
            env.kinds.insert(ty.name.clone(), ty.kind);
            env.bases.insert(ty.name.clone(), ty.bases.clone());
        }
        populate_delegate_signatures(program, &mut env);
        populate_function_signatures(program, &mut env);
        populate_method_signatures(program, &mut env);
        populate_constructor_signatures(program, &mut env);
        for ty in &program.types {
            for field in &ty.fields {
                let field_type = type_syntax_to_ir(&field.ty, &env);
                let field_signature = FieldSignature {
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
        env
    }

    fn single_function(&self, name: &str) -> Option<&FunctionSignature> {
        let signatures = self.functions.get(name)?;
        if signatures.len() == 1 {
            signatures.first()
        } else {
            None
        }
    }

    fn resolve_function(&self, name: &str, arg_types: &[IrType]) -> Option<&FunctionSignature> {
        let signatures = self.functions.get(name)?;
        resolve_signature(signatures, arg_types)
    }

    fn resolve_function_call(
        &self,
        name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<&FunctionSignature>, String> {
        let Some(signatures) = self.functions.get(name) else {
            return Ok(None);
        };
        resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("call to '{name}'"),
        )
    }

    fn resolve_method(
        &self,
        type_name: &str,
        method_name: &str,
        arg_types: &[IrType],
    ) -> Option<&FunctionSignature> {
        if let Some(signatures) = self
            .methods
            .get(&(type_name.to_string(), method_name.to_string()))
        {
            if let Some(sig) = resolve_signature(signatures, arg_types) {
                return Some(sig);
            }
        }
        self.bases.get(type_name)?.iter().find_map(|base| {
            let simple = base.rsplit('.').next().unwrap_or(base);
            self.resolve_method(simple, method_name, arg_types)
        })
    }

    fn resolve_method_call(
        &self,
        type_name: &str,
        method_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<&FunctionSignature>, String> {
        let resolve = |args: &[TypedExpr]| -> Result<Option<&FunctionSignature>, String> {
            if let Some(signatures) = self
                .methods
                .get(&(type_name.to_string(), method_name.to_string()))
            {
                if let Some(sig) = resolve_call_signature(
                    signatures,
                    args,
                    |expected, arg| ir_conversion_rank(expected, arg, self),
                    || format!("call to '{}.{}'", type_name, method_name),
                )? {
                    return Ok(Some(sig));
                }
            }
            Ok(None)
        };
        if let Some(sig) = resolve(args)? {
            return Ok(Some(sig));
        }
        if method_name.ends_with("Async") && args.len() > 1 {
            if let Some(sig) = resolve(&args[..args.len() - 1])? {
                return Ok(Some(sig));
            }
        }
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                if let Some(sig) = self.resolve_method_call(simple, method_name, args)? {
                    return Ok(Some(sig));
                }
            }
        }
        Ok(None)
    }

    fn resolve_extension_method_call(
        &self,
        type_name: &str,
        method_name: &str,
        receiver: &TypedExpr,
        args: &[TypedExpr],
    ) -> Result<Option<&FunctionSignature>, String> {
        let Some(signatures) = self
            .extension_methods
            .get(&(type_name.to_string(), method_name.to_string()))
        else {
            return Ok(None);
        };
        let mut combined = Vec::with_capacity(args.len() + 1);
        combined.push(receiver.clone());
        combined.extend(args.iter().cloned());
        resolve_call_signature(
            signatures,
            &combined,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("extension call to '{}.{}'", type_name, method_name),
        )
    }

    fn resolve_constructor(
        &self,
        type_name: &str,
        arg_types: &[IrType],
    ) -> Option<&FunctionSignature> {
        let signatures = self.constructors.get(type_name)?;
        resolve_signature(signatures, arg_types)
    }

    fn resolve_constructor_call(
        &self,
        type_name: &str,
        args: &[TypedExpr],
    ) -> Result<Option<&FunctionSignature>, String> {
        let Some(signatures) = self.constructors.get(type_name) else {
            return Ok(None);
        };
        resolve_call_signature(
            signatures,
            args,
            |expected, arg| ir_conversion_rank(expected, arg, self),
            || format!("constructor call '{type_name}'"),
        )
    }

    fn resolve_field(&self, type_name: &str, field_name: &str) -> Option<FieldSignature> {
        if let Some(ty) = self
            .fields
            .get(&(type_name.to_string(), field_name.to_string()))
        {
            return Some(ty.clone());
        }
        self.bases.get(type_name)?.iter().find_map(|base| {
            let simple = base.rsplit('.').next().unwrap_or(base);
            self.resolve_field(simple, field_name)
        })
    }

    fn all_fields(&self, type_name: &str) -> Vec<(String, IrType)> {
        self.all_field_infos(type_name)
            .into_iter()
            .map(|(name, field)| (name, field.ty))
            .collect()
    }

    fn all_field_infos(&self, type_name: &str) -> Vec<(String, FieldSignature)> {
        let mut fields = Vec::new();
        if let Some(bases) = self.bases.get(type_name) {
            for base in bases {
                let simple = base.rsplit('.').next().unwrap_or(base);
                fields.extend(self.all_field_infos(simple));
            }
        }
        if let Some(own_fields) = self.field_order.get(type_name) {
            for (name, field) in own_fields {
                if let Some(existing) = fields.iter_mut().find(|(field, _)| field == name) {
                    *existing = (name.clone(), field.clone());
                } else {
                    fields.push((name.clone(), field.clone()));
                }
            }
        }
        fields
    }
}

fn resolve_signature<'a>(
    signatures: &'a [FunctionSignature],
    arg_types: &[IrType],
) -> Option<&'a FunctionSignature> {
    let args = arg_types
        .iter()
        .map(|ty| TypedExpr {
            kind: TypedExprKind::Var("<type-only>".to_string()),
            ty: ty.clone(),
            ownership: ownership_for_type(ty),
            drop_kind: drop_kind_for_type(ty, &ownership_for_type(ty)),
        })
        .collect::<Vec<_>>();
    resolve_call_signature(
        signatures,
        &args,
        |expected, arg| {
            if ir_arg_matches(expected, &arg.ty) {
                Some(0)
            } else {
                None
            }
        },
        || "type-only overload lookup".to_string(),
    )
    .ok()
    .flatten()
}

fn ir_arg_matches(expected: &IrType, actual: &IrType) -> bool {
    expected == actual || matches!((expected, actual), (IrType::Int, IrType::Long))
}

fn resolve_call_signature<'a, F, C>(
    signatures: &'a [FunctionSignature],
    args: &[TypedExpr],
    rank: F,
    context: C,
) -> Result<Option<&'a FunctionSignature>, String>
where
    F: Fn(&IrType, &TypedExpr) -> Option<u16>,
    C: Fn() -> String,
{
    let mut applicable = Vec::<(&FunctionSignature, Vec<u16>)>::new();
    for signature in signatures {
        if signature.params.len() == args.len() {
            if let Some(ranks) = signature
                .params
                .iter()
                .zip(args.iter())
                .map(|(expected, arg)| rank(expected, arg))
                .collect::<Option<Vec<_>>>()
            {
                applicable.push((signature, ranks));
                continue;
            }
        }
        let Some(params_element_type) = &signature.params_element_type else {
            continue;
        };
        let fixed_len = signature.params.len().saturating_sub(1);
        if args.len() < fixed_len {
            continue;
        }
        let mut ranks = Vec::new();
        let mut applicable_signature = true;
        for (expected, arg) in signature
            .params
            .iter()
            .take(fixed_len)
            .zip(args.iter().take(fixed_len))
        {
            let Some(rank_value) = rank(expected, arg) else {
                applicable_signature = false;
                break;
            };
            ranks.push(rank_value);
        }
        if !applicable_signature {
            continue;
        }
        for arg in args.iter().skip(fixed_len) {
            let Some(rank_value) = rank(params_element_type, arg) else {
                applicable_signature = false;
                break;
            };
            ranks.push(rank_value);
        }
        if applicable_signature {
            applicable.push((signature, ranks));
        }
    }

    if applicable.is_empty() {
        return Ok(None);
    }

    let Some((best, best_ranks)) = applicable.iter().find(|(_, ranks)| {
        applicable.iter().all(|(_, other)| {
            ranks == other
                || ranks
                    .iter()
                    .zip(other.iter())
                    .all(|(candidate, other)| candidate <= other)
        })
    }) else {
        return Err(format!("ambiguous overload resolution for {}", context()));
    };

    let tied = applicable
        .iter()
        .filter(|(_, ranks)| {
            ranks == best_ranks
                || (ranks
                    .iter()
                    .zip(best_ranks.iter())
                    .all(|(candidate, best)| candidate <= best)
                    && best_ranks
                        .iter()
                        .zip(ranks.iter())
                        .all(|(best, candidate)| best <= candidate))
        })
        .count();
    if tied > 1 {
        return Err(format!("ambiguous overload resolution for {}", context()));
    }

    Ok(Some(*best))
}

fn ir_conversion_rank(expected: &IrType, arg: &TypedExpr, env: &TypeEnv) -> Option<u16> {
    if expected == &arg.ty {
        return Some(
            if matches!(arg.kind, TypedExprKind::Int(_)) && expected == &IrType::Long {
                1
            } else {
                0
            },
        );
    }
    if matches!(arg.kind, TypedExprKind::Int(_)) && expected == &IrType::Int {
        return Some(0);
    }
    if let Some(rank) = ir_numeric_conversion_rank(expected, &arg.ty, &arg.kind) {
        return Some(rank);
    }
    match (expected, &arg.ty) {
        (
            IrType::Class(_)
            | IrType::Interface(_)
            | IrType::Struct(_)
            | IrType::String
            | IrType::Array(_)
            | IrType::List(_)
            | IrType::Dictionary(_, _)
            | IrType::Enumerable(_)
            | IrType::Task(_),
            IrType::Unknown(actual),
        ) if actual == "null" => Some(50),
        (IrType::Class(expected), IrType::Class(actual))
        | (IrType::Interface(expected), IrType::Class(actual))
        | (IrType::Interface(expected), IrType::Struct(actual)) => {
            inheritance_distance(actual, expected, &env.bases).map(|distance| 10 + distance)
        }
        (_, IrType::Unknown(_)) | (IrType::Unknown(_), _) => Some(100),
        _ => None,
    }
}

fn ir_numeric_conversion_rank(
    expected: &IrType,
    actual: &IrType,
    actual_kind: &TypedExprKind,
) -> Option<u16> {
    if let TypedExprKind::Int(value) = actual_kind {
        return int_literal_conversion_rank(expected, *value);
    }
    match (actual, expected) {
        (IrType::Byte, IrType::Short) => Some(1),
        (IrType::Byte, IrType::Int) => Some(2),
        (IrType::Byte, IrType::UInt) => Some(2),
        (IrType::Byte, IrType::Long) => Some(3),
        (IrType::Byte, IrType::Double) | (IrType::Byte, IrType::Decimal) => Some(4),
        (IrType::Short, IrType::Int) => Some(1),
        (IrType::Short, IrType::Long) => Some(2),
        (IrType::Short, IrType::Double) | (IrType::Short, IrType::Decimal) => Some(3),
        (IrType::Int, IrType::Long) => Some(1),
        (IrType::Int, IrType::Double) | (IrType::Int, IrType::Decimal) => Some(2),
        (IrType::UInt, IrType::Long) => Some(1),
        (IrType::UInt, IrType::Double) | (IrType::UInt, IrType::Decimal) => Some(2),
        (IrType::Long, IrType::Double) | (IrType::Long, IrType::Decimal) => Some(1),
        _ => None,
    }
}

fn int_literal_conversion_rank(expected: &IrType, value: i64) -> Option<u16> {
    match expected {
        IrType::Int if i32::try_from(value).is_ok() => Some(0),
        IrType::Long => Some(1),
        IrType::Short if i16::try_from(value).is_ok() => Some(2),
        IrType::Byte if u8::try_from(value).is_ok() => Some(3),
        IrType::UInt if u32::try_from(value).is_ok() => Some(4),
        IrType::Double | IrType::Decimal => Some(5),
        _ => None,
    }
}

fn inheritance_distance(
    actual: &str,
    expected: &str,
    bases: &HashMap<String, Vec<String>>,
) -> Option<u16> {
    if actual == expected {
        return Some(0);
    }
    for base in bases.get(actual)? {
        if base == expected {
            return Some(1);
        }
        if let Some(distance) = inheritance_distance(base, expected, bases) {
            return Some(distance + 1);
        }
    }
    None
}

fn summarize_typed_stmts(stmts: &[TypedStmt], indent: &str, out: &mut String) {
    for stmt in stmts {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                out.push_str(&format!(
                    "{indent}tir let {}: {:?} {:?}\n",
                    binding.name, binding.ownership, binding.ty
                ));
                summarize_typed_expr(expr, indent, out);
            }
            TypedStmtKind::Assign { expr, .. }
            | TypedStmtKind::AssignTarget { expr, .. }
            | TypedStmtKind::Print(expr)
            | TypedStmtKind::Expr(expr)
            | TypedStmtKind::Throw(expr) => summarize_typed_expr(expr, indent, out),
            TypedStmtKind::Return(Some(expr)) => summarize_typed_expr(expr, indent, out),
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
            TypedStmtKind::Block(body) => summarize_typed_stmts(body, indent, out),
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                summarize_typed_expr(condition, indent, out);
                summarize_typed_stmts(then_body, indent, out);
                summarize_typed_stmts(else_body, indent, out);
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                summarize_typed_stmts(try_body, indent, out);
                summarize_typed_stmts(catch_body, indent, out);
                summarize_typed_stmts(finally_body, indent, out);
            }
            TypedStmtKind::Switch {
                expr,
                cases,
                default,
            } => {
                summarize_typed_expr(expr, indent, out);
                for case in cases {
                    summarize_typed_expr(&case.value, indent, out);
                    summarize_typed_stmts(&case.body, indent, out);
                }
                summarize_typed_stmts(default, indent, out);
            }
            TypedStmtKind::While { condition, body } => {
                summarize_typed_expr(condition, indent, out);
                summarize_typed_stmts(body, indent, out);
            }
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                if let Some(init) = init {
                    summarize_typed_stmts(std::slice::from_ref(init.as_ref()), indent, out);
                }
                if let Some(condition) = condition {
                    summarize_typed_expr(condition, indent, out);
                }
                if let Some(increment) = increment {
                    summarize_typed_stmts(std::slice::from_ref(increment.as_ref()), indent, out);
                }
                summarize_typed_stmts(body, indent, out);
            }
            TypedStmtKind::ForEach {
                collection, body, ..
            } => {
                summarize_typed_expr(collection, indent, out);
                summarize_typed_stmts(body, indent, out);
            }
        }
    }
}

fn summarize_typed_expr(expr: &TypedExpr, indent: &str, out: &mut String) {
    match &expr.kind {
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Function { name, symbol } => out.push_str(&format!(
                    "{indent}tir call function {} symbol={} -> {:?} {:?} drop={:?}\n",
                    name, symbol, expr.ownership, expr.ty, expr.drop_kind
                )),
                TypedCallKind::Method {
                    name,
                    symbol,
                    resolution,
                    ..
                } => out.push_str(&format!(
                    "{indent}tir call method {} symbol={} resolution={:?} -> {:?} {:?} drop={:?}\n",
                    name, symbol, resolution, expr.ownership, expr.ty, expr.drop_kind
                )),
            }
            for arg in &call.args {
                summarize_typed_expr(arg, indent, out);
            }
        }
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                summarize_typed_expr(value, indent, out);
            }
        }
        TypedExprKind::NewArray { length, values, .. } => {
            if let Some(length) = length {
                summarize_typed_expr(length, indent, out);
            }
            for value in values {
                summarize_typed_expr(value, indent, out);
            }
        }
        TypedExprKind::Index { target, index } => {
            summarize_typed_expr(target, indent, out);
            summarize_typed_expr(index, indent, out);
        }
        TypedExprKind::Field { target, .. }
        | TypedExprKind::IsPattern { expr: target, .. }
        | TypedExprKind::Await(target) => {
            summarize_typed_expr(target, indent, out);
        }
        TypedExprKind::Throw(expr) => {
            summarize_typed_expr(expr, indent, out);
        }
        TypedExprKind::Assign { target, value } => {
            summarize_typed_expr(target, indent, out);
            summarize_typed_expr(value, indent, out);
        }
        TypedExprKind::Lambda { body, .. } => {
            summarize_typed_expr(body, indent, out);
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            summarize_typed_expr(condition, indent, out);
            summarize_typed_expr(when_true, indent, out);
            summarize_typed_expr(when_false, indent, out);
        }
        TypedExprKind::Unary { expr, .. } => {
            summarize_typed_expr(expr, indent, out);
        }
        TypedExprKind::IncDec { .. } => {}
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                summarize_typed_expr(arg, indent, out);
            }
            for field in fields {
                summarize_typed_expr(&field.expr, indent, out);
            }
        }
        TypedExprKind::Binary { left, right, .. } => {
            summarize_typed_expr(left, indent, out);
            summarize_typed_expr(right, indent, out);
        }
        TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Borrow { .. } => {}
    }
}

#[derive(Debug, Clone)]
struct CheckedVar {
    ty: IrType,
    ownership: Ownership,
    scope_depth: usize,
    view_source: Option<String>,
}

#[derive(Debug, Clone)]
struct OwnershipState {
    scopes: Vec<Vec<String>>,
    vars: HashMap<String, CheckedVar>,
}

impl OwnershipState {
    fn new(params: Vec<TypedBinding>) -> Self {
        let mut state = Self {
            scopes: vec![Vec::new()],
            vars: HashMap::new(),
        };
        for param in params {
            state.declare(param.name, param.ty, param.ownership, None);
        }
        state
    }

    fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for name in scope {
                self.vars.remove(&name);
            }
        }
    }

    fn depth(&self) -> usize {
        self.scopes.len().saturating_sub(1)
    }

    fn declare(
        &mut self,
        name: String,
        ty: IrType,
        ownership: Ownership,
        view_source: Option<String>,
    ) {
        self.scopes.last_mut().unwrap().push(name.clone());
        self.vars.insert(
            name,
            CheckedVar {
                ty,
                ownership,
                scope_depth: self.depth(),
                view_source,
            },
        );
    }

    fn assign(
        &mut self,
        name: &str,
        ty: IrType,
        ownership: Ownership,
        view_source: Option<String>,
    ) -> Result<(), String> {
        let Some(existing) = self.vars.get_mut(name) else {
            return Ok(());
        };
        existing.ty = ty;
        existing.ownership = ownership;
        existing.view_source = view_source;
        Ok(())
    }

    fn get(&self, name: &str) -> Option<&CheckedVar> {
        self.vars.get(name)
    }
}

fn merge_checked_vars(left: &CheckedVar, right: &CheckedVar) -> CheckedVar {
    CheckedVar {
        ty: if left.ty == right.ty {
            left.ty.clone()
        } else if matches!(left.ty, IrType::Unknown(_)) {
            right.ty.clone()
        } else {
            left.ty.clone()
        },
        ownership: merge_ownership(&left.ownership, &right.ownership),
        scope_depth: left.scope_depth.min(right.scope_depth),
        view_source: if left.view_source == right.view_source {
            left.view_source.clone()
        } else if matches!(
            (&left.view_source, &right.view_source),
            (Some(_), Some(_))
        ) {
            left.view_source.clone().or(right.view_source.clone())
        } else {
            None
        },
    }
}

fn merge_ownership(left: &Ownership, right: &Ownership) -> Ownership {
    fn rank(value: &Ownership) -> u8 {
        match value {
            Ownership::Copy => 0,
            Ownership::Owned => 1,
            Ownership::Shared => 2,
            Ownership::View => 3,
            Ownership::Borrowed => 4,
            Ownership::Moved => 5,
        }
    }

    match rank(left).max(rank(right)) {
        0 => Ownership::Copy,
        1 => Ownership::Owned,
        2 => Ownership::Shared,
        3 => Ownership::View,
        4 => Ownership::Borrowed,
        _ => Ownership::Moved,
    }
}

#[derive(Debug, Clone)]
struct CheckedExpr {
    ty: IrType,
    ownership: Ownership,
    source_var: Option<String>,
    is_move: bool,
}

struct OwnershipChecker;

impl OwnershipChecker {
    fn check_function(
        function: &Function,
        env: &TypeEnv,
        implicit_params: Vec<TypedBinding>,
    ) -> Result<(), String> {
        let mut params = implicit_params;
        for param in &function.params {
            let ty = type_syntax_to_ir(&param.ty, env);
            params.push(TypedBinding {
                name: param.name.clone(),
                ownership: ownership_for_declared_type_syntax(&param.ty, env),
                ty,
            });
        }
        Self::check_body(
            &format!("function {}", function.name),
            &function.body,
            env,
            params,
            &type_syntax_to_ir(&function.return_type, env),
            ownership_for_declared_type_syntax(&function.return_type, env),
        )
    }

    fn check_body(
        context: &str,
        body: &[Stmt],
        env: &TypeEnv,
        params: Vec<TypedBinding>,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<(), String> {
        let mut state = OwnershipState::new(params);
        Self::check_stmts(context, body, env, &mut state, return_type, return_ownership)
    }

    fn check_stmts(
        context: &str,
        stmts: &[Stmt],
        env: &TypeEnv,
        state: &mut OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<(), String> {
        for stmt in stmts {
            Self::check_stmt(context, stmt, env, state, return_type, return_ownership.clone())?;
        }
        Ok(())
    }

    fn check_block_snapshot(
        context: &str,
        stmts: &[Stmt],
        env: &TypeEnv,
        state: &OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<OwnershipState, String> {
        let mut branch_state = state.clone();
        Self::check_stmts(
            context,
            stmts,
            env,
            &mut branch_state,
            return_type,
            return_ownership,
        )?;
        Ok(branch_state)
    }

    fn merge_branch_states(base: &OwnershipState, branches: &[OwnershipState]) -> OwnershipState {
        let mut merged = base.clone();
        for (name, base_var) in base.vars.iter() {
            let mut current = base_var.clone();
            for branch in branches {
                if let Some(branch_var) = branch.vars.get(name) {
                    current = merge_checked_vars(&current, branch_var);
                }
            }
            merged.vars.insert(name.clone(), current);
        }
        merged
    }

    fn check_stmt(
        context: &str,
        stmt: &Stmt,
        env: &TypeEnv,
        state: &mut OwnershipState,
        return_type: &IrType,
        return_ownership: Ownership,
    ) -> Result<(), String> {
        match stmt {
            Stmt::Let {
                name,
                declared_type,
                expr,
                ..
            } => {
                let expr = Self::check_expr(expr, env, state)?;
                let ty = declared_type
                    .as_ref()
                    .map(|ty| type_syntax_to_ir(ty, env))
                    .unwrap_or_else(|| expr.ty.clone());
                let expr = Self::coerce_for_target(&ty, expr);
                let target_ownership = declared_type
                    .as_ref()
                    .map(|declared| ownership_for_declared_type_syntax(declared, env))
                    .unwrap_or_else(|| ownership_for_type(&ty));
                Self::check_assignment_safety(context, env, &ty, target_ownership, &expr)?;
                Self::check_view_source_lifetime(context, state, name, &expr)?;
                let ownership = declared_type
                    .as_ref()
                    .map(|declared| ownership_for_declared_type_syntax(declared, env))
                    .unwrap_or_else(|| expr.ownership.clone());
                state.declare(name.clone(), ty, ownership, expr.source_var);
            }
            Stmt::Assign { name, expr } => {
                let expr = Self::check_expr(expr, env, state)?;
                let target_ty = state
                    .get(name)
                    .map(|var| var.ty.clone())
                    .unwrap_or_else(|| expr.ty.clone());
                let target_ownership = state
                    .get(name)
                    .map(|var| var.ownership.clone())
                    .unwrap_or_else(|| ownership_for_type(&target_ty));
                let expr = Self::coerce_for_target(&target_ty, expr);
                Self::check_assignment_safety(context, env, &target_ty, target_ownership, &expr)?;
                Self::check_view_source_lifetime(context, state, name, &expr)?;
                state.assign(name, target_ty, expr.ownership, expr.source_var)?;
            }
            Stmt::AssignTarget { target, expr } => {
                let (target_ty, target_ownership) = Self::target_type(target, env, state)
                    .unwrap_or_else(|| (IrType::Unknown("target".to_string()), Ownership::Shared));
                let expr = Self::check_expr(expr, env, state)?;
                let expr = Self::coerce_for_target(&target_ty, expr);
                Self::check_assignment_safety(context, env, &target_ty, target_ownership, &expr)?;
            }
            Stmt::Block(body) => {
                state.push_scope();
                Self::check_stmts(context, body, env, state, return_type, return_ownership.clone())?;
                state.pop_scope();
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                Self::check_expr(condition, env, state)?;
                let base = state.clone();
                let then_state = Self::check_block_snapshot(
                    context,
                    then_body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                let else_state = Self::check_block_snapshot(
                    context,
                    else_body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                *state = Self::merge_branch_states(&base, &[then_state, else_state]);
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                let base = state.clone();
                let try_state = Self::check_block_snapshot(
                    context,
                    try_body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                let mut branches = vec![try_state];
                if let Some(catch) = catch {
                    let mut catch_state = base.clone();
                    catch_state.push_scope();
                    if let Some(name) = &catch.name {
                        catch_state.declare(
                            name.clone(),
                            IrType::Exception,
                            Ownership::Borrowed,
                            None,
                        );
                    }
                    Self::check_stmts(
                        context,
                        &catch.body,
                        env,
                        &mut catch_state,
                        return_type,
                        return_ownership.clone(),
                    )?;
                    catch_state.pop_scope();
                    branches.push(catch_state);
                }
                let mut joined = Self::merge_branch_states(&base, &branches);
                joined.push_scope();
                Self::check_stmts(
                    context,
                    finally_body,
                    env,
                    &mut joined,
                    return_type,
                    return_ownership.clone(),
                )?;
                joined.pop_scope();
                *state = joined;
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                Self::check_expr(expr, env, state)?;
                let base = state.clone();
                let mut branches = Vec::new();
                for case in cases {
                    Self::check_expr(&case.value, env, state)?;
                    branches.push(Self::check_block_snapshot(
                        context,
                        &case.body,
                        env,
                        &base,
                        return_type,
                        return_ownership.clone(),
                    )?);
                }
                branches.push(Self::check_block_snapshot(
                    context,
                    default,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?);
                *state = Self::merge_branch_states(&base, &branches);
            }
            Stmt::While { condition, body } => {
                Self::check_expr(condition, env, state)?;
                let base = state.clone();
                let body_state = Self::check_block_snapshot(
                    context,
                    body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                *state = Self::merge_branch_states(&base, &[body_state]);
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                state.push_scope();
                if let Some(init) = init {
                    Self::check_stmt(context, init, env, state, return_type, return_ownership.clone())?;
                }
                if let Some(condition) = condition {
                    Self::check_expr(condition, env, state)?;
                }
                let base = state.clone();
                let mut body_state = Self::check_block_snapshot(
                    context,
                    body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                if let Some(increment) = increment {
                    Self::check_stmt(
                        context,
                        increment,
                        env,
                        &mut body_state,
                        return_type,
                        return_ownership.clone(),
                    )?;
                }
                *state = Self::merge_branch_states(&base, &[body_state]);
                state.pop_scope();
            }
            Stmt::ForEach {
                item_type,
                item_name,
                collection,
                body,
            } => {
                Self::check_expr(collection, env, state)?;
                let item_ty = type_syntax_to_ir(item_type, env);
                state.push_scope();
                state.declare(
                    item_name.clone(),
                    item_ty.clone(),
                    ownership_for_declared_type_syntax(item_type, env),
                    None,
                );
                let base = state.clone();
                let body_state = Self::check_block_snapshot(
                    context,
                    body,
                    env,
                    &base,
                    return_type,
                    return_ownership.clone(),
                )?;
                *state = Self::merge_branch_states(&base, &[body_state]);
                state.pop_scope();
            }
            Stmt::Print(expr) | Stmt::Expr(expr) | Stmt::Throw(expr) => {
                Self::check_expr(expr, env, state)?;
            }
            Stmt::Return(Some(expr)) => {
                let expr = Self::check_expr(expr, env, state)?;
                Self::check_assignment_safety(context, env, return_type, return_ownership.clone(), &expr)?;
                if matches!(expr.ownership, Ownership::View | Ownership::Borrowed)
                    && !matches!(return_ownership, Ownership::Borrowed | Ownership::View)
                {
                    if matches!(return_type, IrType::String) && expr.source_var.as_deref() == Some("this")
                    {
                        return Ok(());
                    }
                    return Err(format!(
                        "ownership checker: {context}: cannot return {:?} value into {:?} result without owning clone",
                        expr.ownership, return_ownership
                    ));
                }
                if matches!(expr.ownership, Ownership::View | Ownership::Borrowed) {
                    if let Some(source) = &expr.source_var {
                        let Some(source_var) = state.get(source) else {
                            return Ok(());
                        };
                        if source_var.scope_depth > 0 {
                            return Err(format!(
                                "ownership checker: {context}: returned value from '{source}' would outlive its source"
                            ));
                        }
                    }
                }
            }
            Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
        }
        Ok(())
    }

    fn check_expr(
        expr: &Expr,
        env: &TypeEnv,
        state: &OwnershipState,
    ) -> Result<CheckedExpr, String> {
        let checked = match expr {
            Expr::Int(_) => checked_temp(IrType::Long),
            Expr::Float(_) => checked_temp(IrType::Double),
            Expr::Bool(_) => checked_temp(IrType::Bool),
            Expr::Null => checked_temp(IrType::Unknown("null".to_string())),
            Expr::String(_) => CheckedExpr {
                ty: IrType::String,
                ownership: Ownership::Owned,
                source_var: None,
                is_move: false,
            },
            Expr::Var(name) => {
                if let Some(var) = state.get(name) {
                    CheckedExpr {
                        ty: var.ty.clone(),
                        ownership: var.ownership.clone(),
                        source_var: Some(name.clone()),
                        is_move: false,
                    }
                } else if let Some(signature) = env.single_function(name) {
                    CheckedExpr {
                        ty: IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        },
                        ownership: Ownership::Copy,
                        source_var: None,
                        is_move: false,
                    }
                } else {
                    CheckedExpr {
                        ty: IrType::Unknown(name.clone()),
                        ownership: Ownership::Shared,
                        source_var: Some(name.clone()),
                        is_move: false,
                    }
                }
            }
            Expr::Move(name) => {
                let Some(var) = state.get(name) else {
                    return Ok(CheckedExpr {
                        ty: IrType::Unknown(name.clone()),
                        ownership: Ownership::Moved,
                        source_var: Some(name.clone()),
                        is_move: true,
                    });
                };
                CheckedExpr {
                    ty: var.ty.clone(),
                    ownership: Ownership::Moved,
                    source_var: Some(name.clone()),
                    is_move: true,
                }
            }
            Expr::ArrayLiteral(values) => {
                for value in values {
                    Self::check_expr(value, env, state)?;
                }
                CheckedExpr {
                    ty: IrType::Array(Box::new(IrType::Long)),
                    ownership: Ownership::Owned,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::NewArray {
                element_type,
                length,
                values,
            } => {
                if let Some(length) = length {
                    Self::check_expr(length, env, state)?;
                }
                for value in values {
                    Self::check_expr(value, env, state)?;
                }
                CheckedExpr {
                    ty: IrType::Array(Box::new(type_syntax_to_ir(element_type, env))),
                    ownership: Ownership::Owned,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::Index { target, index } => {
                let target = Self::check_expr(target, env, state)?;
                Self::check_expr(index, env, state)?;
                let ty = match target.ty {
                    IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => {
                        *inner
                    }
                    IrType::Dictionary(_, value) => *value,
                    _ => IrType::Unknown("index".to_string()),
                };
                let ownership = if ty == IrType::String {
                    Ownership::Borrowed
                } else {
                    ownership_for_type(&ty)
                };
                CheckedExpr {
                    ty,
                    ownership,
                    source_var: target.source_var,
                    is_move: false,
                }
            }
            Expr::Field { target, name } => {
                let target = Self::check_expr(target, env, state)?;
                let field_info = match (&target.ty, name.as_str()) {
                    (IrType::Class(type_name), field) | (IrType::Struct(type_name), field) => {
                        env.resolve_field(type_name, field)
                    }
                    (IrType::List(_), "Count")
                    | (IrType::Dictionary(_, _), "Count")
                    | (IrType::Array(_), "Length" | "Count") => Some(FieldSignature {
                        ty: IrType::Int,
                        ownership: Ownership::Copy,
                    }),
                    (IrType::Task(inner), "Result") => Some(FieldSignature {
                        ty: inner.as_ref().clone(),
                        ownership: ownership_for_type(inner),
                    }),
                    (IrType::Task(_), "IsCompleted") => Some(FieldSignature {
                        ty: IrType::Bool,
                        ownership: Ownership::Copy,
                    }),
                    (IrType::Exception, "Message") => Some(FieldSignature {
                        ty: IrType::String,
                        ownership: Ownership::Borrowed,
                    }),
                    _ => None,
                };
                let (ty, ownership) = field_info
                    .map(|field| (field.ty, field.ownership))
                    .unwrap_or_else(|| {
                        let ty = IrType::Unknown(name.clone());
                        let ownership = ownership_for_type(&ty);
                        (ty, ownership)
                    });
                CheckedExpr {
                    ty,
                    ownership,
                    source_var: target.source_var,
                    is_move: false,
                }
            }
            Expr::IsPattern { expr, .. } => {
                Self::check_expr(expr, env, state)?;
                checked_temp(IrType::Bool)
            }
            Expr::Throw(expr) => {
                let inner = Self::check_expr(expr, env, state)?;
                CheckedExpr {
                    ty: inner.ty,
                    ownership: inner.ownership,
                    source_var: inner.source_var,
                    is_move: false,
                }
            }
            Expr::Assign { target, value } => {
                let target = Self::check_expr(target, env, state)?;
                let value = Self::check_expr(value, env, state)?;
                CheckedExpr {
                    ty: value.ty,
                    ownership: value.ownership,
                    source_var: target.source_var,
                    is_move: false,
                }
            }
            Expr::MethodCall { target, name, args } => {
                let target = Self::check_expr(target, env, state)?;
                let args = args
                    .iter()
                    .map(|arg| Self::check_expr(arg, env, state))
                    .collect::<Result<Vec<_>, _>>()?;
                let ty = match (&target.ty, name.as_str()) {
                    (IrType::Class(type_name), "MapGet" | "MapPost")
                        if type_name == "WebApplication" =>
                    {
                        IrType::Void
                    }
                    (IrType::Task(inner), "GetResult") => inner.as_ref().clone(),
                    (IrType::Task(inner), "GetAwaiter") => IrType::Task(inner.clone()),
                    (_, "Contains") | (_, "ContainsKey") | (_, "Remove") | (_, "TryGetValue") => IrType::Bool,
                    (_, "Add") | (_, "Clear") | (_, "Wait") => IrType::Void,
                    (IrType::Unknown(target) | IrType::Class(target), "Run")
                        if target == "Task" =>
                    {
                        args.first()
                            .map(|arg| IrType::Task(Box::new(arg.ty.clone())))
                            .unwrap_or_else(|| IrType::Task(Box::new(IrType::Void)))
                    }
                    _ => IrType::Unknown(name.clone()),
                };
                let ownership = if target.ty == IrType::Exception && matches!(name.as_str(), "Message")
                {
                    Ownership::Borrowed
                } else if let IrType::Class(type_name)
                | IrType::Struct(type_name)
                | IrType::Interface(type_name) = &target.ty
                {
                    env.resolve_method(type_name, name, &args.iter().map(|arg| arg.ty.clone()).collect::<Vec<_>>())
                        .map(|signature| signature.return_ownership.clone())
                        .unwrap_or_else(|| ownership_for_type(&ty))
                } else {
                    ownership_for_type(&ty)
                };
                CheckedExpr {
                    ownership,
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::FunctionCall { name, args } => {
                if name == "sizeof" {
                    return Ok(CheckedExpr {
                        ownership: Ownership::Copy,
                        ty: IrType::Int,
                        source_var: None,
                        is_move: false,
                    });
                }
                let checked_args = args
                    .iter()
                    .map(|arg| Self::check_expr(arg, env, state))
                    .collect::<Result<Vec<_>, _>>()?;
                let signature = env.resolve_function(
                    name,
                    checked_args
                        .iter()
                        .map(|arg| arg.ty.clone())
                        .collect::<Vec<_>>()
                        .as_slice(),
                );
                let (ty, ownership) = signature
                    .map(|signature| {
                        (
                            signature.return_type.clone(),
                            signature.return_ownership.clone(),
                        )
                    })
                    .unwrap_or_else(|| (IrType::Unknown(name.clone()), Ownership::Shared));
                CheckedExpr {
                    ownership,
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::NewObject {
                type_name,
                args,
                fields,
            } => {
                for arg in args {
                    Self::check_expr(arg, env, state)?;
                }
                for field in fields {
                    let expr = Self::check_expr(&field.expr, env, state)?;
                    let target_info = env
                        .resolve_field(type_name, &field.name)
                        .unwrap_or_else(|| FieldSignature {
                            ty: expr.ty.clone(),
                            ownership: ownership_for_type(&expr.ty),
                        });
                    let expr = Self::coerce_for_target(&target_info.ty, expr);
                    Self::check_assignment_safety(
                        &format!("object initializer {type_name}.{}", field.name),
                        env,
                        &target_info.ty,
                        target_info.ownership,
                        &expr,
                    )?;
                }
                let ty = match env.kinds.get(type_name) {
                    Some(TypeKind::Class) => IrType::Class(type_name.clone()),
                    _ if type_name == "Exception" || type_name == "System.Exception" => {
                        IrType::Exception
                    }
                    _ => IrType::Struct(type_name.clone()),
                };
                CheckedExpr {
                    ownership: ownership_for_type(&ty),
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::NewCollection(ty) => CheckedExpr {
                ty: type_syntax_to_ir(ty, env),
                ownership: Ownership::Owned,
                source_var: None,
                is_move: false,
            },
            Expr::NewThread(_) => CheckedExpr {
                ty: IrType::Thread,
                ownership: Ownership::Owned,
                source_var: None,
                is_move: false,
            },
            Expr::Borrow { name, .. } => {
                let ty = state
                    .get(name)
                    .map(|var| var.ty.clone())
                    .unwrap_or_else(|| IrType::Unknown(name.clone()));
                CheckedExpr {
                    ty: IrType::Ref(Box::new(ty)),
                    ownership: Ownership::Borrowed,
                    source_var: Some(name.clone()),
                    is_move: false,
                }
            }
            Expr::Await(inner) => {
                let inner = Self::check_expr(inner, env, state)?;
                let ty = match inner.ty {
                    IrType::Task(result) => *result,
                    other => IrType::Unknown(format!("await {other:?}")),
                };
                CheckedExpr {
                    ownership: ownership_for_type(&ty),
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::Unary { op, expr } => {
                let expr = Self::check_expr(expr, env, state)?;
                let ty = match op {
                    UnaryOp::Not => IrType::Bool,
                    UnaryOp::Neg => expr.ty,
                };
                CheckedExpr {
                    ty,
                    ownership: Ownership::Copy,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::IncDec { name, .. } => {
                let ty = if let Some(var) = state.get(name) {
                    var.ty.clone()
                } else if let Some(this_var) = state.get("this") {
                    let owner = match &this_var.ty {
                        IrType::Class(name) | IrType::Struct(name) => Some(name.as_str()),
                        IrType::Ref(inner) => match inner.as_ref() {
                            IrType::Struct(name) | IrType::Class(name) => Some(name.as_str()),
                            _ => None,
                        },
                        _ => None,
                    };
                    let field_ty = owner.and_then(|o| env.resolve_field(o, name));
                    field_ty
                        .map(|field| field.ty)
                        .unwrap_or_else(|| IrType::Unknown(name.clone()))
                } else {
                    IrType::Unknown(name.clone())
                };
                CheckedExpr {
                    ty,
                    ownership: Ownership::Copy,
                    source_var: Some(name.clone()),
                    is_move: false,
                }
            }
            Expr::Lambda { body, .. } => {
                Self::check_expr(body, env, state)?;
                CheckedExpr {
                    ty: IrType::Unknown("lambda".to_string()),
                    ownership: Ownership::Shared,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                Self::check_expr(condition, env, state)?;
                let when_true = Self::check_expr(when_true, env, state)?;
                let when_false = Self::check_expr(when_false, env, state)?;
                let ty = if when_true.ty == IrType::Unknown("null".to_string()) {
                    when_false.ty
                } else {
                    when_true.ty
                };
                CheckedExpr {
                    ownership: ownership_for_type(&ty),
                    ty,
                    source_var: None,
                    is_move: false,
                }
            }
            Expr::Binary { left, op, right } => {
                let left = Self::check_expr(left, env, state)?;
                Self::check_expr(right, env, state)?;
                let ty = if op.is_comparison() {
                    IrType::Bool
                } else {
                    left.ty
                };
                checked_temp(ty)
            }
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                Self::check_expr(expr, env, state)?
            }
        };
        Ok(checked)
    }

    fn target_type(
        target: &Expr,
        env: &TypeEnv,
        state: &OwnershipState,
    ) -> Option<(IrType, Ownership)> {
        match target {
            Expr::Var(name) => state.get(name).map(|var| (var.ty.clone(), var.ownership.clone())),
            Expr::Field { target, name } => {
                let target = Self::check_expr(target, env, state).ok()?;
                match target.ty {
                    IrType::Class(type_name) | IrType::Struct(type_name) => env
                        .resolve_field(&type_name, name)
                        .map(|field| (field.ty, field.ownership)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn check_assignment_safety(
        context: &str,
        env: &TypeEnv,
        target_ty: &IrType,
        target_ownership: Ownership,
        expr: &CheckedExpr,
    ) -> Result<(), String> {
        if matches!(expr.ownership, Ownership::View | Ownership::Borrowed)
            && !matches!(target_ownership, Ownership::Borrowed | Ownership::View)
            && !matches!(
                target_ty,
                IrType::Enumerable(_) | IrType::Ref(_) | IrType::String
            )
        {
            return Err(format!(
                "ownership checker: {context}: cannot store {:?} value into owned target {:?}",
                expr.ownership, target_ty
            ));
        }
        if matches!(target_ty, IrType::Class(_) | IrType::Interface(_))
            && expr.source_var.is_some()
            && !expr.is_move
            && !allows_shared_reference_flow(env, target_ty)
        {
            return Err(format!(
                "ownership checker: {context}: assigning class/interface value requires explicit move"
            ));
        }
        Ok(())
    }

    fn coerce_for_target(target_ty: &IrType, mut expr: CheckedExpr) -> CheckedExpr {
        match target_ty {
            IrType::Enumerable(_) if expr.source_var.is_some() => {
                expr.ty = target_ty.clone();
                expr.ownership = Ownership::View;
                expr.is_move = false;
                expr
            }
            IrType::Ref(_) if expr.source_var.is_some() => {
                expr.ty = target_ty.clone();
                expr.ownership = Ownership::Borrowed;
                expr.is_move = false;
                expr
            }
            _ => expr,
        }
    }

    fn check_view_source_lifetime(
        context: &str,
        state: &OwnershipState,
        target_name: &str,
        expr: &CheckedExpr,
    ) -> Result<(), String> {
        if !matches!(expr.ownership, Ownership::View | Ownership::Borrowed) {
            return Ok(());
        }
        let Some(source) = &expr.source_var else {
            return Ok(());
        };
        let Some(source_var) = state.get(source) else {
            return Ok(());
        };
        let target_depth = state
            .get(target_name)
            .map(|target| target.scope_depth)
            .unwrap_or_else(|| state.depth());
        if source_var.scope_depth > target_depth {
            return Err(format!(
                "ownership checker: {context}: '{target_name}' would outlive borrowed/view source '{source}'"
            ));
        }
        Ok(())
    }
}

fn checked_temp(ty: IrType) -> CheckedExpr {
    CheckedExpr {
        ownership: ownership_for_type(&ty),
        ty,
        source_var: None,
        is_move: false,
    }
}

fn function_signature(function: &Function, env: &TypeEnv, overloaded: bool) -> FunctionSignature {
            let params = function
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let param_ownerships = function
                .params
                .iter()
                .map(|param| ownership_for_declared_type_syntax(&param.ty, env))
                .collect::<Vec<_>>();
            let params_element_type = params_element_type(&function.params, env);
            FunctionSignature {
                symbol: if overloaded {
                    overloaded_function_symbol(&qualified_function_symbol_name(
                        &function.namespace,
                        &function.name,
            ), &params)
        } else {
            qualified_function_symbol_name(&function.namespace, &function.name)
                },
                params,
                param_ownerships,
                params_element_type,
                return_type: type_syntax_to_ir(&function.return_type, env),
                return_ownership: ownership_for_declared_type_syntax(&function.return_type, env),
            }
}

fn params_element_type(params: &[Param], env: &TypeEnv) -> Option<IrType> {
    let last = params.last()?;
    if !last.is_params {
        return None;
    }
    match type_syntax_to_ir(&last.ty, env) {
        IrType::Array(inner) => Some(*inner),
        other => Some(other),
    }
}

fn overloaded_function_symbol(name: &str, params: &[IrType]) -> String {
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    if suffix.is_empty() {
        format!("{name}__overload")
    } else {
        format!("{name}__{suffix}")
    }
}

fn qualified_function_symbol_name(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        sanitize_ir_symbol(name)
    } else {
        format!(
            "{}_{}",
            sanitize_ir_symbol(&namespace.join("_")),
            sanitize_ir_symbol(name)
        )
    }
}

fn qualified_type_symbol_name(
    namespace: &[String],
    name: &str,
    generic_arity: usize,
    type_id: usize,
) -> String {
    if namespace.is_empty() {
        format!("{}__g{}__t{}", sanitize_ir_symbol(name), generic_arity, type_id)
    } else {
        format!(
            "{}_{}__g{}__t{}",
            sanitize_ir_symbol(&namespace.join("_")),
            sanitize_ir_symbol(name),
            generic_arity,
            type_id
        )
    }
}

fn type_method_symbol(ty: &TypeDef, type_id: usize, method: &Function, env: &TypeEnv) -> String {
    let overloaded = ty
        .methods
        .iter()
        .filter(|candidate| candidate.name == method.name)
        .count()
        > 1;
    let params = method
        .params
        .iter()
        .map(|param| type_syntax_to_ir(&param.ty, env))
        .collect::<Vec<_>>();
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    let base = format!(
        "{}_{}__g{}",
        qualified_type_symbol_name(
            &ty.namespace,
            &ty.name,
            ty.generic_params.len(),
            type_id,
        ),
        sanitize_ir_symbol(&method.name),
        method.generic_params.len()
    );
    let base = if method.is_extension {
        format!("{base}__ext")
    } else {
        base
    };
    if overloaded {
        if suffix.is_empty() {
            format!("{base}__overload")
        } else {
            format!("{base}__{suffix}")
        }
    } else {
        base
    }
}

fn type_constructor_symbol(
    ty: &TypeDef,
    type_id: usize,
    constructor: &Constructor,
    env: &TypeEnv,
) -> String {
    let overloaded = ty.constructors.len() > 1;
    let params = constructor
        .params
        .iter()
        .map(|param| type_syntax_to_ir(&param.ty, env))
        .collect::<Vec<_>>();
    let suffix = params
        .iter()
        .map(ir_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    let base = format!(
        "{}_ctor",
        qualified_type_symbol_name(
            &ty.namespace,
            &ty.name,
            ty.generic_params.len(),
            type_id,
        )
    );
    if overloaded {
        if suffix.is_empty() {
            format!("{base}__empty")
        } else {
            format!("{base}__{suffix}")
        }
    } else {
        base
    }
}

fn ir_symbol_suffix(ty: &IrType) -> String {
    match ty {
        IrType::Bool => "bool".to_string(),
        IrType::Byte => "byte".to_string(),
        IrType::Short => "short".to_string(),
        IrType::Int => "int".to_string(),
        IrType::Long => "long".to_string(),
        IrType::UInt => "uint".to_string(),
        IrType::Double => "double".to_string(),
        IrType::Decimal => "decimal".to_string(),
        IrType::String => "string".to_string(),
        IrType::Void => "void".to_string(),
        IrType::Class(name)
        | IrType::Struct(name)
        | IrType::Interface(name)
        | IrType::Unknown(name) => sanitize_ir_symbol(name),
        IrType::Weak(inner) => format!("weak_{}", ir_symbol_suffix(inner)),
        IrType::Array(inner) => format!("array_{}", ir_symbol_suffix(inner)),
        IrType::Ref(inner) => format!("ref_{}", ir_symbol_suffix(inner)),
        IrType::List(inner) => format!("list_{}", ir_symbol_suffix(inner)),
        IrType::Dictionary(key, value) => {
            format!("dict_{}_{}", ir_symbol_suffix(key), ir_symbol_suffix(value))
        }
        IrType::Enumerable(inner) => format!("ienumerable_{}", ir_symbol_suffix(inner)),
        IrType::Thread => "thread".to_string(),
        IrType::Task(inner) => format!("task_{}", ir_symbol_suffix(inner)),
        IrType::Function { .. } => "fn".to_string(),
        IrType::Exception => "exception".to_string(),
    }
}

fn sanitize_ir_symbol(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

fn qualified_delegate_name(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", namespace.join("."), name)
    }
}

fn collect_instantiation(ty: &IrType, output: &mut Vec<GenericInstantiation>) {
    let instantiation = match ty {
        IrType::List(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "List".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Dictionary(key, value) => {
            collect_instantiation(key, output);
            collect_instantiation(value, output);
            Some(GenericInstantiation {
                name: "Dictionary".to_string(),
                args: vec![key.as_ref().clone(), value.as_ref().clone()],
            })
        }
        IrType::Enumerable(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "IEnumerable".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Task(inner) => {
            collect_instantiation(inner, output);
            Some(GenericInstantiation {
                name: "Task".to_string(),
                args: vec![inner.as_ref().clone()],
            })
        }
        IrType::Array(inner) | IrType::Ref(inner) | IrType::Weak(inner) => {
            collect_instantiation(inner, output);
            None
        }
        IrType::Function {
            params,
            return_type,
        } => {
            for param in params {
                collect_instantiation(param, output);
            }
            collect_instantiation(return_type, output);
            None
        }
        _ => None,
    };
    if let Some(instantiation) = instantiation {
        if !output.contains(&instantiation) {
            output.push(instantiation);
        }
    }
}

fn collect_endpoint_handlers(
    program: &Program,
    env: &TypeEnv,
) -> Result<Vec<EndpointHandlerBinding>, String> {
    let mut collector = EndpointHandlerCollector {
        env,
        scopes: vec![HashMap::new()],
        handlers: Vec::new(),
    };
    for function in &program.functions {
        collector.scopes.clear();
        collector.scopes.push(HashMap::new());
        for param in &function.params {
            let ty = type_syntax_to_ir(&param.ty, env);
            collector.declare(param.name.clone(), ty);
        }
        collector.collect_stmts(&function.body)?;
    }
    for ty in &program.types {
        for constructor in &ty.constructors {
            collector.scopes.clear();
            collector.scopes.push(HashMap::new());
            collector.declare("this".to_string(), IrType::Class(ty.name.clone()));
            for param in &constructor.params {
                collector.declare(param.name.clone(), type_syntax_to_ir(&param.ty, env));
            }
            collector.collect_stmts(&constructor.body)?;
        }
        for method in &ty.methods {
            collector.scopes.clear();
            collector.scopes.push(HashMap::new());
            collector.declare("this".to_string(), IrType::Class(ty.name.clone()));
            for param in &method.params {
                collector.declare(param.name.clone(), type_syntax_to_ir(&param.ty, env));
            }
            collector.collect_stmts(&method.body)?;
        }
    }
    collect_controller_endpoint_handlers(program, env, &mut collector.handlers);
    Ok(collector.handlers)
}

struct EndpointHandlerCollector<'a> {
    env: &'a TypeEnv,
    scopes: Vec<HashMap<String, IrType>>,
    handlers: Vec<EndpointHandlerBinding>,
}

impl<'a> EndpointHandlerCollector<'a> {
    fn declare(&mut self, name: String, ty: IrType) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    fn lookup(&self, name: &str) -> Option<IrType> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    fn collect_stmts(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.collect_stmt(stmt)?;
        }
        Ok(())
    }

    fn collect_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let {
                name,
                declared_type,
                expr,
                ..
            } => {
                self.collect_expr(expr)?;
                let ty = declared_type
                    .as_ref()
                    .map(|ty| type_syntax_to_ir(ty, self.env))
                    .unwrap_or_else(|| self.expr_type(expr));
                self.declare(name.clone(), ty);
            }
            Stmt::Assign { expr, .. }
            | Stmt::AssignTarget { expr, .. }
            | Stmt::Print(expr)
            | Stmt::Expr(expr)
            | Stmt::Return(Some(expr))
            | Stmt::Throw(expr) => self.collect_expr(expr)?,
            Stmt::Block(body) => {
                self.scopes.push(HashMap::new());
                self.collect_stmts(body)?;
                self.scopes.pop();
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_expr(condition)?;
                self.scopes.push(HashMap::new());
                self.collect_stmts(then_body)?;
                self.scopes.pop();
                self.scopes.push(HashMap::new());
                self.collect_stmts(else_body)?;
                self.scopes.pop();
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                self.scopes.push(HashMap::new());
                self.collect_stmts(try_body)?;
                self.scopes.pop();
                if let Some(catch) = catch {
                    self.scopes.push(HashMap::new());
                    if let Some(name) = &catch.name {
                        self.declare(name.clone(), IrType::Exception);
                    }
                    self.collect_stmts(&catch.body)?;
                    self.scopes.pop();
                }
                self.scopes.push(HashMap::new());
                self.collect_stmts(finally_body)?;
                self.scopes.pop();
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                self.collect_expr(expr)?;
                for case in cases {
                    self.collect_expr(&case.value)?;
                    self.scopes.push(HashMap::new());
                    self.collect_stmts(&case.body)?;
                    self.scopes.pop();
                }
                self.scopes.push(HashMap::new());
                self.collect_stmts(default)?;
                self.scopes.pop();
            }
            Stmt::While { condition, body } => {
                self.collect_expr(condition)?;
                self.scopes.push(HashMap::new());
                self.collect_stmts(body)?;
                self.scopes.pop();
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                self.scopes.push(HashMap::new());
                if let Some(init) = init {
                    self.collect_stmt(init)?;
                }
                if let Some(condition) = condition {
                    self.collect_expr(condition)?;
                }
                self.collect_stmts(body)?;
                if let Some(increment) = increment {
                    self.collect_stmt(increment)?;
                }
                self.scopes.pop();
            }
            Stmt::ForEach {
                item_type,
                item_name,
                collection,
                body,
            } => {
                self.collect_expr(collection)?;
                self.scopes.push(HashMap::new());
                self.declare(item_name.clone(), type_syntax_to_ir(item_type, self.env));
                self.collect_stmts(body)?;
                self.scopes.pop();
            }
            Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
        }
        Ok(())
    }

    fn collect_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::MethodCall { target, name, args } => {
                self.collect_endpoint_handler(target, name, args)?;
                self.collect_expr(target)?;
                for arg in args {
                    self.collect_expr(arg)?;
                }
            }
            Expr::ArrayLiteral(values) => {
                for value in values {
                    self.collect_expr(value)?;
                }
            }
            Expr::NewArray { length, values, .. } => {
                if let Some(length) = length {
                    self.collect_expr(length)?;
                }
                for value in values {
                    self.collect_expr(value)?;
                }
            }
            Expr::Index { target, index }
            | Expr::Binary {
                left: target,
                right: index,
                ..
            } => {
                self.collect_expr(target)?;
                self.collect_expr(index)?;
            }
            Expr::Unary { expr, .. } => self.collect_expr(expr)?,
            Expr::IncDec { .. } => {}
            Expr::Lambda { body, .. } => self.collect_expr(body)?,
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.collect_expr(condition)?;
                self.collect_expr(when_true)?;
                self.collect_expr(when_false)?;
            }
            Expr::Field { target, .. }
            | Expr::IsPattern { expr: target, .. }
            | Expr::Await(target) => self.collect_expr(target)?,
            Expr::Throw(expr) => self.collect_expr(expr)?,
            Expr::Assign { target, value } => {
                self.collect_expr(target)?;
                self.collect_expr(value)?;
            }
            Expr::FunctionCall { args, .. } => {
                for arg in args {
                    self.collect_expr(arg)?;
                }
            }
            Expr::NewObject { args, fields, .. } => {
                for arg in args {
                    self.collect_expr(arg)?;
                }
                for field in fields {
                    self.collect_expr(&field.expr)?;
                }
            }
            Expr::Int(_)
            | Expr::Float(_)
            | Expr::Bool(_)
            | Expr::Null
            | Expr::String(_)
            | Expr::Var(_)
            | Expr::Move(_)
            | Expr::NewCollection(_)
            | Expr::NewThread(_)
            | Expr::Borrow { .. } => {}
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                self.collect_expr(expr)?;
            }
        }
        Ok(())
    }

    fn collect_endpoint_handler(
        &mut self,
        target: &Expr,
        name: &str,
        args: &[Expr],
    ) -> Result<(), String> {
        if !matches!(name, "MapGet" | "MapPost")
            || self.expr_type(target) != IrType::Class("WebApplication".to_string())
        {
            return Ok(());
        }
        let [Expr::String(path), Expr::Var(function)] = args else {
            return Ok(());
        };
        let Some(signature) = self.env.single_function(function) else {
            return Ok(());
        };
        if !signature.params.is_empty() || signature.return_type != IrType::String {
            return Err(format!(
                "typed IR: endpoint handler '{function}' must be string {}()",
                function
            ));
        }
        self.handlers.push(EndpointHandlerBinding {
            http_method: if name == "MapGet" { "GET" } else { "POST" }.to_string(),
            path: path.clone(),
            function: signature.symbol.clone(),
            return_type: signature.return_type.clone(),
            response_type: signature.return_type.clone(),
            ownership: ownership_for_type(&signature.return_type),
            controller: None,
            constructor: None,
            constructor_params: Vec::new(),
            params: Vec::new(),
        });
        Ok(())
    }

    fn expr_type(&self, expr: &Expr) -> IrType {
        match expr {
            Expr::Var(name) => self
                .lookup(name)
                .or_else(|| {
                    self.env
                        .single_function(name)
                        .map(|signature| IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        })
                })
                .unwrap_or_else(|| IrType::Unknown(name.clone())),
            Expr::NewObject { type_name, .. } => match self.env.kinds.get(type_name) {
                Some(TypeKind::Class) | Some(TypeKind::Interface) => {
                    IrType::Class(type_name.clone())
                }
                _ => IrType::Struct(type_name.clone()),
            },
            Expr::FunctionCall { name, args } => self
                .env
                .resolve_function(
                    name,
                    &args
                        .iter()
                        .map(|arg| self.expr_type(arg))
                        .collect::<Vec<_>>(),
                )
                .map(|signature| signature.return_type.clone())
                .unwrap_or_else(|| IrType::Unknown(name.clone())),
            Expr::String(_) => IrType::String,
            Expr::Int(_) => IrType::Long,
            Expr::Float(_) => IrType::Double,
            Expr::Bool(_) => IrType::Bool,
            Expr::Null => IrType::Unknown("null".to_string()),
            _ => IrType::Unknown("<expr>".to_string()),
        }
    }
}

fn collect_controller_endpoint_handlers(
    program: &Program,
    env: &TypeEnv,
    handlers: &mut Vec<EndpointHandlerBinding>,
) {
    let types = program
        .types
        .iter()
        .map(|ty| (ty.name.as_str(), ty))
        .collect::<HashMap<_, _>>();
    for ty in &program.types {
        // Open generic controller bases provide metadata and actions, but are not
        // themselves activatable endpoints.
        if !ty.generic_params.is_empty() {
            continue;
        }
        let Some(metadata_type) = controller_metadata_type(ty, &types) else {
            continue;
        };
        let controller_name = ty.name.strip_suffix("Controller").unwrap_or(&ty.name);
        let versions = attribute_string_args(&metadata_type.attributes, "ApiVersion");
        let mut prefixes = attribute_string_args(&metadata_type.attributes, "Route");
        if prefixes.is_empty() {
            prefixes.push(String::new());
        }
        let constructor_signature = env.constructors.get(&ty.name).and_then(|constructors| {
            constructors
                .iter()
                .find(|constructor| constructor.params.is_empty())
                .or_else(|| constructors.first())
        });
        let constructor = constructor_signature.map(|constructor| constructor.symbol.clone());
        let constructor_params = constructor_signature
            .map(|constructor| constructor.params.clone())
            .unwrap_or_default();
        let can_activate = ty.constructors.is_empty() || constructor.is_some();

        for (method_owner, method) in controller_methods(ty, &types) {
            let Some((http_method, inline_path)) = http_method_attribute(&method.attributes) else {
                continue;
            };
            let mut action_paths = attribute_string_args(&method.attributes, "Route");
            if action_paths.is_empty() {
                action_paths.push(inline_path.unwrap_or_default());
            }
            let parameter_types = method
                .params
                .iter()
                .map(|param| type_syntax_to_ir(&param.ty, env))
                .collect::<Vec<_>>();
            let Some(signature) =
                env.resolve_method(&method_owner.name, &method.name, &parameter_types)
            else {
                continue;
            };

            for prefix in &prefixes {
                let expanded_prefixes = expand_controller_route(prefix, controller_name, &versions);
                for expanded_prefix in expanded_prefixes {
                    for action_path in &action_paths {
                        let path = join_route_path(&expanded_prefix, action_path);
                        if handlers.iter().any(|handler| {
                            handler.http_method == http_method && handler.path == path
                        }) {
                            continue;
                        }
                        let params = method
                            .params
                            .iter()
                            .map(|param| EndpointParameterBinding {
                                name: param.name.clone(),
                                ty: type_syntax_to_ir(&param.ty, env),
                                source: endpoint_parameter_source(param, &path, http_method),
                            })
                            .collect();
                        handlers.push(EndpointHandlerBinding {
                            http_method: http_method.to_string(),
                            path,
                            function: signature.symbol.clone(),
                            return_type: signature.return_type.clone(),
                            response_type: endpoint_response_type(&method.return_type, env),
                            ownership: ownership_for_type(&signature.return_type),
                            controller: can_activate.then(|| ty.name.clone()),
                            constructor: constructor.clone(),
                            constructor_params: constructor_params.clone(),
                            params,
                        });
                    }
                }
            }
        }
    }
}

fn endpoint_response_type(ty: &TypeSyntax, env: &TypeEnv) -> IrType {
    match ty {
        TypeSyntax::Task(inner) => endpoint_response_type(inner, env),
        TypeSyntax::GenericNamed { name, args }
            if matches!(
                name.rsplit('.').next().unwrap_or(name),
                "Task" | "ValueTask" | "ActionResult"
            ) =>
        {
            args.first()
                .map(|arg| endpoint_response_type(arg, env))
                .unwrap_or(IrType::String)
        }
        _ => type_syntax_to_ir(ty, env),
    }
}

fn endpoint_parameter_source(
    param: &Param,
    route: &str,
    http_method: &str,
) -> EndpointParameterSource {
    for (attribute, source) in [
        ("FromRoute", EndpointParameterSource::Route),
        ("FromQuery", EndpointParameterSource::Query),
        ("FromBody", EndpointParameterSource::Body),
        ("FromForm", EndpointParameterSource::Form),
        ("FromHeader", EndpointParameterSource::Header),
    ] {
        if has_attribute(&param.attributes, attribute) {
            return source;
        }
    }
    if route.contains(&format!("{{{}", param.name)) {
        return EndpointParameterSource::Route;
    }
    if matches!(http_method, "GET" | "DELETE") {
        EndpointParameterSource::Query
    } else if matches!(
        param.ty,
        TypeSyntax::Named(_) | TypeSyntax::GenericNamed { .. }
    ) {
        EndpointParameterSource::Body
    } else {
        EndpointParameterSource::Auto
    }
}

fn controller_metadata_type<'a>(
    ty: &'a TypeDef,
    types: &HashMap<&str, &'a TypeDef>,
) -> Option<&'a TypeDef> {
    if has_attribute(&ty.attributes, "ApiController") {
        return Some(ty);
    }
    ty.bases.iter().find_map(|base| {
        controller_base_type(base, types).and_then(|base| controller_metadata_type(base, types))
    })
}

fn controller_methods<'a>(
    ty: &'a TypeDef,
    types: &HashMap<&str, &'a TypeDef>,
) -> Vec<(&'a TypeDef, &'a Function)> {
    fn collect<'a>(
        ty: &'a TypeDef,
        types: &HashMap<&str, &'a TypeDef>,
        seen: &mut std::collections::HashSet<(String, usize)>,
        output: &mut Vec<(&'a TypeDef, &'a Function)>,
    ) {
        for method in &ty.methods {
            let key = (method.name.clone(), method.params.len());
            if seen.insert(key) {
                output.push((ty, method));
            }
        }
        for base in &ty.bases {
            if let Some(base) = controller_base_type(base, types) {
                collect(base, types, seen, output);
            }
        }
    }

    let mut output = Vec::new();
    let mut seen = std::collections::HashSet::new();
    collect(ty, types, &mut seen, &mut output);
    output
}

fn controller_base_type<'a>(base: &str, types: &HashMap<&str, &'a TypeDef>) -> Option<&'a TypeDef> {
    types
        .get(base)
        .copied()
        .or_else(|| types.get(base.split('<').next().unwrap_or(base)).copied())
}

fn has_attribute(attributes: &[Attribute], expected: &str) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute_name_matches(&attribute.name, expected))
}

fn attribute_string_args(attributes: &[Attribute], expected: &str) -> Vec<String> {
    attributes
        .iter()
        .filter(|attribute| attribute_name_matches(&attribute.name, expected))
        .filter_map(|attribute| match attribute.args.first() {
            Some(Expr::String(value)) => Some(value.clone()),
            None => Some(String::new()),
            _ => None,
        })
        .collect()
}

fn attribute_name_matches(actual: &str, expected: &str) -> bool {
    let short = actual.rsplit('.').next().unwrap_or(actual);
    short == expected || short.strip_suffix("Attribute") == Some(expected)
}

fn http_method_attribute(attributes: &[Attribute]) -> Option<(&'static str, Option<String>)> {
    [
        ("HttpGet", "GET"),
        ("HttpPost", "POST"),
        ("HttpPut", "PUT"),
        ("HttpDelete", "DELETE"),
        ("HttpPatch", "PATCH"),
    ]
    .into_iter()
    .find_map(|(attribute_name, method)| {
        attributes
            .iter()
            .find(|attribute| attribute_name_matches(&attribute.name, attribute_name))
            .map(|attribute| {
                let path = match attribute.args.first() {
                    Some(Expr::String(value)) => Some(value.clone()),
                    _ => None,
                };
                (method, path)
            })
    })
}

fn expand_controller_route(prefix: &str, controller: &str, versions: &[String]) -> Vec<String> {
    let controller_path = prefix
        .replace("[controller]", controller)
        .replace("[action]", "");
    if !controller_path.contains("{version:apiVersion}") {
        return vec![controller_path];
    }
    if versions.is_empty() {
        return vec![controller_path.replace("{version:apiVersion}", "1.0")];
    }
    versions
        .iter()
        .map(|version| controller_path.replace("{version:apiVersion}", version))
        .collect()
}

fn join_route_path(prefix: &str, path: &str) -> String {
    let left = prefix.trim().trim_matches('/');
    let right = path.trim().trim_matches('/');
    match (left.is_empty(), right.is_empty()) {
        (true, true) => "/".to_string(),
        (true, false) => format!("/{right}"),
        (false, true) => format!("/{left}"),
        (false, false) => format!("/{left}/{right}"),
    }
}

fn lower_type(ty: &TypeDef, symbol_id: usize, env: &TypeEnv) -> Result<TypedType, String> {
    let fields = env
        .all_field_infos(&ty.name)
        .into_iter()
        .map(|(name, field)| TypedBinding {
            name,
            ownership: field.ownership,
            ty: field.ty,
        })
        .collect::<Vec<_>>();
    let this_binding = TypedBinding {
        name: "this".to_string(),
        ty: match ty.kind {
            TypeKind::Class | TypeKind::Interface => IrType::Class(ty.name.clone()),
            _ => IrType::Ref(Box::new(IrType::Struct(ty.name.clone()))),
        },
        ownership: Ownership::Borrowed,
    };
    let constructors = ty
        .constructors
        .iter()
        .map(|constructor| {
            let function = Function {
                namespace: constructor.namespace.clone(),
                attributes: constructor.attributes.clone(),
                is_async: false,
                is_extern: false,
                is_extension: false,
                name: ty.name.clone(),
                generic_params: Vec::new(),
                params: constructor.params.clone(),
                return_type: TypeSyntax::Void,
                body: constructor.body.clone(),
            };
            lower_function(
                &function,
                env,
                std::slice::from_ref(&this_binding),
                Some(type_constructor_symbol(ty, symbol_id, constructor, env)),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let methods = ty
        .methods
        .iter()
        .map(|method| {
            let implicit_params: &[TypedBinding] = if method.is_extension {
                &[]
            } else {
                std::slice::from_ref(&this_binding)
            };
            lower_function(
                method,
                env,
                implicit_params,
                Some(type_method_symbol(ty, symbol_id, method, env)),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TypedType {
        name: ty.name.clone(),
        namespace: ty.namespace.clone(),
        generic_params: ty
            .generic_params
            .iter()
            .map(|param| param.name.clone())
            .collect(),
        symbol_id,
        is_extension: ty.is_extension,
        kind: ty.kind,
        bases: ty.bases.clone(),
        fields,
        constructors,
        methods,
    })
}

fn lower_function(
    function: &Function,
    env: &TypeEnv,
    implicit_params: &[TypedBinding],
    symbol_override: Option<String>,
) -> Result<TypedFunction, String> {
    let return_type = type_syntax_to_ir(&function.return_type, env);
    let mut scopes = vec![HashMap::new()];
    let mut params = Vec::new();
    for param in implicit_params {
        scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), param.clone());
        params.push(param.clone());
    }
    for param in &function.params {
        let ty = type_syntax_to_ir(&param.ty, env);
        let binding = TypedBinding {
            name: param.name.clone(),
            ownership: ownership_for_declared_type_syntax(&param.ty, env),
            ty,
        };
        scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), binding.clone());
        params.push(binding);
    }
    let mut locals = Vec::new();
    lower_stmts(&function.body, env, &mut scopes, &mut locals)?;
    let mut typed_scopes = vec![HashMap::new()];
    for param in &params {
        typed_scopes
            .last_mut()
            .unwrap()
            .insert(param.name.clone(), param.clone());
    }
    let body = lower_typed_stmts(&function.body, env, &mut typed_scopes)?;
    Ok(TypedFunction {
        name: function.name.clone(),
        symbol: symbol_override.unwrap_or_else(|| {
            env.resolve_function(
                &function.name,
                &function
                    .params
                    .iter()
                    .map(|param| type_syntax_to_ir(&param.ty, env))
                    .collect::<Vec<_>>(),
            )
            .map(|signature| signature.symbol.clone())
            .unwrap_or_else(|| function.name.clone())
        }),
        is_extern: function.is_extern,
        return_ownership: ownership_for_declared_type_syntax(&function.return_type, env),
        return_type,
        params,
        locals,
        body,
    })
}

fn lower_stmts(
    stmts: &[Stmt],
    env: &TypeEnv,
    scopes: &mut Vec<HashMap<String, TypedBinding>>,
    locals: &mut Vec<TypedBinding>,
) -> Result<(), String> {
    for stmt in stmts {
        lower_stmt(stmt, env, scopes, locals)?;
    }
    Ok(())
}

fn lower_typed_stmts(
    stmts: &[Stmt],
    env: &TypeEnv,
    scopes: &mut Vec<HashMap<String, TypedBinding>>,
) -> Result<Vec<TypedStmt>, String> {
    stmts
        .iter()
        .map(|stmt| lower_typed_stmt(stmt, env, scopes))
        .collect()
}

fn lower_typed_stmt(
    stmt: &Stmt,
    env: &TypeEnv,
    scopes: &mut Vec<HashMap<String, TypedBinding>>,
) -> Result<TypedStmt, String> {
    let kind = match stmt {
        Stmt::Let {
            name,
            declared_type,
            expr,
            ..
        } => {
            let expected_ty = declared_type.as_ref().map(|ty| type_syntax_to_ir(ty, env));
            let expr = lower_typed_expr_with_expected(expr, env, scopes, expected_ty.as_ref())?;
            let ty = declared_type
                .as_ref()
                .map(|ty| type_syntax_to_ir(ty, env))
                .unwrap_or_else(|| expr.ty.clone());
            let ownership = declared_type
                .as_ref()
                .map(|declared| ownership_for_declared_type_syntax(declared, env))
                .unwrap_or_else(|| expr.ownership.clone());
            let binding = TypedBinding {
                name: name.clone(),
                ty,
                ownership,
            };
            scopes
                .last_mut()
                .unwrap()
                .insert(name.clone(), binding.clone());
            TypedStmtKind::Let { binding, expr }
        }
        Stmt::Assign { name, expr } => {
            let expected_ty = lookup(scopes, name).map(|binding| binding.ty.clone());
            let expr = lower_typed_expr_with_expected(expr, env, scopes, expected_ty.as_ref())?;
            if lookup(scopes, name).is_none() {
                if let Some((this_type, field_type, field_ownership)) =
                    implicit_field(env, scopes, name)
                {
                    let this_expr = typed_expr_with_ownership(
                        TypedExprKind::Var("this".to_string()),
                        this_type,
                        Ownership::Borrowed,
                    );
                    TypedStmtKind::AssignTarget {
                        target: typed_expr_with_ownership(
                            TypedExprKind::Field {
                                target: Box::new(this_expr),
                                name: name.clone(),
                            },
                            field_type,
                            field_ownership,
                        ),
                        expr,
                    }
                } else {
                    TypedStmtKind::Assign {
                        name: name.clone(),
                        expr,
                    }
                }
            } else {
                TypedStmtKind::Assign {
                    name: name.clone(),
                    expr,
                }
            }
        }
        Stmt::AssignTarget { target, expr } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let expr = lower_typed_expr_with_expected(expr, env, scopes, Some(&target.ty))?;
            TypedStmtKind::AssignTarget { target, expr }
        }
        Stmt::Block(body) => {
            scopes.push(HashMap::new());
            let body = lower_typed_stmts(body, env, scopes)?;
            scopes.pop();
            TypedStmtKind::Block(body)
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let pattern_bindings = collect_condition_bindings(condition, env);
            scopes.push(
                pattern_bindings
                    .iter()
                    .map(|binding| (binding.name.clone(), binding.clone()))
                    .collect(),
            );
            let condition = lower_typed_expr(condition, env, scopes)?;
            let then_body = lower_typed_stmts(then_body, env, scopes)?;
            scopes.pop();
            scopes.push(HashMap::new());
            let else_body = lower_typed_stmts(else_body, env, scopes)?;
            scopes.pop();
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            }
        }
        Stmt::Try {
            try_body,
            catch,
            finally_body,
        } => {
            scopes.push(HashMap::new());
            let try_body = lower_typed_stmts(try_body, env, scopes)?;
            scopes.pop();
            let (catch_name, catch_body) = if let Some(catch) = catch {
                scopes.push(HashMap::new());
                if let Some(name) = &catch.name {
                    scopes.last_mut().unwrap().insert(
                        name.clone(),
                        TypedBinding {
                            name: name.clone(),
                            ty: IrType::Exception,
                            ownership: Ownership::Borrowed,
                        },
                    );
                }
                let body = lower_typed_stmts(&catch.body, env, scopes)?;
                scopes.pop();
                (catch.name.clone(), body)
            } else {
                (None, Vec::new())
            };
            scopes.push(HashMap::new());
            let finally_body = lower_typed_stmts(finally_body, env, scopes)?;
            scopes.pop();
            TypedStmtKind::Try {
                try_body,
                catch_name,
                catch_body,
                finally_body,
            }
        }
        Stmt::Switch {
            expr,
            cases,
            default,
        } => {
            let expr = lower_typed_expr(expr, env, scopes)?;
            let mut typed_cases = Vec::new();
            for case in cases {
                let value = lower_typed_expr(&case.value, env, scopes)?;
                scopes.push(HashMap::new());
                if let Expr::IsPattern {
                    ty,
                    name: Some(name),
                    ..
                } = &case.value
                {
                    scopes.last_mut().unwrap().insert(
                        name.clone(),
                        TypedBinding {
                            name: name.clone(),
                            ty: type_syntax_to_ir(ty, env),
                            ownership: Ownership::Borrowed,
                        },
                    );
                }
                let body = lower_typed_stmts(&case.body, env, scopes)?;
                scopes.pop();
                typed_cases.push(TypedSwitchCase { value, body });
            }
            scopes.push(HashMap::new());
            let default = lower_typed_stmts(default, env, scopes)?;
            scopes.pop();
            TypedStmtKind::Switch {
                expr,
                cases: typed_cases,
                default,
            }
        }
        Stmt::While { condition, body } => {
            let condition = lower_typed_expr(condition, env, scopes)?;
            scopes.push(HashMap::new());
            let body = lower_typed_stmts(body, env, scopes)?;
            scopes.pop();
            TypedStmtKind::While { condition, body }
        }
        Stmt::For {
            init,
            condition,
            increment,
            body,
        } => {
            scopes.push(HashMap::new());
            let init = init
                .as_ref()
                .map(|stmt| lower_typed_stmt(stmt, env, scopes).map(Box::new))
                .transpose()?;
            let condition = condition
                .as_ref()
                .map(|expr| lower_typed_expr(expr, env, scopes))
                .transpose()?;
            let body = lower_typed_stmts(body, env, scopes)?;
            let increment = increment
                .as_ref()
                .map(|stmt| lower_typed_stmt(stmt, env, scopes).map(Box::new))
                .transpose()?;
            scopes.pop();
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            }
        }
        Stmt::ForEach {
            item_type,
            item_name,
            collection,
            body,
        } => {
            let collection = lower_typed_expr(collection, env, scopes)?;
            let ty = type_syntax_to_ir(item_type, env);
                let item = TypedBinding {
                    name: item_name.clone(),
                    ty,
                    ownership: ownership_for_declared_type_syntax(item_type, env),
                };
            scopes.push(HashMap::new());
            scopes
                .last_mut()
                .unwrap()
                .insert(item_name.clone(), item.clone());
            let body = lower_typed_stmts(body, env, scopes)?;
            scopes.pop();
            TypedStmtKind::ForEach {
                item,
                collection,
                body,
            }
        }
        Stmt::Print(expr) => TypedStmtKind::Print(lower_typed_expr(expr, env, scopes)?),
        Stmt::Expr(expr) => TypedStmtKind::Expr(lower_typed_expr(expr, env, scopes)?),
        Stmt::Return(Some(expr)) => {
            TypedStmtKind::Return(Some(lower_typed_expr(expr, env, scopes)?))
        }
        Stmt::Return(None) => TypedStmtKind::Return(None),
        Stmt::Throw(expr) => TypedStmtKind::Throw(lower_typed_expr(expr, env, scopes)?),
        Stmt::Break => TypedStmtKind::Break,
        Stmt::Continue => TypedStmtKind::Continue,
    };
    Ok(TypedStmt { kind })
}

fn lower_stmt(
    stmt: &Stmt,
    env: &TypeEnv,
    scopes: &mut Vec<HashMap<String, TypedBinding>>,
    locals: &mut Vec<TypedBinding>,
) -> Result<(), String> {
    match stmt {
        Stmt::Let {
            name,
            declared_type,
            expr,
            ..
        } => {
            let inferred = lower_expr(expr, env, scopes)?;
            let ty = declared_type
                .as_ref()
                .map(|ty| type_syntax_to_ir(ty, env))
                .unwrap_or(inferred.ty);
            let ownership = declared_type
                .as_ref()
                .map(|declared| ownership_for_declared_type_syntax(declared, env))
                .unwrap_or(inferred.ownership);
            let binding = TypedBinding {
                name: name.clone(),
                ty,
                ownership,
            };
            scopes
                .last_mut()
                .unwrap()
                .insert(name.clone(), binding.clone());
            locals.push(binding);
        }
        Stmt::Assign { expr, .. } | Stmt::AssignTarget { expr, .. } => {
            lower_expr(expr, env, scopes)?;
        }
        Stmt::Block(body) => {
            scopes.push(HashMap::new());
            lower_stmts(body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            lower_expr(condition, env, scopes)?;
            scopes.push(HashMap::new());
            lower_stmts(then_body, env, scopes, locals)?;
            scopes.pop();
            scopes.push(HashMap::new());
            lower_stmts(else_body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::Try {
            try_body,
            catch,
            finally_body,
        } => {
            scopes.push(HashMap::new());
            lower_stmts(try_body, env, scopes, locals)?;
            scopes.pop();
            if let Some(catch) = catch {
                scopes.push(HashMap::new());
                if let Some(name) = &catch.name {
                    scopes.last_mut().unwrap().insert(
                        name.clone(),
                        TypedBinding {
                            name: name.clone(),
                            ty: IrType::Exception,
                            ownership: Ownership::Borrowed,
                        },
                    );
                }
                lower_stmts(&catch.body, env, scopes, locals)?;
                scopes.pop();
            }
            scopes.push(HashMap::new());
            lower_stmts(finally_body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::Switch {
            expr,
            cases,
            default,
        } => {
            lower_expr(expr, env, scopes)?;
            for case in cases {
                lower_expr(&case.value, env, scopes)?;
                scopes.push(HashMap::new());
                lower_stmts(&case.body, env, scopes, locals)?;
                scopes.pop();
            }
            scopes.push(HashMap::new());
            lower_stmts(default, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::While { condition, body } => {
            lower_expr(condition, env, scopes)?;
            scopes.push(HashMap::new());
            lower_stmts(body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::For {
            init,
            condition,
            increment,
            body,
        } => {
            scopes.push(HashMap::new());
            if let Some(init) = init {
                lower_stmt(init, env, scopes, locals)?;
            }
            if let Some(condition) = condition {
                lower_expr(condition, env, scopes)?;
            }
            lower_stmts(body, env, scopes, locals)?;
            if let Some(increment) = increment {
                lower_stmt(increment, env, scopes, locals)?;
            }
            scopes.pop();
        }
        Stmt::ForEach {
            item_type,
            item_name,
            collection,
            body,
        } => {
            lower_expr(collection, env, scopes)?;
            let ty = type_syntax_to_ir(item_type, env);
            let binding = TypedBinding {
                name: item_name.clone(),
                ownership: ownership_for_declared_type_syntax(item_type, env),
                ty,
            };
            scopes.push(HashMap::new());
            scopes
                .last_mut()
                .unwrap()
                .insert(item_name.clone(), binding.clone());
            locals.push(binding);
            lower_stmts(body, env, scopes, locals)?;
            scopes.pop();
        }
        Stmt::Print(expr) | Stmt::Expr(expr) | Stmt::Return(Some(expr)) | Stmt::Throw(expr) => {
            lower_expr(expr, env, scopes)?;
        }
        Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
    }
    Ok(())
}

fn find_expected_types(candidates: &[FunctionSignature], args: &[Expr]) -> Vec<Option<IrType>> {
    let mut expected = vec![None; args.len()];
    for sig in candidates {
        if let Some(element_ty) = &sig.params_element_type {
            let fixed_len = sig.params.len().saturating_sub(1);
            if args.len() >= fixed_len {
                for (i, param_ty) in sig.params.iter().take(fixed_len).enumerate() {
                    if expected[i].is_none() {
                        expected[i] = Some(param_ty.clone());
                    }
                }
                for i in fixed_len..args.len() {
                    if expected[i].is_none() {
                        expected[i] = Some(element_ty.clone());
                    }
                }
            }
        } else if sig.params.len() == args.len() {
            for (i, param_ty) in sig.params.iter().enumerate() {
                if expected[i].is_none() {
                    expected[i] = Some(param_ty.clone());
                }
            }
        }
    }
    expected
}

fn lower_lambda(
    params: &[String],
    body: &Expr,
    expected_type: Option<&IrType>,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedExpr, String> {
    let mut captures = Vec::new();
    collect_lambda_captures(body, params, &mut captures);
    captures.sort();
    captures.dedup();
    for capture in &captures {
        if let Some(binding) = lookup(scopes, capture) {
            if matches!(binding.ownership, Ownership::Borrowed | Ownership::View) {
                return Err(format!(
                    "ownership checker: lambda capture '{capture}' may outlive borrowed/view source; move the value into the closure or use an owned copy"
                ));
            }
        }
    }
    let param_types: Vec<IrType> = if let Some(IrType::Function {
        params: expected_params,
        ..
    }) = expected_type
    {
        expected_params.iter().cloned().collect()
    } else {
        vec![IrType::Unknown("lambda_param".to_string()); params.len()]
    };
    let mut new_scopes = scopes.to_vec();
    let mut lambda_scope = HashMap::new();
    for (name, ty) in params.iter().zip(param_types.iter()) {
        lambda_scope.insert(
            name.clone(),
            TypedBinding {
                name: name.clone(),
                ty: ty.clone(),
                ownership: ownership_for_type(ty),
            },
        );
    }
    new_scopes.push(lambda_scope);
    let typed_body = lower_typed_expr(&body, env, &new_scopes)?;
    let return_type = typed_body.ty.clone();
    let lambda_ty = IrType::Function {
        params: param_types,
        return_type: Box::new(return_type),
    };
    Ok(typed_expr_with_ownership(
        TypedExprKind::Lambda {
            params: params.to_vec(),
            body: Box::new(typed_body),
        },
        lambda_ty,
        Ownership::Shared,
    ))
}

fn collect_lambda_captures(expr: &Expr, params: &[String], out: &mut Vec<String>) {
    match expr {
        Expr::Var(name) | Expr::Move(name) | Expr::Borrow { name, .. } => {
            if !params.contains(name) {
                out.push(name.clone());
            }
        }
        Expr::ArrayLiteral(values) => {
            for value in values {
                collect_lambda_captures(value, params, out);
            }
        }
        Expr::NewArray { length, values, .. } => {
            if let Some(length) = length {
                collect_lambda_captures(length, params, out);
            }
            for value in values {
                collect_lambda_captures(value, params, out);
            }
        }
        Expr::Index { target, index } => {
            collect_lambda_captures(target, params, out);
            collect_lambda_captures(index, params, out);
        }
        Expr::Field { target, .. } => collect_lambda_captures(target, params, out),
        Expr::IsPattern { expr, .. } | Expr::Await(expr) | Expr::Unary { expr, .. } => {
            collect_lambda_captures(expr, params, out)
        }
        Expr::Throw(expr) => collect_lambda_captures(expr, params, out),
        Expr::Assign { target, value } => {
            collect_lambda_captures(target, params, out);
            collect_lambda_captures(value, params, out);
        }
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_lambda_captures(condition, params, out);
            collect_lambda_captures(when_true, params, out);
            collect_lambda_captures(when_false, params, out);
        }
        Expr::Binary { left, right, .. } => {
            collect_lambda_captures(left, params, out);
            collect_lambda_captures(right, params, out);
        }
        Expr::MethodCall { target, args, .. } => {
            collect_lambda_captures(target, params, out);
            for arg in args {
                collect_lambda_captures(arg, params, out);
            }
        }
        Expr::IncDec { name, .. } => {
            if !params.contains(name) {
                out.push(name.clone());
            }
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_lambda_captures(arg, params, out);
            }
        }
        Expr::NewObject { args, fields, .. } => {
            for arg in args {
                collect_lambda_captures(arg, params, out);
            }
            for field in fields {
                collect_lambda_captures(&field.expr, params, out);
            }
        }
        Expr::Lambda { params: inner_params, body } => {
            let mut merged = params.to_vec();
            merged.extend(inner_params.iter().cloned());
            collect_lambda_captures(body, &merged, out);
        }
        Expr::Int(_)
        | Expr::Float(_)
        | Expr::Bool(_)
        | Expr::Null
        | Expr::String(_)
        | Expr::NewCollection(_)
        | Expr::NewThread(_)
        => {}
        Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
            collect_lambda_captures(expr, params, out);
        }
    }
}

fn lower_call_args(
    args: &[Expr],
    expected_types: &[Option<IrType>],
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<Vec<TypedExpr>, String> {
    let mut lowered = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        let expected = expected_types.get(i).and_then(|x| x.as_ref());
        let typed_arg = match arg {
            Expr::Lambda { params, body } => lower_lambda(params, body, expected, env, scopes)?,
            Expr::NamedArg { name: _, expr } => {
                let inner = match expr.as_ref() {
                    Expr::Lambda { params, body } => {
                        lower_lambda(params, body, expected, env, scopes)?
                    }
                    _ => lower_typed_expr(expr, env, scopes)?,
                };
                inner
            }
            _ => lower_typed_expr(arg, env, scopes)?,
        };
        lowered.push(typed_arg);
    }
    Ok(lowered)
}

fn pack_params_args(
    signature: &FunctionSignature,
    args: Vec<TypedExpr>,
) -> Vec<TypedExpr> {
    let Some(element_ty) = &signature.params_element_type else {
        return args;
    };
    let fixed_len = signature.params.len().saturating_sub(1);
    if args.len() == signature.params.len()
        && signature
            .params
            .last()
            .is_some_and(|expected| ir_arg_matches(expected, args.last().map(|arg| &arg.ty).unwrap()))
    {
        return args;
    }
    if args.len() < fixed_len {
        return args;
    }
    let mut packed = args[..fixed_len].to_vec();
    let tail = args[fixed_len..].to_vec();
    let array_ty = IrType::Array(Box::new(element_ty.clone()));
    packed.push(typed_expr_with_ownership(
        TypedExprKind::NewArray {
            element_type: element_ty.clone(),
            length: None,
            values: tail,
        },
        array_ty,
        Ownership::Owned,
    ));
    packed
}

fn lower_typed_expr(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedExpr, String> {
    lower_typed_expr_with_expected(expr, env, scopes, None)
}

fn lower_typed_expr_with_expected(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
    expected: Option<&IrType>,
) -> Result<TypedExpr, String> {
    let typed = match expr {
        Expr::Int(value) => typed_expr(TypedExprKind::Int(*value), IrType::Long),
        Expr::Float(value) => typed_expr(TypedExprKind::Float(*value), IrType::Double),
        Expr::Bool(value) => typed_expr(TypedExprKind::Bool(*value), IrType::Bool),
        Expr::Null => typed_expr(TypedExprKind::Null, IrType::Unknown("null".to_string())),
        Expr::String(value) => typed_expr_with_ownership(
            TypedExprKind::String(value.clone()),
            IrType::String,
            Ownership::Owned,
        ),
        Expr::Var(name) => {
            if let Some(binding) = lookup(scopes, name) {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    binding.ty,
                    binding.ownership,
                )
            } else if let Some((this_type, field_type, field_ownership)) =
                implicit_field(env, scopes, name)
            {
                let this_expr = typed_expr_with_ownership(
                    TypedExprKind::Var("this".to_string()),
                    this_type,
                    Ownership::Borrowed,
                );
                typed_expr_with_ownership(
                    TypedExprKind::Field {
                        target: Box::new(this_expr),
                        name: name.clone(),
                    },
                    field_type.clone(),
                    field_ownership,
                )
            } else if let Some(kind) = env.kinds.get(name) {
                let ty = match kind {
                    TypeKind::Class => IrType::Class(name.clone()),
                    TypeKind::Interface => IrType::Interface(name.clone()),
                    TypeKind::Enum => IrType::Int,
                    _ => IrType::Struct(name.clone()),
                };
                typed_expr_with_ownership(TypedExprKind::Var(name.clone()), ty, Ownership::Shared)
            } else if name == "Exception" || name == "System.Exception" {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    IrType::Exception,
                    Ownership::Shared,
                )
            } else if let Some(signature) = env.single_function(name) {
                typed_expr_with_ownership(
                    TypedExprKind::FunctionSymbol(name.clone()),
                    IrType::Function {
                        params: signature.params.clone(),
                        return_type: Box::new(signature.return_type.clone()),
                    },
                    Ownership::Copy,
                )
            } else {
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    IrType::Unknown(name.clone()),
                    Ownership::Shared,
                )
            }
        }
        Expr::Move(name) => {
            let ty = lookup(scopes, name)
                .map(|binding| binding.ty)
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            typed_expr_with_ownership(TypedExprKind::Move(name.clone()), ty, Ownership::Moved)
        }
        Expr::ArrayLiteral(values) => {
            let values = values
                .iter()
                .map(|value| lower_typed_expr(value, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            typed_expr_with_ownership(
                TypedExprKind::ArrayLiteral(values),
                IrType::Array(Box::new(IrType::Long)),
                Ownership::Owned,
            )
        }
        Expr::NewArray {
            element_type,
            length,
            values,
        } => {
            let element_type = type_syntax_to_ir(element_type, env);
            let length = length
                .as_ref()
                .map(|length| lower_typed_expr(length, env, scopes))
                .transpose()?
                .map(Box::new);
            let values = values
                .iter()
                .map(|value| lower_typed_expr(value, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            typed_expr_with_ownership(
                TypedExprKind::NewArray {
                    element_type: element_type.clone(),
                    length,
                    values,
                },
                IrType::Array(Box::new(element_type)),
                Ownership::Owned,
            )
        }
        Expr::Index { target, index } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let index = lower_typed_expr(index, env, scopes)?;
            let ty = match &target.ty {
                IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => {
                    inner.as_ref().clone()
                }
                IrType::Dictionary(_, value) => value.as_ref().clone(),
                IrType::String => IrType::Byte,
                _ => IrType::Unknown("index".to_string()),
            };
            let ownership = if ty == IrType::String {
                Ownership::Borrowed
            } else {
                ownership_for_type(&ty)
            };
            typed_expr_with_ownership(
                TypedExprKind::Index {
                    target: Box::new(target),
                    index: Box::new(index),
                },
                ty,
                ownership,
            )
        }
        Expr::Field { target, name } => {
            let target = lower_typed_expr(target, env, scopes)?;
            if let TypedExprKind::Var(enum_name) = &target.kind {
                if let Some(value) = env.enum_values.get(&(enum_name.clone(), name.clone())) {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Int(*value),
                        IrType::Int,
                        Ownership::Copy,
                    ));
                }
            }
            if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                let getter_name = format!("get_{name}");
                if let Some(signature) = env.resolve_method(type_name, &getter_name, &[]) {
                    return Ok(typed_expr_with_ownership(
                        TypedExprKind::Call(TypedCall {
                            kind: TypedCallKind::Method {
                                target: Box::new(target),
                                name: getter_name,
                                symbol: signature.symbol.clone(),
                                resolution: CallResolution::InstanceMethod,
                            },
                            args: Vec::new(),
                        }),
                        signature.return_type.clone(),
                        signature.return_ownership.clone(),
                    ));
                }
            }
            let field_info = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), field) | (IrType::Struct(type_name), field) => {
                    env.resolve_field(type_name, field)
                }
                (IrType::List(_), "Count")
                | (IrType::Dictionary(_, _), "Count")
                | (IrType::Array(_), "Length" | "Count") => Some(FieldSignature {
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::String, "Length") => Some(FieldSignature {
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Task(inner), "Result") => Some(FieldSignature {
                    ty: inner.as_ref().clone(),
                    ownership: ownership_for_type(inner),
                }),
                (IrType::Task(_), "IsCompleted") => Some(FieldSignature {
                    ty: IrType::Bool,
                    ownership: Ownership::Copy,
                }),
                (IrType::Exception, "Message") => Some(FieldSignature {
                    ty: IrType::String,
                    ownership: Ownership::Borrowed,
                }),
                (IrType::Weak(inner), "Target") => Some(FieldSignature {
                    ty: inner.as_ref().clone(),
                    ownership: Ownership::Borrowed,
                }),
                _ => None,
            };
            let (ty, ownership) = field_info
                .map(|field| (field.ty, field.ownership))
                .unwrap_or_else(|| {
                    let ty = IrType::Unknown(name.clone());
                    let ownership = ownership_for_type(&ty);
                    (ty, ownership)
                });
            typed_expr_with_ownership(
                TypedExprKind::Field {
                    target: Box::new(target),
                    name: name.clone(),
                },
                ty,
                ownership,
            )
        }
        Expr::IsPattern { expr, ty, name } => {
            let expr = lower_typed_expr(expr, env, scopes)?;
            let pattern_type = type_syntax_to_ir(ty, env);
            let binding = name.as_ref().map(|name| TypedBinding {
                name: name.clone(),
                ty: pattern_type,
                ownership: Ownership::Borrowed,
            });
            typed_expr(
                TypedExprKind::IsPattern {
                    expr: Box::new(expr),
                    binding,
                },
                IrType::Bool,
            )
        }
        Expr::MethodCall { target, name, args } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let mut candidates = Vec::new();
            if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                if let Some(sigs) = env.methods.get(&(type_name.clone(), name.clone())) {
                    candidates = sigs.clone();
                }
            }
            if candidates.is_empty() {
                if let Some(sigs) = env.functions.get(name) {
                    candidates = sigs.clone();
                }
            }
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            let (ty, ownership, symbol, resolution) = resolve_method_call(env, &target, name, &args)?;
            let args = if let Some(signature) =
                candidates.iter().find(|signature| signature.symbol == symbol)
            {
                pack_params_args(signature, args)
            } else {
                args
            };
            typed_expr_with_ownership(
                TypedExprKind::Call(TypedCall {
                    kind: TypedCallKind::Method {
                        target: Box::new(target),
                        name: name.clone(),
                        symbol,
                        resolution,
                    },
                    args,
                }),
                ty,
                ownership,
            )
        }
        Expr::FunctionCall { name, args } => {
            if name == "sizeof" {
                let type_name = if let Some(Expr::Var(tn)) = args.first() {
                    tn.clone()
                } else {
                    "int".to_string()
                };
                return Ok(typed_expr(
                    TypedExprKind::Call(TypedCall {
                        kind: TypedCallKind::Function {
                            name: name.clone(),
                            symbol: name.clone(),
                        },
                        args: vec![typed_expr(
                            TypedExprKind::Var(type_name.clone()),
                            IrType::Unknown(type_name),
                        )],
                    }),
                    IrType::Int,
                ));
            }
            let mut candidates = Vec::new();
            if let Some(sigs) = env.functions.get(name) {
                candidates = sigs.clone();
            }
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            let signature = env.resolve_function_call(name, &args)?;
            let args = if let Some(signature) = signature.as_ref() {
                pack_params_args(signature, args)
            } else {
                args
            };
            let ty = signature
                .as_ref()
                .map(|signature| signature.return_type.clone())
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            let ownership = signature
                .as_ref()
                .map(|signature| signature.return_ownership.clone())
                .unwrap_or_else(|| Ownership::Shared);
            typed_expr_with_ownership(
                TypedExprKind::Call(TypedCall {
                    kind: TypedCallKind::Function {
                        name: name.clone(),
                        symbol: signature
                            .as_ref()
                            .map(|signature| signature.symbol.clone())
                            .unwrap_or_else(|| name.clone()),
                    },
                    args,
                }),
                ty,
                ownership,
            )
        }
        Expr::NewObject {
            type_name,
            args,
            fields,
        } => {
            let mut candidates = Vec::new();
            if let Some(sigs) = env.constructors.get(type_name) {
                candidates = sigs.clone();
            }
            let expected_types = find_expected_types(&candidates, args);
            let args = lower_call_args(args, &expected_types, env, scopes)?;
            if type_name.starts_with("Weak_")
                || type_name.starts_with("System_WeakReference_")
                || type_name.starts_with("WeakReference_")
            {
                let inner_ty = if let Some(arg) = args.first() {
                    arg.ty.clone()
                } else {
                    IrType::Unknown("T".to_string())
                };
                let ty = IrType::Weak(Box::new(inner_ty));
                return Ok(typed_expr_with_ownership(
                    TypedExprKind::NewObject {
                        type_name: type_name.clone(),
                        constructor: None,
                        args,
                        fields: Vec::new(),
                    },
                    ty.clone(),
                    Ownership::Copy,
                ));
            }
            let constructor_signature = env.resolve_constructor_call(type_name, &args)?;
            let args = if let Some(signature) = constructor_signature.as_ref() {
                pack_params_args(signature, args)
            } else {
                args
            };
            let constructor = constructor_signature.map(|signature| signature.symbol.clone());
            let fields = fields
                .iter()
                .map(|field| {
                    Ok(TypedFieldInit {
                        name: field.name.clone(),
                        expr: lower_typed_expr(&field.expr, env, scopes)?,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let ty = if type_name == "__anonymous" {
                IrType::Unknown("anonymous".to_string())
            } else {
                match env.kinds.get(type_name) {
                    Some(TypeKind::Class) => IrType::Class(type_name.clone()),
                    Some(TypeKind::Interface) => IrType::Interface(type_name.clone()),
                    _ if type_name == "Exception" || type_name == "System.Exception" => {
                        IrType::Exception
                    }
                    _ => IrType::Struct(type_name.clone()),
                }
            };
            typed_expr_with_ownership(
                TypedExprKind::NewObject {
                    type_name: type_name.clone(),
                    constructor,
                    args,
                    fields,
                },
                ty.clone(),
                ownership_for_type(&ty),
            )
        }
        Expr::NewCollection(ty) => {
            let ty = type_syntax_to_ir(ty, env);
            typed_expr_with_ownership(
                TypedExprKind::NewCollection(ty.clone()),
                ty,
                Ownership::Owned,
            )
        }
        Expr::NewThread(entry) => typed_expr_with_ownership(
            TypedExprKind::NewThread(entry.clone()),
            IrType::Thread,
            Ownership::Owned,
        ),
        Expr::Borrow { mutable, name } => {
            let target = lookup(scopes, name)
                .map(|binding| binding.ty)
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            typed_expr_with_ownership(
                TypedExprKind::Borrow {
                    mutable: *mutable,
                    name: name.clone(),
                },
                IrType::Ref(Box::new(target)),
                Ownership::Borrowed,
            )
        }
        Expr::Await(inner) => {
            let inner = lower_typed_expr(inner, env, scopes)?;
            let ty = match &inner.ty {
                IrType::Task(result) => result.as_ref().clone(),
                other => IrType::Unknown(format!("await {other:?}")),
            };
            typed_expr(TypedExprKind::Await(Box::new(inner)), ty)
        }
        Expr::Throw(expr) => {
            let inner = lower_typed_expr_with_expected(expr, env, scopes, expected)?;
            let ty = expected.cloned().unwrap_or_else(|| inner.ty.clone());
            typed_expr_with_ownership(
                TypedExprKind::Throw(Box::new(inner)),
                ty,
                Ownership::Copy,
            )
        }
        Expr::Assign { target, value } => {
            let target = lower_typed_expr(target, env, scopes)?;
            let value = lower_typed_expr_with_expected(value, env, scopes, Some(&target.ty))?;
            typed_expr_with_ownership(
                TypedExprKind::Assign {
                    target: Box::new(target),
                    value: Box::new(value.clone()),
                },
                value.ty.clone(),
                value.ownership.clone(),
            )
        }
        Expr::Unary { op, expr } => {
            let expr = lower_typed_expr(expr, env, scopes)?;
            let ty = match op {
                UnaryOp::Not => IrType::Bool,
                UnaryOp::Neg => expr.ty.clone(),
            };
            typed_expr(
                TypedExprKind::Unary {
                    op: *op,
                    expr: Box::new(expr),
                },
                ty,
            )
        }
        Expr::IncDec {
            name,
            delta,
            prefix,
        } => {
            let target = if let Some((this_type, field_type, field_ownership)) =
                implicit_field(env, scopes, name)
            {
                let this_expr = typed_expr_with_ownership(
                    TypedExprKind::Var("this".to_string()),
                    this_type,
                    Ownership::Borrowed,
                );
                typed_expr_with_ownership(
                    TypedExprKind::Field {
                        target: Box::new(this_expr),
                        name: name.clone(),
                    },
                    field_type.clone(),
                    field_ownership,
                )
            } else {
                let ty = lookup(scopes, name)
                    .map(|binding| binding.ty.clone())
                    .unwrap_or_else(|| IrType::Unknown(name.clone()));
                let binding = lookup(scopes, name);
                let ownership = binding.map(|b| b.ownership).unwrap_or(Ownership::Shared);
                typed_expr_with_ownership(
                    TypedExprKind::Var(name.clone()),
                    ty,
                    ownership,
                )
            };
            let ty = target.ty.clone();
            typed_expr(
                TypedExprKind::IncDec {
                    target: Box::new(target),
                    delta: *delta,
                    prefix: *prefix,
                },
                ty,
            )
        }
        Expr::Lambda { params, body } => lower_lambda(params, body, expected, env, scopes)?,
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            let condition = lower_typed_expr(condition, env, scopes)?;
            let when_true = lower_typed_expr(when_true, env, scopes)?;
            let when_false = lower_typed_expr(when_false, env, scopes)?;
            let ty = if when_true.ty == IrType::Unknown("null".to_string()) {
                when_false.ty.clone()
            } else {
                when_true.ty.clone()
            };
            typed_expr_with_ownership(
                TypedExprKind::Conditional {
                    condition: Box::new(condition),
                    when_true: Box::new(when_true),
                    when_false: Box::new(when_false),
                },
                ty.clone(),
                ownership_for_type(&ty),
            )
        }
        Expr::Binary { left, op, right } => {
            let left = lower_typed_expr(left, env, scopes)?;
            let right = if *op == BinaryOp::Coalesce {
                lower_typed_expr_with_expected(right, env, scopes, Some(&left.ty))?
            } else {
                lower_typed_expr(right, env, scopes)?
            };
            let ty = if op.is_comparison() {
                IrType::Bool
            } else {
                left.ty.clone()
            };
            typed_expr(
                TypedExprKind::Binary {
                    left: Box::new(left),
                    op: *op,
                    right: Box::new(right),
                },
                ty,
            )
        }
        Expr::NamedArg { expr, .. } => lower_typed_expr(expr, env, scopes)?,
        Expr::RefArg {
            modifier,
            expr: argument,
        } => {
            if matches!(
                modifier,
                ParamModifier::Out | ParamModifier::Ref | ParamModifier::In
            ) {
                if let Expr::Var(name) = argument.as_ref() {
                    let target = lookup(scopes, name).unwrap_or(TypedBinding {
                        name: name.clone(),
                        ty: IrType::Unknown(format!("out {name}")),
                        ownership: Ownership::Shared,
                    });
                    typed_expr_with_ownership(
                        TypedExprKind::Borrow {
                            mutable: !matches!(modifier, ParamModifier::In),
                            name: name.clone(),
                        },
                        IrType::Ref(Box::new(target.ty)),
                        Ownership::Borrowed,
                    )
                } else {
                    lower_typed_expr(argument, env, scopes)?
                }
            } else {
                lower_typed_expr(argument, env, scopes)?
            }
        }
    };
    Ok(typed)
}

fn lower_expr(
    expr: &Expr,
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
) -> Result<TypedBinding, String> {
    let binding = match expr {
        Expr::Int(_) => typed_temp(IrType::Long),
        Expr::Float(_) => typed_temp(IrType::Double),
        Expr::Bool(_) => typed_temp(IrType::Bool),
        Expr::Null => typed_temp(IrType::Unknown("null".to_string())),
        Expr::String(_) => TypedBinding {
            name: "<expr>".to_string(),
            ty: IrType::String,
            ownership: Ownership::Owned,
        },
        Expr::Var(name) => lookup(scopes, name)
            .or_else(|| {
                if let Some(kind) = env.kinds.get(name) {
                    let ty = match kind {
                        TypeKind::Class => IrType::Class(name.clone()),
                        TypeKind::Interface => IrType::Interface(name.clone()),
                        TypeKind::Enum => IrType::Int,
                        _ => IrType::Struct(name.clone()),
                    };
                    Some(TypedBinding {
                        name: name.clone(),
                        ty,
                        ownership: Ownership::Shared,
                    })
                } else if name == "Exception" || name == "System.Exception" {
                    Some(TypedBinding {
                        name: name.clone(),
                        ty: IrType::Exception,
                        ownership: Ownership::Shared,
                    })
                } else {
                    env.single_function(name).map(|signature| TypedBinding {
                        name: name.clone(),
                        ty: IrType::Function {
                            params: signature.params.clone(),
                            return_type: Box::new(signature.return_type.clone()),
                        },
                        ownership: Ownership::Copy,
                    })
                }
            })
            .unwrap_or(TypedBinding {
                name: name.clone(),
                ty: IrType::Unknown(name.clone()),
                ownership: Ownership::Shared,
            }),
        Expr::Move(name) => {
            let mut binding = lookup(scopes, name).unwrap_or(TypedBinding {
                name: name.clone(),
                ty: IrType::Unknown(name.clone()),
                ownership: Ownership::Moved,
            });
            binding.ownership = Ownership::Moved;
            binding
        }
        Expr::ArrayLiteral(values) => {
            for value in values {
                lower_expr(value, env, scopes)?;
            }
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Array(Box::new(IrType::Long)),
                ownership: Ownership::Owned,
            }
        }
        Expr::NewArray {
            element_type,
            length,
            values,
        } => {
            if let Some(length) = length {
                lower_expr(length, env, scopes)?;
            }
            for value in values {
                lower_expr(value, env, scopes)?;
            }
            let element = type_syntax_to_ir(element_type, env);
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Array(Box::new(element)),
                ownership: Ownership::Owned,
            }
        }
        Expr::Index { target, index } => {
            let target = lower_expr(target, env, scopes)?;
            lower_expr(index, env, scopes)?;
            let ty = match target.ty {
                IrType::List(inner) | IrType::Array(inner) | IrType::Enumerable(inner) => *inner,
                IrType::Dictionary(_, value) => *value,
                _ => IrType::Unknown("index".to_string()),
            };
            let ownership = if ty == IrType::String {
                Ownership::Borrowed
            } else {
                ownership_for_type(&ty)
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ty,
                ownership,
            }
        }
        Expr::Field { target, name } => {
            let target = lower_expr(target, env, scopes)?;
            if target.ty == IrType::Int {
                if let Some(value) = env.enum_values.get(&(target.name.clone(), name.clone())) {
                    let _ = value;
                    return Ok(TypedBinding {
                        name: "<expr>".to_string(),
                        ty: IrType::Int,
                        ownership: Ownership::Copy,
                    });
                }
            }
            let field_info = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), field) | (IrType::Struct(type_name), field) => {
                    env.resolve_field(type_name, field)
                }
                (IrType::List(_), "Count")
                | (IrType::Dictionary(_, _), "Count")
                | (IrType::Array(_), "Length" | "Count") => Some(FieldSignature {
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::String, "Length") => Some(FieldSignature {
                    ty: IrType::Int,
                    ownership: Ownership::Copy,
                }),
                (IrType::Task(inner), "Result") => Some(FieldSignature {
                    ty: inner.as_ref().clone(),
                    ownership: ownership_for_type(inner),
                }),
                (IrType::Task(_), "IsCompleted") => Some(FieldSignature {
                    ty: IrType::Bool,
                    ownership: Ownership::Copy,
                }),
                (IrType::Exception, "Message") => Some(FieldSignature {
                    ty: IrType::String,
                    ownership: Ownership::Borrowed,
                }),
                (IrType::Weak(inner), "Target") => Some(FieldSignature {
                    ty: inner.as_ref().clone(),
                    ownership: Ownership::Borrowed,
                }),
                _ => None,
            };
            let (ty, ownership) = field_info
                .map(|field| (field.ty, field.ownership))
                .unwrap_or_else(|| {
                    let ty = IrType::Unknown(name.clone());
                    let ownership = ownership_for_type(&ty);
                    (ty, ownership)
                });
            TypedBinding {
                name: "<expr>".to_string(),
                ty,
                ownership,
            }
        }
        Expr::IsPattern { expr, .. } => {
            lower_expr(expr, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Bool,
                ownership: Ownership::Copy,
            }
        }
        Expr::MethodCall { target, name, args } => {
            let target = lower_expr(target, env, scopes)?;
            let args = args
                .iter()
                .map(|arg| lower_expr(arg, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            let ty = match (&target.ty, name.as_str()) {
                (IrType::Class(type_name), "MapGet" | "MapPost")
                    if type_name == "WebApplication" =>
                {
                    IrType::Void
                }
                (IrType::Task(inner), "GetResult") => inner.as_ref().clone(),
                (IrType::Task(inner), "GetAwaiter") => IrType::Task(inner.clone()),
                (_, "Contains") | (_, "ContainsKey") | (_, "Remove") | (_, "TryGetValue") => IrType::Bool,
                (_, "Add") | (_, "Clear") | (_, "Wait") => IrType::Void,
                (IrType::Unknown(target) | IrType::Class(target), "Run") if target == "Task" => {
                    args.first()
                        .map(|arg| IrType::Task(Box::new(arg.ty.clone())))
                        .unwrap_or_else(|| IrType::Task(Box::new(IrType::Void)))
                }
                (IrType::Unknown(target) | IrType::Class(target), "ReadAllText")
                    if target == "File" || target == "System.IO.File" =>
                {
                    IrType::String
                }
                (IrType::Unknown(target) | IrType::Class(target), "WriteAllText")
                    if target == "File" || target == "System.IO.File" =>
                {
                    IrType::Void
                }
                _ => IrType::Unknown(name.clone()),
            };
            let ownership = if let IrType::Class(type_name)
            | IrType::Struct(type_name)
            | IrType::Interface(type_name) = &target.ty
            {
                env.resolve_method(type_name, name, &args.iter().map(|arg| arg.ty.clone()).collect::<Vec<_>>())
                    .map(|signature| signature.return_ownership.clone())
                    .unwrap_or_else(|| ownership_for_type(&ty))
            } else {
                ownership_for_type(&ty)
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership,
                ty,
            }
        }
        Expr::FunctionCall { name, args } => {
            if name == "sizeof" {
                return Ok(TypedBinding {
                    name: "<expr>".to_string(),
                    ownership: Ownership::Copy,
                    ty: IrType::Int,
                });
            }
            let args = args
                .iter()
                .map(|arg| lower_expr(arg, env, scopes))
                .collect::<Result<Vec<_>, _>>()?;
            let signature = env.resolve_function(
                name,
                &args.iter().map(|arg| arg.ty.clone()).collect::<Vec<_>>(),
            );
            let ty = signature
                .as_ref()
                .map(|signature| signature.return_type.clone())
                .unwrap_or_else(|| IrType::Unknown(name.clone()));
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: signature
                    .as_ref()
                    .map(|signature| signature.return_ownership.clone())
                    .unwrap_or_else(|| Ownership::Shared),
                ty,
            }
        }
        Expr::NewObject {
            type_name,
            args,
            fields,
        } => {
            for arg in args {
                lower_expr(arg, env, scopes)?;
            }
            for field in fields {
                lower_expr(&field.expr, env, scopes)?;
            }
            let ty = match env.kinds.get(type_name) {
                Some(TypeKind::Class) => IrType::Class(type_name.clone()),
                _ if type_name == "Exception" || type_name == "System.Exception" => {
                    IrType::Exception
                }
                _ => IrType::Struct(type_name.clone()),
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: ownership_for_type(&ty),
                ty,
            }
        }
        Expr::NewCollection(ty) => {
            let ty = type_syntax_to_ir(ty, env);
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: Ownership::Owned,
                ty,
            }
        }
        Expr::NewThread(_) => TypedBinding {
            name: "<expr>".to_string(),
            ty: IrType::Thread,
            ownership: Ownership::Owned,
        },
        Expr::Borrow { name, .. } => {
            let target =
                lookup(scopes, name).unwrap_or_else(|| typed_temp(IrType::Unknown(name.clone())));
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Ref(Box::new(target.ty)),
                ownership: Ownership::Borrowed,
            }
        }
        Expr::Await(inner) => {
            let inner = lower_expr(inner, env, scopes)?;
            let ty = match inner.ty {
                IrType::Task(result) => *result,
                other => IrType::Unknown(format!("await {other:?}")),
            };
            TypedBinding {
                name: "<expr>".to_string(),
                ownership: ownership_for_type(&ty),
                ty,
            }
        }
        Expr::Throw(expr) => {
            lower_expr(expr, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: IrType::Unknown("throw".to_string()),
                ownership: Ownership::Copy,
            }
        }
        Expr::Assign { target, value } => {
            lower_expr(target, env, scopes)?;
            let value = lower_expr(value, env, scopes)?;
            TypedBinding {
                name: "<expr>".to_string(),
                ty: value.ty,
                ownership: value.ownership,
            }
        }
        Expr::Unary { expr, .. } => {
            lower_expr(expr, env, scopes)?;
            typed_temp(IrType::Bool)
        }
        Expr::Lambda { body, .. } => {
            lower_expr(body, env, scopes)?;
            typed_temp(IrType::Unknown("lambda".to_string()))
        }
        Expr::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            lower_expr(condition, env, scopes)?;
            let when_true = lower_expr(when_true, env, scopes)?;
            let when_false = lower_expr(when_false, env, scopes)?;
            if when_true.ty == IrType::Unknown("null".to_string()) {
                typed_temp(when_false.ty)
            } else {
                typed_temp(when_true.ty)
            }
        }
        Expr::Binary { left, op, right } => {
            let left = lower_expr(left, env, scopes)?;
            lower_expr(right, env, scopes)?;
            let ty = if op.is_comparison() {
                IrType::Bool
            } else {
                left.ty
            };
            typed_temp(ty)
        }
        Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => lower_expr(expr, env, scopes)?,
        Expr::IncDec { name, .. } => {
            let ty = if let Some((_, field_ty, _)) = implicit_field(env, scopes, name) {
                field_ty
            } else {
                lookup(scopes, name)
                    .map(|binding| binding.ty)
                    .unwrap_or_else(|| IrType::Unknown(name.clone()))
            };
            typed_temp(ty)
        }
    };
    Ok(binding)
}

fn lookup(scopes: &[HashMap<String, TypedBinding>], name: &str) -> Option<TypedBinding> {
    scopes
        .iter()
        .rev()
        .find_map(|scope| scope.get(name).cloned())
}

fn implicit_field(
    env: &TypeEnv,
    scopes: &[HashMap<String, TypedBinding>],
    name: &str,
) -> Option<(IrType, IrType, Ownership)> {
    let this_type = lookup(scopes, "this")?.ty;
    let owner = match &this_type {
        IrType::Class(name) | IrType::Struct(name) => name.as_str(),
        IrType::Ref(inner) => match inner.as_ref() {
            IrType::Struct(name) | IrType::Class(name) => name.as_str(),
            _ => return None,
        },
        _ => return None,
    };
    let field_type = env.resolve_field(owner, name)?;
    Some((this_type, field_type.ty, field_type.ownership))
}

fn collect_condition_bindings(expr: &Expr, env: &TypeEnv) -> Vec<TypedBinding> {
    let mut bindings = Vec::new();
    collect_condition_bindings_inner(expr, env, &mut bindings);
    bindings
}

fn collect_condition_bindings_inner(expr: &Expr, env: &TypeEnv, bindings: &mut Vec<TypedBinding>) {
    match expr {
        Expr::IsPattern {
            ty,
            name: Some(name),
            ..
        } => {
            let ty = type_syntax_to_ir(ty, env);
            bindings.push(TypedBinding {
                name: name.clone(),
                ty,
                ownership: Ownership::Borrowed,
            });
        }
        Expr::Binary { left, right, .. } => {
            collect_condition_bindings_inner(left, env, bindings);
            collect_condition_bindings_inner(right, env, bindings);
        }
        Expr::MethodCall { target, args, .. } => {
            collect_condition_bindings_inner(target, env, bindings);
            for arg in args {
                collect_condition_bindings_inner(arg, env, bindings);
            }
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_condition_bindings_inner(arg, env, bindings);
            }
        }
        Expr::RefArg {
            modifier: ParamModifier::Out,
            expr,
        } => {
            if let Expr::Var(name) = expr.as_ref() {
                bindings.push(TypedBinding {
                    name: name.clone(),
                    ty: IrType::Unknown(format!("out {name}")),
                    ownership: Ownership::Shared,
                });
            }
        }
        Expr::Unary { expr, .. } => collect_condition_bindings_inner(expr, env, bindings),
        _ => {}
    }
}

fn typed_temp(ty: IrType) -> TypedBinding {
    TypedBinding {
        name: "<expr>".to_string(),
        ownership: ownership_for_type(&ty),
        ty,
    }
}

fn typed_expr(kind: TypedExprKind, ty: IrType) -> TypedExpr {
    let ownership = ownership_for_type(&ty);
    typed_expr_with_ownership(kind, ty, ownership)
}

fn typed_expr_with_ownership(kind: TypedExprKind, ty: IrType, ownership: Ownership) -> TypedExpr {
    let drop_kind = drop_kind_for_type(&ty, &ownership);
    TypedExpr {
        kind,
        ty,
        ownership,
        drop_kind,
    }
}

fn allows_shared_reference_flow(_env: &TypeEnv, ty: &IrType) -> bool {
    matches!(ty, IrType::Class(_) | IrType::Interface(_))
}

fn is_csharp_shared_handle_name(name: &str) -> bool {
    let simple = name.rsplit('.').next().unwrap_or(name);
    let looks_like_interface = simple
        .strip_prefix('I')
        .and_then(|rest| rest.chars().next())
        .is_some_and(|ch| ch.is_ascii_uppercase());
    looks_like_interface
        || matches!(
            simple,
            "IServiceCollection"
                | "ServiceCollection"
                | "ControllerBase"
                | "ActionResult"
                | "WebApplication"
                | "WebApplicationBuilder"
                | "HttpContext"
                | "HttpRequest"
                | "HttpResponse"
        )
        || simple.ends_with("Options")
        || simple.ends_with("Settings")
        || simple.ends_with("Configuration")
        || simple.ends_with("Manager")
        || simple.ends_with("Builder")
        || simple.ends_with("Collection")
        || simple.ends_with("Logger")
        || simple.ends_with("Provider")
        || simple.ends_with("Context")
        || simple.ends_with("Factory")
}

fn derives_from(env: &TypeEnv, name: &str, expected_base: &str) -> bool {
    let Some(bases) = env.bases.get(name) else {
        return false;
    };
    bases.iter().any(|base| {
        base == expected_base
            || base.rsplit('.').next() == Some(expected_base)
            || derives_from(env, base, expected_base)
    })
}

fn resolve_method_call(
    env: &TypeEnv,
    target: &TypedExpr,
    name: &str,
    args: &[TypedExpr],
) -> Result<(IrType, Ownership, String, CallResolution), String> {
    if matches!(target.ty, IrType::Unknown(_)) {
        if let Some(signature) = env.resolve_function_call(name, args)? {
            return Ok((
                signature.return_type.clone(),
                signature.return_ownership.clone(),
                signature.symbol.clone(),
                CallResolution::StaticFunction,
            ));
        }
    }
    if matches!(target.ty, IrType::Class(_)) {
        if let Some(signature) = env.resolve_function_call(name, args)? {
            return Ok((
                signature.return_type.clone(),
                signature.return_ownership.clone(),
                signature.symbol.clone(),
                CallResolution::StaticFunction,
            ));
        }
    }
    match (&target.ty, name) {
        (IrType::Class(type_name), "MapGet" | "MapPost") if type_name == "WebApplication" => {
            if let [TypedExpr {
                kind: TypedExprKind::String(path),
                ..
            }, TypedExpr {
                kind: TypedExprKind::FunctionSymbol(handler),
                ty:
                    IrType::Function {
                        params,
                        return_type,
                    },
                ..
            }] = args
            {
                if params.is_empty() && return_type.as_ref() == &IrType::String {
                    return Ok((
                        IrType::Void,
                        Ownership::Copy,
                        name.to_string(),
                        CallResolution::EndpointHandlerRegistration {
                            http_method: if name == "MapGet" { "GET" } else { "POST" }.to_string(),
                            path: path.clone(),
                            handler: handler.clone(),
                        },
                    ));
                }
            }
            if let Some(signature) = env.resolve_method_call(type_name, name, args)? {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else if let Some(signature) =
                env.resolve_extension_method_call(type_name, name, target, args)?
            {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else {
                Ok((
                    IrType::Unknown(name.to_string()),
                    Ownership::Shared,
                    name.to_string(),
                    CallResolution::InstanceMethod,
                ))
            }
        }
        (IrType::Unknown(target) | IrType::Class(target), "Run") if target == "Task" => {
            let result = args
                .first()
                .map(|arg| arg.ty.clone())
                .unwrap_or(IrType::Void);
            Ok((
                IrType::Task(Box::new(result)),
                Ownership::Owned,
                name.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "FromResult")
            if target == "Task"
                || target == "ValueTask"
                || target == "System.Threading.Tasks.Task"
                || target == "System.Threading.Tasks.ValueTask" =>
        {
            let result = args
                .first()
                .map(|arg| arg.ty.clone())
                .unwrap_or(IrType::Void);
            Ok((
                IrType::Task(Box::new(result)),
                Ownership::Owned,
                name.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "ReadAllText")
            if target == "File" || target == "System.IO.File" =>
        {
            Ok((
                IrType::String,
                Ownership::Owned,
                "System_IO_File_ReadAllText".to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "WriteAllText")
            if target == "File" || target == "System.IO.File" =>
        {
            Ok((
                IrType::Void,
                Ownership::Copy,
                "System_IO_File_WriteAllText".to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (IrType::Unknown(target) | IrType::Class(target), "WriteLine")
            if target == "Console" || target == "System.Console" =>
        {
            let arg_ty = args.first().map(|arg| &arg.ty).unwrap_or(&IrType::Void);
            let symbol = match arg_ty {
                IrType::String => "System_Console_WriteLine_String",
                IrType::Byte | IrType::Short | IrType::Int | IrType::Long | IrType::UInt => {
                    "System_Console_WriteLine_I64"
                }
                IrType::Double => "System_Console_WriteLine_Double",
                IrType::Bool => "System_Console_WriteLine_Bool",
                _ => "System_Console_WriteLine_String",
            };
            Ok((
                IrType::Void,
                Ownership::Copy,
                symbol.to_string(),
                CallResolution::StaticFunction,
            ))
        }
        (
            IrType::Class(type_name) | IrType::Struct(type_name) | IrType::Interface(type_name),
            method_name,
        ) => {
            if let Some(signature) = env.resolve_method_call(type_name, method_name, args)? {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else if let Some(signature) =
                env.resolve_extension_method_call(type_name, method_name, target, args)?
            {
                Ok((
                    signature.return_type.clone(),
                    signature.return_ownership.clone(),
                    signature.symbol.clone(),
                    CallResolution::InstanceMethod,
                ))
            } else {
                Ok((
                    IrType::Unknown(name.to_string()),
                    Ownership::Shared,
                    name.to_string(),
                    CallResolution::InstanceMethod,
                ))
            }
        }
        (IrType::Task(inner), "GetResult") => Ok((
            inner.as_ref().clone(),
            ownership_for_type(inner),
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Task(inner), "GetAwaiter") => Ok((
            IrType::Task(inner.clone()),
            Ownership::Owned,
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Task(inner), "AsTask") => Ok((
            IrType::Task(inner.clone()),
            Ownership::Owned,
            name.to_string(),
            CallResolution::TaskMethod,
        )),
        (IrType::Weak(_), "TryGetTarget") => {
            Ok((IrType::Bool, Ownership::Copy, name.to_string(), CallResolution::WeakMethod))
        }
        (IrType::List(element), "ToArray") => Ok((
            IrType::Array(Box::new(element.as_ref().clone())),
            Ownership::Owned,
            name.to_string(),
            CallResolution::CollectionMethod,
        )),
        (IrType::List(_), "Contains")
        | (IrType::Dictionary(_, _), "ContainsKey" | "Remove" | "TryGetValue") => {
            Ok((
                IrType::Bool,
                Ownership::Copy,
                name.to_string(),
                CallResolution::CollectionMethod,
            ))
        }
        (IrType::List(_), "Add" | "Clear") | (IrType::Dictionary(_, _), "Add" | "Clear") => Ok((
            IrType::Void,
            Ownership::Copy,
            name.to_string(),
            CallResolution::CollectionMethod,
        )),
        (_, "Wait") => Ok((IrType::Void, Ownership::Copy, name.to_string(), CallResolution::TaskMethod)),
        _ => Ok((
            IrType::Unknown(name.to_string()),
            Ownership::Shared,
            name.to_string(),
            CallResolution::Unknown,
        )),
    }
}

fn type_syntax_to_ir(ty: &TypeSyntax, env: &TypeEnv) -> IrType {
    type_syntax_to_ir_with_subst(ty, env, &HashMap::new())
}

fn type_syntax_to_ir_with_subst(
    ty: &TypeSyntax,
    env: &TypeEnv,
    subst: &HashMap<String, TypeSyntax>,
) -> IrType {
    match ty {
        TypeSyntax::GenericNamed { name, args }
            if name == "own"
                || name == "shared"
                || name == "borrow"
                || name == "view"
                || name == "System.Ownership.own"
                || name == "System.Ownership.shared"
                || name == "System.Ownership.borrow"
                || name == "System.Ownership.view" =>
        {
            if let Some(first_arg) = args.first() {
                type_syntax_to_ir_with_subst(first_arg, env, subst)
            } else {
                IrType::Unknown("T".to_string())
            }
        }
        TypeSyntax::Scalar(scalar) => scalar_to_ir(*scalar),
        TypeSyntax::String => IrType::String,
        TypeSyntax::Array(inner) => {
            IrType::Array(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Ref(inner) => {
            IrType::Ref(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Named(name) if name == "Exception" || name == "System.Exception" => {
            IrType::Exception
        }
        TypeSyntax::Named(name) if name == "Action" || name == "System.Action" => {
            IrType::Function {
                params: vec![],
                return_type: Box::new(IrType::Void),
            }
        }
        TypeSyntax::Named(name) => {
            if let Some(replacement) = subst.get(name) {
                return type_syntax_to_ir_with_subst(replacement, env, subst);
            }
            if let Some(signature) = env.delegates.get(name) {
                return delegate_signature_to_ir(signature, &[], env, subst);
            }
            match env.kinds.get(name) {
                Some(TypeKind::Class) => IrType::Class(name.clone()),
                Some(TypeKind::Interface) => IrType::Interface(name.clone()),
                Some(TypeKind::Enum) => IrType::Int,
                Some(_) => IrType::Struct(name.clone()),
                None => IrType::Unknown(name.clone()),
            }
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "Weak"
                || name == "WeakReference"
                || name == "System.WeakReference"
                || name == "System.Ownership.weakref"
                || name == "weakref" =>
        {
            if let Some(first_arg) = args.first() {
                IrType::Weak(Box::new(type_syntax_to_ir_with_subst(first_arg, env, subst)))
            } else {
                IrType::Weak(Box::new(IrType::Unknown("T".to_string())))
            }
        }
        TypeSyntax::GenericNamed { name, args } if name == "Func" || name == "System.Func" => {
            if args.is_empty() {
                IrType::Function {
                    params: vec![],
                    return_type: Box::new(IrType::Void),
                }
            } else {
                let (params_args, ret_arg) = args.split_at(args.len() - 1);
                let params = params_args
                    .iter()
                    .map(|arg| type_syntax_to_ir_with_subst(arg, env, subst))
                    .collect();
                let return_type = Box::new(type_syntax_to_ir_with_subst(&ret_arg[0], env, subst));
                IrType::Function {
                    params,
                    return_type,
                }
            }
        }
        TypeSyntax::GenericNamed { name, args } if name == "Action" || name == "System.Action" => {
            let params = args
                .iter()
                .map(|arg| type_syntax_to_ir_with_subst(arg, env, subst))
                .collect();
            IrType::Function {
                params,
                return_type: Box::new(IrType::Void),
            }
        }
        TypeSyntax::GenericNamed { name, args } => {
            if let Some(replacement) = subst.get(name) {
                return type_syntax_to_ir_with_subst(replacement, env, subst);
            }
            if let Some(signature) = env.delegates.get(name) {
                return delegate_signature_to_ir(signature, args, env, subst);
            }
            if name == "IReadOnlyDictionary" || name == "System.Collections.Generic.IReadOnlyDictionary" {
                if args.len() == 2 {
                    return IrType::Dictionary(
                        Box::new(type_syntax_to_ir_with_subst(&args[0], env, subst)),
                        Box::new(type_syntax_to_ir_with_subst(&args[1], env, subst)),
                    );
                }
            }
            match env.kinds.get(name) {
                Some(TypeKind::Class) => IrType::Class(name.clone()),
                Some(TypeKind::Interface) => IrType::Interface(name.clone()),
                Some(TypeKind::Enum) => IrType::Int,
                Some(_) => IrType::Struct(name.clone()),
                None => IrType::Unknown(format!(
                    "{}<{}>",
                    name,
                    args.iter()
                        .map(|arg| ir_symbol_suffix(&type_syntax_to_ir_with_subst(arg, env, subst)))
                        .collect::<Vec<_>>()
                        .join(",")
                )),
            }
        }
        TypeSyntax::List(inner) => {
            IrType::List(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Dictionary(key, value) => IrType::Dictionary(
            Box::new(type_syntax_to_ir_with_subst(key, env, subst)),
            Box::new(type_syntax_to_ir_with_subst(value, env, subst)),
        ),
        TypeSyntax::IEnumerable(inner) => {
            IrType::Enumerable(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Thread => IrType::Thread,
        TypeSyntax::Task(inner) => {
            IrType::Task(Box::new(type_syntax_to_ir_with_subst(inner, env, subst)))
        }
        TypeSyntax::Nullable(inner) => type_syntax_to_ir_with_subst(inner, env, subst),
        TypeSyntax::Void => IrType::Void,
    }
}

fn ownership_for_declared_type_syntax(ty: &TypeSyntax, env: &TypeEnv) -> Ownership {
    match ty {
        TypeSyntax::GenericNamed { name, args }
            if name == "own" || name == "System.Ownership.own" =>
        {
            let _ = args;
            Ownership::Owned
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "shared" || name == "System.Ownership.shared" =>
        {
            let _ = args;
            Ownership::Shared
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "borrow" || name == "System.Ownership.borrow" =>
        {
            let _ = args;
            Ownership::Borrowed
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "view" || name == "System.Ownership.view" =>
        {
            let _ = args;
            Ownership::View
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "weakref" || name == "System.Ownership.weakref" =>
        {
            let _ = args;
            Ownership::Copy
        }
        TypeSyntax::GenericNamed { name, args }
            if name == "IReadOnlyDictionary"
                || name == "System.Collections.Generic.IReadOnlyDictionary" =>
        {
            let _ = args;
            Ownership::View
        }
        _ => ownership_for_type(&type_syntax_to_ir(ty, env)),
    }
}

fn delegate_signature_to_ir(
    signature: &DelegateSignature,
    args: &[TypeSyntax],
    env: &TypeEnv,
    outer_subst: &HashMap<String, TypeSyntax>,
) -> IrType {
    let mut subst = outer_subst.clone();
    for (index, generic_name) in signature.generic_params.iter().enumerate() {
        let replacement = args
            .get(index)
            .cloned()
            .unwrap_or_else(|| TypeSyntax::Named(generic_name.clone()));
        subst.insert(generic_name.clone(), replacement);
    }
    let params = signature
        .params
        .iter()
        .map(|param| type_syntax_to_ir_with_subst(param, env, &subst))
        .collect();
    let return_type = Box::new(type_syntax_to_ir_with_subst(&signature.return_type, env, &subst));
    IrType::Function {
        params,
        return_type,
    }
}

fn scalar_to_ir(scalar: ScalarType) -> IrType {
    match scalar {
        ScalarType::Bool => IrType::Bool,
        ScalarType::Byte => IrType::Byte,
        ScalarType::Short => IrType::Short,
        ScalarType::I32 => IrType::Int,
        ScalarType::I64 => IrType::Long,
        ScalarType::U32 => IrType::UInt,
        ScalarType::F64 => IrType::Double,
        ScalarType::Decimal => IrType::Decimal,
    }
}

fn ownership_for_type(ty: &IrType) -> Ownership {
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
        IrType::Function { .. } => Ownership::Shared,
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
            IrType::Class(_) | IrType::Interface(_) => DropKind::DropClass,
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
