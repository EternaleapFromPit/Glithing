#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

use crate::ast::*;
use crate::tir::*;

mod delegates;
mod entrypoints;
mod endpoint;
mod objects;
mod freevars;
mod generics;
mod helpers;
mod support;
mod drop_glue;
mod stmt_expr;
mod collections;
mod runtime;
mod tasks;

use self::freevars::*;
use self::generics::*;
use self::helpers::*;
use self::support::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LlType {
    I1,
    I8,
    I16,
    I32,
    I64,
    Double,
    Ptr,
    Void,
}

impl LlType {
    fn as_ir(&self) -> &'static str {
        match self {
            LlType::I1 => "i1",
            LlType::I8 => "i8",
            LlType::I16 => "i16",
            LlType::I32 => "i32",
            LlType::I64 => "i64",
            LlType::Double => "double",
            LlType::Ptr => "ptr",
            LlType::Void => "void",
        }
    }

    fn default_value(&self) -> &'static str {
        match self {
            LlType::I1 | LlType::I8 | LlType::I16 | LlType::I32 | LlType::I64 => "0",
            LlType::Double => "0.0",
            LlType::Ptr => "null",
            LlType::Void => "",
        }
    }

    fn is_integer(&self) -> bool {
        matches!(
            self,
            LlType::I1 | LlType::I8 | LlType::I16 | LlType::I32 | LlType::I64
        )
    }
}

#[derive(Debug, Clone)]
struct LlVar {
    ptr: String,
    ty: LlType,
    ir_ty: IrType,
    drop_kind: DropKind,
}

#[derive(Debug, Clone)]
struct LlValue {
    value: String,
    ty: LlType,
}

#[derive(Debug, Clone)]
struct LlFunctionSig {
    return_type: LlType,
    params: Vec<LlType>,
    required_params: usize,
}

#[derive(Debug, Clone)]
struct LlField {
    index: usize,
    ty: IrType,
    drop_kind: DropKind,
}

#[derive(Debug, Clone)]
struct LlObjectType {
    name: String,
    kind: TypeKind,
    bases: Vec<String>,
    fields: HashMap<String, LlField>,
    constructor: Option<String>,
    constructor_params: Vec<IrType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ServiceLifetime {
    Singleton,
    Transient,
    Scoped,
}

#[derive(Debug, Clone)]
struct ServiceRegistration {
    lifetime: ServiceLifetime,
    implementation_name: String,
    source_expr: Option<TypedExpr>,
    stored_ty: IrType,
}

fn is_string_like_type(ty: &IrType) -> bool {
    match ty {
        IrType::String => true,
        IrType::Class(name) | IrType::Struct(name) | IrType::Unknown(name) => {
            name == "string" || name == "String" || name == "System.String"
        }
        _ => false,
    }
}

fn is_bool_like_type(ty: &IrType) -> bool {
    match ty {
        IrType::Bool => true,
        IrType::Class(name) | IrType::Struct(name) | IrType::Unknown(name) => {
            matches!(base_type_name(name), "bool" | "Boolean" | "System.Boolean")
        }
        _ => false,
    }
}

fn is_task_surface_type(ty: &IrType) -> bool {
    matches!(
        ty,
        IrType::Unknown(name) | IrType::Class(name)
            if matches!(
                base_type_name(name),
                "Task" | "ValueTask" | "System.Threading.Tasks.Task" | "System.Threading.Tasks.ValueTask"
            )
    )
}

fn is_object_surface_type(ty: &IrType) -> bool {
    matches!(
        ty,
        IrType::Unknown(name) | IrType::Class(name)
            if matches!(base_type_name(name), "object" | "Object")
    )
}

fn is_service_provider_lookup_type(ty: &IrType) -> bool {
    matches!(
        ty,
        IrType::Unknown(name) | IrType::Class(name) | IrType::Interface(name)
            if matches!(
                base_type_name(name),
                "ServiceProvider"
                    | "IServiceProvider"
                    | "Microsoft.Extensions.DependencyInjection.ServiceProvider"
                    | "Microsoft.Extensions.DependencyInjection.IServiceProvider"
            )
    )
}

fn is_service_collection_type(ty: &IrType) -> bool {
    matches!(
        ty,
        IrType::Unknown(name) | IrType::Class(name) | IrType::Interface(name)
            if matches!(
                base_type_name(name),
                "ServiceCollection"
                    | "IServiceCollection"
                    | "Microsoft.Extensions.DependencyInjection.ServiceCollection"
                    | "Microsoft.Extensions.DependencyInjection.IServiceCollection"
            )
    )
}

fn is_null_literal_expr(expr: &TypedExpr) -> bool {
    matches!(expr.kind, TypedExprKind::Null)
}

fn should_drop_argument_after_call(expr: &TypedExpr) -> bool {
    matches!(
        expr.kind,
        TypedExprKind::FunctionSymbol(_)
            | TypedExprKind::Lambda { .. }
            | TypedExprKind::Call(_)
            | TypedExprKind::Await(_)
            | TypedExprKind::NewObject { .. }
            | TypedExprKind::NewCollection(_)
            | TypedExprKind::NewArray { .. }
            | TypedExprKind::ArrayLiteral(_)
            | TypedExprKind::NewThread(_)
            | TypedExprKind::Conditional { .. }
            | TypedExprKind::NullableSome(_)
    )
}

fn value_uses_boxed_scalar_temporary(value: &LlValue, expected: &LlType) -> bool {
    *expected == LlType::Ptr && (value.ty.is_integer() || value.ty == LlType::Double)
}

fn boxed_scalar_tag(ty: &IrType) -> Option<i32> {
    match ty {
        IrType::Bool => Some(1),
        IrType::Byte => Some(2),
        IrType::Short => Some(3),
        IrType::Int => Some(4),
        IrType::UInt => Some(5),
        IrType::Long => Some(6),
        IrType::Double => Some(7),
        IrType::Decimal => Some(8),
        _ => None,
    }
}

fn is_weak_reference_type_name(name: &str) -> bool {
    name.starts_with("Weak_")
        || name.starts_with("System_WeakReference_")
        || name.starts_with("WeakReference_")
        || name.starts_with("Weak<")
        || name.starts_with("WeakReference<")
        || name.starts_with("System.WeakReference<")
}

pub(crate) struct LlvmEmitter {
    type_defs: Vec<String>,
    globals: Vec<String>,
    body: String,
    vars: HashMap<String, LlVar>,
    functions: HashMap<String, LlFunctionSig>,
    specialized_symbols: HashMap<(String, Vec<IrType>), String>,
    specialized_instance_symbols: HashMap<(String, String), String>,
    owner_generic_templates: HashMap<String, TypedFunction>,
    specialized_functions: Vec<TypedFunction>,
    object_types: HashMap<String, LlObjectType>,
    nullable_layouts: HashSet<String>,
    endpoint_handlers: Vec<EndpointHandlerBinding>,
    service_registrations: HashMap<String, String>,
    service_collection_registrations: HashMap<String, HashMap<String, ServiceRegistration>>,
    service_provider_registrations: HashMap<String, HashMap<String, ServiceRegistration>>,
    drop_order: Vec<String>,
    tmp: usize,
    label: usize,
    string_id: usize,
    lambda_id: usize,
    current_return: LlType,
    current_is_main: bool,
    current_unwind_label: String,
    exception_handler: Option<String>,
    loop_targets: Vec<(String, String)>,
    async_state_pc_ptr: Option<String>,
    async_suspend_index: usize,
    entry_insert_pos: Option<usize>,
    terminated: bool,
    startup: Option<TypedStartup>,
}

impl LlvmEmitter {
    pub(crate) fn emit_typed_program(program: &TypedProgram) -> Result<String, String> {
        let mut emitter = Self {
            type_defs: vec![
                "%glitch.array = type { i64, ptr }\n".to_string(),
                "%glitch.list = type { i64, i64, ptr }\n".to_string(),
                "%glitch.dict = type { i64, i64, ptr, ptr }\n".to_string(),
                "%glitch.string_node = type { i64, i64, [0 x i8] }\n".to_string(),
                "%glitch.delegate = type { i64, ptr, ptr, ptr }\n".to_string(),
            ],
            globals: Vec::new(),
            body: String::new(),
            vars: HashMap::new(),
            functions: HashMap::new(),
            specialized_symbols: HashMap::new(),
            specialized_instance_symbols: HashMap::new(),
            owner_generic_templates: HashMap::new(),
            specialized_functions: Vec::new(),
            object_types: HashMap::new(),
            nullable_layouts: HashSet::new(),
            endpoint_handlers: program.endpoint_handlers.clone(),
            service_registrations: HashMap::new(),
            service_collection_registrations: HashMap::new(),
            service_provider_registrations: HashMap::new(),
            drop_order: Vec::new(),
            tmp: 0,
            label: 0,
            string_id: 0,
            lambda_id: 0,
            current_return: LlType::Void,
            current_is_main: false,
            current_unwind_label: String::new(),
            exception_handler: None,
            loop_targets: Vec::new(),
            async_state_pc_ptr: None,
            async_suspend_index: 0,
            entry_insert_pos: None,
            terminated: false,
            startup: program.startup.clone(),
        };
        for ty in &program.types {
            if ty.generic_params.is_empty() {
                emitter.register_object_type(ty);
            }
        }
        emitter.register_rc_instantiations(program);
        emitter.register_generic_type_specializations(program)?;
        emitter.register_generic_specializations(program)?;
        for function in &program.functions {
            emitter.register_function(function);
        }
        emitter.functions.insert(
            "System_IO_File_ReadAllText".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "System_IO_File_WriteAllText".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::Ptr, LlType::Ptr],
                required_params: 2,
            },
        );
        emitter.functions.insert(
            "System_String_Substring_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr, LlType::I64],
                required_params: 2,
            },
        );
        emitter.functions.insert(
            "System_String_TrimEnd_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr, LlType::Ptr],
                required_params: 2,
            },
        );
        emitter.functions.insert(
            "System_String_ToLower_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "System_String_ToLowerInvariant_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "System_String_Replace_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr, LlType::Ptr, LlType::Ptr],
                required_params: 3,
            },
        );
        emitter.functions.insert(
            "System_String_Trim_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "System_String_Split_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr, LlType::Ptr],
                required_params: 2,
            },
        );
        emitter.functions.insert(
            "System_String_Contains_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::I1,
                params: vec![LlType::Ptr, LlType::Ptr],
                required_params: 2,
            },
        );
        emitter.functions.insert(
            "System_String_TrimStart_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr, LlType::Ptr],
                required_params: 2,
            },
        );
        emitter.functions.insert(
            "System_Array_Empty_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![],
                required_params: 0,
            },
        );
        emitter.functions.insert(
            "System_Console_WriteLine_String".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::Ptr],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "System_Console_WriteLine_I64".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::I64],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "System_Console_WriteLine_Double".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::Double],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "System_Console_WriteLine_Bool".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::I1],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "JsonSerializer_Serialize_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "JsonSerializer_Deserialize_Native".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr],
                required_params: 1,
            },
        );
        emitter.functions.insert(
            "GlitchRestHost_read_env_int".to_string(),
            LlFunctionSig {
                return_type: LlType::I32,
                params: vec![LlType::Ptr, LlType::I32],
                required_params: 2,
            },
        );
        for ty in &program.types {
            if !ty.generic_params.is_empty() {
                continue;
            }
            for constructor in &ty.constructors {
                emitter.register_function(constructor);
            }
            for method in &ty.methods {
                emitter.register_function(method);
            }
        }
        let specialized_functions = emitter.specialized_functions.clone();
        for function in &specialized_functions {
            emitter.register_function(function);
        }
        emitter.emit_drop_glue();
        let mut emitted_symbols = HashSet::new();
        for ty in &program.types {
            if !ty.generic_params.is_empty() {
                continue;
            }
            if ty.kind == TypeKind::Interface && ty.methods.iter().all(|method| method.body.is_empty()) {
                continue;
            }
            for constructor in &ty.constructors {
                if emitted_symbols.insert(constructor.symbol.clone()) {
                    emitter.emit_typed_function(constructor)?;
                }
            }
            for method in &ty.methods {
                if emitted_symbols.insert(method.symbol.clone()) {
                    emitter.emit_typed_function(method)?;
                }
            }
        }
        for function in &program.functions {
            if function.is_extern {
                emitter.emit_external_declaration(function);
            } else {
                if emitted_symbols.insert(function.symbol.clone()) {
                    emitter.emit_typed_function(function)?;
                }
            }
        }
        for function in &specialized_functions {
            if emitted_symbols.insert(function.symbol.clone()) {
                emitter.emit_typed_function(function)?;
            }
        }
        emitter.emit_startup_wrapper()?;
        emitter.emit_web_application_handle_wrapper(program)?;
        emitter.emit_endpoint_dispatch()?;
        emitter.finish_module()
    }

    fn register_object_type(&mut self, ty: &TypedType) {
        let has_ref_count = matches!(ty.kind, TypeKind::Class | TypeKind::Interface);
        let field_offset = if has_ref_count { 2 } else { 0 };
        let mut fields = HashMap::new();
        for (field_index, field) in ty.fields.iter().enumerate() {
            fields.insert(
                field.name.clone(),
                LlField {
                    index: field_index + field_offset,
                    ty: field.ty.clone(),
                    drop_kind: field.drop_kind(),
                },
            );
        }
        let mut layout = Vec::new();
        if has_ref_count {
            layout.push("i64".to_string());
            layout.push("ptr".to_string());
        }
        layout.extend(
            ty.fields
                .iter()
                .map(|field| llvm_ir_type(&field.ty).as_ir().to_string()),
        );
        if layout.is_empty() {
            layout.push("i8".to_string());
        }
        self.type_defs.push(format!(
            "%{} = type {{ {} }}\n",
            llvm_object_name(&qualified_name(
                &ty.namespace,
                &ty.name,
                ty.generic_params.len(),
                ty.symbol_id,
            )),
            layout.join(", ")
        ));
        self.object_types.insert(
            ty.name.clone(),
            LlObjectType {
                name: qualified_name(
                    &ty.namespace,
                    &ty.name,
                    ty.generic_params.len(),
                    ty.symbol_id,
                ),
                kind: ty.kind,
                bases: ty.bases.clone(),
                fields,
                constructor: ty
                    .constructors
                    .first()
                    .map(|constructor| constructor.symbol.clone()),
                constructor_params: ty
                    .constructors
                    .first()
                    .map(|constructor| {
                        constructor
                            .params
                            .iter()
                            .skip(1)
                            .map(|param| param.ty.clone())
                            .collect()
                    })
                    .unwrap_or_default(),
            },
        );
    }

    fn register_rc_instantiations(&mut self, program: &TypedProgram) {
        let mut instantiations = HashSet::new();
        collect_rc_instantiations_program(program, &mut instantiations);
        for type_name in instantiations {
            self.register_rc_instantiation(&type_name);
        }
    }

    fn register_rc_instantiation(&mut self, type_name: &str) {
        if self.object_types.contains_key(type_name) {
            return;
        }
        let Some(inner_name) = type_name.strip_prefix("Rc_") else {
            return;
        };
        let inner_ty = parse_monomorphized_rc_inner_type(inner_name, &self.object_types)
            .unwrap_or_else(|| IrType::Unknown(inner_name.to_string()));
        let inner_ll = llvm_ir_type(&inner_ty);
        let mut fields = HashMap::new();
        fields.insert(
            "value".to_string(),
            LlField {
                index: 2,
                ty: inner_ty.clone(),
                drop_kind: drop_kind_for_type(&inner_ty, &Ownership::Owned),
            },
        );
        fields.insert(
            "refCount".to_string(),
            LlField {
                index: 3,
                ty: IrType::Int,
                drop_kind: DropKind::None,
            },
        );
        self.type_defs.push(format!(
            "%{} = type {{ i64, ptr, {}, i32 }}\n",
            llvm_object_name(type_name),
            inner_ll.as_ir()
        ));
        self.object_types.insert(
            type_name.to_string(),
            LlObjectType {
                name: type_name.to_string(),
                kind: TypeKind::Class,
                bases: Vec::new(),
                fields,
                constructor: None,
                constructor_params: vec![inner_ty],
            },
        );
    }

    fn nullable_type_name(inner: &IrType) -> String {
        format!("Nullable_{}", ir_symbol_suffix(inner))
    }

    fn ensure_nullable_object_type(&mut self, inner: &IrType) -> String {
        let type_name = Self::nullable_type_name(inner);
        if self.nullable_layouts.insert(type_name.clone()) {
            let payload_ty = llvm_ir_type(inner).as_ir().to_string();
            self.type_defs.push(format!(
                "%{} = type {{ i64, ptr, i1, {} }}\n",
                llvm_object_name(&type_name),
                payload_ty
            ));
            let mut fields = HashMap::new();
            fields.insert(
                "HasValue".to_string(),
                LlField {
                    index: 2,
                    ty: IrType::Bool,
                    drop_kind: DropKind::None,
                },
            );
            fields.insert(
                "Value".to_string(),
                LlField {
                    index: 3,
                    ty: inner.clone(),
                    drop_kind: drop_kind_for_type(inner, &Ownership::Owned),
                },
            );
            self.object_types.insert(
                type_name.clone(),
                LlObjectType {
                    name: type_name.clone(),
                    kind: TypeKind::Class,
                    bases: Vec::new(),
                    fields,
                    constructor: None,
                    constructor_params: vec![inner.clone()],
                },
            );
            let llvm_name = llvm_object_name(&type_name);
            let retain_name = retain_symbol(&type_name);
            let drop_name = drop_symbol(&type_name);
            self.globals.push(format!(
                "define void @{retain_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %retain\nretain:\n  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 0\n  %rc = load i64, ptr %rc_ptr\n  %next = add i64 %rc, 1\n  store i64 %next, ptr %rc_ptr\n  br label %done\ndone:\n  ret void\n}}\n\n"
            ));
            self.globals.push(format!(
                "define void @{drop_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 0\n  %rc = load i64, ptr %rc_ptr\n  %next = sub i64 %rc, 1\n  %destroy = icmp eq i64 %next, 0\n  br i1 %destroy, label %destroy_object, label %keep\ndestroy_object:\n  call void @glitch_free(ptr %object)\n  br label %done\nkeep:\n  store i64 %next, ptr %rc_ptr\n  br label %done\ndone:\n  ret void\n}}\n\n"
            ));
        }
        type_name
    }

    fn boxed_scalar_type_name(ty: &IrType) -> Option<String> {
        match ty {
            IrType::Bool
            | IrType::Byte
            | IrType::Short
            | IrType::Int
            | IrType::Long
            | IrType::UInt
            | IrType::Double
            | IrType::Decimal => Some(format!("Box_{}", ir_symbol_suffix(ty))),
            _ => None,
        }
    }

    fn ensure_boxed_scalar_object_type(&mut self, ty: &IrType) -> Option<String> {
        let type_name = Self::boxed_scalar_type_name(ty)?;
        if !self.object_types.contains_key(&type_name) {
            self.type_defs.push(format!(
                "%{} = type {{ i64, ptr, i32, {} }}\n",
                llvm_object_name(&type_name),
                llvm_ir_type(ty).as_ir()
            ));
            let mut fields = HashMap::new();
            fields.insert(
                "Value".to_string(),
                LlField {
                    index: 3,
                    ty: ty.clone(),
                    drop_kind: drop_kind_for_type(ty, &Ownership::Owned),
                },
            );
            self.object_types.insert(
                type_name.clone(),
                LlObjectType {
                    name: type_name.clone(),
                    kind: TypeKind::Class,
                    bases: Vec::new(),
                    fields,
                    constructor: None,
                    constructor_params: Vec::new(),
                },
            );
        }
        Some(type_name)
    }

    fn register_generic_specializations(
        &mut self,
        program: &TypedProgram,
    ) -> Result<(), String> {
        let mut generic_symbols = HashSet::new();
        let mut definitions = HashMap::new();
        for function in &program.functions {
            if !function.generic_params.is_empty() {
                generic_symbols.insert(function.symbol.clone());
                definitions.insert(function.symbol.clone(), function.clone());
            }
        }
        for ty in &program.types {
            for constructor in &ty.constructors {
                if !constructor.generic_params.is_empty() {
                    generic_symbols.insert(constructor.symbol.clone());
                    definitions.insert(constructor.symbol.clone(), constructor.clone());
                }
            }
            for method in &ty.methods {
                if !method.generic_params.is_empty() {
                    generic_symbols.insert(method.symbol.clone());
                    definitions.insert(method.symbol.clone(), method.clone());
                }
            }
        }
        for (symbol, function) in &self.owner_generic_templates {
            if !function.generic_params.is_empty() {
                generic_symbols.insert(symbol.clone());
                definitions.insert(symbol.clone(), function.clone());
            }
        }

        let mut queue = VecDeque::new();
        let mut queued = HashSet::new();
        for function in &program.functions {
            let mut discovered = Vec::new();
            collect_generic_instantiations_from_function(
                function,
                &generic_symbols,
                &self.specialized_instance_symbols,
                &mut discovered,
            );
            for instantiation in discovered {
                if !is_concrete_instantiation(&instantiation.args) {
                    continue;
                }
                let key = (instantiation.name.clone(), instantiation.args.clone());
                if queued.insert(key) {
                    queue.push_back(instantiation);
                }
            }
        }
        for ty in &program.types {
            for constructor in &ty.constructors {
                let mut discovered = Vec::new();
                collect_generic_instantiations_from_function(
                    constructor,
                    &generic_symbols,
                    &self.specialized_instance_symbols,
                    &mut discovered,
                );
                for instantiation in discovered {
                    if !is_concrete_instantiation(&instantiation.args) {
                        continue;
                    }
                    let key = (instantiation.name.clone(), instantiation.args.clone());
                    if queued.insert(key) {
                        queue.push_back(instantiation);
                    }
                }
            }
            for method in &ty.methods {
                let mut discovered = Vec::new();
                collect_generic_instantiations_from_function(
                    method,
                    &generic_symbols,
                    &self.specialized_instance_symbols,
                    &mut discovered,
                );
                for instantiation in discovered {
                    if !is_concrete_instantiation(&instantiation.args) {
                        continue;
                    }
                    let key = (instantiation.name.clone(), instantiation.args.clone());
                    if queued.insert(key) {
                        queue.push_back(instantiation);
                    }
                }
            }
        }
        for function in self.specialized_functions.clone() {
            let mut discovered = Vec::new();
            collect_generic_instantiations_from_function(
                &function,
                &generic_symbols,
                &self.specialized_instance_symbols,
                &mut discovered,
            );
            for instantiation in discovered {
                if !is_concrete_instantiation(&instantiation.args) {
                    continue;
                }
                let key = (instantiation.name.clone(), instantiation.args.clone());
                if queued.insert(key) {
                    queue.push_back(instantiation);
                }
            }
        }

        let mut pending = Vec::new();
        while let Some(instantiation) = queue.pop_front() {
            if instantiation.args.is_empty() {
                continue;
            }
            let Some(definition) = definitions.get(&instantiation.name) else {
                continue;
            };
            if definition.is_extern || definition.body.is_empty() {
                continue;
            }
            if definition.generic_params.len() != instantiation.args.len() {
                continue;
            }
            if !is_concrete_instantiation(&instantiation.args)
                && !is_safe_generic_wrapper_symbol(&definition.symbol)
            {
                continue;
            }
            let key = (definition.symbol.clone(), instantiation.args.clone());
            if self.specialized_symbols.contains_key(&key) {
                continue;
            }
            let suffix = instantiation
                .args
                .iter()
                .map(ir_symbol_suffix)
                .collect::<Vec<_>>()
                .join("_");
            let specialized_symbol = if suffix.is_empty() {
                format!("{}__specialized", definition.symbol)
            } else {
                format!("{}__{}", definition.symbol, suffix)
            };
            self.specialized_symbols
                .insert(key, specialized_symbol.clone());
            let mut subst = HashMap::new();
            for (name, ty) in definition
                .generic_params
                .iter()
                .cloned()
                .zip(instantiation.args.iter().cloned())
            {
                subst.insert(name, ty);
            }
            let specialized = specialize_typed_function(definition, &subst, specialized_symbol);
            let mut discovered_types = HashSet::new();
            collect_generic_object_instantiations_function(&specialized, &mut discovered_types);
            let previous_specialized_len = self.specialized_functions.len();
            self.ensure_generic_type_specializations(program, discovered_types)?;
            let mut discovered = Vec::new();
            collect_generic_instantiations_from_function(
                &specialized,
                &generic_symbols,
                &self.specialized_instance_symbols,
                &mut discovered,
            );
            for next in discovered {
                if !is_concrete_instantiation(&next.args) {
                    continue;
                }
                let next_key = (next.name.clone(), next.args.clone());
                if self.specialized_symbols.contains_key(&next_key) || !queued.insert(next_key) {
                    continue;
                }
                queue.push_back(next);
            }
            for function in self.specialized_functions[previous_specialized_len..].iter() {
                let mut nested = Vec::new();
                collect_generic_instantiations_from_function(
                    function,
                    &generic_symbols,
                    &self.specialized_instance_symbols,
                    &mut nested,
                );
                for next in nested {
                    if !is_concrete_instantiation(&next.args) {
                        continue;
                    }
                    let next_key = (next.name.clone(), next.args.clone());
                    if self.specialized_symbols.contains_key(&next_key) || !queued.insert(next_key) {
                        continue;
                    }
                    queue.push_back(next);
                }
            }
            pending.push(specialized);
        }
        self.specialized_functions.extend(pending);
        Ok(())
    }

    fn register_generic_type_specializations(
        &mut self,
        program: &TypedProgram,
    ) -> Result<(), String> {
        let mut instantiations = HashSet::new();
        collect_generic_object_instantiations_program(program, &mut instantiations);
        self.ensure_generic_type_specializations(program, instantiations)
    }

    fn ensure_generic_type_specializations<I>(
        &mut self,
        program: &TypedProgram,
        instantiations: I,
    ) -> Result<(), String>
    where
        I: IntoIterator<Item = String>,
    {
        let templates = program
            .types
            .iter()
            .filter(|ty| !ty.generic_params.is_empty())
            .map(|ty| ((ty.name.clone(), ty.generic_params.len()), ty))
            .collect::<HashMap<_, _>>();
        let mut queue = VecDeque::new();
        let mut queued = HashSet::new();
        for type_name in instantiations {
            if self.object_types.contains_key(&type_name) || !queued.insert(type_name.clone()) {
                continue;
            }
            queue.push_back(type_name);
        }
        while let Some(type_name) = queue.pop_front() {
            if self.object_types.contains_key(&type_name) {
                continue;
            }
            let Some((base_name, args)) = split_monomorphized_type(&type_name) else {
                continue;
            };
            let base_name = base_type_name(base_name).to_string();
            let Some(template) = templates.get(&(base_name.clone(), args.len())).copied() else {
                continue;
            };
            let generic_args = args
                .into_iter()
                .map(|arg| parse_monomorphized_rc_inner_type(&arg, &self.object_types))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    format!(
                        "LLVM TIR backend: could not parse generic type instantiation '{type_name}'"
                    )
                })?;
            if !is_concrete_instantiation(&generic_args) {
                continue;
            }
            let specialized = specialize_typed_type_owner(template, &type_name, &generic_args);
            for (template_constructor, constructor) in
                template.constructors.iter().zip(specialized.constructors.iter())
            {
                self.specialized_instance_symbols.insert(
                    (template_constructor.symbol.clone(), type_name.clone()),
                    constructor.symbol.clone(),
                );
                self.register_function(constructor);
            }
            for (template_method, method) in template.methods.iter().zip(specialized.methods.iter()) {
                self.specialized_instance_symbols.insert(
                    (template_method.symbol.clone(), type_name.clone()),
                    method.symbol.clone(),
                );
                if !method.generic_params.is_empty() {
                    self.owner_generic_templates
                        .insert(method.symbol.clone(), method.clone());
                }
                self.register_function(method);
            }
            self.register_object_type(&specialized);
            self.specialized_functions
                .extend(specialized.constructors.iter().cloned());
            self.specialized_functions
                .extend(specialized.methods.iter().cloned());

            let mut discovered = HashSet::new();
            for base in &specialized.bases {
                if base.contains('<') {
                    discovered.insert(base.clone());
                }
            }
            for field in &specialized.fields {
                collect_generic_object_instantiation_type(&field.ty, &mut discovered);
            }
            for constructor in &specialized.constructors {
                collect_generic_object_instantiations_function(constructor, &mut discovered);
            }
            for method in &specialized.methods {
                collect_generic_object_instantiations_function(method, &mut discovered);
            }
            for next in discovered {
                if self.object_types.contains_key(&next) || !queued.insert(next.clone()) {
                    continue;
                }
                queue.push_back(next);
            }
        }
        Ok(())
    }

    pub(crate) fn emit_program(program: &Program) -> Result<String, String> {
        let mut emitter = Self {
            type_defs: vec![
                "%glitch.array = type { i64, ptr }\n".to_string(),
                "%glitch.list = type { i64, i64, ptr }\n".to_string(),
                "%glitch.dict = type { i64, i64, ptr, ptr }\n".to_string(),
                "%glitch.string_node = type { i64, i64, [0 x i8] }\n".to_string(),
            ],
            globals: Vec::new(),
            body: String::new(),
            vars: HashMap::new(),
            functions: HashMap::new(),
            specialized_symbols: HashMap::new(),
            specialized_instance_symbols: HashMap::new(),
            owner_generic_templates: HashMap::new(),
            specialized_functions: Vec::new(),
            object_types: HashMap::new(),
            nullable_layouts: HashSet::new(),
            endpoint_handlers: Vec::new(),
            service_registrations: HashMap::new(),
            service_collection_registrations: HashMap::new(),
            service_provider_registrations: HashMap::new(),
            drop_order: Vec::new(),
            tmp: 0,
            label: 0,
            string_id: 0,
            lambda_id: 0,
            current_return: LlType::Void,
            current_is_main: false,
            current_unwind_label: String::new(),
            exception_handler: None,
            loop_targets: Vec::new(),
            async_state_pc_ptr: None,
            async_suspend_index: 0,
            entry_insert_pos: None,
            terminated: false,
            startup: None,
        };
        for function in &program.functions {
            emitter.functions.insert(
                function.name.clone(),
                LlFunctionSig {
                    return_type: llvm_type(&function.return_type),
                    params: function
                        .params
                        .iter()
                        .map(|param| llvm_type(&param.ty))
                        .collect(),
                    required_params: function
                        .params
                        .iter()
                        .take_while(|param| param.default.is_none())
                        .count(),
                },
            );
        }
        for function in &program.functions {
            emitter.emit_function(function)?;
        }
        emitter.finish_module()
    }

    fn finish_module(&self) -> Result<String, String> {
        runtime::finish_module(self)
    }

    fn emit_external_declaration(&mut self, function: &TypedFunction) {
        if matches!(
            function.symbol.as_str(),
            "GlitchRestHost_read_env_int"
                | "glitch_string_equals"
                | "JsonSerializer_Serialize_Native"
                | "JsonSerializer_Deserialize_Native"
        ) {
            return;
        }
        let return_type = llvm_ir_type(&function.return_type).as_ir();
        let params = function
            .params
            .iter()
            .map(|param| llvm_ir_type(&param.ty).as_ir())
            .collect::<Vec<_>>()
            .join(", ");
        self.type_defs.push(format!(
            "declare {} @{}({})\n",
            return_type,
            sanitize(&function.symbol),
            params
        ));
    }

    fn emit_typed_function(&mut self, function: &TypedFunction) -> Result<(), String> {
        if function.is_extern {
            return Ok(());
        }
        if function.is_async {
            return self.emit_async_function(function);
        }
        self.vars.clear();
        self.service_collection_registrations.clear();
        self.service_provider_registrations.clear();
        self.drop_order.clear();
        self.tmp = 0;
        self.label = 0;
        self.terminated = false;
        self.exception_handler = None;
        self.current_unwind_label = "exception_unwind".to_string();
        self.async_state_pc_ptr = None;
        self.async_suspend_index = 0;
        let is_main = self
            .startup
            .as_ref()
            .is_some_and(|startup| startup.symbol == function.symbol && function.symbol == "main");
        self.current_is_main = is_main;
        self.current_return = if is_main {
            LlType::I32
        } else {
            llvm_ir_type(&function.return_type)
        };
        let has_native_args = is_main
            && function.params.len() == 1
            && function.params[0].name == "args"
            && matches!(function.params[0].ty, IrType::Array(ref item) if item.as_ref() == &IrType::String);
        let params = if has_native_args {
            "i32 %argc, ptr %argv".to_string()
        } else {
            function
                .params
                .iter()
                .map(|param| {
                    format!(
                        "{} %{}",
                        llvm_ir_type(&param.ty).as_ir(),
                        sanitize(&param.name)
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        self.body.push_str(&format!(
            "define {} @{}({}) {{\nentry:\n",
            self.current_return.as_ir(),
            sanitize(&function.symbol),
            params
        ));
        self.entry_insert_pos = Some(self.body.len());
        for param in &function.params {
            let ty = llvm_ir_type(&param.ty);
            let ptr = self.tmp();
            self.body
                .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
            if has_native_args && param.name == "args" {
                let array = self.tmp();
                let has_args = self.tmp();
                let argc_minus_one = self.tmp();
                let argc = self.tmp();
                let argv = self.tmp();
                let len_ptr = self.tmp();
                let len = self.tmp();
                let data_ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {array} = alloca %glitch.array\n  {has_args} = icmp ugt i32 %argc, 0\n  {argc_minus_one} = sub i32 %argc, 1\n  {argc} = select i1 {has_args}, i32 {argc_minus_one}, i32 0\n  {argv} = getelementptr inbounds ptr, ptr %argv, i64 1\n  {len_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 0\n  {len} = zext i32 {argc} to i64\n  store i64 {len}, ptr {len_ptr}\n  {data_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 1\n  store ptr {argv}, ptr {data_ptr}\n  store ptr {array}, ptr {ptr}\n"
                ));
            } else {
                self.body.push_str(&format!(
                    "  store {} %{}, ptr {ptr}\n",
                    ty.as_ir(),
                    sanitize(&param.name)
                ));
            }
            self.vars.insert(
                param.name.clone(),
                LlVar {
                    ptr,
                    ty,
                    ir_ty: param.ty.clone(),
                    drop_kind: DropKind::BorrowOnly,
                },
            );
        }
        for local in &function.locals {
            if self.vars.contains_key(&local.name) {
                continue;
            }
            let ty = llvm_ir_type(&local.ty);
            let ptr = self.tmp();
            self.body
                .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
            self.body.push_str(&format!(
                "  store {} {}, ptr {ptr}\n",
                ty.as_ir(),
                ty.default_value()
            ));
            self.vars.insert(
                local.name.clone(),
                LlVar {
                    ptr,
                    ty,
                    ir_ty: local.ty.clone(),
                    drop_kind: local.drop_kind(),
                },
            );
            self.drop_order.push(local.name.clone());
        }
        self.emit_typed_stmts(&function.body)?;
        if !self.terminated {
            self.emit_local_drops(None);
            self.emit_default_return();
        }
        self.body
            .push_str(&format!("{}:\n", self.current_unwind_label));
        self.terminated = false;
        self.emit_local_drops(None);
        self.emit_default_return();
        self.body.push_str("}\n\n");
        self.entry_insert_pos = None;
        Ok(())
    }

    fn emit_async_function(&mut self, function: &TypedFunction) -> Result<(), String> {
        let result_ty = match &function.return_type {
            IrType::Task(inner) => inner.as_ref().clone(),
            other => {
                return Err(format!(
                    "LLVM backend: async function '{}' must lower to Task<T>, got {:?}",
                    function.name, other
                ));
            }
        };
        let result_ll = llvm_ir_type(&result_ty);
        let state_type = format!("glitch_async_state_{}", sanitize(&function.symbol));
        let resume_symbol = format!("glitch_async_resume_{}", sanitize(&function.symbol));
        let destroy_symbol = format!("glitch_async_destroy_{}", sanitize(&function.symbol));

        let mut state_fields = vec!["i32".to_string()];
        if result_ll != LlType::Void {
            state_fields.push(result_ll.as_ir().to_string());
        }
        for param in &function.params {
            state_fields.push(llvm_ir_type(&param.ty).as_ir().to_string());
        }
        self.type_defs.push(format!(
            "%{state_type} = type {{ {} }}\n",
            state_fields.join(", ")
        ));

        self.emit_async_resume_function(function, &state_type, &resume_symbol, &result_ty)?;
        self.emit_async_destroy_function(function, &state_type, &destroy_symbol);

        self.vars.clear();
        self.drop_order.clear();
        self.tmp = 0;
        self.label = 0;
        self.terminated = false;
        self.exception_handler = None;
        self.current_unwind_label = "exception_unwind".to_string();
        self.async_state_pc_ptr = None;
        self.async_suspend_index = 0;
        self.current_is_main = false;
        self.current_return = LlType::Ptr;

        let params = function
            .params
            .iter()
            .map(|param| {
                format!(
                    "{} %{}",
                    llvm_ir_type(&param.ty).as_ir(),
                    sanitize(&param.name)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        self.body.push_str(&format!(
            "define ptr @{}({}) {{\nentry:\n",
            sanitize(&function.symbol),
            params
        ));
        self.entry_insert_pos = Some(self.body.len());

        let state_size_ptr = self.tmp();
        let state_size = self.tmp();
        let state_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {state_size_ptr} = getelementptr %{state_type}, ptr null, i32 1\n  {state_size} = ptrtoint ptr {state_size_ptr} to i64\n  {state_ptr} = call ptr @glitch_calloc(i64 1, i64 {state_size})\n"
        ));
        let pc_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {pc_ptr} = getelementptr inbounds %{state_type}, ptr {state_ptr}, i32 0, i32 0\n  store i32 0, ptr {pc_ptr}\n"
        ));
        if result_ll != LlType::Void {
            let result_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {result_ptr} = getelementptr inbounds %{state_type}, ptr {state_ptr}, i32 0, i32 1\n  store {} {}, ptr {result_ptr}\n",
                result_ll.as_ir(),
                result_ll.default_value()
            ));
        }
        let param_start = if result_ll == LlType::Void { 1 } else { 2 };
        for (index, param) in function.params.iter().enumerate() {
            let field_ptr = self.tmp();
            let ty = llvm_ir_type(&param.ty);
            self.body.push_str(&format!(
                "  {field_ptr} = getelementptr inbounds %{state_type}, ptr {state_ptr}, i32 0, i32 {}\n",
                param_start + index
            ));
            self.emit_async_capture_retain(param, &format!("%{}", sanitize(&param.name)));
            self.body.push_str(&format!(
                "  store {} %{}, ptr {field_ptr}\n",
                ty.as_ir(),
                sanitize(&param.name)
            ));
        }

        let delegate_size_ptr = self.tmp();
        let delegate_size = self.tmp();
        let delegate_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {delegate_size_ptr} = getelementptr %glitch.delegate, ptr null, i32 1\n  {delegate_size} = ptrtoint ptr {delegate_size_ptr} to i64\n  {delegate_ptr} = call ptr @glitch_calloc(i64 1, i64 {delegate_size})\n"
        ));
        let rc_field = self.tmp();
        let fn_field = self.tmp();
        let env_field = self.tmp();
        let destroy_field = self.tmp();
        self.body.push_str(&format!(
            "  {rc_field} = getelementptr inbounds %glitch.delegate, ptr {delegate_ptr}, i32 0, i32 0\n  store i64 1, ptr {rc_field}\n  {fn_field} = getelementptr inbounds %glitch.delegate, ptr {delegate_ptr}, i32 0, i32 1\n  store ptr @{resume_symbol}, ptr {fn_field}\n  {env_field} = getelementptr inbounds %glitch.delegate, ptr {delegate_ptr}, i32 0, i32 2\n  store ptr {state_ptr}, ptr {env_field}\n  {destroy_field} = getelementptr inbounds %glitch.delegate, ptr {delegate_ptr}, i32 0, i32 3\n  store ptr @{destroy_symbol}, ptr {destroy_field}\n"
        ));

        let task_ptr = self.tmp();
        let helper_name = if matches!(result_ty, IrType::Void) {
            "glitch_task_run_void"
        } else if llvm_ir_type(&result_ty) == LlType::I1 || is_bool_like_type(&result_ty) {
            "glitch_task_run_bool"
        } else {
            match result_ty {
                IrType::Int | IrType::UInt => "glitch_task_run_i32",
                IrType::Long => "glitch_task_run_i64",
                IrType::Double | IrType::Decimal => "glitch_task_run_double",
                _ => "glitch_task_run_ptr",
            }
        };
        self.body.push_str(&format!(
            "  {task_ptr} = call ptr @{helper_name}(ptr {delegate_ptr})\n  ret ptr {task_ptr}\n}}\n\n"
        ));
        self.entry_insert_pos = None;
        Ok(())
    }

    fn emit_async_resume_function(
        &mut self,
        function: &TypedFunction,
        state_type: &str,
        resume_symbol: &str,
        result_ty: &IrType,
    ) -> Result<(), String> {
        let saved_vars = self.vars.clone();
        let saved_drop_order = self.drop_order.clone();
        let saved_tmp = self.tmp;
        let saved_label = self.label;
        let saved_return = self.current_return.clone();
        let saved_is_main = self.current_is_main;
        let saved_unwind = self.current_unwind_label.clone();
        let saved_handler = self.exception_handler.clone();
        let saved_loop = self.loop_targets.clone();
        let saved_terminated = self.terminated;
        let saved_async_pc_ptr = self.async_state_pc_ptr.clone();
        let saved_async_suspend_index = self.async_suspend_index;
        let saved_body = std::mem::take(&mut self.body);

        self.vars.clear();
        self.drop_order.clear();
        self.tmp = 0;
        self.label = 0;
        self.terminated = false;
        self.current_is_main = false;
        self.current_return = llvm_ir_type(result_ty);
        self.current_unwind_label = "exception_unwind".to_string();
        self.exception_handler = None;
        self.loop_targets.clear();
        self.async_suspend_index = 0;

        self.body.push_str(&format!(
            "define {} @{resume_symbol}(ptr %env) {{\nentry:\n",
            self.current_return.as_ir()
        ));
        self.entry_insert_pos = Some(self.body.len());
        let pc_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {pc_ptr} = getelementptr inbounds %{state_type}, ptr %env, i32 0, i32 0\n"
        ));
        self.async_state_pc_ptr = Some(pc_ptr);

        let param_start = if self.current_return == LlType::Void { 1 } else { 2 };
        for (index, param) in function.params.iter().enumerate() {
            let ty = llvm_ir_type(&param.ty);
            let ptr = self.tmp();
            let field_ptr = self.tmp();
            let loaded = self.tmp();
            self.body.push_str(&format!(
                "  {ptr} = alloca {}\n  {field_ptr} = getelementptr inbounds %{state_type}, ptr %env, i32 0, i32 {}\n  {loaded} = load {}, ptr {field_ptr}\n  store {} {loaded}, ptr {ptr}\n",
                ty.as_ir(),
                param_start + index,
                ty.as_ir(),
                ty.as_ir()
            ));
            self.vars.insert(
                param.name.clone(),
                LlVar {
                    ptr,
                    ty,
                    ir_ty: param.ty.clone(),
                    drop_kind: DropKind::BorrowOnly,
                },
            );
        }
        for local in &function.locals {
            if self.vars.contains_key(&local.name) {
                continue;
            }
            let ty = llvm_ir_type(&local.ty);
            let ptr = self.tmp();
            self.body
                .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
            self.body.push_str(&format!(
                "  store {} {}, ptr {ptr}\n",
                ty.as_ir(),
                ty.default_value()
            ));
            self.vars.insert(
                local.name.clone(),
                LlVar {
                    ptr,
                    ty,
                    ir_ty: local.ty.clone(),
                    drop_kind: local.drop_kind(),
                },
            );
            self.drop_order.push(local.name.clone());
        }

        self.emit_typed_stmts(&function.body)?;
        if !self.terminated {
            self.emit_local_drops(None);
            self.emit_default_return();
        }
        self.body
            .push_str(&format!("{}:\n", self.current_unwind_label));
        self.terminated = false;
        self.emit_local_drops(None);
        self.emit_default_return();
        self.body.push_str("}\n\n");
        self.entry_insert_pos = None;

        let resume_body = std::mem::replace(&mut self.body, saved_body);
        self.globals.push(resume_body);
        self.vars = saved_vars;
        self.drop_order = saved_drop_order;
        self.tmp = saved_tmp;
        self.label = saved_label;
        self.current_return = saved_return;
        self.current_is_main = saved_is_main;
        self.current_unwind_label = saved_unwind;
        self.exception_handler = saved_handler;
        self.loop_targets = saved_loop;
        self.terminated = saved_terminated;
        self.async_state_pc_ptr = saved_async_pc_ptr;
        self.async_suspend_index = saved_async_suspend_index;
        self.entry_insert_pos = None;
        Ok(())
    }

    fn emit_async_destroy_function(
        &mut self,
        function: &TypedFunction,
        state_type: &str,
        destroy_symbol: &str,
    ) {
        let mut body = format!("define void @{destroy_symbol}(ptr %env) {{\nentry:\n");
        let param_start = if matches!(&function.return_type, IrType::Task(inner) if llvm_ir_type(inner) == LlType::Void) {
            1
        } else {
            2
        };
        for (index, param) in function.params.iter().enumerate() {
            let field_ptr = format!("%async_field_{}_ptr", index);
            let loaded = format!("%async_field_{}", index);
            let ty = llvm_ir_type(&param.ty);
            body.push_str(&format!(
                "  {field_ptr} = getelementptr inbounds %{state_type}, ptr %env, i32 0, i32 {}\n  {loaded} = load {}, ptr {field_ptr}\n",
                param_start + index,
                ty.as_ir()
            ));
            match &param.ty {
                IrType::String | IrType::Exception => {
                    body.push_str(&format!(
                        "  call void @glitch_string_release(ptr {loaded})\n"
                    ));
                }
                IrType::Function { .. } => {
                    body.push_str(&format!(
                        "  call void @glitch_delegate_release(ptr {loaded})\n"
                    ));
                }
                IrType::Class(type_name) | IrType::Interface(type_name) => {
                    if let Some(object) = self.object_types.get(type_name) {
                        body.push_str(&format!(
                            "  call void @{}(ptr {loaded})\n",
                            drop_symbol(&object.name)
                        ));
                    }
                }
                _ => {}
            }
        }
        body.push_str("  call void @glitch_free(ptr %env)\n  ret void\n}\n\n");
        self.globals.push(body);
    }

    fn emit_async_capture_retain(&mut self, binding: &TypedBinding, value: &str) {
        match &binding.ty {
            IrType::String | IrType::Exception => self
                .body
                .push_str(&format!("  call void @glitch_string_retain(ptr {value})\n")),
            IrType::Function { .. } => self
                .body
                .push_str(&format!("  call void @glitch_delegate_retain(ptr {value})\n")),
            IrType::Class(type_name) | IrType::Interface(type_name) => {
                if self.object_types.contains_key(type_name) {
                    self.emit_retain(type_name, value);
                }
            }
            _ => {}
        }
    }

    fn emit_function(&mut self, function: &Function) -> Result<(), String> {
        self.vars.clear();
        self.service_collection_registrations.clear();
        self.service_provider_registrations.clear();
        self.tmp = 0;
        self.label = 0;
        self.terminated = false;
        self.exception_handler = None;
        self.current_unwind_label = "exception_unwind".to_string();
        let is_main = function.name == "main";
        self.current_is_main = is_main;
        self.current_return = if is_main {
            LlType::I32
        } else {
            llvm_type(&function.return_type)
        };
        let params = function
            .params
            .iter()
            .map(|param| {
                format!(
                    "{} %{}",
                    llvm_type(&param.ty).as_ir(),
                    sanitize(&param.name)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        self.body.push_str(&format!(
            "define {} @{}({}) {{\nentry:\n",
            self.current_return.as_ir(),
            sanitize(&function.name),
            params
        ));
        self.entry_insert_pos = Some(self.body.len());
        for param in &function.params {
            let ty = llvm_type(&param.ty);
            let ptr = self.tmp();
            self.body
                .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
            self.body.push_str(&format!(
                "  store {} %{}, ptr {ptr}\n",
                ty.as_ir(),
                sanitize(&param.name)
            ));
            self.vars.insert(
                param.name.clone(),
                LlVar {
                    ptr,
                    ty,
                    ir_ty: IrType::Unknown(param.name.clone()),
                    drop_kind: DropKind::None,
                },
            );
        }
        self.emit_stmts(&function.body)?;
        if !self.terminated {
            self.emit_default_return();
        }
        self.body.push_str("}\n\n");
        self.entry_insert_pos = None;
        Ok(())
    }

    fn emit_binary_value(
        &mut self,
        left: LlValue,
        op: BinaryOp,
        right: LlValue,
    ) -> Result<LlValue, String> {
        let tmp = self.tmp();
        if op.is_comparison() {
            if left.ty == LlType::Double {
                let pred = match op {
                    BinaryOp::Eq => "oeq",
                    BinaryOp::NotEq => "one",
                    BinaryOp::Less => "olt",
                    BinaryOp::LessEq => "ole",
                    BinaryOp::Greater => "ogt",
                    BinaryOp::GreaterEq => "oge",
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Mod
                    | BinaryOp::Coalesce
                    | BinaryOp::And
                    | BinaryOp::Or
                    | BinaryOp::BitAnd
                    | BinaryOp::BitOr => unreachable!(),
                };
                self.body.push_str(&format!(
                    "  {tmp} = fcmp {pred} double {}, {}\n",
                    left.value, right.value
                ));
            } else {
                let pred = match op {
                    BinaryOp::Eq => "eq",
                    BinaryOp::NotEq => "ne",
                    BinaryOp::Less => "slt",
                    BinaryOp::LessEq => "sle",
                    BinaryOp::Greater => "sgt",
                    BinaryOp::GreaterEq => "sge",
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Mod
                    | BinaryOp::Coalesce
                    | BinaryOp::And
                    | BinaryOp::Or
                    | BinaryOp::BitAnd
                    | BinaryOp::BitOr => unreachable!(),
                };
                self.body.push_str(&format!(
                    "  {tmp} = icmp {pred} {} {}, {}\n",
                    left.ty.as_ir(),
                    left.value,
                    right.value
                ));
            }
            Ok(LlValue {
                value: tmp,
                ty: LlType::I1,
            })
        } else if matches!(op, BinaryOp::BitAnd | BinaryOp::BitOr) {
            let op_name = if op == BinaryOp::BitAnd { "and" } else { "or" };
            self.body.push_str(&format!(
                "  {tmp} = {op_name} {} {}, {}\n",
                left.ty.as_ir(),
                left.value,
                right.value
            ));
            Ok(LlValue {
                value: tmp,
                ty: left.ty,
            })
        } else if left.ty == LlType::Double {
            let op_name = match op {
                BinaryOp::Sub => "fsub",
                BinaryOp::Mul => "fmul",
                BinaryOp::Div => "fdiv",
                BinaryOp::Mod => "frem",
                _ => "fadd",
            };
            self.body.push_str(&format!(
                "  {tmp} = {op_name} double {}, {}\n",
                left.value, right.value
            ));
            Ok(LlValue {
                value: tmp,
                ty: LlType::Double,
            })
        } else {
            let op_name = match op {
                BinaryOp::Sub => "sub",
                BinaryOp::Mul => "mul",
                BinaryOp::Div => "sdiv",
                BinaryOp::Mod => "srem",
                _ => "add",
            };
            self.body.push_str(&format!(
                "  {tmp} = {op_name} {} {}, {}\n",
                left.ty.as_ir(),
                left.value,
                right.value
            ));
            Ok(LlValue {
                value: tmp,
                ty: left.ty,
            })
        }
    }

    fn emit_not_value(&mut self, value: LlValue) -> Result<LlValue, String> {
        let value = self.to_i1(value)?;
        let tmp = self.tmp();
        self.body
            .push_str(&format!("  {tmp} = xor i1 {}, true\n", value.value));
        Ok(LlValue {
            value: tmp,
            ty: LlType::I1,
        })
    }

    fn emit_neg_value(&mut self, value: LlValue) -> Result<LlValue, String> {
        let tmp = self.tmp();
        if value.ty == LlType::Double {
            self.body
                .push_str(&format!("  {tmp} = fsub double 0.0, {}\n", value.value));
        } else {
            self.body.push_str(&format!(
                "  {tmp} = sub {} {}, {}\n",
                value.ty.as_ir(),
                value.ty.default_value(),
                value.value
            ));
        }
        Ok(LlValue {
            value: tmp,
            ty: value.ty,
        })
    }

    fn emit_inc_dec_value(
        &mut self,
        target: &TypedExpr,
        delta: i32,
        prefix: bool,
    ) -> Result<LlValue, String> {
        let field_ptr = if matches!(target.kind, TypedExprKind::Field { .. }) {
            Some(self.emit_field_ptr(target)?)
        } else {
            None
        };
        let (ty, ptr) = if let Some(field_ptr) = &field_ptr {
            (llvm_ir_type(&target.ty), field_ptr.value.clone())
        } else {
            let name = match &target.kind {
                TypedExprKind::Var(n) => n.as_str(),
                _ => return Err("LLVM TIR backend: unsupported IncDec target".to_string()),
            };
            let var = self
                .vars
                .get(name)
                .cloned()
                .ok_or_else(|| format!("LLVM TIR backend: unknown variable '{name}'"))?;
            (var.ty, var.ptr)
        };
        self.emit_inc_dec_raw(ty, &ptr, delta, prefix)
    }

    fn emit_inc_dec_untyped(
        &mut self,
        name: &str,
        delta: i32,
        prefix: bool,
    ) -> Result<LlValue, String> {
        let var = self
            .vars
            .get(name)
            .cloned()
            .ok_or_else(|| format!("LLVM backend: unknown variable '{name}'"))?;
        self.emit_inc_dec_raw(var.ty, &var.ptr, delta, prefix)
    }

    fn emit_inc_dec_raw(
        &mut self,
        ty: LlType,
        ptr: &str,
        delta: i32,
        prefix: bool,
    ) -> Result<LlValue, String> {
        let old_value = self.tmp();
        self.body.push_str(&format!(
            "  {old_value} = load {}, ptr {}\n",
            ty.as_ir(),
            ptr
        ));
        let new_value = self.tmp();
        if ty == LlType::Double {
            let op = if delta >= 0 { "fadd" } else { "fsub" };
            self.body.push_str(&format!(
                "  {new_value} = {op} double {old_value}, 1.0\n"
            ));
        } else {
            let op = if delta >= 0 { "add" } else { "sub" };
            self.body.push_str(&format!(
                "  {new_value} = {op} {} {old_value}, 1\n",
                ty.as_ir()
            ));
        }
        self.body.push_str(&format!(
            "  store {} {new_value}, ptr {}\n",
            ty.as_ir(),
            ptr
        ));
        Ok(LlValue {
            value: if prefix { new_value } else { old_value },
            ty,
        })
    }

    fn emit_logical_value(
        &mut self,
        left: LlValue,
        op: BinaryOp,
        right: LlValue,
    ) -> Result<LlValue, String> {
        let left = self.to_i1(left)?;
        let right = self.to_i1(right)?;
        let tmp = self.tmp();
        let op_name = match op {
            BinaryOp::And => "and",
            BinaryOp::Or => "or",
            _ => unreachable!(),
        };
        self.body.push_str(&format!(
            "  {tmp} = {op_name} i1 {}, {}\n",
            left.value, right.value
        ));
        Ok(LlValue {
            value: tmp,
            ty: LlType::I1,
        })
    }

    fn emit_select_value(
        &mut self,
        condition: LlValue,
        when_true: LlValue,
        when_false: LlValue,
    ) -> Result<LlValue, String> {
        if when_true.ty == LlType::Ptr {
            return Err("LLVM backend: pointer ternary select is not supported yet".to_string());
        }
        let tmp = self.tmp();
        self.body.push_str(&format!(
            "  {tmp} = select i1 {}, {} {}, {} {}\n",
            condition.value,
            when_true.ty.as_ir(),
            when_true.value,
            when_false.ty.as_ir(),
            when_false.value
        ));
        Ok(LlValue {
            value: tmp,
            ty: when_true.ty,
        })
    }

    fn emit_print(&mut self, value: LlValue) {
        match value.ty {
            LlType::Ptr => self.body.push_str(&format!(
                "  call i32 (ptr, ...) @printf(ptr {}, ptr {})\n",
                fmt_ptr("str"),
                value.value
            )),
            LlType::Double => self.body.push_str(&format!(
                "  call i32 (ptr, ...) @printf(ptr {}, double {})\n",
                fmt_ptr("double"),
                value.value
            )),
            LlType::I64 => self.body.push_str(&format!(
                "  call i32 (ptr, ...) @printf(ptr {}, i64 {})\n",
                fmt_ptr("i64"),
                value.value
            )),
            LlType::I1 => {
                let text = self.tmp();
                self.body.push_str(&format!(
                    "  {text} = select i1 {}, ptr getelementptr inbounds ([5 x i8], ptr @.json_true, i64 0, i64 0), ptr getelementptr inbounds ([6 x i8], ptr @.json_false, i64 0, i64 0)\n  call i32 (ptr, ...) @printf(ptr {}, ptr {text})\n",
                    value.value,
                    fmt_ptr("str")
                ));
            }
            LlType::I8 | LlType::I16 => {
                let widened = self.tmp();
                self.body.push_str(&format!(
                    "  {widened} = zext {} {} to i32\n  call i32 (ptr, ...) @printf(ptr {}, i32 {widened})\n",
                    value.ty.as_ir(),
                    value.value,
                    fmt_ptr("i32")
                ));
            }
            LlType::I32 => self.body.push_str(&format!(
                "  call i32 (ptr, ...) @printf(ptr {}, i32 {})\n",
                fmt_ptr("i32"),
                value.value
            )),
            LlType::Void => {}
        }
    }

    fn emit_default_return(&mut self) {
        if self.current_is_main {
            let live = self.tmp();
            let leaked = self.tmp();
            let exception = self.tmp();
            let has_exception = self.tmp();
            let failed = self.tmp();
            let code = self.tmp();
            let report_env = self.tmp();
            let should_report = self.tmp();
            let report_label = self.next_label("report_leaks");
            let return_label = self.next_label("main_return");
            self.body.push_str(&format!(
                "  {live} = call i64 @GlitchLiveAllocations_Load()\n  {leaked} = icmp ne i64 {live}, 0\n  {exception} = load ptr, ptr @glitch_exception_pending\n  {has_exception} = icmp ne ptr {exception}, null\n  {failed} = or i1 {leaked}, {has_exception}\n  {code} = zext i1 {failed} to i32\n  {report_env} = call ptr @getenv(ptr @.env_report_leaks)\n  {should_report} = icmp ne ptr {report_env}, null\n  br i1 {should_report}, label %{report_label}, label %{return_label}\n{report_label}:\n  call i32 (ptr, ...) @printf(ptr {}, i64 {live})\n  br label %{return_label}\n{return_label}:\n  ret i32 {code}\n",
                fmt_ptr("i64")
            ));
        } else if self.current_return == LlType::Void {
            self.body.push_str("  ret void\n");
        } else {
            self.body.push_str(&format!(
                "  ret {} {}\n",
                self.current_return.as_ir(),
                self.current_return.default_value()
            ));
        }
        self.terminated = true;
    }

    fn cast_value(&mut self, value: LlValue, target: &LlType) -> Result<LlValue, String> {
        if &value.ty == target {
            return Ok(value);
        }
        let value_ty = value.ty.clone();
        if value.ty.is_integer() && target.is_integer() {
            let tmp = self.tmp();
            let op = if integer_bits(&value.ty) < integer_bits(target) {
                "sext"
            } else {
                "trunc"
            };
            self.body.push_str(&format!(
                "  {tmp} = {op} {} {} to {}\n",
                value.ty.as_ir(),
                value.value,
                target.as_ir()
            ));
            return Ok(LlValue {
                value: tmp,
                ty: target.clone(),
            });
        }
        if value.ty.is_integer() && target == &LlType::Double {
            let tmp = self.tmp();
            self.body.push_str(&format!(
                "  {tmp} = sitofp {} {} to double\n",
                value.ty.as_ir(),
                value.value
            ));
            return Ok(LlValue {
                value: tmp,
                ty: LlType::Double,
            });
        }
        if value.ty == LlType::Ptr && target.is_integer() {
            return Ok(LlValue {
                value: target.default_value().to_string(),
                ty: target.clone(),
            });
        }
        if value.ty == LlType::Ptr && target == &LlType::Double {
            return Ok(LlValue {
                value: "0.0".to_string(),
                ty: LlType::Double,
            });
        }
        if target == &LlType::Ptr && (value_ty.is_integer() || value_ty == LlType::Double) {
            return self.emit_boxed_scalar_value(value, &match value_ty {
                LlType::I1 => IrType::Bool,
                LlType::I8 => IrType::Byte,
                LlType::I16 => IrType::Short,
                LlType::I32 => IrType::Int,
                LlType::I64 => IrType::Long,
                LlType::Double => IrType::Double,
                _ => IrType::Unknown("boxed".to_string()),
            });
        }
        Err(format!(
            "LLVM backend: cannot cast {} to {}",
            value.ty.as_ir(),
            target.as_ir()
        ))
    }

    fn to_i1(&mut self, value: LlValue) -> Result<LlValue, String> {
        if value.ty == LlType::I1 {
            return Ok(value);
        }
        if value.ty.is_integer() {
            let tmp = self.tmp();
            self.body.push_str(&format!(
                "  {tmp} = icmp ne {} {}, 0\n",
                value.ty.as_ir(),
                value.value
            ));
            return Ok(LlValue {
                value: tmp,
                ty: LlType::I1,
            });
        }
        if value.ty == LlType::Ptr {
            let tmp = self.tmp();
            self.body
                .push_str(&format!("  {tmp} = icmp ne ptr {}, null\n", value.value));
            return Ok(LlValue {
                value: tmp,
                ty: LlType::I1,
            });
        }
        Err("LLVM backend: condition must be integer/bool".to_string())
    }

    fn string_global(&mut self, value: &str) -> String {
        let id = self.string_id;
        self.string_id += 1;
        let bytes = escaped_bytes(value);
        let len_without_null = value.as_bytes().len();
        let len = len_without_null + 1;
        self.globals.push(format!(
            "@.str.{id} = private unnamed_addr global {{ i64, i64, [{len} x i8] }} {{ i64 1000000000, i64 {len_without_null}, [{len} x i8] c\"{bytes}\\00\" }}\n"
        ));
        format!("getelementptr inbounds ({{ i64, i64, [{len} x i8] }}, ptr @.str.{id}, i32 0, i32 2, i64 0)")
    }

    fn tmp(&mut self) -> String {
        let tmp = format!("%t{}", self.tmp);
        self.tmp += 1;
        tmp
    }

    fn next_label(&mut self, prefix: &str) -> String {
        let label = format!("{prefix}_{}", self.label);
        self.label += 1;
        label
    }
}

fn split_monomorphized_type(text: &str) -> Option<(&str, Vec<String>)> {
    let mut depth = 0usize;
    let mut open = None;
    let mut close = None;
    for (index, ch) in text.char_indices() {
        match ch {
            '<' => {
                if depth == 0 {
                    open = Some(index);
                }
                depth += 1;
            }
            '>' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    close = Some(index);
                    break;
                }
            }
            _ => {}
        }
    }
    let open = open?;
    let close = close?;
    let base = &text[..open];
    let args = &text[open + 1..close];
    let mut parsed_args = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    for ch in args.chars() {
        match ch {
            '<' => {
                depth += 1;
                current.push(ch);
            }
            '>' => {
                depth = depth.checked_sub(1)?;
                current.push(ch);
            }
            ',' if depth == 0 => {
                let arg = current.trim();
                if arg.is_empty() {
                    return None;
                }
                parsed_args.push(arg.to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    let tail = current.trim();
    if !tail.is_empty() {
        parsed_args.push(tail.to_string());
    }
    Some((base, parsed_args))
}


