#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::*;
use crate::tir::*;

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

pub(crate) struct LlvmEmitter {
    type_defs: Vec<String>,
    globals: Vec<String>,
    body: String,
    vars: HashMap<String, LlVar>,
    functions: HashMap<String, LlFunctionSig>,
    object_types: HashMap<String, LlObjectType>,
    endpoint_handlers: Vec<EndpointHandlerBinding>,
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
    terminated: bool,
}

impl LlvmEmitter {
    pub(crate) fn emit_typed_program(program: &TypedProgram) -> Result<String, String> {
        let mut emitter = Self {
            type_defs: vec![
                "%glitch.array = type { i64, ptr }\n".to_string(),
                "%glitch.list = type { i64, i64, ptr }\n".to_string(),
                "%glitch.dict = type { i64, i64, ptr, ptr }\n".to_string(),
                "%glitch.string_node = type { i64, i64, [0 x i8] }\n".to_string(),
                "%glitch.delegate = type { ptr, ptr }\n".to_string(),
            ],
            globals: Vec::new(),
            body: String::new(),
            vars: HashMap::new(),
            functions: HashMap::new(),
            object_types: HashMap::new(),
            endpoint_handlers: program.endpoint_handlers.clone(),
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
            terminated: false,
        };
        for ty in &program.types {
            emitter.register_object_type(ty);
        }
        emitter.register_rc_instantiations(program);
        for function in &program.functions {
            emitter.register_function(function);
        }
        emitter.functions.insert(
            "System_IO_File_ReadAllText".to_string(),
            LlFunctionSig {
                return_type: LlType::Ptr,
                params: vec![LlType::Ptr],
            },
        );
        emitter.functions.insert(
            "System_IO_File_WriteAllText".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::Ptr, LlType::Ptr],
            },
        );
        emitter.functions.insert(
            "System_Console_WriteLine_String".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::Ptr],
            },
        );
        emitter.functions.insert(
            "System_Console_WriteLine_I64".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::I64],
            },
        );
        emitter.functions.insert(
            "System_Console_WriteLine_Double".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::Double],
            },
        );
        emitter.functions.insert(
            "System_Console_WriteLine_Bool".to_string(),
            LlFunctionSig {
                return_type: LlType::Void,
                params: vec![LlType::I1],
            },
        );
        for ty in &program.types {
            for constructor in &ty.constructors {
                emitter.register_function(constructor);
            }
            for method in &ty.methods {
                emitter.register_function(method);
            }
        }
        emitter.emit_drop_glue();
        for ty in &program.types {
            if ty.kind == TypeKind::Interface {
                continue;
            }
            for constructor in &ty.constructors {
                emitter.emit_typed_function(constructor)?;
            }
            for method in &ty.methods {
                emitter.emit_typed_function(method)?;
            }
        }
        for function in &program.functions {
            if function.is_extern {
                emitter.emit_external_declaration(function);
            } else {
                emitter.emit_typed_function(function)?;
            }
        }
        emitter.emit_endpoint_dispatch()?;
        emitter.finish_module()
    }

    fn register_function(&mut self, function: &TypedFunction) {
        self.functions.insert(
            function.symbol.clone(),
            LlFunctionSig {
                return_type: llvm_ir_type(&function.return_type),
                params: function
                    .params
                    .iter()
                    .map(|param| llvm_ir_type(&param.ty))
                    .collect(),
            },
        );
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
            llvm_object_name(&ty.name),
            layout.join(", ")
        ));
        self.object_types.insert(
            ty.name.clone(),
            LlObjectType {
                name: ty.name.clone(),
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

    fn emit_drop_glue(&mut self) {
        let objects = self.object_types.values().cloned().collect::<Vec<_>>();
        for object in objects {
            let llvm_name = llvm_object_name(&object.name);
            let retain_name = retain_symbol(&object.name);
            let drop_name = drop_symbol(&object.name);
            if matches!(object.kind, TypeKind::Class | TypeKind::Interface) {
                let destroy_name = destroy_symbol(&object.name);
                self.body.push_str(&format!(
                    "define void @{destroy_name}(ptr %object) {{\nentry:\n"
                ));
                self.emit_field_drops(&object, &llvm_name);
                self.body
                    .push_str("  call void @glitch_free(ptr %object)\n  ret void\n}\n\n");
                self.body.push_str(&format!(
                    "define void @{retain_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %retain\nretain:\n  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 0\n  %rc = load i64, ptr %rc_ptr\n  %next = add i64 %rc, 1\n  store i64 %next, ptr %rc_ptr\n  br label %done\ndone:\n  ret void\n}}\n\n"
                ));
                self.body.push_str(&format!(
                    "define void @{drop_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 0\n  %rc = load i64, ptr %rc_ptr\n  %next = sub i64 %rc, 1\n  %destroy = icmp eq i64 %next, 0\n  br i1 %destroy, label %destroy_object, label %keep\ndestroy_object:\n  %drop_ptr_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 1\n  %drop_ptr = load ptr, ptr %drop_ptr_ptr\n  %has_dynamic_drop = icmp ne ptr %drop_ptr, null\n  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop\ndynamic_drop:\n  call void %drop_ptr(ptr %object)\n  br label %done\nstatic_drop:\n  call void @{destroy_name}(ptr %object)\n  br label %done\n"
                ));
                self.body.push_str(
                    "keep:\n  store i64 %next, ptr %rc_ptr\n  br label %done\ndone:\n  ret void\n}\n\n",
                );
            } else {
                self.body.push_str(&format!(
                    "define void @{drop_name}(ptr %object) {{\nentry:\n  %is_null = icmp eq ptr %object, null\n  br i1 %is_null, label %done, label %destroy_object\ndestroy_object:\n"
                ));
                self.emit_field_drops(&object, &llvm_name);
                self.body
                    .push_str("  call void @glitch_free(ptr %object)\n  br label %done\ndone:\n  ret void\n}\n\n");
            }
        }
    }

    fn emit_field_drops(&mut self, object: &LlObjectType, llvm_name: &str) {
        for field in object.fields.values() {
            if matches!(field.drop_kind, DropKind::DropCollection) {
                let ptr_name = format!("%field_{}_ptr", field.index);
                let value_name = format!("%field_{}", field.index);
                self.body.push_str(&format!(
                    "  {ptr_name} = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  {value_name} = load ptr, ptr {ptr_name}\n",
                    field.index
                ));
                self.emit_collection_drop_value(&field.ty, &value_name);
                continue;
            }
            if matches!(field.drop_kind, DropKind::Free) && matches!(field.ty, IrType::Array(_)) {
                let ptr_name = format!("%field_{}_ptr", field.index);
                let value_name = format!("%field_{}", field.index);
                self.body.push_str(&format!(
                    "  {ptr_name} = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  {value_name} = load ptr, ptr {ptr_name}\n",
                    field.index
                ));
                if let IrType::Array(element) = &field.ty {
                    self.emit_array_drop_value(&value_name, element);
                }
                continue;
            }
            if matches!(field.drop_kind, DropKind::Free) && matches!(field.ty, IrType::String) {
                self.body.push_str(&format!(
                    "  %field_{}_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %field_{} = load ptr, ptr %field_{}_ptr\n  call void @glitch_string_release(ptr %field_{})\n",
                    field.index, field.index, field.index, field.index, field.index
                ));
                continue;
            }
            let Some(type_name) = object_type_name(&field.ty) else {
                continue;
            };
            if !self.object_types.contains_key(type_name) {
                continue;
            }
            if !matches!(field.drop_kind, DropKind::DropClass | DropKind::DropStruct) {
                continue;
            }
            self.body.push_str(&format!(
                "  %field_{}_ptr = getelementptr inbounds %{llvm_name}, ptr %object, i32 0, i32 {}\n  %field_{} = load ptr, ptr %field_{}_ptr\n  call void @{}(ptr %field_{})\n",
                field.index,
                field.index,
                field.index,
                field.index,
                drop_symbol(type_name),
                field.index
            ));
        }
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
            object_types: HashMap::new(),
            endpoint_handlers: Vec::new(),
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
            terminated: false,
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
                },
            );
        }
        for function in &program.functions {
            emitter.emit_function(function)?;
        }
        emitter.finish_module()
    }

    fn finish_module(&self) -> Result<String, String> {
        let mut out = String::new();
        out.push_str("; ModuleID = 'glitching'\n");
        for type_def in &self.type_defs {
            out.push_str(type_def);
        }
        out.push_str("declare i32 @printf(ptr, ...)\n");
        out.push_str("declare ptr @calloc(i64, i64)\n");
        out.push_str("declare ptr @realloc(ptr, i64)\n");
        out.push_str("declare void @free(ptr)\n");
        out.push_str("declare i32 @strcmp(ptr, ptr)\n");
        out.push_str("declare i32 @strncmp(ptr, ptr, i64)\n");
        out.push_str("declare i64 @strlen(ptr)\n");
        out.push_str("declare i64 @strtoll(ptr, ptr, i32)\n");
        out.push_str("declare double @strtod(ptr, ptr)\n");
        out.push_str("declare ptr @strstr(ptr, ptr)\n");
        out.push_str("declare i32 @snprintf(ptr, i64, ptr, ...)\n");
        out.push_str("declare ptr @memcpy(ptr, ptr, i64)\n");
        out.push_str("declare ptr @getenv(ptr)\n");
        out.push_str("declare void @GlitchRestHost_Run(ptr, i32, i32, ptr, ptr)\n");
        out.push_str("declare ptr @System_IO_File_ReadAllText(ptr)\n");
        out.push_str("declare void @System_IO_File_WriteAllText(ptr, ptr)\n");
        out.push_str("declare void @System_Console_WriteLine_String(ptr)\n");
        out.push_str("declare void @System_Console_WriteLine_I64(i64)\n");
        out.push_str("declare void @System_Console_WriteLine_Double(double)\n");
        out.push_str("declare void @System_Console_WriteLine_Bool(i1)\n");
        out.push_str("declare ptr @GlitchString_Lock()\n");
        out.push_str("declare void @GlitchString_Unlock(ptr)\n");
        out.push_str("@glitch_live_allocations = internal global i64 0\n");
        out.push_str("@glitch_exception_pending = internal global ptr null\n");
        out.push_str(
            "%glitch.task = type { i32, ptr }\n\
            define ptr @glitch_task_from_result_ptr(ptr %result) {\n\
            entry:\n\
              %task = call ptr @glitch_calloc(i64 1, i64 16)\n\
              %completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0\n\
              store i32 1, ptr %completed_ptr\n\
              %result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1\n\
              store ptr %result, ptr %result_ptr\n\
              ret ptr %task\n\
            }\n\
            define ptr @glitch_task_from_result_i32(i32 %result) {\n\
            entry:\n\
              %val_ptr = inttoptr i32 %result to ptr\n\
              %task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)\n\
              ret ptr %task\n\
            }\n\
            define ptr @glitch_task_from_result_i64(i64 %result) {\n\
            entry:\n\
              %val_ptr = inttoptr i64 %result to ptr\n\
              %task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)\n\
              ret ptr %task\n\
            }\n\
            define ptr @glitch_task_from_result_double(double %result) {\n\
            entry:\n\
              %cast = bitcast double %result to i64\n\
              %val_ptr = inttoptr i64 %cast to ptr\n\
              %task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)\n\
              ret ptr %task\n\
            }\n\
            define ptr @glitch_task_get_result_ptr(ptr %task) {\n\
            entry:\n\
              %is_null = icmp eq ptr %task, null\n\
              br i1 %is_null, label %null_case, label %normal_case\n\
            null_case:\n\
              ret ptr null\n\
            normal_case:\n\
              %result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1\n\
              %result = load ptr, ptr %result_ptr\n\
              ret ptr %result\n\
            }\n\
            define i32 @glitch_task_get_result_i32(ptr %task) {\n\
            entry:\n\
              %ptr = call ptr @glitch_task_get_result_ptr(ptr %task)\n\
              %val = ptrtoint ptr %ptr to i32\n\
              ret i32 %val\n\
            }\n\
            define i64 @glitch_task_get_result_i64(ptr %task) {\n\
            entry:\n\
              %ptr = call ptr @glitch_task_get_result_ptr(ptr %task)\n\
              %val = ptrtoint ptr %ptr to i64\n\
              ret i64 %val\n\
            }\n\
            define double @glitch_task_get_result_double(ptr %task) {\n\
            entry:\n\
              %ptr = call ptr @glitch_task_get_result_ptr(ptr %task)\n\
              %val = ptrtoint ptr %ptr to i64\n\
              %res = bitcast i64 %val to double\n\
              ret double %res\n\
            }\n",
        );
        out.push_str(
            "define ptr @glitch_calloc(i64 %count, i64 %size) {\nentry:\n  %value = call ptr @calloc(i64 %count, i64 %size)\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %count_alloc\ncount_alloc:\n  %live = load i64, ptr @glitch_live_allocations\n  %next = add i64 %live, 1\n  store i64 %next, ptr @glitch_live_allocations\n  br label %done\ndone:\n  ret ptr %value\n}\n",
        );
        out.push_str(
            "define ptr @glitch_realloc(ptr %old, i64 %size) {\nentry:\n  %value = call ptr @realloc(ptr %old, i64 %size)\n  %old_null = icmp eq ptr %old, null\n  %new_valid = icmp ne ptr %value, null\n  %count_it = and i1 %old_null, %new_valid\n  br i1 %count_it, label %count_alloc, label %done\ncount_alloc:\n  %live = load i64, ptr @glitch_live_allocations\n  %next = add i64 %live, 1\n  store i64 %next, ptr @glitch_live_allocations\n  br label %done\ndone:\n  ret ptr %value\n}\n",
        );
        out.push_str(
            "define void @glitch_free(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  call void @free(ptr %value)\n  %live = load i64, ptr @glitch_live_allocations\n  %next = sub i64 %live, 1\n  store i64 %next, ptr @glitch_live_allocations\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define i64 @glitch_string_length(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %empty, label %read_len\nempty:\n  ret i64 0\nread_len:\n  %length_ptr = getelementptr i8, ptr %value, i64 -8\n  %length = load i64, ptr %length_ptr\n  ret i64 %length\n}\n",
        );
        out.push_str(
            "define i1 @glitch_route_match(ptr %template, ptr %path) {\nentry:\n  %template_index = alloca i64\n  %path_index = alloca i64\n  store i64 0, ptr %template_index\n  store i64 0, ptr %path_index\n  br label %scan\nscan:\n  %ti = load i64, ptr %template_index\n  %pi = load i64, ptr %path_index\n  %template_ptr = getelementptr inbounds i8, ptr %template, i64 %ti\n  %path_ptr = getelementptr inbounds i8, ptr %path, i64 %pi\n  %template_char = load i8, ptr %template_ptr\n  %path_char = load i8, ptr %path_ptr\n  %template_done = icmp eq i8 %template_char, 0\n  br i1 %template_done, label %finish, label %inspect\ninspect:\n  %parameter_start = icmp eq i8 %template_char, 123\n  br i1 %parameter_start, label %skip_parameter_name, label %literal\nliteral:\n  %same = icmp eq i8 %template_char, %path_char\n  br i1 %same, label %advance_both, label %no_match\nadvance_both:\n  %ti_next = add i64 %ti, 1\n  %pi_next = add i64 %pi, 1\n  store i64 %ti_next, ptr %template_index\n  store i64 %pi_next, ptr %path_index\n  br label %scan\nskip_parameter_name:\n  %parameter_template_index = alloca i64\n  %after_open = add i64 %ti, 1\n  store i64 %after_open, ptr %parameter_template_index\n  br label %parameter_name_loop\nparameter_name_loop:\n  %parameter_ti = load i64, ptr %parameter_template_index\n  %parameter_template_ptr = getelementptr inbounds i8, ptr %template, i64 %parameter_ti\n  %parameter_template_char = load i8, ptr %parameter_template_ptr\n  %parameter_name_done = icmp eq i8 %parameter_template_char, 125\n  %parameter_template_done = icmp eq i8 %parameter_template_char, 0\n  br i1 %parameter_template_done, label %no_match, label %parameter_name_check\nparameter_name_check:\n  br i1 %parameter_name_done, label %consume_parameter, label %advance_parameter_name\nadvance_parameter_name:\n  %parameter_ti_next = add i64 %parameter_ti, 1\n  store i64 %parameter_ti_next, ptr %parameter_template_index\n  br label %parameter_name_loop\nconsume_parameter:\n  %after_close = add i64 %parameter_ti, 1\n  store i64 %after_close, ptr %template_index\n  %parameter_path_start = load i64, ptr %path_index\n  %parameter_first_ptr = getelementptr inbounds i8, ptr %path, i64 %parameter_path_start\n  %parameter_first = load i8, ptr %parameter_first_ptr\n  %parameter_empty = icmp eq i8 %parameter_first, 0\n  %parameter_slash = icmp eq i8 %parameter_first, 47\n  %parameter_invalid = or i1 %parameter_empty, %parameter_slash\n  br i1 %parameter_invalid, label %no_match, label %parameter_path_loop\nparameter_path_loop:\n  %parameter_pi = load i64, ptr %path_index\n  %parameter_path_ptr = getelementptr inbounds i8, ptr %path, i64 %parameter_pi\n  %parameter_path_char = load i8, ptr %parameter_path_ptr\n  %parameter_path_done = icmp eq i8 %parameter_path_char, 0\n  %parameter_path_query = icmp eq i8 %parameter_path_char, 63\n  %parameter_path_slash = icmp eq i8 %parameter_path_char, 47\n  %parameter_path_end = or i1 %parameter_path_done, %parameter_path_query\n  %parameter_segment_done = or i1 %parameter_path_end, %parameter_path_slash\n  br i1 %parameter_segment_done, label %scan, label %advance_parameter_path\nadvance_parameter_path:\n  %parameter_pi_next = add i64 %parameter_pi, 1\n  store i64 %parameter_pi_next, ptr %path_index\n  br label %parameter_path_loop\nfinish:\n  %path_done = icmp eq i8 %path_char, 0\n  %path_query = icmp eq i8 %path_char, 63\n  %path_finished = or i1 %path_done, %path_query\n  ret i1 %path_finished\nno_match:\n  ret i1 false\n}\n",
        );
        out.push_str(
            "define ptr @glitch_path_segment_string(ptr %path, i64 %target) {\nentry:\n  %position = alloca i64\n  %segment = alloca i64\n  %start = alloca i64\n  store i64 0, ptr %position\n  store i64 0, ptr %segment\n  br label %skip_slashes\nskip_slashes:\n  %skip_position = load i64, ptr %position\n  %skip_ptr = getelementptr inbounds i8, ptr %path, i64 %skip_position\n  %skip_char = load i8, ptr %skip_ptr\n  %skip_end = icmp eq i8 %skip_char, 0\n  %skip_query = icmp eq i8 %skip_char, 63\n  %skip_done = or i1 %skip_end, %skip_query\n  br i1 %skip_done, label %missing, label %skip_check\nskip_check:\n  %is_slash = icmp eq i8 %skip_char, 47\n  br i1 %is_slash, label %advance_slash, label %begin_segment\nadvance_slash:\n  %after_slash = add i64 %skip_position, 1\n  store i64 %after_slash, ptr %position\n  br label %skip_slashes\nbegin_segment:\n  store i64 %skip_position, ptr %start\n  %current_segment = load i64, ptr %segment\n  %is_target = icmp eq i64 %current_segment, %target\n  br i1 %is_target, label %scan_target, label %skip_segment\nscan_target:\n  %target_position = load i64, ptr %position\n  %target_ptr = getelementptr inbounds i8, ptr %path, i64 %target_position\n  %target_char = load i8, ptr %target_ptr\n  %target_end = icmp eq i8 %target_char, 0\n  %target_query = icmp eq i8 %target_char, 63\n  %target_slash = icmp eq i8 %target_char, 47\n  %target_path_end = or i1 %target_end, %target_query\n  %target_done = or i1 %target_path_end, %target_slash\n  br i1 %target_done, label %copy_target, label %advance_target\nadvance_target:\n  %target_next = add i64 %target_position, 1\n  store i64 %target_next, ptr %position\n  br label %scan_target\ncopy_target:\n  %target_start = load i64, ptr %start\n  %target_length = sub i64 %target_position, %target_start\n  %target_data = call ptr @glitch_string_allocate(i64 %target_length)\n  %target_source = getelementptr inbounds i8, ptr %path, i64 %target_start\n  call ptr @memcpy(ptr %target_data, ptr %target_source, i64 %target_length)\n  ret ptr %target_data\nskip_segment:\n  %segment_position = load i64, ptr %position\n  %segment_ptr = getelementptr inbounds i8, ptr %path, i64 %segment_position\n  %segment_char = load i8, ptr %segment_ptr\n  %segment_end = icmp eq i8 %segment_char, 0\n  %segment_query = icmp eq i8 %segment_char, 63\n  %segment_done = or i1 %segment_end, %segment_query\n  br i1 %segment_done, label %missing, label %segment_check\nsegment_check:\n  %segment_slash = icmp eq i8 %segment_char, 47\n  br i1 %segment_slash, label %next_segment, label %advance_segment\nadvance_segment:\n  %segment_next_position = add i64 %segment_position, 1\n  store i64 %segment_next_position, ptr %position\n  br label %skip_segment\nnext_segment:\n  %next_segment_value = add i64 %current_segment, 1\n  store i64 %next_segment_value, ptr %segment\n  br label %skip_slashes\nmissing:\n  %empty = call ptr @glitch_string_allocate(i64 0)\n  ret ptr %empty\n}\ndefine i64 @glitch_path_segment_i64(ptr %path, i64 %target) {\nentry:\n  %text = call ptr @glitch_path_segment_string(ptr %path, i64 %target)\n  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)\n  call void @glitch_string_release(ptr %text)\n  ret i64 %value\n}\n",
        );
        out.push_str(
            "define ptr @glitch_query_value_string(ptr %path, ptr %key, i64 %key_length) {\nentry:\n  %position = alloca i64\n  store i64 0, ptr %position\n  br label %find_query\nfind_query:\n  %find_position = load i64, ptr %position\n  %find_ptr = getelementptr inbounds i8, ptr %path, i64 %find_position\n  %find_char = load i8, ptr %find_ptr\n  %find_end = icmp eq i8 %find_char, 0\n  br i1 %find_end, label %query_missing, label %find_check\nfind_check:\n  %find_question = icmp eq i8 %find_char, 63\n  br i1 %find_question, label %next_pair, label %advance_find\nadvance_find:\n  %find_next = add i64 %find_position, 1\n  store i64 %find_next, ptr %position\n  br label %find_query\nnext_pair:\n  %pair_position = load i64, ptr %position\n  %pair_start = add i64 %pair_position, 1\n  store i64 %pair_start, ptr %position\n  br label %inspect_pair\ninspect_pair:\n  %inspect_position = load i64, ptr %position\n  %inspect_ptr = getelementptr inbounds i8, ptr %path, i64 %inspect_position\n  %inspect_char = load i8, ptr %inspect_ptr\n  %inspect_end = icmp eq i8 %inspect_char, 0\n  br i1 %inspect_end, label %query_missing, label %compare_key\ncompare_key:\n  %key_cmp = call i32 @strncmp(ptr %inspect_ptr, ptr %key, i64 %key_length)\n  %key_equal = icmp eq i32 %key_cmp, 0\n  %after_key_position = add i64 %inspect_position, %key_length\n  %after_key_ptr = getelementptr inbounds i8, ptr %path, i64 %after_key_position\n  %after_key_char = load i8, ptr %after_key_ptr\n  %has_equals = icmp eq i8 %after_key_char, 61\n  %key_match = and i1 %key_equal, %has_equals\n  br i1 %key_match, label %scan_value, label %skip_pair\nscan_value:\n  %value_start = add i64 %after_key_position, 1\n  store i64 %value_start, ptr %position\n  br label %value_loop\nvalue_loop:\n  %value_position = load i64, ptr %position\n  %value_ptr = getelementptr inbounds i8, ptr %path, i64 %value_position\n  %value_char = load i8, ptr %value_ptr\n  %value_end = icmp eq i8 %value_char, 0\n  %value_amp = icmp eq i8 %value_char, 38\n  %value_done = or i1 %value_end, %value_amp\n  br i1 %value_done, label %copy_value, label %advance_value\nadvance_value:\n  %value_next = add i64 %value_position, 1\n  store i64 %value_next, ptr %position\n  br label %value_loop\ncopy_value:\n  %value_length = sub i64 %value_position, %value_start\n  %value_data = call ptr @glitch_string_allocate(i64 %value_length)\n  %value_source = getelementptr inbounds i8, ptr %path, i64 %value_start\n  call ptr @memcpy(ptr %value_data, ptr %value_source, i64 %value_length)\n  ret ptr %value_data\nskip_pair:\n  %skip_pair_position = load i64, ptr %position\n  %skip_pair_ptr = getelementptr inbounds i8, ptr %path, i64 %skip_pair_position\n  %skip_pair_char = load i8, ptr %skip_pair_ptr\n  %skip_pair_end = icmp eq i8 %skip_pair_char, 0\n  br i1 %skip_pair_end, label %query_missing, label %skip_pair_check\nskip_pair_check:\n  %skip_pair_amp = icmp eq i8 %skip_pair_char, 38\n  br i1 %skip_pair_amp, label %next_pair, label %advance_skip_pair\nadvance_skip_pair:\n  %skip_pair_next = add i64 %skip_pair_position, 1\n  store i64 %skip_pair_next, ptr %position\n  br label %skip_pair\nquery_missing:\n  %query_empty = call ptr @glitch_string_allocate(i64 0)\n  ret ptr %query_empty\n}\ndefine i64 @glitch_query_value_i64(ptr %path, ptr %key, i64 %key_length) {\nentry:\n  %text = call ptr @glitch_query_value_string(ptr %path, ptr %key, i64 %key_length)\n  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)\n  call void @glitch_string_release(ptr %text)\n  ret i64 %value\n}\n",
        );
        out.push_str(
            "define ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length) {\nentry:\n  %match = call ptr @strstr(ptr %json, ptr %token)\n  %missing = icmp eq ptr %match, null\n  br i1 %missing, label %json_missing, label %after_token\n  \nafter_token:\n  %cursor = alloca ptr\n  %token_end = getelementptr inbounds i8, ptr %match, i64 %token_length\n  store ptr %token_end, ptr %cursor\n  br label %find_colon\nfind_colon:\n  %colon_ptr = load ptr, ptr %cursor\n  %colon_char = load i8, ptr %colon_ptr\n  %colon_end = icmp eq i8 %colon_char, 0\n  br i1 %colon_end, label %json_missing, label %colon_check\ncolon_check:\n  %is_colon = icmp eq i8 %colon_char, 58\n  br i1 %is_colon, label %after_colon, label %advance_colon\nadvance_colon:\n  %colon_next = getelementptr inbounds i8, ptr %colon_ptr, i64 1\n  store ptr %colon_next, ptr %cursor\n  br label %find_colon\nafter_colon:\n  %value_candidate = getelementptr inbounds i8, ptr %colon_ptr, i64 1\n  store ptr %value_candidate, ptr %cursor\n  br label %skip_json_space\nskip_json_space:\n  %space_ptr = load ptr, ptr %cursor\n  %space_char = load i8, ptr %space_ptr\n  %is_space = icmp eq i8 %space_char, 32\n  %is_tab = icmp eq i8 %space_char, 9\n  %is_cr = icmp eq i8 %space_char, 13\n  %is_lf = icmp eq i8 %space_char, 10\n  %space_a = or i1 %is_space, %is_tab\n  %space_b = or i1 %is_cr, %is_lf\n  %whitespace = or i1 %space_a, %space_b\n  br i1 %whitespace, label %advance_json_space, label %value_kind\nadvance_json_space:\n  %space_next = getelementptr inbounds i8, ptr %space_ptr, i64 1\n  store ptr %space_next, ptr %cursor\n  br label %skip_json_space\nvalue_kind:\n  %is_quote = icmp eq i8 %space_char, 34\n  br i1 %is_quote, label %quoted_start, label %plain_start\nquoted_start:\n  %quoted_value = getelementptr inbounds i8, ptr %space_ptr, i64 1\n  store ptr %quoted_value, ptr %cursor\n  br label %scan_quoted\nscan_quoted:\n  %quoted_ptr = load ptr, ptr %cursor\n  %quoted_char = load i8, ptr %quoted_ptr\n  %quoted_done = icmp eq i8 %quoted_char, 34\n  %quoted_end = icmp eq i8 %quoted_char, 0\n  br i1 %quoted_end, label %json_missing, label %quoted_check\nquoted_check:\n  br i1 %quoted_done, label %copy_quoted, label %advance_quoted\nadvance_quoted:\n  %quoted_next = getelementptr inbounds i8, ptr %quoted_ptr, i64 1\n  store ptr %quoted_next, ptr %cursor\n  br label %scan_quoted\ncopy_quoted:\n  %quoted_start_int = ptrtoint ptr %quoted_value to i64\n  %quoted_end_int = ptrtoint ptr %quoted_ptr to i64\n  %quoted_length = sub i64 %quoted_end_int, %quoted_start_int\n  %quoted_data = call ptr @glitch_string_allocate(i64 %quoted_length)\n  call ptr @memcpy(ptr %quoted_data, ptr %quoted_value, i64 %quoted_length)\n  ret ptr %quoted_data\nplain_start:\n  store ptr %space_ptr, ptr %cursor\n  br label %scan_plain\nscan_plain:\n  %plain_ptr = load ptr, ptr %cursor\n  %plain_char = load i8, ptr %plain_ptr\n  %plain_end = icmp eq i8 %plain_char, 0\n  %plain_comma = icmp eq i8 %plain_char, 44\n  %plain_brace = icmp eq i8 %plain_char, 125\n  %plain_space = icmp eq i8 %plain_char, 32\n  %plain_a = or i1 %plain_end, %plain_comma\n  %plain_b = or i1 %plain_brace, %plain_space\n  %plain_done = or i1 %plain_a, %plain_b\n  br i1 %plain_done, label %copy_plain, label %advance_plain\nadvance_plain:\n  %plain_next = getelementptr inbounds i8, ptr %plain_ptr, i64 1\n  store ptr %plain_next, ptr %cursor\n  br label %scan_plain\ncopy_plain:\n  %plain_start_int = ptrtoint ptr %space_ptr to i64\n  %plain_end_int = ptrtoint ptr %plain_ptr to i64\n  %plain_length = sub i64 %plain_end_int, %plain_start_int\n  %plain_data = call ptr @glitch_string_allocate(i64 %plain_length)\n  call ptr @memcpy(ptr %plain_data, ptr %space_ptr, i64 %plain_length)\n  ret ptr %plain_data\njson_missing:\n  %json_empty = call ptr @glitch_string_allocate(i64 0)\n  ret ptr %json_empty\n}\ndefine i64 @glitch_json_value_i64(ptr %json, ptr %token, i64 %token_length) {\nentry:\n  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)\n  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)\n  call void @glitch_string_release(ptr %text)\n  ret i64 %value\n}\ndefine double @glitch_json_value_double(ptr %json, ptr %token, i64 %token_length) {\nentry:\n  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)\n  %value = call double @strtod(ptr %text, ptr null)\n  call void @glitch_string_release(ptr %text)\n  ret double %value\n}\ndefine i1 @glitch_json_value_bool(ptr %json, ptr %token, i64 %token_length) {\nentry:\n  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)\n  %first = load i8, ptr %text\n  %is_t = icmp eq i8 %first, 116\n  %is_T = icmp eq i8 %first, 84\n  %is_one = icmp eq i8 %first, 49\n  %is_true_text = or i1 %is_t, %is_T\n  %value = or i1 %is_true_text, %is_one\n  call void @glitch_string_release(ptr %text)\n  ret i1 %value\n}\n",
        );
        out.push_str(
            "define ptr @glitch_i64_to_string(i64 %value) {\nentry:\n  %buffer = alloca [32 x i8]\n  %buffer_ptr = getelementptr inbounds [32 x i8], ptr %buffer, i64 0, i64 0\n  %length_i32 = call i32 (ptr, i64, ptr, ...) @snprintf(ptr %buffer_ptr, i64 32, ptr getelementptr inbounds ([5 x i8], ptr @.fmt_json_i64, i64 0, i64 0), i64 %value)\n  %length = sext i32 %length_i32 to i64\n  %text = call ptr @glitch_string_allocate(i64 %length)\n  call ptr @memcpy(ptr %text, ptr %buffer_ptr, i64 %length)\n  ret ptr %text\n}\ndefine ptr @glitch_double_to_string(double %value) {\nentry:\n  %buffer = alloca [64 x i8]\n  %buffer_ptr = getelementptr inbounds [64 x i8], ptr %buffer, i64 0, i64 0\n  %length_i32 = call i32 (ptr, i64, ptr, ...) @snprintf(ptr %buffer_ptr, i64 64, ptr getelementptr inbounds ([6 x i8], ptr @.fmt_json_double, i64 0, i64 0), double %value)\n  %length = sext i32 %length_i32 to i64\n  %text = call ptr @glitch_string_allocate(i64 %length)\n  call ptr @memcpy(ptr %text, ptr %buffer_ptr, i64 %length)\n  ret ptr %text\n}\n",
        );
        out.push_str(
            "define ptr @glitch_string_allocate(i64 %length) {\nentry:\n  %with_null = add i64 %length, 1\n  %size = add i64 %with_null, 16\n  %node = call ptr @glitch_calloc(i64 1, i64 %size)\n  %refs_ptr = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 0\n  store i64 1, ptr %refs_ptr\n  %length_ptr = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 1\n  store i64 %length, ptr %length_ptr\n  %data = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 2, i64 0\n  ret ptr %data\n}\n",
        );
        out.push_str(
            "define ptr @glitch_string_concat(ptr %left, ptr %right) {\nentry:\n  %left_length = call i64 @glitch_string_length(ptr %left)\n  %right_length = call i64 @glitch_string_length(ptr %right)\n  %length = add i64 %left_length, %right_length\n  %data = call ptr @glitch_string_allocate(i64 %length)\n  %left_empty = icmp eq i64 %left_length, 0\n  br i1 %left_empty, label %copy_right, label %copy_left\ncopy_left:\n  call ptr @memcpy(ptr %data, ptr %left, i64 %left_length)\n  br label %copy_right\ncopy_right:\n  %right_empty = icmp eq i64 %right_length, 0\n  br i1 %right_empty, label %done, label %copy_right_data\ncopy_right_data:\n  %right_target = getelementptr inbounds i8, ptr %data, i64 %left_length\n  call ptr @memcpy(ptr %right_target, ptr %right, i64 %right_length)\n  br label %done\ndone:\n  ret ptr %data\n}\n",
        );
        out.push_str(
            "define void @glitch_string_retain(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %retain\nretain:\n  %refs_ptr = getelementptr i8, ptr %value, i64 -16\n  %refs = load i64, ptr %refs_ptr\n  %is_static = icmp uge i64 %refs, 1000000\n  br i1 %is_static, label %done, label %do_retain\ndo_retain:\n  %old_refs = atomicrmw add ptr %refs_ptr, i64 1 seq_cst\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define void @glitch_string_release(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  %refs_ptr = getelementptr i8, ptr %value, i64 -16\n  %refs = load i64, ptr %refs_ptr\n  %is_static = icmp uge i64 %refs, 1000000\n  br i1 %is_static, label %done, label %do_release\ndo_release:\n  %old_refs = atomicrmw sub ptr %refs_ptr, i64 1 seq_cst\n  %destroy = icmp eq i64 %old_refs, 1\n  br i1 %destroy, label %free_node, label %done\nfree_node:\n  %node = getelementptr i8, ptr %value, i64 -16\n  call void @glitch_free(ptr %node)\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str("@.fmt_i64 = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\"\n");
        out.push_str("@.fmt_i32 = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\"\n");
        out.push_str("@.fmt_double = private unnamed_addr constant [4 x i8] c\"%f\\0A\\00\"\n");
        out.push_str("@.fmt_str = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\"\n");
        out.push_str("@.fmt_json_i64 = private unnamed_addr constant [5 x i8] c\"%lld\\00\"\n");
        out.push_str("@.fmt_json_double = private unnamed_addr constant [6 x i8] c\"%.17g\\00\"\n");
        out.push_str("@.json_true = private unnamed_addr constant [5 x i8] c\"true\\00\"\n");
        out.push_str("@.json_false = private unnamed_addr constant [6 x i8] c\"false\\00\"\n");
        out.push_str("@.env_report_leaks = private unnamed_addr constant [20 x i8] c\"GLITCH_REPORT_LEAKS\\00\"\n");
        for global in &self.globals {
            out.push_str(global);
        }
        out.push('\n');
        out.push_str(&self.body);
        Ok(out)
    }

    fn emit_external_declaration(&mut self, function: &TypedFunction) {
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
        self.vars.clear();
        self.drop_order.clear();
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
        for param in &function.params {
            let ty = llvm_ir_type(&param.ty);
            let ptr = self.tmp();
            self.body
                .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
            if has_native_args && param.name == "args" {
                let array = self.tmp();
                let argc = self.tmp();
                let len_ptr = self.tmp();
                let data_ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {array} = alloca %glitch.array\n  {argc} = zext i32 %argc to i64\n  {len_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 0\n  store i64 {argc}, ptr {len_ptr}\n  {data_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 1\n  store ptr %argv, ptr {data_ptr}\n  store ptr {array}, ptr {ptr}\n"
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
        Ok(())
    }

    fn emit_function(&mut self, function: &Function) -> Result<(), String> {
        self.vars.clear();
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
        Ok(())
    }

    fn emit_stmts(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            if self.terminated {
                break;
            }
            self.emit_stmt(stmt)?;
        }
        Ok(())
    }

    fn emit_typed_stmts(&mut self, stmts: &[TypedStmt]) -> Result<(), String> {
        for stmt in stmts {
            if self.terminated {
                break;
            }
            self.emit_typed_stmt(stmt)?;
        }
        Ok(())
    }

    fn emit_typed_stmt(&mut self, stmt: &TypedStmt) -> Result<(), String> {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                let value = self.emit_typed_expr(expr)?;
                let ty = llvm_ir_type(&binding.ty);
                let stored = self.cast_value(value, &ty)?;
                self.retain_for_store(&binding.ty, expr, &stored.value);
                let var = self.vars.get(&binding.name).cloned();
                if let Some(var) = &var {
                    self.emit_var_drop(var);
                }
                let ptr = var.map(|var| var.ptr).unwrap_or_else(|| self.tmp());
                if !self.vars.contains_key(&binding.name) {
                    self.body
                        .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
                }
                self.body.push_str(&format!(
                    "  store {} {}, ptr {ptr}\n",
                    ty.as_ir(),
                    stored.value
                ));
                if let Some(var) = self.vars.get_mut(&binding.name) {
                    var.ty = ty;
                    var.ir_ty = binding.ty.clone();
                    var.drop_kind = binding.drop_kind();
                } else {
                    self.vars.insert(
                        binding.name.clone(),
                        LlVar {
                            ptr,
                            ty,
                            ir_ty: binding.ty.clone(),
                            drop_kind: binding.drop_kind(),
                        },
                    );
                    self.drop_order.push(binding.name.clone());
                }
            }
            TypedStmtKind::Assign { name, expr } => {
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM TIR backend: unknown variable '{name}'"))?;
                let value = self.emit_typed_expr(expr)?;
                let value = self.cast_value(value, &var.ty)?;
                self.retain_for_store(&var.ir_ty, expr, &value.value);
                self.emit_var_drop(&var);
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    var.ty.as_ir(),
                    value.value,
                    var.ptr
                ));
            }
            TypedStmtKind::AssignTarget { target, expr } => match &target.kind {
                TypedExprKind::Field { .. } => {
                    if self.is_opaque_field(target) {
                        self.emit_typed_expr(expr)?;
                        return Ok(());
                    }
                    let field_ptr = self.emit_field_ptr(target)?;
                    let value = self.emit_typed_expr(expr)?;
                    let field_ty = llvm_ir_type(&target.ty);
                    let value = self.cast_value(value, &field_ty)?;
                    self.retain_for_store(&target.ty, expr, &value.value);
                    if let Some(type_name) = object_type_name(&target.ty) {
                        if self.object_types.contains_key(type_name) {
                            let old = self.tmp();
                            self.body.push_str(&format!(
                                "  {old} = load ptr, ptr {}\n",
                                field_ptr.value
                            ));
                            self.emit_drop(type_name, &old);
                        }
                    } else if matches!(target.ty, IrType::String) {
                        let old = self.tmp();
                        self.body.push_str(&format!(
                                "  {old} = load ptr, ptr {}\n  call void @glitch_string_release(ptr {old})\n",
                                field_ptr.value
                            ));
                    }
                    self.body.push_str(&format!(
                        "  store {} {}, ptr {}\n",
                        field_ty.as_ir(),
                        value.value,
                        field_ptr.value
                    ));
                }
                TypedExprKind::Index { target, index } => {
                    self.emit_index_assignment(target, index, expr)?;
                }
                _ => {
                    return Err(format!(
                        "LLVM TIR backend: unsupported assignment target {:?} with type {:?}",
                        target.kind, target.ty
                    ));
                }
            },
            TypedStmtKind::Block(body) => self.emit_typed_stmts(body)?,
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                let raw_condition = self.emit_typed_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                let then_label = self.next_label("if_then");
                let else_label = self.next_label("if_else");
                let end_label = self.next_label("if_end");
                self.body.push_str(&format!(
                    "  br i1 {}, label %{then_label}, label %{else_label}\n",
                    condition.value
                ));
                self.body.push_str(&format!("{then_label}:\n"));
                self.terminated = false;
                self.emit_typed_stmts(then_body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{end_label}\n"));
                }
                self.body.push_str(&format!("{else_label}:\n"));
                self.terminated = false;
                self.emit_typed_stmts(else_body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{end_label}\n"));
                }
                self.body.push_str(&format!("{end_label}:\n"));
                self.terminated = false;
            }
            TypedStmtKind::While { condition, body } => {
                let start = self.next_label("while_cond");
                let loop_body = self.next_label("while_body");
                let end = self.next_label("while_end");
                self.body.push_str(&format!("  br label %{start}\n"));
                self.body.push_str(&format!("{start}:\n"));
                let raw_condition = self.emit_typed_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                self.body.push_str(&format!(
                    "  br i1 {}, label %{loop_body}, label %{end}\n",
                    condition.value
                ));
                self.body.push_str(&format!("{loop_body}:\n"));
                self.terminated = false;
                self.loop_targets.push((start.clone(), end.clone()));
                self.emit_typed_stmts(body)?;
                self.loop_targets.pop();
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{start}\n"));
                }
                self.body.push_str(&format!("{end}:\n"));
                self.terminated = false;
            }
            TypedStmtKind::Print(expr) => {
                let value = self.emit_typed_expr(expr)?;
                self.emit_print(value);
            }
            TypedStmtKind::Return(Some(expr)) => {
                let value = self.emit_typed_expr(expr)?;
                if self.current_return == LlType::Void {
                    self.emit_local_drops(expr_source_name(expr));
                    self.body.push_str("  ret void\n");
                    self.terminated = true;
                    return Ok(());
                }
                let value = self.cast_value(value, &self.current_return.clone())?;
                if let Some(type_name) = object_type_name(&expr.ty) {
                    if !matches!(
                        expr.kind,
                        TypedExprKind::NewObject { .. } | TypedExprKind::Move(_)
                    ) && self.object_types.contains_key(type_name)
                    {
                        self.emit_retain(type_name, &value.value);
                    }
                } else if matches!(expr.ty, IrType::String)
                    && (matches!(
                        expr.kind,
                        TypedExprKind::Field { .. } | TypedExprKind::Index { .. }
                    ) || matches!(
                        &expr.kind,
                        TypedExprKind::Var(name)
                            if self.vars.get(name).is_some_and(|var| var.drop_kind == DropKind::BorrowOnly)
                    ))
                {
                    self.body.push_str(&format!(
                        "  call void @glitch_string_retain(ptr {})\n",
                        value.value
                    ));
                }
                self.emit_local_drops(expr_source_name(expr));
                self.body.push_str(&format!(
                    "  ret {} {}\n",
                    self.current_return.as_ir(),
                    value.value
                ));
                self.terminated = true;
            }
            TypedStmtKind::Return(None) => {
                self.emit_local_drops(None);
                self.emit_default_return();
                self.terminated = true;
            }
            TypedStmtKind::Expr(expr) => {
                let value = self.emit_typed_expr(expr)?;
                self.emit_temporary_drop(expr, &value);
            }
            TypedStmtKind::Try {
                try_body,
                catch_name,
                catch_body,
                finally_body,
            } => {
                self.emit_typed_try(try_body, catch_name.as_deref(), catch_body, finally_body)?;
            }
            TypedStmtKind::Throw(expr) => {
                let exception = self.emit_typed_expr(expr)?;
                let exception = self.cast_value(exception, &LlType::Ptr)?;
                self.body.push_str(&format!(
                    "  store ptr {}, ptr @glitch_exception_pending\n",
                    exception.value
                ));
                self.emit_exception_branch();
            }
            TypedStmtKind::ForEach {
                item,
                collection,
                body,
            } => self.emit_typed_foreach(item, collection, body)?,
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => self.emit_typed_for(
                init.as_deref(),
                condition.as_ref(),
                increment.as_deref(),
                body,
            )?,
            TypedStmtKind::Switch {
                expr,
                cases,
                default,
            } => self.emit_typed_switch(expr, cases, default)?,
            TypedStmtKind::Break => {
                let (_, break_target) = self.loop_targets.last().cloned().ok_or_else(|| {
                    "LLVM TIR backend: 'break' has no enclosing loop or switch".to_string()
                })?;
                self.body.push_str(&format!("  br label %{break_target}\n"));
                self.terminated = true;
            }
            TypedStmtKind::Continue => {
                let (continue_target, _) = self.loop_targets.last().cloned().ok_or_else(|| {
                    "LLVM TIR backend: 'continue' has no enclosing loop".to_string()
                })?;
                if continue_target.is_empty() {
                    return Err("LLVM TIR backend: 'continue' has no enclosing loop".to_string());
                }
                self.body
                    .push_str(&format!("  br label %{continue_target}\n"));
                self.terminated = true;
            }
        }
        Ok(())
    }

    fn emit_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let {
                name,
                declared_type,
                expr,
                ..
            } => {
                let value = self.emit_expr(expr)?;
                let ty = declared_type
                    .as_ref()
                    .map(llvm_type)
                    .unwrap_or_else(|| value.ty.clone());
                let stored = self.cast_value(value, &ty)?;
                let ptr = self.tmp();
                self.body
                    .push_str(&format!("  {ptr} = alloca {}\n", ty.as_ir()));
                self.body.push_str(&format!(
                    "  store {} {}, ptr {ptr}\n",
                    ty.as_ir(),
                    stored.value
                ));
                self.vars.insert(
                    name.clone(),
                    LlVar {
                        ptr,
                        ty,
                        ir_ty: IrType::Unknown(name.clone()),
                        drop_kind: DropKind::None,
                    },
                );
            }
            Stmt::Assign { name, expr } => {
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM backend: unknown variable '{name}'"))?;
                let value = self.emit_expr(expr)?;
                let value = self.cast_value(value, &var.ty)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    var.ty.as_ir(),
                    value.value,
                    var.ptr
                ));
            }
            Stmt::Block(body) => self.emit_stmts(body)?,
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let raw_condition = self.emit_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                let then_label = self.next_label("if_then");
                let else_label = self.next_label("if_else");
                let end_label = self.next_label("if_end");
                self.body.push_str(&format!(
                    "  br i1 {}, label %{then_label}, label %{else_label}\n",
                    condition.value
                ));
                self.body.push_str(&format!("{then_label}:\n"));
                self.terminated = false;
                self.emit_stmts(then_body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{end_label}\n"));
                }
                self.body.push_str(&format!("{else_label}:\n"));
                self.terminated = false;
                self.emit_stmts(else_body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{end_label}\n"));
                }
                self.body.push_str(&format!("{end_label}:\n"));
                self.terminated = false;
            }
            Stmt::While { condition, body } => {
                let start = self.next_label("while_cond");
                let loop_body = self.next_label("while_body");
                let end = self.next_label("while_end");
                self.body.push_str(&format!("  br label %{start}\n"));
                self.body.push_str(&format!("{start}:\n"));
                let raw_condition = self.emit_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                self.body.push_str(&format!(
                    "  br i1 {}, label %{loop_body}, label %{end}\n",
                    condition.value
                ));
                self.body.push_str(&format!("{loop_body}:\n"));
                self.terminated = false;
                self.emit_stmts(body)?;
                if !self.terminated {
                    self.body.push_str(&format!("  br label %{start}\n"));
                }
                self.body.push_str(&format!("{end}:\n"));
                self.terminated = false;
            }
            Stmt::Print(expr) => {
                let value = self.emit_expr(expr)?;
                self.emit_print(value);
            }
            Stmt::Return(Some(expr)) => {
                let value = self.emit_expr(expr)?;
                let value = self.cast_value(value, &self.current_return.clone())?;
                self.body.push_str(&format!(
                    "  ret {} {}\n",
                    self.current_return.as_ir(),
                    value.value
                ));
                self.terminated = true;
            }
            Stmt::Return(None) => {
                self.emit_default_return();
                self.terminated = true;
            }
            Stmt::Expr(expr) => {
                self.emit_expr(expr)?;
            }
            Stmt::For { .. }
            | Stmt::ForEach { .. }
            | Stmt::Switch { .. }
            | Stmt::Try { .. }
            | Stmt::AssignTarget { .. }
            | Stmt::Throw(_)
            | Stmt::Break
            | Stmt::Continue => {
                return Err(format!(
                    "LLVM backend: unsupported statement in current slice: {stmt:?}"
                ));
            }
        }
        Ok(())
    }

    fn emit_expr(&mut self, expr: &Expr) -> Result<LlValue, String> {
        match expr {
            Expr::Int(value) => Ok(LlValue {
                value: value.to_string(),
                ty: LlType::I64,
            }),
            Expr::Float(value) => Ok(LlValue {
                value: value.to_string(),
                ty: LlType::Double,
            }),
            Expr::Bool(value) => Ok(LlValue {
                value: if *value { "1" } else { "0" }.to_string(),
                ty: LlType::I1,
            }),
            Expr::Null => Ok(LlValue {
                value: "null".to_string(),
                ty: LlType::Ptr,
            }),
            Expr::String(value) => Ok(LlValue {
                value: self.string_global(value),
                ty: LlType::Ptr,
            }),
            Expr::Var(name) | Expr::Move(name) => {
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM backend: unknown variable '{name}'"))?;
                let tmp = self.tmp();
                self.body.push_str(&format!(
                    "  {tmp} = load {}, ptr {}\n",
                    var.ty.as_ir(),
                    var.ptr
                ));
                Ok(LlValue {
                    value: tmp,
                    ty: var.ty,
                })
            }
            Expr::Binary { left, op, right } => {
                let left = self.emit_expr(left)?;
                if *op == BinaryOp::Coalesce {
                    return Ok(left);
                }
                if matches!(op, BinaryOp::And | BinaryOp::Or) {
                    let right = self.emit_expr(right)?;
                    return self.emit_logical_value(left, *op, right);
                }
                let raw_right = self.emit_expr(right)?;
                let right = self.cast_value(raw_right, &left.ty)?;
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
                    let op_name = if *op == BinaryOp::BitAnd { "and" } else { "or" };
                    self.body.push_str(&format!(
                        "  {tmp} = {op_name} {} {}, {}\n",
                        left.ty.as_ir(),
                        left.value, right.value
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
            Expr::Unary { op, expr } => {
                let value = self.emit_expr(expr)?;
                match op {
                    UnaryOp::Not => self.emit_not_value(value),
                    UnaryOp::Neg => self.emit_neg_value(value),
                }
            }
            Expr::IncDec {
                name,
                delta,
                prefix,
            } => self.emit_inc_dec_untyped(name, *delta, *prefix),
            Expr::IsPattern { expr, .. } => {
                let _ = self.emit_expr(expr);
                Ok(LlValue {
                    value: "true".to_string(),
                    ty: LlType::I1,
                })
            }
            Expr::Lambda { .. } => Err(
                "LLVM backend: lambda/delegate lowering is not supported in current slice"
                    .to_string(),
            ),
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                let raw_condition = self.emit_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                let when_true = self.emit_expr(when_true)?;
                let raw_false = self.emit_expr(when_false)?;
                let when_false = self.cast_value(raw_false, &when_true.ty)?;
                self.emit_select_value(condition, when_true, when_false)
            }
            Expr::FunctionCall { name, args } => {
                let mut rendered_args = Vec::new();
                for arg in args {
                    let value = self.emit_expr(arg)?;
                    rendered_args.push(format!("{} {}", value.ty.as_ir(), value.value));
                }
                let ret = self
                    .functions
                    .get(name)
                    .map(|signature| signature.return_type.clone())
                    .unwrap_or(LlType::Void);
                if ret == LlType::Void {
                    self.body.push_str(&format!(
                        "  call void @{}({})\n",
                        sanitize(name),
                        rendered_args.join(", ")
                    ));
                    Ok(LlValue {
                        value: "0".to_string(),
                        ty: LlType::I32,
                    })
                } else {
                    let tmp = self.tmp();
                    self.body.push_str(&format!(
                        "  {tmp} = call {} @{}({})\n",
                        ret.as_ir(),
                        sanitize(name),
                        rendered_args.join(", ")
                    ));
                    Ok(LlValue {
                        value: tmp,
                        ty: ret,
                    })
                }
            }
            Expr::Borrow { name, .. } => {
                let var = self
                    .vars
                    .get(name)
                    .ok_or_else(|| format!("LLVM backend: unknown variable '{name}'"))?;
                Ok(LlValue {
                    value: var.ptr.clone(),
                    ty: LlType::Ptr,
                })
            }
            Expr::Await(inner) => Err(format!(
                "LLVM backend: await is not supported in current slice: {inner:?}"
            )),
            _ => Err(format!(
                "LLVM backend: unsupported expression in current slice: {expr:?}"
            )),
        }
    }

    fn emit_typed_expr(&mut self, expr: &TypedExpr) -> Result<LlValue, String> {
        match &expr.kind {
            TypedExprKind::Int(value) => Ok(LlValue {
                value: value.to_string(),
                ty: LlType::I64,
            }),
            TypedExprKind::Float(value) => Ok(LlValue {
                value: value.to_string(),
                ty: LlType::Double,
            }),
            TypedExprKind::Bool(value) => Ok(LlValue {
                value: if *value { "1" } else { "0" }.to_string(),
                ty: LlType::I1,
            }),
            TypedExprKind::Null => Ok(LlValue {
                value: "null".to_string(),
                ty: LlType::Ptr,
            }),
            TypedExprKind::String(value) => Ok(LlValue {
                value: self.string_global(value),
                ty: LlType::Ptr,
            }),
            TypedExprKind::Var(name) => {
                if self.object_types.contains_key(name) {
                    return Ok(LlValue {
                        value: "null".to_string(),
                        ty: LlType::Ptr,
                    });
                }
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM TIR backend: unknown variable '{name}'"))?;
                let tmp = self.tmp();
                self.body.push_str(&format!(
                    "  {tmp} = load {}, ptr {}\n",
                    var.ty.as_ir(),
                    var.ptr
                ));
                Ok(LlValue {
                    value: tmp,
                    ty: var.ty,
                })
            }
            TypedExprKind::Move(name) => {
                let var = self
                    .vars
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("LLVM TIR backend: unknown variable '{name}'"))?;
                let tmp = self.tmp();
                self.body.push_str(&format!(
                    "  {tmp} = load {}, ptr {}\n",
                    var.ty.as_ir(),
                    var.ptr
                ));
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    var.ty.as_ir(),
                    var.ty.default_value(),
                    var.ptr
                ));
                Ok(LlValue {
                    value: tmp,
                    ty: var.ty,
                })
            }
            TypedExprKind::Field { .. } => {
                if let TypedExprKind::Field { target, name } = &expr.kind {
                    if name == "Result" && matches!(target.ty, IrType::Task(_)) {
                        let task_val = self.emit_typed_expr(target)?;
                        let result_ty = expr.ty.clone();
                        let result_llvm_type = llvm_ir_type(&result_ty);
                        if matches!(result_ty, IrType::Void) {
                            return Ok(void_value());
                        } else {
                            let call_res = self.tmp();
                            let helper_name = match &result_ty {
                                IrType::Int | IrType::UInt => "glitch_task_get_result_i32",
                                IrType::Long => "glitch_task_get_result_i64",
                                IrType::Double | IrType::Decimal => "glitch_task_get_result_double",
                                _ => "glitch_task_get_result_ptr",
                            };
                            self.body.push_str(&format!(
                                "  {} = call {} @{}(ptr {})\n",
                                call_res,
                                result_llvm_type.as_ir(),
                                helper_name,
                                task_val.value
                            ));
                            return Ok(LlValue {
                                value: call_res,
                                ty: result_llvm_type,
                            });
                        }
                    }
                    if name == "Target" && matches!(target.ty, IrType::Weak(_)) {
                        let weak_val = self.emit_typed_expr(target)?;
                        if let IrType::Weak(inner) = &target.ty {
                            if matches!(inner.as_ref(), IrType::String) {
                                self.body.push_str(&format!(
                                    "  call void @glitch_string_retain(ptr {})\n",
                                    weak_val.value
                                ));
                            } else if let Some(type_name) = object_type_name(inner) {
                                self.emit_retain(type_name, &weak_val.value);
                            }
                            return Ok(LlValue {
                                value: weak_val.value,
                                ty: llvm_ir_type(inner),
                            });
                        }
                    }
                    if name == "Message" && matches!(target.ty, IrType::Exception) {
                        return self.emit_typed_expr(target);
                    }
                    if name == "Count"
                        && matches!(target.ty, IrType::List(_) | IrType::Dictionary(_, _))
                    {
                        let collection = self.emit_typed_expr(target)?;
                        let len_ptr = self.tmp();
                        let len = self.tmp();
                        let layout = if matches!(target.ty, IrType::List(_)) {
                            "glitch.list"
                        } else {
                            "glitch.dict"
                        };
                        self.body.push_str(&format!(
                            "  {len_ptr} = getelementptr inbounds %{layout}, ptr {}, i32 0, i32 0\n  {len} = load i64, ptr {len_ptr}\n",
                            collection.value
                        ));
                        let count = self.tmp();
                        self.body
                            .push_str(&format!("  {count} = trunc i64 {len} to i32\n"));
                        return Ok(LlValue {
                            value: count,
                            ty: LlType::I32,
                        });
                    }
                    if matches!(name.as_str(), "Length" | "Count")
                        && matches!(target.ty, IrType::Array(_))
                    {
                        let array = self.emit_typed_expr(target)?;
                        let len_ptr = self.tmp();
                        let len = self.tmp();
                        let count = self.tmp();
                        self.body.push_str(&format!(
                            "  {len_ptr} = getelementptr inbounds %glitch.array, ptr {}, i32 0, i32 0\n  {len} = load i64, ptr {len_ptr}\n  {count} = trunc i64 {len} to i32\n",
                            array.value
                        ));
                        return Ok(LlValue {
                            value: count,
                            ty: LlType::I32,
                        });
                    }
                    if name == "Length" && matches!(target.ty, IrType::String) {
                        let string = self.emit_typed_expr(target)?;
                        let len = self.tmp();
                        let count = self.tmp();
                        self.body.push_str(&format!(
                            "  {len} = call i64 @strlen(ptr {})\n  {count} = trunc i64 {len} to i32\n",
                            string.value
                        ));
                        return Ok(LlValue {
                            value: count,
                            ty: LlType::I32,
                        });
                    }
                    if self.is_opaque_field(expr) {
                        return self.default_typed_value(&expr.ty);
                    }
                }
                let ptr = self.emit_field_ptr(expr)?;
                let ty = llvm_ir_type(&expr.ty);
                let value = self.tmp();
                self.body.push_str(&format!(
                    "  {value} = load {}, ptr {}\n",
                    ty.as_ir(),
                    ptr.value
                ));
                Ok(LlValue { value, ty })
            }
            TypedExprKind::IsPattern {
                expr: inner,
                binding,
            } => {
                let value = self.emit_typed_expr(inner)?;
                if let Some(binding) = binding {
                    let binding_ty = llvm_ir_type(&binding.ty);
                    let stored = self.cast_value(value.clone(), &binding_ty)?;
                    let ptr = if let Some(var) = self.vars.get(&binding.name) {
                        var.ptr.clone()
                    } else {
                        let ptr = self.tmp();
                        self.body
                            .push_str(&format!("  {ptr} = alloca {}\n", binding_ty.as_ir()));
                        self.vars.insert(
                            binding.name.clone(),
                            LlVar {
                                ptr: ptr.clone(),
                                ty: binding_ty.clone(),
                                ir_ty: binding.ty.clone(),
                                drop_kind: DropKind::BorrowOnly,
                            },
                        );
                        ptr
                    };
                    self.body.push_str(&format!(
                        "  store {} {}, ptr {ptr}\n",
                        binding_ty.as_ir(),
                        stored.value
                    ));
                }
                self.to_i1(value)
            }
            TypedExprKind::NewObject {
                type_name,
                constructor: _,
                args,
                fields: _,
            } if type_name.starts_with("Weak_")
                || type_name.starts_with("System_WeakReference_")
                || type_name.starts_with("WeakReference_") =>
            {
                if let Some(target) = args.first() {
                    self.emit_typed_expr(target)
                } else {
                    Ok(LlValue {
                        value: "null".to_string(),
                        ty: LlType::Ptr,
                    })
                }
            }
            TypedExprKind::NewObject {
                type_name,
                constructor: _,
                args,
                fields: _,
            } if type_name == "Exception" || type_name == "System.Exception" => {
                if let Some(message) = args.first() {
                    let message = self.emit_typed_expr(message)?;
                    self.cast_value(message, &LlType::Ptr)
                } else {
                    Ok(LlValue {
                        value: self.string_global(""),
                        ty: LlType::Ptr,
                    })
                }
            }
            TypedExprKind::NewObject {
                type_name,
                constructor,
                args,
                fields,
            } => self.emit_new_object(type_name, constructor.as_deref(), args, fields),
            TypedExprKind::NewCollection(ty) => self.emit_new_collection(ty),
            TypedExprKind::NewArray {
                element_type,
                values,
            } => self.emit_new_array(element_type, values),
            TypedExprKind::ArrayLiteral(values) => {
                let element_type = values
                    .first()
                    .map(|value| value.ty.clone())
                    .unwrap_or(IrType::Long);
                self.emit_new_array(&element_type, values)
            }
            TypedExprKind::Index { target, index } => self.emit_collection_index(target, index),
            TypedExprKind::Binary { left, op, right } => {
                let left = self.emit_typed_expr(left)?;
                if *op == BinaryOp::Coalesce {
                    return Ok(left);
                }
                if matches!(op, BinaryOp::And | BinaryOp::Or) {
                    let right = self.emit_typed_expr(right)?;
                    return self.emit_logical_value(left, *op, right);
                }
                if *op == BinaryOp::Add && matches!(expr.ty, IrType::String) {
                    let left = self.cast_value(left, &LlType::Ptr)?;
                    let right = self.emit_typed_expr(right)?;
                    let right = self.cast_value(right, &LlType::Ptr)?;
                    let value = self.tmp();
                    self.body.push_str(&format!(
                        "  {value} = call ptr @glitch_string_concat(ptr {}, ptr {})\n",
                        left.value, right.value
                    ));
                    return Ok(LlValue {
                        value,
                        ty: LlType::Ptr,
                    });
                }
                let raw_right = self.emit_typed_expr(right)?;
                let right = self.cast_value(raw_right, &left.ty)?;
                if left.ty == LlType::Ptr && !op.is_comparison() {
                    return self.default_typed_value(&expr.ty);
                }
                self.emit_binary_value(left, *op, right)
            }
            TypedExprKind::Unary { op, expr } => {
                let value = self.emit_typed_expr(expr)?;
                match op {
                    UnaryOp::Not => self.emit_not_value(value),
                    UnaryOp::Neg => self.emit_neg_value(value),
                }
            }
            TypedExprKind::IncDec {
                target,
                delta,
                prefix,
            } => self.emit_inc_dec_value(target, *delta, *prefix),
            TypedExprKind::Lambda { params, body } => self.emit_lambda_function(params, body),
            TypedExprKind::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                let raw_condition = self.emit_typed_expr(condition)?;
                let condition = self.to_i1(raw_condition)?;
                let result_ty = llvm_ir_type(&expr.ty);
                let result_ptr = self.tmp();
                let true_label = self.next_label("conditional_true");
                let false_label = self.next_label("conditional_false");
                let end_label = self.next_label("conditional_end");
                self.body.push_str(&format!(
                    "  {result_ptr} = alloca {}\n  br i1 {}, label %{true_label}, label %{false_label}\n{true_label}:\n",
                    result_ty.as_ir(),
                    condition.value
                ));
                let true_value = self.emit_typed_expr(when_true)?;
                let true_value = self.cast_value(true_value, &result_ty)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {result_ptr}\n  br label %{end_label}\n{false_label}:\n",
                    result_ty.as_ir(),
                    true_value.value
                ));
                let false_value = self.emit_typed_expr(when_false)?;
                let false_value = self.cast_value(false_value, &result_ty)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {result_ptr}\n  br label %{end_label}\n{end_label}:\n",
                    result_ty.as_ir(),
                    false_value.value
                ));
                let result = self.tmp();
                self.body.push_str(&format!(
                    "  {result} = load {}, ptr {result_ptr}\n",
                    result_ty.as_ir()
                ));
                Ok(LlValue {
                    value: result,
                    ty: result_ty,
                })
            }
            TypedExprKind::Call(call) => {
                if let TypedCallKind::Method { target, name, .. } = &call.kind {
                    let is_task = match &target.ty {
                        IrType::Unknown(n) => n == "Task" || n.starts_with("Task<"),
                        _ => false,
                    };
                    if is_task && name == "Run" {
                        return self.emit_task_run_inline(call, &expr.ty);
                    }
                    if is_task && name == "FromResult" {
                        return self.emit_task_from_result_inline(call, &expr.ty);
                    }
                }
                match &call.kind {
                    TypedCallKind::Function { name, symbol } => {
                if name == "sizeof" {
                    let size = if let Some(arg) = call.args.first() {
                        if let TypedExprKind::Var(type_name) = &arg.kind {
                            if self.object_types.contains_key(type_name) {
                                let llvm_name = llvm_object_name(type_name);
                                let size_ptr = self.tmp();
                                let size = self.tmp();
                                self.body.push_str(&format!(
                                    "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n"
                                ));
                                return Ok(LlValue {
                                    value: size,
                                    ty: LlType::I64,
                                });
                            }
                            match type_name.as_str() {
                                "bool" => 1,
                                "byte" | "sbyte" => 1,
                                "short" | "ushort" => 2,
                                "int" | "uint" => 4,
                                "long" | "ulong" => 8,
                                "float" => 4,
                                "double" => 8,
                                _ => 8,
                            }
                        } else {
                            4
                        }
                            } else {
                                4
                            };
                            return Ok(LlValue {
                                value: size.to_string(),
                                ty: LlType::I32,
                            });
                        }
                        if matches!(
                            name.as_str(),
                            "Ok" | "Created" | "CreatedAtAction" | "Accepted"
                        ) {
                            return self.emit_mvc_result_value(name, &call.args);
                        }
                        if matches!(name.as_str(), "NoContent" | "NotFound" | "BadRequest") {
                            for arg in &call.args {
                                let value = self.emit_typed_expr(arg)?;
                                self.emit_temporary_drop(arg, &value);
                            }
                            return Ok(LlValue {
                                value: self.string_global(""),
                                ty: LlType::Ptr,
                            });
                        }
                        if name == "GlitchRestHost_Run" || symbol == "GlitchRestHost_Run" {
                            return self.emit_rest_host_run(&call.args);
                        }
                        if name == "glitch_register_attribute_routes"
                            || symbol == "glitch_register_attribute_routes"
                        {
                            for arg in &call.args {
                                let value = self.emit_typed_expr(arg)?;
                                self.emit_temporary_drop(arg, &value);
                            }
                            return Ok(void_value());
                        }
                        if name == "GlitchEndpointHandlers_Contains"
                            || symbol == "GlitchEndpointHandlers_Contains"
                        {
                            return self.emit_generated_endpoint_call(
                                "glitch_endpoint_handlers_contains",
                                &call.args,
                                LlType::I1,
                            );
                        }
                        if name == "GlitchEndpointHandlers_Invoke"
                            || symbol == "GlitchEndpointHandlers_Invoke"
                        {
                            return self.emit_generated_endpoint_call(
                                "glitch_endpoint_handlers_invoke",
                                &call.args,
                                LlType::Ptr,
                            );
                        }
                        if name == "GlitchMiddlewareHandlers_Apply"
                            || symbol == "GlitchMiddlewareHandlers_Apply"
                        {
                            let [app, text] = call.args.as_slice() else {
                                return Err(
                                "LLVM TIR backend: GlitchMiddlewareHandlers_Apply expects app and text"
                                    .to_string(),
                            );
                            };
                            let app_value = self.emit_typed_expr(app)?;
                            let text_value = self.emit_typed_expr(text)?;
                            self.emit_temporary_drop(app, &app_value);
                            return self.cast_value(text_value, &LlType::Ptr);
                        }
                        if self.functions.contains_key(symbol) {
                            self.emit_typed_function_call(symbol, &call.args)
                        } else if self.vars.contains_key(symbol) {
                            let var = self.vars.get(symbol).unwrap().clone();
                            let delegate_ptr = self.tmp();
                            self.body.push_str(&format!(
                                "  {} = load ptr, ptr {}\n",
                                delegate_ptr, var.ptr
                            ));
                            let fn_ptr_addr = self.tmp();
                            self.body.push_str(&format!(
                            "  {} = getelementptr inbounds {{ ptr, ptr }}, ptr {}, i32 0, i32 0\n",
                            fn_ptr_addr, delegate_ptr
                        ));
                            let fn_ptr = self.tmp();
                            self.body.push_str(&format!(
                                "  {} = load ptr, ptr {}\n",
                                fn_ptr, fn_ptr_addr
                            ));
                            let env_ptr_addr = self.tmp();
                            self.body.push_str(&format!(
                            "  {} = getelementptr inbounds {{ ptr, ptr }}, ptr {}, i32 0, i32 1\n",
                            env_ptr_addr, delegate_ptr
                        ));
                            let env_ptr = self.tmp();
                            self.body.push_str(&format!(
                                "  {} = load ptr, ptr {}\n",
                                env_ptr, env_ptr_addr
                            ));
                            let mut arg_vals = Vec::new();
                            arg_vals.push(format!("ptr {env_ptr}"));
                            for arg in &call.args {
                                let val = self.emit_typed_expr(arg)?;
                                arg_vals.push(format!("{} {}", val.ty.as_ir(), val.value));
                            }
                            let result_reg = self.tmp();
                            let ret_ty = llvm_ir_type(&expr.ty);
                            self.body.push_str(&format!(
                                "  {} = call {} {}({})\n",
                                result_reg,
                                ret_ty.as_ir(),
                                fn_ptr,
                                arg_vals.join(", ")
                            ));
                            Ok(LlValue {
                                value: result_reg,
                                ty: ret_ty,
                            })
                        } else {
                            self.emit_opaque_call(None, &call.args, &expr.ty)
                        }
                    }
                    TypedCallKind::Method {
                        target,
                        name,
                        symbol,
                        resolution,
                    } => match resolution {
                        CallResolution::InstanceMethod => {
                            let resolved_symbol = match &target.ty {
                                IrType::Interface(interface_name) => self
                                    .resolve_interface_method_symbol(
                                        interface_name,
                                        name,
                                        call.args.len(),
                                    )
                                    .unwrap_or_else(|| symbol.clone()),
                                _ => symbol.clone(),
                            };
                            if self.functions.contains_key(&resolved_symbol) {
                                let receiver = self.emit_typed_expr(target)?;
                                let result = self.emit_typed_call(
                                    &resolved_symbol,
                                    std::iter::once(receiver.clone()),
                                    &call.args,
                                )?;
                                self.emit_temporary_drop(target, &receiver);
                                Ok(result)
                            } else {
                                self.emit_opaque_call(Some(target), &call.args, &expr.ty)
                            }
                        }
                        CallResolution::StaticFunction => {
                            if self.functions.contains_key(symbol) {
                                self.emit_typed_function_call(symbol, &call.args)
                            } else {
                                self.emit_opaque_call(Some(target), &call.args, &expr.ty)
                            }
                        }
                        CallResolution::CollectionMethod => {
                            self.emit_collection_method(target, name, &call.args)
                        }
                        CallResolution::TaskMethod => {
                            self.emit_task_method(target, name, &call.args, &expr.ty)
                        }
                        CallResolution::WeakMethod => {
                            self.emit_weak_method(target, name, &call.args)
                        }
                        CallResolution::EndpointHandlerRegistration { .. } => {
                            let receiver = self.emit_typed_expr(target)?;
                            for arg in &call.args {
                                if matches!(arg.kind, TypedExprKind::FunctionSymbol(_)) {
                                    continue;
                                }
                                let value = self.emit_typed_expr(arg)?;
                                self.emit_temporary_drop(arg, &value);
                            }
                            self.emit_temporary_drop(target, &receiver);
                            Ok(void_value())
                        }
                        CallResolution::Unknown => {
                            if let IrType::Interface(interface_name) = &target.ty {
                                if let Some(resolved_symbol) = self.resolve_interface_method_symbol(
                                    interface_name,
                                    name,
                                    call.args.len(),
                                ) {
                                    let receiver = self.emit_typed_expr(target)?;
                                    let result = self.emit_typed_call(
                                        &resolved_symbol,
                                        std::iter::once(receiver.clone()),
                                        &call.args,
                                    )?;
                                    self.emit_temporary_drop(target, &receiver);
                                    return Ok(result);
                                }
                            }
                            self.emit_opaque_call(Some(target), &call.args, &expr.ty)
                        }
                    },
                }
            }
            TypedExprKind::Borrow { name, .. } => {
                if !self.vars.contains_key(name) {
                    let inner_type = match &expr.ty {
                        IrType::Ref(inner) => inner.as_ref().clone(),
                        _ => IrType::Unknown(format!("out {name}")),
                    };
                    let ty = llvm_ir_type(&inner_type);
                    let ptr = self.tmp();
                    self.body.push_str(&format!(
                        "  {ptr} = alloca {}\n  store {} {}, ptr {ptr}\n",
                        ty.as_ir(),
                        ty.as_ir(),
                        ty.default_value()
                    ));
                    self.vars.insert(
                        name.clone(),
                        LlVar {
                            ptr,
                            ty,
                            ir_ty: inner_type,
                            drop_kind: DropKind::BorrowOnly,
                        },
                    );
                }
                let var = self.vars.get(name).expect("out variable was inserted");
                Ok(LlValue {
                    value: var.ptr.clone(),
                    ty: LlType::Ptr,
                })
            }
            TypedExprKind::Await(inner) => {
                let task_val = self.emit_typed_expr(inner)?;
                let result_ty = expr.ty.clone();
                let result_llvm_type = llvm_ir_type(&result_ty);
                if matches!(result_ty, IrType::Void) {
                    Ok(void_value())
                } else {
                    let call_res = self.tmp();
                    let helper_name = match &result_ty {
                        IrType::Int | IrType::UInt => "glitch_task_get_result_i32",
                        IrType::Long => "glitch_task_get_result_i64",
                        IrType::Double | IrType::Decimal => "glitch_task_get_result_double",
                        _ => "glitch_task_get_result_ptr",
                    };
                    self.body.push_str(&format!(
                        "  {} = call {} @{}(ptr {})\n",
                        call_res,
                        result_llvm_type.as_ir(),
                        helper_name,
                        task_val.value
                    ));
                    Ok(LlValue {
                        value: call_res,
                        ty: result_llvm_type,
                    })
                }
            }
            TypedExprKind::FunctionSymbol(name) => {
                let id = self.lambda_id;
                self.lambda_id += 1;
                let wrapper_name = format!("glitch_delegate_wrapper_{name}_{id}");

                let (ret_ty, params) = if let Some(sig) = self.functions.get(name) {
                    (sig.return_type.clone(), sig.params.clone())
                } else {
                    (LlType::Ptr, vec![LlType::Ptr])
                };

                let mut wrapper_params = vec!["ptr %env".to_string()];
                let mut call_args = Vec::new();
                for (i, p) in params.iter().enumerate() {
                    wrapper_params.push(format!("{} %arg{}", p.as_ir(), i));
                    call_args.push(format!("{} %arg{}", p.as_ir(), i));
                }

                // Swap self.body to emit the wrapper function globally
                let saved_body = std::mem::take(&mut self.body);
                self.body.push_str(&format!(
                    "define {} @{}({}) {{\nentry:\n",
                    ret_ty.as_ir(),
                    wrapper_name,
                    wrapper_params.join(", ")
                ));
                if ret_ty == LlType::Void {
                    self.body.push_str(&format!(
                        "  call void @{}({})\n  ret void\n}}\n\n",
                        name,
                        call_args.join(", ")
                    ));
                } else {
                    let tmp_reg = format!("%t_wrap_{}", self.tmp);
                    self.tmp += 1;
                    self.body.push_str(&format!(
                        "  {} = call {} @{}({})\n  ret {} {}\n}}\n\n",
                        tmp_reg,
                        ret_ty.as_ir(),
                        name,
                        call_args.join(", "),
                        ret_ty.as_ir(),
                        tmp_reg
                    ));
                }
                let wrapper_func = std::mem::replace(&mut self.body, saved_body);
                self.globals.push(wrapper_func);

                // Allocate the delegate struct on the stack of the current function.
                let delegate_ptr = self.tmp();
                self.body
                    .push_str(&format!("  {} = alloca {{ ptr, ptr }}\n", delegate_ptr));
                let fn_field = self.tmp();
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds {{ ptr, ptr }}, ptr {}, i32 0, i32 0\n",
                    fn_field, delegate_ptr
                ));
                self.body.push_str(&format!(
                    "  store ptr @{}, ptr {}\n",
                    wrapper_name, fn_field
                ));
                let env_field = self.tmp();
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds {{ ptr, ptr }}, ptr {}, i32 0, i32 1\n",
                    env_field, delegate_ptr
                ));
                self.body
                    .push_str(&format!("  store ptr null, ptr {}\n", env_field));

                Ok(LlValue {
                    value: delegate_ptr,
                    ty: LlType::Ptr,
                })
            }
            _ => Err(format!(
                "LLVM TIR backend: unsupported expression in current slice: {expr:?}"
            )),
        }
    }

    fn emit_typed_function_call(
        &mut self,
        name: &str,
        args: &[TypedExpr],
    ) -> Result<LlValue, String> {
        self.emit_typed_call(name, std::iter::empty(), args)
    }

    fn emit_mvc_result_value(&mut self, name: &str, args: &[TypedExpr]) -> Result<LlValue, String> {
        let selected_index = if name == "CreatedAtAction" {
            args.len().checked_sub(1)
        } else {
            (!args.is_empty()).then_some(0)
        };
        let mut values = Vec::with_capacity(args.len());
        for arg in args {
            values.push(self.emit_typed_expr(arg)?);
        }
        let Some(selected_index) = selected_index else {
            return Ok(LlValue {
                value: self.string_global(""),
                ty: LlType::Ptr,
            });
        };
        let selected = values[selected_index].clone();
        let selected_expr = &args[selected_index];
        if !matches!(
            selected_expr.kind,
            TypedExprKind::NewObject { .. } | TypedExprKind::Move(_)
        ) {
            if let Some(type_name) = object_type_name(&selected_expr.ty) {
                if self.object_types.contains_key(type_name) {
                    self.emit_retain(type_name, &selected.value);
                }
            } else if matches!(selected_expr.ty, IrType::String) {
                self.body.push_str(&format!(
                    "  call void @glitch_string_retain(ptr {})\n",
                    selected.value
                ));
            }
        }
        for (index, (arg, value)) in args.iter().zip(values.iter()).enumerate() {
            if index != selected_index {
                self.emit_temporary_drop(arg, value);
            }
        }
        self.cast_value(selected, &LlType::Ptr)
    }

    fn emit_generated_endpoint_call(
        &mut self,
        name: &str,
        args: &[TypedExpr],
        return_type: LlType,
    ) -> Result<LlValue, String> {
        let expected = if name == "glitch_endpoint_handlers_invoke" {
            4
        } else {
            3
        };
        if args.len() != expected {
            return Err(format!(
                "LLVM TIR backend: {name} expects {expected} arguments"
            ));
        }
        let mut rendered = Vec::new();
        for arg in args {
            let value = self.emit_typed_expr(arg)?;
            let value = self.cast_value(value, &LlType::Ptr)?;
            rendered.push(value.value);
        }
        let result = self.tmp();
        self.body.push_str(&format!(
            "  {result} = call {} @{name}({})\n",
            return_type.as_ir(),
            rendered
                .iter()
                .map(|value| format!("ptr {value}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        Ok(LlValue {
            value: result,
            ty: return_type,
        })
    }

    fn emit_rest_host_run(&mut self, args: &[TypedExpr]) -> Result<LlValue, String> {
        let [app, port, max_requests] = args else {
            return Err(
                "LLVM TIR backend: GlitchRestHost_Run expects app, port, and max_requests"
                    .to_string(),
            );
        };
        let app_value = self.emit_typed_expr(app)?;
        let app_value = self.cast_value(app_value, &LlType::Ptr)?;
        let port_value = self.emit_typed_expr(port)?;
        let port_value = self.cast_value(port_value, &LlType::I32)?;
        let max_requests_value = self.emit_typed_expr(max_requests)?;
        let max_requests_value = self.cast_value(max_requests_value, &LlType::I32)?;
        self.body.push_str(&format!(
            "  call void @GlitchRestHost_Run(ptr {}, i32 {}, i32 {}, ptr @WebApplication_Handle, ptr @glitch_string_release)\n",
            app_value.value, port_value.value, max_requests_value.value
        ));
        self.emit_temporary_drop(app, &app_value);
        self.emit_temporary_drop(port, &port_value);
        self.emit_temporary_drop(max_requests, &max_requests_value);
        Ok(void_value())
    }

    fn emit_typed_call<I>(
        &mut self,
        name: &str,
        prefix: I,
        args: &[TypedExpr],
    ) -> Result<LlValue, String>
    where
        I: IntoIterator<Item = LlValue>,
    {
        let signature =
            self.functions.get(name).cloned().ok_or_else(|| {
                format!("LLVM TIR backend: function '{name}' has no lowered body")
            })?;
        let mut values = prefix.into_iter().collect::<Vec<_>>();
        for arg in args {
            values.push(self.emit_typed_expr(arg)?);
        }
        if values.len() != signature.params.len() {
            return Err(format!(
                "LLVM TIR backend: call to '{name}' expected {} arguments but got {}",
                signature.params.len(),
                values.len()
            ));
        }
        let mut rendered_args = Vec::new();
        for (value, expected) in values.into_iter().zip(&signature.params) {
            let value = self.cast_value(value, expected)?;
            rendered_args.push(format!("{} {}", expected.as_ir(), value.value));
        }
        if signature.return_type == LlType::Void {
            self.body.push_str(&format!(
                "  call void @{}({})\n",
                sanitize(name),
                rendered_args.join(", ")
            ));
            self.emit_exception_check();
            Ok(LlValue {
                value: "0".to_string(),
                ty: LlType::I32,
            })
        } else {
            let tmp = self.tmp();
            self.body.push_str(&format!(
                "  {tmp} = call {} @{}({})\n",
                signature.return_type.as_ir(),
                sanitize(name),
                rendered_args.join(", ")
            ));
            self.emit_exception_check();
            Ok(LlValue {
                value: tmp,
                ty: signature.return_type,
            })
        }
    }

    fn emit_typed_try(
        &mut self,
        try_body: &[TypedStmt],
        catch_name: Option<&str>,
        catch_body: &[TypedStmt],
        finally_body: &[TypedStmt],
    ) -> Result<(), String> {
        let outer_handler = self.exception_handler.clone();
        let has_catch = catch_name.is_some() || !catch_body.is_empty();
        let has_finally = !finally_body.is_empty();
        let catch_label = self.next_label("try_catch");
        let finally_label = self.next_label("try_finally");
        let end_label = self.next_label("try_end");
        let propagate_target = outer_handler
            .clone()
            .unwrap_or_else(|| self.current_unwind_label.clone());

        self.exception_handler = Some(if has_catch {
            catch_label.clone()
        } else if has_finally {
            finally_label.clone()
        } else {
            propagate_target.clone()
        });
        self.emit_typed_stmts(try_body)?;
        if !self.terminated {
            let target = if has_finally {
                &finally_label
            } else {
                &end_label
            };
            self.body.push_str(&format!("  br label %{target}\n"));
        }

        if has_catch {
            self.body.push_str(&format!("{catch_label}:\n"));
            self.terminated = false;
            let exception = self.tmp();
            self.body.push_str(&format!(
                "  {exception} = load ptr, ptr @glitch_exception_pending\n  store ptr null, ptr @glitch_exception_pending\n"
            ));
            if let Some(name) = catch_name {
                if let Some(var) = self.vars.get(name) {
                    self.body
                        .push_str(&format!("  store ptr {exception}, ptr {}\n", var.ptr));
                } else {
                    let ptr = self.tmp();
                    self.body.push_str(&format!(
                        "  {ptr} = alloca ptr\n  store ptr {exception}, ptr {ptr}\n"
                    ));
                    self.vars.insert(
                        name.to_string(),
                        LlVar {
                            ptr,
                            ty: LlType::Ptr,
                            ir_ty: IrType::Exception,
                            drop_kind: DropKind::BorrowOnly,
                        },
                    );
                }
            }
            self.exception_handler = Some(if has_finally {
                finally_label.clone()
            } else {
                propagate_target.clone()
            });
            self.emit_typed_stmts(catch_body)?;
            if !self.terminated {
                let target = if has_finally {
                    &finally_label
                } else {
                    &end_label
                };
                self.body.push_str(&format!("  br label %{target}\n"));
            }
        }

        if has_finally {
            self.body.push_str(&format!("{finally_label}:\n"));
            self.terminated = false;
            self.exception_handler = outer_handler.clone();
            self.emit_typed_stmts(finally_body)?;
            if !self.terminated {
                let pending = self.tmp();
                let has_exception = self.tmp();
                self.body.push_str(&format!(
                    "  {pending} = load ptr, ptr @glitch_exception_pending\n  {has_exception} = icmp ne ptr {pending}, null\n  br i1 {has_exception}, label %{propagate_target}, label %{end_label}\n"
                ));
            }
        }

        self.body.push_str(&format!("{end_label}:\n"));
        self.terminated = false;
        self.exception_handler = outer_handler;
        Ok(())
    }

    fn emit_typed_for(
        &mut self,
        init: Option<&TypedStmt>,
        condition: Option<&TypedExpr>,
        increment: Option<&TypedStmt>,
        body: &[TypedStmt],
    ) -> Result<(), String> {
        if let Some(init) = init {
            self.emit_typed_stmt(init)?;
        }
        let condition_label = self.next_label("for_condition");
        let body_label = self.next_label("for_body");
        let increment_label = self.next_label("for_increment");
        let end_label = self.next_label("for_end");
        self.body.push_str(&format!(
            "  br label %{condition_label}\n{condition_label}:\n"
        ));
        if let Some(condition) = condition {
            let condition = self.emit_typed_expr(condition)?;
            let condition = self.to_i1(condition)?;
            self.body.push_str(&format!(
                "  br i1 {}, label %{body_label}, label %{end_label}\n",
                condition.value
            ));
        } else {
            self.body.push_str(&format!("  br label %{body_label}\n"));
        }
        self.body.push_str(&format!("{body_label}:\n"));
        self.terminated = false;
        self.loop_targets
            .push((increment_label.clone(), end_label.clone()));
        self.emit_typed_stmts(body)?;
        self.loop_targets.pop();
        if !self.terminated {
            self.body
                .push_str(&format!("  br label %{increment_label}\n"));
        }
        self.body.push_str(&format!("{increment_label}:\n"));
        self.terminated = false;
        if let Some(increment) = increment {
            self.emit_typed_stmt(increment)?;
        }
        if !self.terminated {
            self.body
                .push_str(&format!("  br label %{condition_label}\n"));
        }
        self.body.push_str(&format!("{end_label}:\n"));
        self.terminated = false;
        Ok(())
    }

    fn emit_typed_switch(
        &mut self,
        expr: &TypedExpr,
        cases: &[TypedSwitchCase],
        default: &[TypedStmt],
    ) -> Result<(), String> {
        let value = self.emit_typed_expr(expr)?;
        let end_label = self.next_label("switch_end");
        let default_label = self.next_label("switch_default");
        let case_labels = cases
            .iter()
            .map(|_| self.next_label("switch_case"))
            .collect::<Vec<_>>();
        let compare_labels = cases
            .iter()
            .map(|_| self.next_label("switch_compare"))
            .collect::<Vec<_>>();

        if let Some(first) = compare_labels.first() {
            self.body.push_str(&format!("  br label %{first}\n"));
        } else {
            self.body
                .push_str(&format!("  br label %{default_label}\n"));
        }

        for (index, case) in cases.iter().enumerate() {
            self.body.push_str(&format!("{}:\n", compare_labels[index]));
            let case_value = self.emit_typed_expr(&case.value)?;
            let case_value = self.cast_value(case_value, &value.ty)?;
            let equal = self.tmp();
            self.emit_equality(&expr.ty, &value.value, &case_value.value, &equal);
            let next = compare_labels.get(index + 1).unwrap_or(&default_label);
            self.body.push_str(&format!(
                "  br i1 {equal}, label %{}, label %{next}\n",
                case_labels[index]
            ));
        }

        let inherited_continue = self
            .loop_targets
            .last()
            .map(|(continue_target, _)| continue_target.clone())
            .unwrap_or_default();
        for (index, case) in cases.iter().enumerate() {
            self.body.push_str(&format!("{}:\n", case_labels[index]));
            self.terminated = false;
            self.loop_targets
                .push((inherited_continue.clone(), end_label.clone()));
            self.emit_typed_stmts(&case.body)?;
            self.loop_targets.pop();
            if !self.terminated {
                self.body.push_str(&format!("  br label %{end_label}\n"));
            }
        }

        self.body.push_str(&format!("{default_label}:\n"));
        self.terminated = false;
        self.loop_targets
            .push((inherited_continue, end_label.clone()));
        self.emit_typed_stmts(default)?;
        self.loop_targets.pop();
        if !self.terminated {
            self.body.push_str(&format!("  br label %{end_label}\n"));
        }
        self.body.push_str(&format!("{end_label}:\n"));
        self.terminated = false;
        Ok(())
    }

    fn emit_typed_foreach(
        &mut self,
        item: &TypedBinding,
        collection: &TypedExpr,
        body: &[TypedStmt],
    ) -> Result<(), String> {
        let element_type = match &collection.ty {
            IrType::Array(element) | IrType::List(element) | IrType::Enumerable(element) => {
                element.as_ref()
            }
            IrType::Unknown(_) => {
                self.emit_typed_expr(collection)?;
                return Ok(());
            }
            other => {
                return Err(format!(
                    "LLVM TIR backend: foreach is unsupported for {other:?}"
                ));
            }
        };
        let collection_value = self.emit_typed_expr(collection)?;
        let layout = if matches!(collection.ty, IrType::Array(_)) {
            "%glitch.array"
        } else {
            "%glitch.list"
        };
        let data_index = if matches!(collection.ty, IrType::Array(_)) {
            1
        } else {
            2
        };
        let len_ptr = self.tmp();
        let data_ptr = self.tmp();
        let len = self.tmp();
        let data = self.tmp();
        let index_ptr = self.tmp();
        let item_ptr = self.tmp();
        let condition_label = self.next_label("foreach_condition");
        let body_label = self.next_label("foreach_body");
        let advance_label = self.next_label("foreach_advance");
        let end_label = self.next_label("foreach_end");
        let element_llvm_type = llvm_ir_type(element_type);
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds {layout}, ptr {}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds {layout}, ptr {}, i32 0, i32 {data_index}\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n  {index_ptr} = alloca i64\n  {item_ptr} = alloca {}\n  store i64 0, ptr {index_ptr}\n  br label %{condition_label}\n{condition_label}:\n",
            collection_value.value,
            collection_value.value,
            element_llvm_type.as_ir()
        ));
        let index = self.tmp();
        let in_range = self.tmp();
        self.body.push_str(&format!(
            "  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{body_label}, label %{end_label}\n{body_label}:\n"
        ));
        let slot = self.tmp();
        let value = self.tmp();
        self.body.push_str(&format!(
            "  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  {value} = load {}, ptr {slot}\n  store {} {value}, ptr {item_ptr}\n",
            element_llvm_type.as_ir(),
            element_llvm_type.as_ir(),
            element_llvm_type.as_ir()
        ));

        let previous = self.vars.insert(
            item.name.clone(),
            LlVar {
                ptr: item_ptr,
                ty: element_llvm_type,
                ir_ty: element_type.clone(),
                drop_kind: DropKind::BorrowOnly,
            },
        );
        self.loop_targets
            .push((advance_label.clone(), end_label.clone()));
        self.terminated = false;
        self.emit_typed_stmts(body)?;
        self.loop_targets.pop();
        if !self.terminated {
            self.body
                .push_str(&format!("  br label %{advance_label}\n"));
        }
        self.body.push_str(&format!("{advance_label}:\n"));
        let current_index = self.tmp();
        let next_index = self.tmp();
        self.body.push_str(&format!(
            "  {current_index} = load i64, ptr {index_ptr}\n  {next_index} = add i64 {current_index}, 1\n  store i64 {next_index}, ptr {index_ptr}\n  br label %{condition_label}\n{end_label}:\n"
        ));
        self.terminated = false;
        if let Some(previous) = previous {
            self.vars.insert(item.name.clone(), previous);
        } else {
            self.vars.remove(&item.name);
        }
        Ok(())
    }

    fn emit_exception_check(&mut self) {
        let pending = self.tmp();
        let has_exception = self.tmp();
        let continue_label = self.next_label("call_continue");
        let target = self
            .exception_handler
            .clone()
            .unwrap_or_else(|| self.current_unwind_label.clone());
        self.body.push_str(&format!(
            "  {pending} = load ptr, ptr @glitch_exception_pending\n  {has_exception} = icmp ne ptr {pending}, null\n  br i1 {has_exception}, label %{target}, label %{continue_label}\n{continue_label}:\n"
        ));
        self.terminated = false;
    }

    fn emit_exception_branch(&mut self) {
        let target = self
            .exception_handler
            .clone()
            .unwrap_or_else(|| self.current_unwind_label.clone());
        self.body.push_str(&format!("  br label %{target}\n"));
        self.terminated = true;
    }

    fn emit_new_array(
        &mut self,
        element_type: &IrType,
        values: &[TypedExpr],
    ) -> Result<LlValue, String> {
        let array = self.tmp();
        let data = self.tmp();
        let len_ptr = self.tmp();
        let data_ptr = self.tmp();
        let element_size = self.emit_type_size(element_type);
        self.body.push_str(&format!(
            "  {array} = call ptr @glitch_calloc(i64 1, i64 16)\n  {data} = call ptr @glitch_calloc(i64 {}, i64 {element_size})\n  {len_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 0\n  store i64 {}, ptr {len_ptr}\n  {data_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 1\n  store ptr {data}, ptr {data_ptr}\n",
            values.len(),
            values.len()
        ));
        let element_ll_type = llvm_ir_type(element_type);
        for (index, source) in values.iter().enumerate() {
            let value = self.emit_typed_expr(source)?;
            let value = self.cast_value(value, &element_ll_type)?;
            self.retain_for_store(element_type, source, &value.value);
            let slot = self.tmp();
            self.body.push_str(&format!(
                "  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  store {} {}, ptr {slot}\n",
                element_ll_type.as_ir(),
                element_ll_type.as_ir(),
                value.value
            ));
        }
        Ok(LlValue {
            value: array,
            ty: LlType::Ptr,
        })
    }

    fn emit_new_collection(&mut self, ty: &IrType) -> Result<LlValue, String> {
        match ty {
            IrType::List(element) => {
                let list = self.tmp();
                let element_size = self.emit_type_size(element);
                let data = self.tmp();
                let cap_ptr = self.tmp();
                let data_ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {list} = call ptr @glitch_calloc(i64 1, i64 24)\n  {data} = call ptr @glitch_calloc(i64 4, i64 {element_size})\n"
                ));
                self.body.push_str(&format!(
                    "  {cap_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 1\n  store i64 4, ptr {cap_ptr}\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 2\n  store ptr {data}, ptr {data_ptr}\n"
                ));
                Ok(LlValue {
                    value: list,
                    ty: LlType::Ptr,
                })
            }
            IrType::Dictionary(key, value) => {
                let dict = self.tmp();
                let key_size = self.emit_type_size(key);
                let value_size = self.emit_type_size(value);
                let keys = self.tmp();
                let values = self.tmp();
                let cap_ptr = self.tmp();
                let keys_ptr = self.tmp();
                let values_ptr = self.tmp();
                self.body.push_str(&format!(
                    "  {dict} = call ptr @glitch_calloc(i64 1, i64 32)\n  {keys} = call ptr @glitch_calloc(i64 4, i64 {key_size})\n  {values} = call ptr @glitch_calloc(i64 4, i64 {value_size})\n  {cap_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 1\n  store i64 4, ptr {cap_ptr}\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 2\n  store ptr {keys}, ptr {keys_ptr}\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 3\n  store ptr {values}, ptr {values_ptr}\n"
                ));
                Ok(LlValue {
                    value: dict,
                    ty: LlType::Ptr,
                })
            }
            other => Err(format!(
                "LLVM TIR backend: unsupported collection allocation {other:?}"
            )),
        }
    }

    fn emit_task_run_inline(
        &mut self,
        call: &TypedCall,
        return_type: &IrType,
    ) -> Result<LlValue, String> {
        let [worker_expr] = &call.args[..] else {
            return Err("Task.Run expects exactly one argument".to_string());
        };
        let worker_val = self.emit_typed_expr(worker_expr)?;

        let result_ty = match return_type {
            IrType::Task(inner) => inner.as_ref().clone(),
            _ => IrType::Void,
        };

        let result_llvm_type = llvm_ir_type(&result_ty);

        if matches!(result_ty, IrType::Void) {
            self.body
                .push_str(&format!("  call void {}()\n", worker_val.value));
            let task_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {} = call ptr @glitch_task_from_result_ptr(ptr null)\n",
                task_ptr
            ));
            Ok(LlValue {
                value: task_ptr,
                ty: LlType::Ptr,
            })
        } else {
            let call_res = self.tmp();
            self.body.push_str(&format!(
                "  {} = call {} {}()\n",
                call_res,
                result_llvm_type.as_ir(),
                worker_val.value
            ));

            let task_ptr = self.tmp();
            let helper_name = match &result_ty {
                IrType::Int | IrType::UInt => "glitch_task_from_result_i32",
                IrType::Long => "glitch_task_from_result_i64",
                IrType::Double | IrType::Decimal => "glitch_task_from_result_double",
                _ => "glitch_task_from_result_ptr",
            };
            self.body.push_str(&format!(
                "  {} = call ptr @{}({} {})\n",
                task_ptr,
                helper_name,
                result_llvm_type.as_ir(),
                call_res
            ));
            Ok(LlValue {
                value: task_ptr,
                ty: LlType::Ptr,
            })
        }
    }

    fn emit_task_from_result_inline(
        &mut self,
        call: &TypedCall,
        return_type: &IrType,
    ) -> Result<LlValue, String> {
        let [val_expr] = &call.args[..] else {
            return Err("Task.FromResult expects exactly one argument".to_string());
        };
        let value_val = self.emit_typed_expr(val_expr)?;

        let result_ty = match return_type {
            IrType::Task(inner) => inner.as_ref().clone(),
            _ => IrType::Void,
        };

        let result_llvm_type = llvm_ir_type(&result_ty);
        let task_ptr = self.tmp();
        let helper_name = match &result_ty {
            IrType::Int | IrType::UInt => "glitch_task_from_result_i32",
            IrType::Long => "glitch_task_from_result_i64",
            IrType::Double | IrType::Decimal => "glitch_task_from_result_double",
            _ => "glitch_task_from_result_ptr",
        };

        self.body.push_str(&format!(
            "  {} = call ptr @{}({} {})\n",
            task_ptr,
            helper_name,
            result_llvm_type.as_ir(),
            value_val.value
        ));

        Ok(LlValue {
            value: task_ptr,
            ty: LlType::Ptr,
        })
    }

    fn emit_weak_method(
        &mut self,
        target: &TypedExpr,
        name: &str,
        args: &[TypedExpr],
    ) -> Result<LlValue, String> {
        if name == "TryGetTarget" {
            if let IrType::Weak(inner) = &target.ty {
                let weak_val = self.emit_typed_expr(target)?;
                let out_ptr_val = self.emit_typed_expr(&args[0])?;

                let is_null = self.tmp();
                let non_null_lbl = self.next_label("tryget_not_null");
                let null_lbl = self.next_label("tryget_null");
                let done_lbl = self.next_label("tryget_done");
                let success = self.tmp();

                self.body.push_str(&format!(
                    "  {is_null} = icmp eq ptr {}, null\n  br i1 {is_null}, label %{null_lbl}, label %{non_null_lbl}\n",
                    weak_val.value
                ));

                // Non-null block
                self.body.push_str(&format!("\n{non_null_lbl}:\n"));
                if matches!(inner.as_ref(), IrType::String) {
                    self.body.push_str(&format!(
                        "  call void @glitch_string_retain(ptr {})\n",
                        weak_val.value
                    ));
                } else if let Some(type_name) = object_type_name(inner) {
                    self.emit_retain(type_name, &weak_val.value);
                }
                self.body.push_str(&format!(
                    "  store ptr {}, ptr {}\n  br label %{done_lbl}\n",
                    weak_val.value, out_ptr_val.value
                ));

                // Null block
                self.body.push_str(&format!("\n{null_lbl}:\n"));
                self.body.push_str(&format!(
                    "  store ptr null, ptr {}\n  br label %{done_lbl}\n",
                    out_ptr_val.value
                ));

                // Done block
                self.body.push_str(&format!("\n{done_lbl}:\n"));
                self.body.push_str(&format!(
                    "  {success} = phi i1 [ true, %{non_null_lbl} ], [ false, %{null_lbl} ]\n"
                ));

                return Ok(LlValue {
                    value: success,
                    ty: LlType::I1,
                });
            }
        }
        Err(format!("unsupported weak method: {name}"))
    }

    fn emit_task_method(
        &mut self,
        target: &TypedExpr,
        name: &str,
        args: &[TypedExpr],
        expr_ty: &IrType,
    ) -> Result<LlValue, String> {
        let _ = args;
        let task_val = self.emit_typed_expr(target)?;
        if name == "Wait" {
            Ok(void_value())
        } else if name == "GetResult" || name == "GetAwaiter" {
            if name == "GetAwaiter" {
                Ok(task_val)
            } else {
                let result_ty = expr_ty.clone();
                let result_llvm_type = llvm_ir_type(&result_ty);
                if matches!(result_ty, IrType::Void) {
                    Ok(void_value())
                } else {
                    let call_res = self.tmp();
                    let helper_name = match &result_ty {
                        IrType::Int | IrType::UInt => "glitch_task_get_result_i32",
                        IrType::Long => "glitch_task_get_result_i64",
                        IrType::Double | IrType::Decimal => "glitch_task_get_result_double",
                        _ => "glitch_task_get_result_ptr",
                    };
                    self.body.push_str(&format!(
                        "  {} = call {} @{}(ptr {})\n",
                        call_res,
                        result_llvm_type.as_ir(),
                        helper_name,
                        task_val.value
                    ));
                    Ok(LlValue {
                        value: call_res,
                        ty: result_llvm_type,
                    })
                }
            }
        } else {
            Err(format!(
                "LLVM TIR backend: unsupported Task method '{name}'"
            ))
        }
    }

    fn emit_collection_method(
        &mut self,
        target: &TypedExpr,
        name: &str,
        args: &[TypedExpr],
    ) -> Result<LlValue, String> {
        let collection = self.emit_typed_expr(target)?;
        match (&target.ty, name) {
            (IrType::List(element), "Add") => {
                let [arg] = args else {
                    return Err("LLVM TIR backend: List.Add expects one argument".to_string());
                };
                let value = self.emit_typed_expr(arg)?;
                let value = self.cast_value(value, &llvm_ir_type(element))?;
                self.emit_list_add(&collection.value, element, &value.value, arg);
                Ok(void_value())
            }
            (IrType::Dictionary(key, value_ty), "Add") => {
                let [key_arg, value_arg] = args else {
                    return Err(
                        "LLVM TIR backend: Dictionary.Add expects two arguments".to_string()
                    );
                };
                let key_value = self.emit_typed_expr(key_arg)?;
                let key_value = self.cast_value(key_value, &llvm_ir_type(key))?;
                let value = self.emit_typed_expr(value_arg)?;
                let value = self.cast_value(value, &llvm_ir_type(value_ty))?;
                self.emit_dict_add(
                    &collection.value,
                    key,
                    &key_value.value,
                    key_arg,
                    value_ty,
                    &value.value,
                    value_arg,
                );
                Ok(void_value())
            }
            (IrType::List(element), "Contains") => {
                let [arg] = args else {
                    return Err("LLVM TIR backend: List.Contains expects one argument".to_string());
                };
                let needle = self.emit_typed_expr(arg)?;
                let needle = self.cast_value(needle, &llvm_ir_type(element))?;
                self.emit_list_contains(&collection.value, element, &needle.value)
            }
            (IrType::Dictionary(key, _), "ContainsKey") => {
                let [arg] = args else {
                    return Err(
                        "LLVM TIR backend: Dictionary.ContainsKey expects one argument".to_string(),
                    );
                };
                let needle = self.emit_typed_expr(arg)?;
                let needle = self.cast_value(needle, &llvm_ir_type(key))?;
                self.emit_dict_find(&collection.value, key, &needle.value)
                    .map(|(_, found)| found)
            }
            (IrType::Dictionary(key, value), "Remove") => {
                let [arg] = args else {
                    return Err(
                        "LLVM TIR backend: Dictionary.Remove expects one argument".to_string()
                    );
                };
                let needle = self.emit_typed_expr(arg)?;
                let needle = self.cast_value(needle, &llvm_ir_type(key))?;
                self.emit_dict_remove(&collection.value, key, value, &needle.value)
            }
            (IrType::List(element), "Clear") => {
                self.emit_collection_clear(&collection.value, element, "glitch.list", 0);
                Ok(void_value())
            }
            (IrType::Dictionary(_, value), "Clear") => {
                self.emit_collection_clear(&collection.value, value, "glitch.dict", 0);
                Ok(void_value())
            }
            _ => Err(format!(
                "LLVM TIR backend: unsupported collection method '{name}' on {:?}",
                target.ty
            )),
        }
    }

    fn emit_list_add(&mut self, list: &str, element: &IrType, value: &str, source: &TypedExpr) {
        let len_ptr = self.tmp();
        let cap_ptr = self.tmp();
        let data_ptr = self.tmp();
        let len = self.tmp();
        let cap = self.tmp();
        let data = self.tmp();
        let full = self.tmp();
        let grow = self.next_label("list_grow");
        let ready = self.next_label("list_ready");
        let grown_cap = self.tmp();
        let bytes = self.tmp();
        let grown_data = self.tmp();
        let current_data = self.tmp();
        let slot = self.tmp();
        let next_len = self.tmp();
        let element_size = self.emit_type_size(element);
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 0\n  {cap_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 1\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {cap} = load i64, ptr {cap_ptr}\n  {data} = load ptr, ptr {data_ptr}\n  {full} = icmp eq i64 {len}, {cap}\n  br i1 {full}, label %{grow}, label %{ready}\n{grow}:\n  {grown_cap} = mul i64 {cap}, 2\n  {bytes} = mul i64 {grown_cap}, {element_size}\n  {grown_data} = call ptr @glitch_realloc(ptr {data}, i64 {bytes})\n  store i64 {grown_cap}, ptr {cap_ptr}\n  store ptr {grown_data}, ptr {data_ptr}\n  br label %{ready}\n{ready}:\n  {current_data} = load ptr, ptr {data_ptr}\n  {slot} = getelementptr inbounds {}, ptr {current_data}, i64 {len}\n",
            llvm_ir_type(element).as_ir()
        ));
        self.retain_for_store(element, source, value);
        self.body.push_str(&format!(
            "  store {} {value}, ptr {slot}\n  {next_len} = add i64 {len}, 1\n  store i64 {next_len}, ptr {len_ptr}\n",
            llvm_ir_type(element).as_ir()
        ));
    }

    fn emit_dict_add(
        &mut self,
        dict: &str,
        key_ty: &IrType,
        key: &str,
        key_source: &TypedExpr,
        value_ty: &IrType,
        value: &str,
        source: &TypedExpr,
    ) {
        let len_ptr = self.tmp();
        let cap_ptr = self.tmp();
        let keys_ptr = self.tmp();
        let values_ptr = self.tmp();
        let len = self.tmp();
        let cap = self.tmp();
        let keys = self.tmp();
        let values = self.tmp();
        let full = self.tmp();
        let grow = self.next_label("dict_grow");
        let ready = self.next_label("dict_ready");
        let grown_cap = self.tmp();
        let key_bytes = self.tmp();
        let value_bytes = self.tmp();
        let grown_keys = self.tmp();
        let grown_values = self.tmp();
        let current_keys = self.tmp();
        let current_values = self.tmp();
        let key_slot = self.tmp();
        let value_slot = self.tmp();
        let next_len = self.tmp();
        let key_size = self.emit_type_size(key_ty);
        let value_size = self.emit_type_size(value_ty);
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 0\n  {cap_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 1\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 2\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 3\n  {len} = load i64, ptr {len_ptr}\n  {cap} = load i64, ptr {cap_ptr}\n  {keys} = load ptr, ptr {keys_ptr}\n  {values} = load ptr, ptr {values_ptr}\n  {full} = icmp eq i64 {len}, {cap}\n  br i1 {full}, label %{grow}, label %{ready}\n{grow}:\n  {grown_cap} = mul i64 {cap}, 2\n  {key_bytes} = mul i64 {grown_cap}, {key_size}\n  {value_bytes} = mul i64 {grown_cap}, {value_size}\n  {grown_keys} = call ptr @glitch_realloc(ptr {keys}, i64 {key_bytes})\n  {grown_values} = call ptr @glitch_realloc(ptr {values}, i64 {value_bytes})\n  store i64 {grown_cap}, ptr {cap_ptr}\n  store ptr {grown_keys}, ptr {keys_ptr}\n  store ptr {grown_values}, ptr {values_ptr}\n  br label %{ready}\n{ready}:\n  {current_keys} = load ptr, ptr {keys_ptr}\n  {current_values} = load ptr, ptr {values_ptr}\n  {key_slot} = getelementptr inbounds {}, ptr {current_keys}, i64 {len}\n  {value_slot} = getelementptr inbounds {}, ptr {current_values}, i64 {len}\n",
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir()
        ));
        self.retain_for_store(key_ty, key_source, key);
        self.body.push_str(&format!(
            "  store {} {key}, ptr {key_slot}\n",
            llvm_ir_type(key_ty).as_ir()
        ));
        self.retain_for_store(value_ty, source, value);
        self.body.push_str(&format!(
            "  store {} {value}, ptr {value_slot}\n  {next_len} = add i64 {len}, 1\n  store i64 {next_len}, ptr {len_ptr}\n",
            llvm_ir_type(value_ty).as_ir()
        ));
    }

    fn emit_list_contains(
        &mut self,
        list: &str,
        element: &IrType,
        needle: &str,
    ) -> Result<LlValue, String> {
        let len_ptr = self.tmp();
        let data_ptr = self.tmp();
        let len = self.tmp();
        let data = self.tmp();
        let index_ptr = self.tmp();
        let result_ptr = self.tmp();
        let loop_label = self.next_label("list_contains_loop");
        let found_label = self.next_label("list_contains_found");
        let next_label = self.next_label("list_contains_next");
        let done_label = self.next_label("list_contains_done");
        let index = self.tmp();
        let in_range = self.tmp();
        let slot = self.tmp();
        let item = self.tmp();
        let equal = self.tmp();
        let next = self.tmp();
        let result = self.tmp();
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {list}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n  {index_ptr} = alloca i64\n  {result_ptr} = alloca i1\n  store i64 0, ptr {index_ptr}\n  store i1 false, ptr {result_ptr}\n  br label %{loop_label}\n{loop_label}:\n  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{next_label}, label %{done_label}\n{next_label}:\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  {item} = load {}, ptr {slot}\n",
            llvm_ir_type(element).as_ir(),
            llvm_ir_type(element).as_ir()
        ));
        self.emit_equality(element, &item, needle, &equal);
        self.body.push_str(&format!(
            "  br i1 {equal}, label %{found_label}, label %list_contains_advance_{}\nlist_contains_advance_{}:\n  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{found_label}:\n  store i1 true, ptr {result_ptr}\n  br label %{done_label}\n{done_label}:\n  {result} = load i1, ptr {result_ptr}\n",
            self.label, self.label
        ));
        self.label += 1;
        Ok(LlValue {
            value: result,
            ty: LlType::I1,
        })
    }

    fn emit_dict_find(
        &mut self,
        dict: &str,
        key_ty: &IrType,
        needle: &str,
    ) -> Result<(LlValue, LlValue), String> {
        let len_ptr = self.tmp();
        let keys_ptr = self.tmp();
        let len = self.tmp();
        let keys = self.tmp();
        let index_ptr = self.tmp();
        let found_ptr = self.tmp();
        let loop_label = self.next_label("dict_find_loop");
        let inspect_label = self.next_label("dict_find_inspect");
        let found_label = self.next_label("dict_find_found");
        let advance_label = self.next_label("dict_find_advance");
        let done_label = self.next_label("dict_find_done");
        let index = self.tmp();
        let in_range = self.tmp();
        let slot = self.tmp();
        let item = self.tmp();
        let equal = self.tmp();
        let next = self.tmp();
        let result_index = self.tmp();
        let result_found = self.tmp();
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 0\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {keys} = load ptr, ptr {keys_ptr}\n  {index_ptr} = alloca i64\n  {found_ptr} = alloca i1\n  store i64 0, ptr {index_ptr}\n  store i1 false, ptr {found_ptr}\n  br label %{loop_label}\n{loop_label}:\n  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{inspect_label}, label %{done_label}\n{inspect_label}:\n  {slot} = getelementptr inbounds {}, ptr {keys}, i64 {index}\n  {item} = load {}, ptr {slot}\n",
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(key_ty).as_ir()
        ));
        self.emit_equality(key_ty, &item, needle, &equal);
        self.body.push_str(&format!(
            "  br i1 {equal}, label %{found_label}, label %{advance_label}\n{advance_label}:\n  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{found_label}:\n  store i1 true, ptr {found_ptr}\n  br label %{done_label}\n{done_label}:\n  {result_index} = load i64, ptr {index_ptr}\n  {result_found} = load i1, ptr {found_ptr}\n"
        ));
        Ok((
            LlValue {
                value: result_index,
                ty: LlType::I64,
            },
            LlValue {
                value: result_found,
                ty: LlType::I1,
            },
        ))
    }

    fn emit_dict_remove(
        &mut self,
        dict: &str,
        key_ty: &IrType,
        value_ty: &IrType,
        needle: &str,
    ) -> Result<LlValue, String> {
        let (index, found) = self.emit_dict_find(dict, key_ty, needle)?;
        let remove_label = self.next_label("dict_remove");
        let compact_label = self.next_label("dict_remove_compact");
        let done_label = self.next_label("dict_remove_done");
        let len_ptr = self.tmp();
        let keys_ptr = self.tmp();
        let values_ptr = self.tmp();
        let len = self.tmp();
        let next_len = self.tmp();
        let needs_compact = self.tmp();
        let keys = self.tmp();
        let values = self.tmp();
        let last_key_ptr = self.tmp();
        let last_value_ptr = self.tmp();
        let last_key = self.tmp();
        let last_value = self.tmp();
        let removed_key_ptr = self.tmp();
        let removed_value_ptr = self.tmp();
        self.body.push_str(&format!(
            "  br i1 {}, label %{remove_label}, label %{done_label}\n{remove_label}:\n  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 0\n  {len} = load i64, ptr {len_ptr}\n  {next_len} = sub i64 {len}, 1\n  store i64 {next_len}, ptr {len_ptr}\n  {needs_compact} = icmp ne i64 {}, {next_len}\n  br i1 {needs_compact}, label %{compact_label}, label %{done_label}\n{compact_label}:\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 2\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {dict}, i32 0, i32 3\n  {keys} = load ptr, ptr {keys_ptr}\n  {values} = load ptr, ptr {values_ptr}\n  {last_key_ptr} = getelementptr inbounds {}, ptr {keys}, i64 {next_len}\n  {last_value_ptr} = getelementptr inbounds {}, ptr {values}, i64 {next_len}\n  {last_key} = load {}, ptr {last_key_ptr}\n  {last_value} = load {}, ptr {last_value_ptr}\n  {removed_key_ptr} = getelementptr inbounds {}, ptr {keys}, i64 {}\n  {removed_value_ptr} = getelementptr inbounds {}, ptr {values}, i64 {}\n  store {} {last_key}, ptr {removed_key_ptr}\n  store {} {last_value}, ptr {removed_value_ptr}\n  br label %{done_label}\n{done_label}:\n",
            found.value,
            index.value,
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir(),
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir(),
            llvm_ir_type(key_ty).as_ir(),
            index.value,
            llvm_ir_type(value_ty).as_ir(),
            index.value,
            llvm_ir_type(key_ty).as_ir(),
            llvm_ir_type(value_ty).as_ir()
        ));
        Ok(found)
    }

    fn emit_collection_clear(
        &mut self,
        collection: &str,
        _element: &IrType,
        layout: &str,
        len_index: usize,
    ) {
        let len_ptr = self.tmp();
        self.body.push_str(&format!(
            "  {len_ptr} = getelementptr inbounds %{layout}, ptr {collection}, i32 0, i32 {len_index}\n  store i64 0, ptr {len_ptr}\n"
        ));
    }

    fn emit_collection_index(
        &mut self,
        target: &TypedExpr,
        index: &TypedExpr,
    ) -> Result<LlValue, String> {
        let collection = self.emit_typed_expr(target)?;
        match &target.ty {
            IrType::Array(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let data_ptr = self.tmp();
                let data = self.tmp();
                let slot = self.tmp();
                let value = self.tmp();
                let ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {data_ptr} = getelementptr inbounds %glitch.array, ptr {}, i32 0, i32 1\n  {data} = load ptr, ptr {data_ptr}\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {}\n  {value} = load {}, ptr {slot}\n",
                    collection.value,
                    ty.as_ir(),
                    index.value,
                    ty.as_ir()
                ));
                Ok(LlValue { value, ty })
            }
            IrType::List(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let data_ptr = self.tmp();
                let data = self.tmp();
                let slot = self.tmp();
                let value = self.tmp();
                let ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {data_ptr} = getelementptr inbounds %glitch.list, ptr {}, i32 0, i32 2\n  {data} = load ptr, ptr {data_ptr}\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {}\n  {value} = load {}, ptr {slot}\n",
                    collection.value,
                    ty.as_ir(),
                    index.value,
                    ty.as_ir()
                ));
                Ok(LlValue { value, ty })
            }
            IrType::Dictionary(key_ty, value_ty) => {
                let key = self.emit_typed_expr(index)?;
                let key = self.cast_value(key, &llvm_ir_type(key_ty))?;
                let (found_index, found) =
                    self.emit_dict_find(&collection.value, key_ty, &key.value)?;
                let values_ptr = self.tmp();
                let values = self.tmp();
                let slot = self.tmp();
                let loaded = self.tmp();
                let default_label = self.next_label("dict_index_default");
                let load_label = self.next_label("dict_index_load");
                let done_label = self.next_label("dict_index_done");
                let result_ptr = self.tmp();
                let result = self.tmp();
                let ty = llvm_ir_type(value_ty);
                self.body.push_str(&format!(
                    "  {result_ptr} = alloca {}\n  br i1 {}, label %{load_label}, label %{default_label}\n{load_label}:\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {}, i32 0, i32 3\n  {values} = load ptr, ptr {values_ptr}\n  {slot} = getelementptr inbounds {}, ptr {values}, i64 {}\n  {loaded} = load {}, ptr {slot}\n  store {} {loaded}, ptr {result_ptr}\n  br label %{done_label}\n{default_label}:\n  store {} {}, ptr {result_ptr}\n  br label %{done_label}\n{done_label}:\n  {result} = load {}, ptr {result_ptr}\n",
                    ty.as_ir(),
                    found.value,
                    collection.value,
                    ty.as_ir(),
                    found_index.value,
                    ty.as_ir(),
                    ty.as_ir(),
                    ty.as_ir(),
                    ty.default_value(),
                    ty.as_ir()
                ));
                Ok(LlValue { value: result, ty })
            }
            IrType::String => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let slot = self.tmp();
                let value = self.tmp();
                self.body.push_str(&format!(
                    "  {slot} = getelementptr inbounds i8, ptr {}, i64 {}\n  {value} = load i8, ptr {slot}\n",
                    collection.value, index.value
                ));
                Ok(LlValue {
                    value,
                    ty: LlType::I8,
                })
            }
            IrType::Ref(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let slot = self.tmp();
                let value = self.tmp();
                let ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds {}, ptr {}, i64 {}\n  {} = load {}, ptr {}\n",
                    slot, ty.as_ir(), collection.value, index.value, value, ty.as_ir(), slot
                ));
                Ok(LlValue { value, ty })
            }
            IrType::Unknown(_) => {
                self.emit_typed_expr(index)?;
                Ok(LlValue {
                    value: LlType::Ptr.default_value().to_string(),
                    ty: LlType::Ptr,
                })
            }
            other => Err(format!(
                "LLVM TIR backend: indexing is unsupported for {other:?}"
            )),
        }
    }

    fn emit_index_assignment(
        &mut self,
        target: &TypedExpr,
        index: &TypedExpr,
        source: &TypedExpr,
    ) -> Result<(), String> {
        if matches!(target.ty, IrType::Unknown(_)) || self.is_opaque_field(target) {
            self.emit_typed_expr(target)?;
            self.emit_typed_expr(index)?;
            self.emit_typed_expr(source)?;
            return Ok(());
        }

        let collection = self.emit_typed_expr(target)?;
        match &target.ty {
            IrType::Array(element) | IrType::List(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let data_field = if matches!(target.ty, IrType::Array(_)) {
                    ("%glitch.array", 1)
                } else {
                    ("%glitch.list", 2)
                };
                let data_ptr = self.tmp();
                let data = self.tmp();
                let slot = self.tmp();
                let element_ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {data_ptr} = getelementptr inbounds {}, ptr {}, i32 0, i32 {}\n  {data} = load ptr, ptr {data_ptr}\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {}\n",
                    data_field.0,
                    collection.value,
                    data_field.1,
                    element_ty.as_ir(),
                    index.value
                ));
                let value = self.emit_typed_expr(source)?;
                let value = self.cast_value(value, &element_ty)?;
                self.retain_for_store(element, source, &value.value);
                if let Some(type_name) = object_type_name(element) {
                    if self.object_types.contains_key(type_name) {
                        let old = self.tmp();
                        self.body.push_str(&format!(
                            "  {old} = load {}, ptr {slot}\n",
                            element_ty.as_ir()
                        ));
                        self.emit_drop(type_name, &old);
                    }
                } else if matches!(element.as_ref(), IrType::String) {
                    let old = self.tmp();
                    self.body.push_str(&format!(
                        "  {old} = load ptr, ptr {slot}\n  call void @glitch_string_release(ptr {old})\n"
                    ));
                }
                self.body.push_str(&format!(
                    "  store {} {}, ptr {slot}\n",
                    element_ty.as_ir(),
                    value.value
                ));
                Ok(())
            }
            IrType::Dictionary(key_ty, value_ty) => {
                let key_source = index;
                let key = self.emit_typed_expr(key_source)?;
                let key = self.cast_value(key, &llvm_ir_type(key_ty))?;
                let value = self.emit_typed_expr(source)?;
                let value = self.cast_value(value, &llvm_ir_type(value_ty))?;
                let (found_index, found) =
                    self.emit_dict_find(&collection.value, key_ty, &key.value)?;
                let update_label = self.next_label("dict_set_update");
                let insert_label = self.next_label("dict_set_insert");
                let done_label = self.next_label("dict_set_done");
                self.body.push_str(&format!(
                    "  br i1 {}, label %{update_label}, label %{insert_label}\n{update_label}:\n",
                    found.value
                ));
                let values_ptr = self.tmp();
                let values = self.tmp();
                let slot = self.tmp();
                let value_ty_llvm = llvm_ir_type(value_ty);
                self.body.push_str(&format!(
                    "  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {}, i32 0, i32 3\n  {values} = load ptr, ptr {values_ptr}\n  {slot} = getelementptr inbounds {}, ptr {values}, i64 {}\n",
                    collection.value,
                    value_ty_llvm.as_ir(),
                    found_index.value
                ));
                self.retain_for_store(value_ty, source, &value.value);
                if let Some(type_name) = object_type_name(value_ty) {
                    if self.object_types.contains_key(type_name) {
                        let old = self.tmp();
                        self.body.push_str(&format!(
                            "  {old} = load {}, ptr {slot}\n",
                            value_ty_llvm.as_ir()
                        ));
                        self.emit_drop(type_name, &old);
                    }
                } else if matches!(value_ty.as_ref(), IrType::String) {
                    let old = self.tmp();
                    self.body.push_str(&format!(
                        "  {old} = load ptr, ptr {slot}\n  call void @glitch_string_release(ptr {old})\n"
                    ));
                }
                self.body.push_str(&format!(
                    "  store {} {}, ptr {slot}\n  br label %{done_label}\n{insert_label}:\n",
                    value_ty_llvm.as_ir(),
                    value.value
                ));
                self.emit_dict_add(
                    &collection.value,
                    key_ty,
                    &key.value,
                    key_source,
                    value_ty,
                    &value.value,
                    source,
                );
                self.body
                    .push_str(&format!("  br label %{done_label}\n{done_label}:\n"));
                Ok(())
            }
            IrType::Ref(element) => {
                let index = self.emit_typed_expr(index)?;
                let index = self.cast_value(index, &LlType::I64)?;
                let slot = self.tmp();
                let element_ty = llvm_ir_type(element);
                self.body.push_str(&format!(
                    "  {} = getelementptr inbounds {}, ptr {}, i64 {}\n",
                    slot, element_ty.as_ir(), collection.value, index.value
                ));
                let value = self.emit_typed_expr(source)?;
                let value = self.cast_value(value, &element_ty)?;
                self.body.push_str(&format!(
                    "  store {} {}, ptr {}\n",
                    element_ty.as_ir(), value.value, slot
                ));
                Ok(())
            }
            other => Err(format!(
                "LLVM TIR backend: index assignment is unsupported for {other:?}"
            )),
        }
    }

    fn emit_equality(&mut self, ty: &IrType, left: &str, right: &str, result: &str) {
        if matches!(ty, IrType::String) {
            let cmp = self.tmp();
            self.body.push_str(&format!(
                "  {cmp} = call i32 @strcmp(ptr {left}, ptr {right})\n  {result} = icmp eq i32 {cmp}, 0\n"
            ));
        } else if matches!(llvm_ir_type(ty), LlType::Double) {
            self.body
                .push_str(&format!("  {result} = fcmp oeq double {left}, {right}\n"));
        } else {
            self.body.push_str(&format!(
                "  {result} = icmp eq {} {left}, {right}\n",
                llvm_ir_type(ty).as_ir()
            ));
        }
    }

    fn emit_type_size(&mut self, ty: &IrType) -> String {
        match llvm_ir_type(ty) {
            LlType::I1 | LlType::I8 => "1".to_string(),
            LlType::I16 => "2".to_string(),
            LlType::I32 => "4".to_string(),
            LlType::I64 | LlType::Double | LlType::Ptr => "8".to_string(),
            LlType::Void => "1".to_string(),
        }
    }

    fn emit_new_object(
        &mut self,
        type_name: &str,
        constructor: Option<&str>,
        args: &[TypedExpr],
        fields: &[TypedFieldInit],
    ) -> Result<LlValue, String> {
        let Some(object) = self.object_types.get(type_name).cloned() else {
            for arg in args {
                self.emit_typed_expr(arg)?;
            }
            for field in fields {
                self.emit_typed_expr(&field.expr)?;
            }
            return Ok(LlValue {
                value: "null".to_string(),
                ty: LlType::Ptr,
            });
        };
        if matches!(object.kind, TypeKind::Interface) {
            return Err(format!(
                "LLVM TIR backend: interface '{type_name}' cannot be allocated"
            ));
        }
        if type_name.starts_with("Rc_") {
            let [value_expr] = args else {
                return Err(format!(
                    "LLVM TIR backend: {type_name} constructor expects exactly one argument"
                ));
            };
            let llvm_name = llvm_object_name(type_name);
            let size_ptr = self.tmp();
            let size = self.tmp();
            let value = self.tmp();
            self.body.push_str(&format!(
                "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {value} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
            ));
            let rc_ptr = self.tmp();
            let drop_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 1\n  store ptr @{}, ptr {drop_ptr}\n",
                destroy_symbol(type_name)
            ));
            let field_value = self.emit_typed_expr(value_expr)?;
            let inner_field = object.fields.get("value").cloned().ok_or_else(|| {
                format!(
                    "LLVM TIR backend: {type_name} layout is missing the value field"
                )
            })?;
            let field_ty = llvm_ir_type(&inner_field.ty);
            let field_value = self.cast_value(field_value, &field_ty)?;
            let ptr = self.tmp();
            self.body.push_str(&format!(
                "  {ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 2\n  store {} {}, ptr {ptr}\n",
                field_ty.as_ir(),
                field_value.value
            ));
            let ref_count_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {ref_count_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 3\n  store i32 1, ptr {ref_count_ptr}\n"
            ));
            return Ok(LlValue { value, ty: LlType::Ptr });
        }
        let llvm_name = llvm_object_name(type_name);
        let size_ptr = self.tmp();
        let size = self.tmp();
        let value = self.tmp();
        self.body.push_str(&format!(
            "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {value} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
        ));
        if matches!(object.kind, TypeKind::Class) {
            let rc_ptr = self.tmp();
            let drop_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 1\n  store ptr @{}, ptr {drop_ptr}\n",
                destroy_symbol(type_name)
            ));
        }
        if let Some(constructor) = constructor {
            self.emit_typed_call(
                constructor,
                std::iter::once(LlValue {
                    value: value.clone(),
                    ty: LlType::Ptr,
                }),
                args,
            )?;
        } else if !args.is_empty() {
            return Err(format!(
                "LLVM TIR backend: type '{type_name}' has no constructor accepting arguments"
            ));
        }
        for field_init in fields {
            let field = object.fields.get(&field_init.name).ok_or_else(|| {
                format!(
                    "LLVM TIR backend: type '{type_name}' has no field '{}'",
                    field_init.name
                )
            })?;
            let field_value = self.emit_typed_expr(&field_init.expr)?;
            let field_ty = llvm_ir_type(&field.ty);
            let field_value = self.cast_value(field_value, &field_ty)?;
            self.retain_for_store(&field.ty, &field_init.expr, &field_value.value);
            let ptr = self.tmp();
            self.body.push_str(&format!(
                "  {ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 {}\n  store {} {}, ptr {ptr}\n",
                field.index,
                field_ty.as_ir(),
                field_value.value
            ));
        }
        Ok(LlValue {
            value,
            ty: LlType::Ptr,
        })
    }

    fn emit_field_ptr(&mut self, expr: &TypedExpr) -> Result<LlValue, String> {
        let TypedExprKind::Field { target, name } = &expr.kind else {
            return Err(format!(
                "LLVM TIR backend: unsupported assignment target {:?} with type {:?}",
                expr.kind, expr.ty
            ));
        };
        let type_name = object_type_name(&target.ty).ok_or_else(|| {
            format!(
                "LLVM TIR backend: field '{}' target has no object layout: {:?}",
                name, target.ty
            )
        })?;
        let object =
            self.object_types.get(type_name).cloned().ok_or_else(|| {
                format!("LLVM TIR backend: type '{type_name}' has no LLVM layout")
            })?;
        let field =
            object.fields.get(name).cloned().ok_or_else(|| {
                format!("LLVM TIR backend: type '{type_name}' has no field '{name}'")
            })?;
        let target = self.emit_typed_expr(target)?;
        let ptr = self.tmp();
        self.body.push_str(&format!(
            "  {ptr} = getelementptr inbounds %{}, ptr {}, i32 0, i32 {}\n",
            llvm_object_name(type_name),
            target.value,
            field.index
        ));
        Ok(LlValue {
            value: ptr,
            ty: LlType::Ptr,
        })
    }

    fn is_opaque_field(&self, expr: &TypedExpr) -> bool {
        let TypedExprKind::Field { target, name } = &expr.kind else {
            return false;
        };
        if matches!(target.ty, IrType::Unknown(_)) {
            return true;
        }
        object_type_name(&target.ty).is_some_and(|type_name| {
            self.object_types
                .get(type_name)
                .is_none_or(|object| !object.fields.contains_key(name))
        })
    }

    fn emit_opaque_call(
        &mut self,
        target: Option<&TypedExpr>,
        args: &[TypedExpr],
        return_type: &IrType,
    ) -> Result<LlValue, String> {
        if let Some(target) = target {
            if !matches!(target.kind, TypedExprKind::Var(_))
                || self.vars.contains_key(match &target.kind {
                    TypedExprKind::Var(name) => name,
                    _ => unreachable!(),
                })
            {
                let value = self.emit_typed_expr(target)?;
                self.emit_temporary_drop(target, &value);
            }
        }
        for arg in args {
            let value = self.emit_typed_expr(arg)?;
            self.emit_temporary_drop(arg, &value);
        }
        self.default_typed_value(return_type)
    }

    fn default_typed_value(&mut self, ty: &IrType) -> Result<LlValue, String> {
        let ll_type = llvm_ir_type(ty);
        let value = if matches!(ty, IrType::String) {
            self.string_global("")
        } else {
            ll_type.default_value().to_string()
        };
        Ok(LlValue { value, ty: ll_type })
    }

    fn retain_for_store(&mut self, ty: &IrType, expr: &TypedExpr, value: &str) {
        if matches!(ty, IrType::String) {
            if matches!(
                expr.kind,
                TypedExprKind::Var(_) | TypedExprKind::Field { .. } | TypedExprKind::Index { .. }
            ) {
                self.body
                    .push_str(&format!("  call void @glitch_string_retain(ptr {value})\n"));
            }
            return;
        }
        let Some(type_name) = object_type_name(ty) else {
            return;
        };
        if !self.object_types.contains_key(type_name)
            || matches!(
                expr.kind,
                TypedExprKind::NewObject { .. }
                    | TypedExprKind::Call(_)
                    | TypedExprKind::Await(_)
                    | TypedExprKind::Move(_)
            )
        {
            return;
        }
        self.emit_retain(type_name, value);
    }

    fn emit_retain(&mut self, type_name: &str, value: &str) {
        if self
            .object_types
            .get(type_name)
            .is_some_and(|object| matches!(object.kind, TypeKind::Class | TypeKind::Interface))
        {
            self.body.push_str(&format!(
                "  call void @{}(ptr {value})\n",
                retain_symbol(type_name)
            ));
        }
    }

    /// Lift a lambda body to a named top-level LLVM function and return a
    /// function pointer to it.
    ///
    /// Captured variables (free variables referenced in the body that are not
    /// lambda parameters) are appended as extra `ptr` arguments so the lifted
    /// function does not require a heap-allocated closure environment.  The
    /// caller (the site that creates the lambda) passes the current values of
    /// those variables directly.
    fn emit_lambda_function(
        &mut self,
        params: &[String],
        body: &TypedExpr,
    ) -> Result<LlValue, String> {
        let id = self.lambda_id;
        self.lambda_id += 1;
        let fn_name = format!("glitch_lambda_{id}");

        // Collect free variables: Var references in `body` that are NOT in
        // `params` but ARE in the current scope's vars map.
        let mut free_vars: Vec<(String, LlVar)> = Vec::new();
        let mut seen = std::collections::HashSet::new();
        collect_free_vars_expr(body, params, &self.vars, &mut seen, &mut free_vars);

        // Define the environment struct type if there are captures.
        let env_struct_name = format!("glitch.lambda.{id}.env");
        if !free_vars.is_empty() {
            let field_types: Vec<String> = free_vars
                .iter()
                .map(|(_, lv)| lv.ty.as_ir().to_string())
                .collect();
            self.type_defs.push(format!(
                "%{} = type {{ {} }}\n",
                env_struct_name,
                field_types.join(", ")
            ));
        }

        // Save emitter state that belongs to the enclosing function.
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

        // Swap self.body so we can emit the lambda function globally
        let saved_body = std::mem::take(&mut self.body);

        // Set up the lambda's own scope.
        self.vars.clear();
        self.drop_order.clear();
        self.tmp = 0;
        self.label = 0;
        self.terminated = false;
        self.current_is_main = false;
        self.exception_handler = None;
        self.current_unwind_label = "exception_unwind".to_string();
        self.loop_targets.clear();
        // Lambda bodies always return a ptr (the serialised result or null).
        self.current_return = LlType::Ptr;

        // Build the LLVM function signature.
        // The first parameter is always the env pointer.
        let mut param_decls = vec!["ptr %env".to_string()];
        for p in params {
            param_decls.push(format!("ptr %{}", sanitize(p)));
        }

        self.body.push_str(&format!(
            "define ptr @{fn_name}({}) {{\nentry:\n",
            param_decls.join(", ")
        ));

        // Bind explicit params into alloca slots.
        for p in params {
            let ptr = self.tmp();
            self.body.push_str(&format!(
                "  {ptr} = alloca ptr\n  store ptr %{}, ptr {ptr}\n",
                sanitize(p)
            ));
            self.vars.insert(
                p.clone(),
                LlVar {
                    ptr,
                    ty: LlType::Ptr,
                    ir_ty: IrType::Unknown("lambda_param".to_string()),
                    drop_kind: DropKind::BorrowOnly,
                },
            );
        }

        // Bind captured vars: load them from the environment pointer.
        for (i, (name, lv)) in free_vars.iter().enumerate() {
            let ptr = self.tmp();
            self.body
                .push_str(&format!("  {ptr} = alloca {}\n", lv.ty.as_ir()));
            let field_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {} = getelementptr inbounds %{}, ptr %env, i32 0, i32 {}\n",
                field_ptr, env_struct_name, i
            ));
            let loaded_val = self.tmp();
            self.body.push_str(&format!(
                "  {} = load {}, ptr {}\n",
                loaded_val,
                lv.ty.as_ir(),
                field_ptr
            ));
            self.body.push_str(&format!(
                "  store {} {}, ptr {}\n",
                lv.ty.as_ir(),
                loaded_val,
                ptr
            ));
            self.vars.insert(
                name.clone(),
                LlVar {
                    ptr,
                    ty: lv.ty.clone(),
                    ir_ty: lv.ir_ty.clone(),
                    drop_kind: DropKind::BorrowOnly,
                },
            );
        }

        // Emit the body expression and return it.
        let result = self.emit_typed_expr(body);
        match result {
            Ok(val) => {
                if !self.terminated {
                    let ret_val = if val.ty == LlType::Ptr {
                        val.value.clone()
                    } else {
                        // Cast scalar to ptr so the return type is uniform.
                        let cast = self.tmp();
                        self.body.push_str(&format!(
                            "  {cast} = inttoptr {} {} to ptr\n",
                            val.ty.as_ir(),
                            val.value
                        ));
                        cast
                    };
                    self.body.push_str(&format!("  ret ptr {ret_val}\n"));
                }
            }
            Err(_) => {
                if !self.terminated {
                    self.body.push_str("  ret ptr null\n");
                }
            }
        }
        self.body.push_str(&format!("{}:\n", self.current_unwind_label));
        self.terminated = false;
        self.body.push_str("  ret ptr null\n");
        self.body.push_str("}\n\n");

        // Swap the emitted lambda function out and restore enclosing function's body
        let lambda_func = std::mem::replace(&mut self.body, saved_body);
        self.globals.push(lambda_func);

        // Restore the enclosing function's state.
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

        // Build the delegate struct `{ ptr fn_ptr, ptr env_ptr }` on the stack.
        let delegate_ptr = self.tmp();
        self.body
            .push_str(&format!("  {} = alloca {{ ptr, ptr }}\n", delegate_ptr));

        // Store fn_ptr
        let fn_field = self.tmp();
        self.body.push_str(&format!(
            "  {} = getelementptr inbounds {{ ptr, ptr }}, ptr {}, i32 0, i32 0\n",
            fn_field, delegate_ptr
        ));
        self.body
            .push_str(&format!("  store ptr @{}, ptr {}\n", fn_name, fn_field));

        // Store env_ptr
        let env_field = self.tmp();
        self.body.push_str(&format!(
            "  {} = getelementptr inbounds {{ ptr, ptr }}, ptr {}, i32 0, i32 1\n",
            env_field, delegate_ptr
        ));

        if free_vars.is_empty() {
            self.body
                .push_str(&format!("  store ptr null, ptr {}\n", env_field));
        } else {
            // Allocate the environment struct on the stack and store captures.
            let env_ptr = self.tmp();
            self.body
                .push_str(&format!("  {} = alloca %{}\n", env_ptr, env_struct_name));

            for (i, (name, lv)) in free_vars.iter().enumerate() {
                if let Some(var) = self.vars.get(name).cloned() {
                    let cap_val = self.tmp();
                    self.body.push_str(&format!(
                        "  {} = load {}, ptr {}\n",
                        cap_val,
                        lv.ty.as_ir(),
                        var.ptr
                    ));
                    let field_ptr = self.tmp();
                    self.body.push_str(&format!(
                        "  {} = getelementptr inbounds %{}, ptr {}, i32 0, i32 {}\n",
                        field_ptr, env_struct_name, env_ptr, i
                    ));
                    self.body.push_str(&format!(
                        "  store {} {}, ptr {}\n",
                        lv.ty.as_ir(),
                        cap_val,
                        field_ptr
                    ));
                }
            }

            self.body
                .push_str(&format!("  store ptr {}, ptr {}\n", env_ptr, env_field));
        }

        Ok(LlValue {
            value: delegate_ptr,
            ty: LlType::Ptr,
        })
    }

    fn emit_temporary_drop(&mut self, expr: &TypedExpr, value: &LlValue) {
        if matches!(
            expr.kind,
            TypedExprKind::Var(_)
                | TypedExprKind::Field { .. }
                | TypedExprKind::Index { .. }
                | TypedExprKind::Move(_)
                | TypedExprKind::Borrow { .. }
                | TypedExprKind::FunctionSymbol(_)
        ) {
            return;
        }
        match expr.drop_kind {
            DropKind::Free if matches!(expr.ty, IrType::String) => {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {})\n",
                    value.value
                ));
            }
            DropKind::Free if matches!(expr.ty, IrType::Array(_)) => {
                if let IrType::Array(element) = &expr.ty {
                    self.emit_array_drop_value(&value.value, element);
                }
            }
            DropKind::DropCollection => {
                self.emit_collection_drop_value(&expr.ty, &value.value);
            }
            DropKind::DropClass | DropKind::DropStruct => {
                if let Some(type_name) = object_type_name(&expr.ty) {
                    self.emit_drop(type_name, &value.value);
                }
            }
            _ => {}
        }
    }

    fn emit_drop(&mut self, type_name: &str, value: &str) {
        if self.object_types.contains_key(type_name) {
            self.body.push_str(&format!(
                "  call void @{}(ptr {value})\n",
                drop_symbol(type_name)
            ));
        }
    }

    fn emit_var_drop(&mut self, var: &LlVar) {
        if matches!(var.drop_kind, DropKind::Free) && matches!(var.ir_ty, IrType::String) {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            self.body.push_str(&format!(
                "  call void @glitch_string_release(ptr {value})\n"
            ));
            return;
        }
        if matches!(var.drop_kind, DropKind::Free) && matches!(var.ir_ty, IrType::Array(_)) {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            if let IrType::Array(element) = &var.ir_ty {
                self.emit_array_drop_value(&value, element);
            }
            return;
        }
        if matches!(var.drop_kind, DropKind::DropCollection) {
            let value = self.tmp();
            self.body
                .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
            self.emit_collection_drop_value(&var.ir_ty, &value);
            return;
        }
        if !matches!(var.drop_kind, DropKind::DropClass | DropKind::DropStruct) {
            return;
        }
        let Some(type_name) = object_type_name(&var.ir_ty) else {
            return;
        };
        if !self.object_types.contains_key(type_name) {
            return;
        }
        let value = self.tmp();
        self.body
            .push_str(&format!("  {value} = load ptr, ptr {}\n", var.ptr));
        self.emit_drop(type_name, &value);
    }

    fn emit_local_drops(&mut self, returned_local: Option<&str>) {
        let names = self.drop_order.iter().rev().cloned().collect::<Vec<_>>();
        for name in names {
            if returned_local == Some(name.as_str()) {
                continue;
            }
            if let Some(var) = self.vars.get(&name).cloned() {
                self.emit_var_drop(&var);
            }
        }
    }

    fn emit_collection_drop_value(&mut self, ty: &IrType, value: &str) {
        let is_null = self.tmp();
        let release_label = self.next_label("collection_release");
        let done_label = self.next_label("collection_release_done");
        self.body.push_str(&format!(
            "  {is_null} = icmp eq ptr {value}, null\n  br i1 {is_null}, label %{done_label}, label %{release_label}\n{release_label}:\n"
        ));
        match ty {
            IrType::List(element) => {
                let len_ptr = self.tmp();
                let data_ptr = self.tmp();
                let len = self.tmp();
                let data = self.tmp();
                self.body.push_str(&format!(
                    "  {len_ptr} = getelementptr inbounds %glitch.list, ptr {value}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds %glitch.list, ptr {value}, i32 0, i32 2\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n"
                ));
                self.emit_buffer_element_drops(element, &data, &len);
                self.body.push_str(&format!(
                    "  call void @glitch_free(ptr {data})\n  call void @glitch_free(ptr {value})\n  br label %{done_label}\n"
                ));
            }
            IrType::Dictionary(key, item) => {
                let len_ptr = self.tmp();
                let keys_ptr = self.tmp();
                let values_ptr = self.tmp();
                let len = self.tmp();
                let keys = self.tmp();
                let values = self.tmp();
                self.body.push_str(&format!(
                    "  {len_ptr} = getelementptr inbounds %glitch.dict, ptr {value}, i32 0, i32 0\n  {keys_ptr} = getelementptr inbounds %glitch.dict, ptr {value}, i32 0, i32 2\n  {values_ptr} = getelementptr inbounds %glitch.dict, ptr {value}, i32 0, i32 3\n  {len} = load i64, ptr {len_ptr}\n  {keys} = load ptr, ptr {keys_ptr}\n  {values} = load ptr, ptr {values_ptr}\n"
                ));
                self.emit_buffer_element_drops(key, &keys, &len);
                self.emit_buffer_element_drops(item, &values, &len);
                self.body.push_str(&format!(
                    "  call void @glitch_free(ptr {keys})\n  call void @glitch_free(ptr {values})\n  call void @glitch_free(ptr {value})\n  br label %{done_label}\n"
                ));
            }
            _ => {
                self.body.push_str(&format!(
                    "  call void @glitch_free(ptr {value})\n  br label %{done_label}\n"
                ));
            }
        }
        self.body.push_str(&format!("{done_label}:\n"));
    }

    fn emit_array_drop_value(&mut self, value: &str, element: &IrType) {
        let is_null = self.tmp();
        let release_label = self.next_label("array_release");
        let done_label = self.next_label("array_release_done");
        let len_ptr = self.tmp();
        let len = self.tmp();
        let data_ptr = self.tmp();
        let data = self.tmp();
        self.body.push_str(&format!(
            "  {is_null} = icmp eq ptr {value}, null\n  br i1 {is_null}, label %{done_label}, label %{release_label}\n{release_label}:\n  {len_ptr} = getelementptr inbounds %glitch.array, ptr {value}, i32 0, i32 0\n  {data_ptr} = getelementptr inbounds %glitch.array, ptr {value}, i32 0, i32 1\n  {len} = load i64, ptr {len_ptr}\n  {data} = load ptr, ptr {data_ptr}\n"
        ));
        self.emit_buffer_element_drops(element, &data, &len);
        self.body.push_str(&format!(
            "  call void @glitch_free(ptr {data})\n  call void @glitch_free(ptr {value})\n  br label %{done_label}\n{done_label}:\n"
        ));
    }

    fn emit_buffer_element_drops(&mut self, element: &IrType, data: &str, len: &str) {
        let drop_string = matches!(element, IrType::String);
        let drop_collection = matches!(element, IrType::List(_) | IrType::Dictionary(_, _));
        let drop_array = matches!(element, IrType::Array(_));
        let object = object_type_name(element)
            .filter(|name| self.object_types.contains_key(*name))
            .map(str::to_string);
        if !drop_string && !drop_collection && !drop_array && object.is_none() {
            return;
        }
        let index_ptr = self.tmp();
        let loop_label = self.next_label("element_drop_loop");
        let body_label = self.next_label("element_drop_body");
        let done_label = self.next_label("element_drop_done");
        let index = self.tmp();
        let in_range = self.tmp();
        let slot = self.tmp();
        let item = self.tmp();
        let next = self.tmp();
        let element_type = llvm_ir_type(element);
        self.body.push_str(&format!(
            "  {index_ptr} = alloca i64\n  store i64 0, ptr {index_ptr}\n  br label %{loop_label}\n{loop_label}:\n  {index} = load i64, ptr {index_ptr}\n  {in_range} = icmp ult i64 {index}, {len}\n  br i1 {in_range}, label %{body_label}, label %{done_label}\n{body_label}:\n  {slot} = getelementptr inbounds {}, ptr {data}, i64 {index}\n  {item} = load {}, ptr {slot}\n",
            element_type.as_ir(),
            element_type.as_ir()
        ));
        if drop_string {
            self.body
                .push_str(&format!("  call void @glitch_string_release(ptr {item})\n"));
        } else if drop_collection {
            // Recursively drop inner collections (e.g. List<List<string>>).
            self.emit_collection_drop_value(element, &item);
        } else if drop_array {
            if let IrType::Array(inner_el) = element {
                self.emit_array_drop_value(&item, inner_el);
            }
        } else if let Some(type_name) = object {
            self.emit_drop(&type_name, &item);
        }
        self.body.push_str(&format!(
            "  {next} = add i64 {index}, 1\n  store i64 {next}, ptr {index_ptr}\n  br label %{loop_label}\n{done_label}:\n"
        ));
    }

    fn emit_endpoint_dispatch(&mut self) -> Result<(), String> {
        let handlers = self.endpoint_handlers.clone();
        let mut thunk_names = Vec::with_capacity(handlers.len());
        for (index, handler) in handlers.iter().enumerate() {
            let thunk = format!("glitch_endpoint_handler_{index}");
            self.emit_endpoint_thunk(&thunk, handler)?;
            thunk_names.push(thunk);
        }

        let routes = handlers
            .iter()
            .map(|handler| {
                (
                    self.string_global(&handler.http_method),
                    self.string_global(&handler.path),
                )
            })
            .collect::<Vec<_>>();

        self.body.push_str(
            "define i1 @glitch_endpoint_handlers_contains(ptr %app, ptr %method, ptr %path) {\nentry:\n",
        );
        if routes.is_empty() {
            self.body.push_str("  ret i1 false\n}\n\n");
        } else {
            for (index, (method, path)) in routes.iter().enumerate() {
                let next = if index + 1 == routes.len() {
                    "endpoint_not_found".to_string()
                } else {
                    format!("endpoint_next_{index}")
                };
                self.body.push_str(&format!(
                    "  %contains_method_cmp_{index} = call i32 @strcmp(ptr %method, ptr {method})\n  %contains_method_match_{index} = icmp eq i32 %contains_method_cmp_{index}, 0\n  br i1 %contains_method_match_{index}, label %contains_path_{index}, label %{next}\ncontains_path_{index}:\n  %contains_path_match_{index} = call i1 @glitch_route_match(ptr {path}, ptr %path)\n  br i1 %contains_path_match_{index}, label %endpoint_found, label %{next}\n"
                ));
                if index + 1 != routes.len() {
                    self.body.push_str(&format!("{next}:\n"));
                }
            }
            self.body.push_str(
                "endpoint_found:\n  ret i1 true\nendpoint_not_found:\n  ret i1 false\n}\n\n",
            );
        }

        let not_found = self.string_global("404");
        self.body.push_str(
            "define ptr @glitch_endpoint_handlers_invoke(ptr %app, ptr %method, ptr %path, ptr %body) {\nentry:\n",
        );
        if routes.is_empty() {
            self.body
                .push_str(&format!("  ret ptr {not_found}\n}}\n\n"));
        } else {
            for (index, ((method, path), thunk)) in
                routes.iter().zip(thunk_names.iter()).enumerate()
            {
                let next = if index + 1 == routes.len() {
                    "invoke_not_found".to_string()
                } else {
                    format!("invoke_next_{index}")
                };
                self.body.push_str(&format!(
                    "  %invoke_method_cmp_{index} = call i32 @strcmp(ptr %method, ptr {method})\n  %invoke_method_match_{index} = icmp eq i32 %invoke_method_cmp_{index}, 0\n  br i1 %invoke_method_match_{index}, label %invoke_path_{index}, label %{next}\ninvoke_path_{index}:\n  %invoke_path_match_{index} = call i1 @glitch_route_match(ptr {path}, ptr %path)\n  br i1 %invoke_path_match_{index}, label %invoke_handler_{index}, label %{next}\ninvoke_handler_{index}:\n  %invoke_result_{index} = call ptr @{thunk}(ptr %path, ptr %body)\n  ret ptr %invoke_result_{index}\n"
                ));
                if index + 1 != routes.len() {
                    self.body.push_str(&format!("{next}:\n"));
                }
            }
            self.body
                .push_str(&format!("invoke_not_found:\n  ret ptr {not_found}\n}}\n\n"));
        }
        Ok(())
    }

    fn emit_endpoint_thunk(
        &mut self,
        thunk: &str,
        handler: &EndpointHandlerBinding,
    ) -> Result<(), String> {
        let supported_return = self.endpoint_return_supported(&handler.response_type);
        let not_implemented = self.string_global("501 Not Implemented");
        self.body.push_str(&format!(
            "define ptr @{thunk}(ptr %path, ptr %body) {{\nentry:\n"
        ));

        if !supported_return
            || !handler
                .params
                .iter()
                .all(|param| self.endpoint_parameter_supported(param, &handler.path))
        {
            self.body
                .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
            return Ok(());
        }

        if let Some(controller_name) = &handler.controller {
            let Some(object) = self.object_types.get(controller_name).cloned() else {
                self.body
                    .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
                return Ok(());
            };
            let Some(method) = self.functions.get(&handler.function) else {
                self.body
                    .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
                return Ok(());
            };
            let mut expected_method_params = vec![LlType::Ptr];
            expected_method_params
                .extend(handler.params.iter().map(|param| llvm_ir_type(&param.ty)));
            if method.return_type != LlType::Ptr || method.params != expected_method_params {
                self.body
                    .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
                return Ok(());
            }
            if !handler
                .constructor_params
                .iter()
                .all(|ty| self.can_allocate_endpoint_dependency(ty))
            {
                self.body
                    .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
                return Ok(());
            }

            let llvm_name = llvm_object_name(controller_name);
            self.body.push_str(&format!(
                "  %size_ptr = getelementptr %{llvm_name}, ptr null, i32 1\n  %size = ptrtoint ptr %size_ptr to i64\n  %controller = call ptr @glitch_calloc(i64 1, i64 %size)\n"
            ));
            if matches!(object.kind, TypeKind::Class) {
                self.body.push_str(&format!(
                    "  %rc_ptr = getelementptr inbounds %{llvm_name}, ptr %controller, i32 0, i32 0\n  store i64 1, ptr %rc_ptr\n  %controller_drop_ptr = getelementptr inbounds %{llvm_name}, ptr %controller, i32 0, i32 1\n  store ptr @{}, ptr %controller_drop_ptr\n",
                    destroy_symbol(controller_name)
                ));
            }
            let mut dependencies = Vec::new();
            for (index, dependency_type) in handler.constructor_params.iter().enumerate() {
                let type_name = self
                    .resolve_endpoint_dependency_name(dependency_type)
                    .ok_or_else(|| {
                    format!(
                        "LLVM TIR backend: controller dependency has no object type: {dependency_type:?}"
                    )
                })?;
                let value = self
                    .emit_endpoint_object_allocation(&type_name, &format!("dependency_{index}"))?;
                dependencies.push((type_name, value));
            }
            if let Some(constructor) = &handler.constructor {
                let mut args = vec!["ptr %controller".to_string()];
                args.extend(dependencies.iter().map(|(_, value)| format!("ptr {value}")));
                self.body.push_str(&format!(
                    "  call void @{}({})\n",
                    sanitize(constructor),
                    args.join(", ")
                ));
            }
            for (type_name, value) in &dependencies {
                self.body.push_str(&format!(
                    "  call void @{}(ptr {value})\n",
                    drop_symbol(type_name)
                ));
            }
            let (action_args, string_args, object_args) =
                self.emit_endpoint_action_arguments(handler, "%path", "%body")?;
            let mut rendered_action_args = vec!["ptr %controller".to_string()];
            rendered_action_args.extend(action_args);
            self.body.push_str(&format!(
                "  %result = call ptr @{}({})\n",
                sanitize(&handler.function),
                rendered_action_args.join(", ")
            ));
            for value in string_args {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {value})\n"
                ));
            }
            for (type_name, value) in object_args {
                self.body.push_str(&format!(
                    "  call void @{}(ptr {value})\n",
                    drop_symbol(&type_name)
                ));
            }
            self.body.push_str(&format!(
                "  call void @{}(ptr %controller)\n",
                drop_symbol(controller_name)
            ));
            let response = self.emit_endpoint_result(&handler.response_type, "%result")?;
            self.body.push_str(&format!("  ret ptr {response}\n}}\n\n"));
            return Ok(());
        }

        let Some(function) = self.functions.get(&handler.function) else {
            self.body
                .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
            return Ok(());
        };
        if function.return_type == LlType::Ptr && function.params.is_empty() {
            self.body.push_str(&format!(
                "  %result = call ptr @{}()\n  ret ptr %result\n}}\n\n",
                sanitize(&handler.function)
            ));
        } else {
            self.body
                .push_str(&format!("  ret ptr {not_implemented}\n}}\n\n"));
        }
        Ok(())
    }

    fn emit_endpoint_action_arguments(
        &mut self,
        handler: &EndpointHandlerBinding,
        path: &str,
        body: &str,
    ) -> Result<(Vec<String>, Vec<String>, Vec<(String, String)>), String> {
        let mut args = Vec::new();
        let mut strings_to_release = Vec::new();
        let mut objects_to_release = Vec::new();
        for (index, param) in handler.params.iter().enumerate() {
            match (&param.source, &param.ty) {
                (EndpointParameterSource::Route, IrType::Int) => {
                    let segment =
                        route_parameter_segment(&handler.path, &param.name).ok_or_else(|| {
                            format!(
                                "LLVM TIR backend: route '{}' has no parameter '{}'",
                                handler.path, param.name
                            )
                        })?;
                    let raw = format!("%action_arg_{index}_i64");
                    let value = format!("%action_arg_{index}");
                    self.body.push_str(&format!(
                        "  {raw} = call i64 @glitch_path_segment_i64(ptr {path}, i64 {segment})\n  {value} = trunc i64 {raw} to i32\n"
                    ));
                    args.push(format!("i32 {value}"));
                }
                (EndpointParameterSource::Route, IrType::Long) => {
                    let segment =
                        route_parameter_segment(&handler.path, &param.name).ok_or_else(|| {
                            format!(
                                "LLVM TIR backend: route '{}' has no parameter '{}'",
                                handler.path, param.name
                            )
                        })?;
                    let value = format!("%action_arg_{index}");
                    self.body.push_str(&format!(
                        "  {value} = call i64 @glitch_path_segment_i64(ptr {path}, i64 {segment})\n"
                    ));
                    args.push(format!("i64 {value}"));
                }
                (EndpointParameterSource::Route, IrType::String) => {
                    let segment =
                        route_parameter_segment(&handler.path, &param.name).ok_or_else(|| {
                            format!(
                                "LLVM TIR backend: route '{}' has no parameter '{}'",
                                handler.path, param.name
                            )
                        })?;
                    let value = format!("%action_arg_{index}");
                    self.body.push_str(&format!(
                        "  {value} = call ptr @glitch_path_segment_string(ptr {path}, i64 {segment})\n"
                    ));
                    args.push(format!("ptr {value}"));
                    strings_to_release.push(value);
                }
                (EndpointParameterSource::Body, IrType::String) => {
                    let value = format!("%action_arg_{index}");
                    let empty = self.string_global("");
                    self.body.push_str(&format!(
                        "  {value} = call ptr @glitch_string_concat(ptr {body}, ptr {empty})\n"
                    ));
                    args.push(format!("ptr {value}"));
                    strings_to_release.push(value);
                }
                (EndpointParameterSource::Body, IrType::Class(type_name))
                | (EndpointParameterSource::Body, IrType::Struct(type_name)) => {
                    let value = self.emit_endpoint_body_object(
                        type_name,
                        body,
                        &format!("action_arg_{index}"),
                    )?;
                    args.push(format!("ptr {value}"));
                    objects_to_release.push((type_name.clone(), value));
                }
                (EndpointParameterSource::Query, IrType::String) => {
                    let value = format!("%action_arg_{index}");
                    let key = self.string_global(&param.name);
                    let key_length = param.name.len();
                    self.body.push_str(&format!(
                        "  {value} = call ptr @glitch_query_value_string(ptr {path}, ptr {key}, i64 {key_length})\n"
                    ));
                    args.push(format!("ptr {value}"));
                    strings_to_release.push(value);
                }
                (EndpointParameterSource::Query, IrType::Int) => {
                    let raw = format!("%action_arg_{index}_i64");
                    let value = format!("%action_arg_{index}");
                    let key = self.string_global(&param.name);
                    let key_length = param.name.len();
                    self.body.push_str(&format!(
                        "  {raw} = call i64 @glitch_query_value_i64(ptr {path}, ptr {key}, i64 {key_length})\n  {value} = trunc i64 {raw} to i32\n"
                    ));
                    args.push(format!("i32 {value}"));
                }
                (EndpointParameterSource::Query, IrType::Long) => {
                    let value = format!("%action_arg_{index}");
                    let key = self.string_global(&param.name);
                    let key_length = param.name.len();
                    self.body.push_str(&format!(
                        "  {value} = call i64 @glitch_query_value_i64(ptr {path}, ptr {key}, i64 {key_length})\n"
                    ));
                    args.push(format!("i64 {value}"));
                }
                (EndpointParameterSource::Query, IrType::Bool) => {
                    let text = format!("%action_arg_{index}_text");
                    let true_cmp = format!("%action_arg_{index}_true_cmp");
                    let one_cmp = format!("%action_arg_{index}_one_cmp");
                    let is_true = format!("%action_arg_{index}_true");
                    let is_one = format!("%action_arg_{index}_one");
                    let value = format!("%action_arg_{index}");
                    let key = self.string_global(&param.name);
                    let true_text = self.string_global("true");
                    let one_text = self.string_global("1");
                    let key_length = param.name.len();
                    self.body.push_str(&format!(
                        "  {text} = call ptr @glitch_query_value_string(ptr {path}, ptr {key}, i64 {key_length})\n  {true_cmp} = call i32 @strcmp(ptr {text}, ptr {true_text})\n  {one_cmp} = call i32 @strcmp(ptr {text}, ptr {one_text})\n  {is_true} = icmp eq i32 {true_cmp}, 0\n  {is_one} = icmp eq i32 {one_cmp}, 0\n  {value} = or i1 {is_true}, {is_one}\n  call void @glitch_string_release(ptr {text})\n"
                    ));
                    args.push(format!("i1 {value}"));
                }
                _ => {
                    return Err(format!(
                        "LLVM TIR backend: unsupported endpoint parameter {param:?}"
                    ));
                }
            }
        }
        Ok((args, strings_to_release, objects_to_release))
    }

    fn endpoint_parameter_supported(&self, param: &EndpointParameterBinding, route: &str) -> bool {
        match (&param.source, &param.ty) {
            (EndpointParameterSource::Route, IrType::Int | IrType::Long | IrType::String) => {
                route_parameter_segment(route, &param.name).is_some()
            }
            (EndpointParameterSource::Body, IrType::String) => true,
            (EndpointParameterSource::Body, IrType::Class(name) | IrType::Struct(name)) => {
                self.object_types.get(name).is_some_and(|object| {
                    !matches!(object.kind, TypeKind::Interface)
                        && object.fields.values().all(|field| {
                            matches!(
                                field.ty,
                                IrType::Bool
                                    | IrType::Byte
                                    | IrType::Short
                                    | IrType::Int
                                    | IrType::Long
                                    | IrType::UInt
                                    | IrType::Double
                                    | IrType::String
                            )
                        })
                        && object.constructor.as_ref().is_none_or(|constructor| {
                            self.functions
                                .get(constructor)
                                .is_some_and(|signature| signature.params == vec![LlType::Ptr])
                        })
                })
            }
            (
                EndpointParameterSource::Query,
                IrType::Bool | IrType::Int | IrType::Long | IrType::String,
            ) => true,
            _ => false,
        }
    }

    fn endpoint_return_supported(&self, ty: &IrType) -> bool {
        match ty {
            IrType::String => true,
            IrType::Class(name) | IrType::Struct(name) => {
                self.object_types.get(name).is_some_and(|object| {
                    !matches!(object.kind, TypeKind::Interface)
                        && object.fields.values().all(|field| {
                            matches!(
                                field.ty,
                                IrType::Bool
                                    | IrType::Byte
                                    | IrType::Short
                                    | IrType::Int
                                    | IrType::Long
                                    | IrType::UInt
                                    | IrType::Double
                                    | IrType::Decimal
                                    | IrType::String
                            )
                        })
                })
            }
            _ => false,
        }
    }

    fn emit_endpoint_result(&mut self, ty: &IrType, value: &str) -> Result<String, String> {
        match ty {
            IrType::String => Ok(value.to_string()),
            IrType::Class(name) | IrType::Struct(name) => {
                let null_label = self.next_label("endpoint_result_null");
                let value_label = self.next_label("endpoint_result_value");
                let end_label = self.next_label("endpoint_result_end");
                let is_null = self.tmp();
                self.body.push_str(&format!(
                    "  {is_null} = icmp eq ptr {value}, null\n  br i1 {is_null}, label %{null_label}, label %{value_label}\n{value_label}:\n"
                ));
                let json = self.emit_json_serialize_object(name, value)?;
                self.body.push_str(&format!(
                    "  call void @{}(ptr {value})\n  br label %{end_label}\n{null_label}:\n  br label %{end_label}\n{end_label}:\n",
                    drop_symbol(name)
                ));
                let result = self.tmp();
                let null_text = self.string_global("null");
                self.body.push_str(&format!(
                    "  {result} = phi ptr [{json}, %{value_label}], [{null_text}, %{null_label}]\n"
                ));
                Ok(result)
            }
            _ => Err(format!(
                "LLVM TIR backend: unsupported endpoint result type {ty:?}"
            )),
        }
    }

    fn emit_json_serialize_object(
        &mut self,
        type_name: &str,
        value: &str,
    ) -> Result<String, String> {
        let object = self.object_types.get(type_name).cloned().ok_or_else(|| {
            format!("LLVM TIR backend: result type '{type_name}' has no object layout")
        })?;
        let llvm_name = llvm_object_name(type_name);
        let mut fields = object.fields.iter().collect::<Vec<_>>();
        fields.sort_by_key(|(_, field)| field.index);
        let mut current = self.string_global("{");
        let mut current_is_managed = false;
        for (position, (field_name, field)) in fields.into_iter().enumerate() {
            let key = self.string_global(&format!(
                "{}\"{}\":",
                if position == 0 { "" } else { "," },
                field_name
            ));
            let with_key = format!("%json_{}_{}_key", sanitize(type_name), field.index);
            self.body.push_str(&format!(
                "  {with_key} = call ptr @glitch_string_concat(ptr {current}, ptr {key})\n"
            ));
            if current_is_managed {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {current})\n"
                ));
            }
            let field_ptr = format!("%json_{}_{}_ptr", sanitize(type_name), field.index);
            let field_value = format!("%json_{}_{}", sanitize(type_name), field.index);
            let field_type = llvm_ir_type(&field.ty);
            self.body.push_str(&format!(
                "  {field_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 {}\n  {field_value} = load {}, ptr {field_ptr}\n",
                field.index,
                field_type.as_ir()
            ));
            let (rendered, rendered_is_managed) = match &field.ty {
                IrType::String => {
                    let quote = self.string_global("\"");
                    let quoted = format!("%json_{}_{}_quoted", sanitize(type_name), field.index);
                    let complete =
                        format!("%json_{}_{}_complete", sanitize(type_name), field.index);
                    self.body.push_str(&format!(
                        "  {quoted} = call ptr @glitch_string_concat(ptr {quote}, ptr {field_value})\n  {complete} = call ptr @glitch_string_concat(ptr {quoted}, ptr {quote})\n  call void @glitch_string_release(ptr {quoted})\n"
                    ));
                    (complete, true)
                }
                IrType::Bool => {
                    let rendered = format!("%json_{}_{}_bool", sanitize(type_name), field.index);
                    self.body.push_str(&format!(
                        "  {rendered} = select i1 {field_value}, ptr getelementptr inbounds ([5 x i8], ptr @.json_true, i64 0, i64 0), ptr getelementptr inbounds ([6 x i8], ptr @.json_false, i64 0, i64 0)\n"
                    ));
                    (rendered, false)
                }
                IrType::Double | IrType::Decimal => {
                    let rendered = format!("%json_{}_{}_double", sanitize(type_name), field.index);
                    self.body.push_str(&format!(
                        "  {rendered} = call ptr @glitch_double_to_string(double {field_value})\n"
                    ));
                    (rendered, true)
                }
                IrType::Byte | IrType::Short | IrType::Int | IrType::Long | IrType::UInt => {
                    let wide = if field_type == LlType::I64 {
                        field_value.clone()
                    } else {
                        let wide = format!("%json_{}_{}_wide", sanitize(type_name), field.index);
                        let extension = if matches!(field.ty, IrType::Byte | IrType::UInt) {
                            "zext"
                        } else {
                            "sext"
                        };
                        self.body.push_str(&format!(
                            "  {wide} = {extension} {} {field_value} to i64\n",
                            field_type.as_ir()
                        ));
                        wide
                    };
                    let rendered = format!("%json_{}_{}_integer", sanitize(type_name), field.index);
                    self.body.push_str(&format!(
                        "  {rendered} = call ptr @glitch_i64_to_string(i64 {wide})\n"
                    ));
                    (rendered, true)
                }
                _ => {
                    return Err(format!(
                        "LLVM TIR backend: JSON result field '{type_name}.{field_name}' has unsupported type {:?}",
                        field.ty
                    ));
                }
            };
            let next = format!("%json_{}_{}_value", sanitize(type_name), field.index);
            self.body.push_str(&format!(
                "  {next} = call ptr @glitch_string_concat(ptr {with_key}, ptr {rendered})\n  call void @glitch_string_release(ptr {with_key})\n"
            ));
            if rendered_is_managed {
                self.body.push_str(&format!(
                    "  call void @glitch_string_release(ptr {rendered})\n"
                ));
            }
            current = next;
            current_is_managed = true;
        }
        let close = self.string_global("}");
        let result = format!("%json_{}_result", sanitize(type_name));
        self.body.push_str(&format!(
            "  {result} = call ptr @glitch_string_concat(ptr {current}, ptr {close})\n"
        ));
        if current_is_managed {
            self.body.push_str(&format!(
                "  call void @glitch_string_release(ptr {current})\n"
            ));
        }
        Ok(result)
    }

    fn emit_endpoint_body_object(
        &mut self,
        type_name: &str,
        body: &str,
        prefix: &str,
    ) -> Result<String, String> {
        let object = self.object_types.get(type_name).cloned().ok_or_else(|| {
            format!("LLVM TIR backend: body type '{type_name}' has no object layout")
        })?;
        let value = self.emit_endpoint_object_allocation(type_name, prefix)?;
        let llvm_name = llvm_object_name(type_name);
        for (field_name, field) in &object.fields {
            let token = self.string_global(&format!("\"{field_name}\""));
            let token_length = field_name.len() + 2;
            let field_ptr = format!("%{prefix}_field_{}_ptr", sanitize(field_name));
            self.body.push_str(&format!(
                "  {field_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 {}\n",
                field.index
            ));
            match &field.ty {
                IrType::String => {
                    let field_value = format!("%{prefix}_field_{}", sanitize(field_name));
                    self.body.push_str(&format!(
                        "  {field_value} = call ptr @glitch_json_value_string(ptr {body}, ptr {token}, i64 {token_length})\n  store ptr {field_value}, ptr {field_ptr}\n"
                    ));
                }
                IrType::Bool => {
                    let field_value = format!("%{prefix}_field_{}", sanitize(field_name));
                    self.body.push_str(&format!(
                        "  {field_value} = call i1 @glitch_json_value_bool(ptr {body}, ptr {token}, i64 {token_length})\n  store i1 {field_value}, ptr {field_ptr}\n"
                    ));
                }
                IrType::Byte | IrType::Short | IrType::Int | IrType::Long | IrType::UInt => {
                    let raw = format!("%{prefix}_field_{}_i64", sanitize(field_name));
                    let field_type = llvm_ir_type(&field.ty);
                    let field_value = format!("%{prefix}_field_{}", sanitize(field_name));
                    self.body.push_str(&format!(
                        "  {raw} = call i64 @glitch_json_value_i64(ptr {body}, ptr {token}, i64 {token_length})\n"
                    ));
                    if field_type == LlType::I64 {
                        self.body
                            .push_str(&format!("  store i64 {raw}, ptr {field_ptr}\n"));
                    } else {
                        self.body.push_str(&format!(
                            "  {field_value} = trunc i64 {raw} to {}\n  store {} {field_value}, ptr {field_ptr}\n",
                            field_type.as_ir(),
                            field_type.as_ir()
                        ));
                    }
                }
                IrType::Double | IrType::Decimal => {
                    let field_value = format!("%{prefix}_field_{}", sanitize(field_name));
                    self.body.push_str(&format!(
                        "  {field_value} = call double @glitch_json_value_double(ptr {body}, ptr {token}, i64 {token_length})\n  store double {field_value}, ptr {field_ptr}\n"
                    ));
                }
                _ => {
                    return Err(format!(
                        "LLVM TIR backend: JSON field '{type_name}.{field_name}' has unsupported type {:?}",
                        field.ty
                    ));
                }
            }
        }
        Ok(value)
    }

    fn can_allocate_endpoint_dependency(&self, ty: &IrType) -> bool {
        self.can_allocate_endpoint_dependency_inner(ty, &mut std::collections::HashSet::new())
    }

    fn can_allocate_endpoint_dependency_inner(
        &self,
        ty: &IrType,
        visiting: &mut std::collections::HashSet<String>,
    ) -> bool {
        let Some(type_name) = self.resolve_endpoint_dependency_name(ty) else {
            return false;
        };
        if !visiting.insert(type_name.clone()) {
            return false;
        }
        let Some(object) = self.object_types.get(&type_name) else {
            visiting.remove(&type_name);
            return false;
        };
        if matches!(object.kind, TypeKind::Interface) {
            visiting.remove(&type_name);
            return false;
        }
        let supported = object
            .constructor_params
            .iter()
            .all(|param| self.can_allocate_endpoint_dependency_inner(param, visiting));
        visiting.remove(&type_name);
        supported
    }

    fn resolve_endpoint_dependency_name(&self, ty: &IrType) -> Option<String> {
        match ty {
            IrType::Class(name) | IrType::Struct(name) => {
                self.object_types.contains_key(name).then(|| name.clone())
            }
            IrType::Interface(name) => self.resolve_interface_implementation(name),
            _ => None,
        }
    }

    fn resolve_interface_implementation(&self, interface_name: &str) -> Option<String> {
        let mut candidates = self
            .object_types
            .values()
            .filter(|object| {
                matches!(object.kind, TypeKind::Class)
                    && object.bases.iter().any(|base| {
                        base == interface_name
                            || base_type_name(base) == base_type_name(interface_name)
                    })
            })
            .map(|object| object.name.clone())
            .collect::<Vec<_>>();
        candidates.sort();
        candidates.dedup();
        (candidates.len() == 1).then(|| candidates.remove(0))
    }

    fn resolve_interface_method_symbol(
        &self,
        interface_name: &str,
        method_name: &str,
        arg_count: usize,
    ) -> Option<String> {
        let implementation = self.resolve_interface_implementation(interface_name)?;
        let prefix = format!("{}_{}", sanitize(&implementation), sanitize(method_name));
        let mut candidates = self
            .functions
            .iter()
            .filter(|(symbol, signature)| {
                (symbol.as_str() == prefix || symbol.starts_with(&format!("{prefix}__")))
                    && signature.params.len() == arg_count + 1
            })
            .map(|(symbol, _)| symbol.clone())
            .collect::<Vec<_>>();
        candidates.sort();
        candidates.dedup();
        (candidates.len() == 1).then(|| candidates.remove(0))
    }

    fn emit_endpoint_object_allocation(
        &mut self,
        type_name: &str,
        prefix: &str,
    ) -> Result<String, String> {
        self.emit_endpoint_object_allocation_inner(
            type_name,
            prefix,
            &mut std::collections::HashSet::new(),
        )
    }

    fn emit_endpoint_object_allocation_inner(
        &mut self,
        type_name: &str,
        prefix: &str,
        visiting: &mut std::collections::HashSet<String>,
    ) -> Result<String, String> {
        if !visiting.insert(type_name.to_string()) {
            return Err(format!(
                "LLVM TIR backend: dependency cycle while activating '{type_name}'"
            ));
        }
        let object = self.object_types.get(type_name).cloned().ok_or_else(|| {
            format!("LLVM TIR backend: dependency type '{type_name}' has no layout")
        })?;
        let llvm_name = llvm_object_name(type_name);
        let size_ptr = format!("%{prefix}_size_ptr");
        let size = format!("%{prefix}_size");
        let value = format!("%{prefix}");
        self.body.push_str(&format!(
            "  {size_ptr} = getelementptr %{llvm_name}, ptr null, i32 1\n  {size} = ptrtoint ptr {size_ptr} to i64\n  {value} = call ptr @glitch_calloc(i64 1, i64 {size})\n"
        ));
        if matches!(object.kind, TypeKind::Class) {
            let rc_ptr = format!("%{prefix}_rc_ptr");
            let drop_ptr = format!("%{prefix}_drop_ptr");
            self.body.push_str(&format!(
                "  {rc_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 0\n  store i64 1, ptr {rc_ptr}\n  {drop_ptr} = getelementptr inbounds %{llvm_name}, ptr {value}, i32 0, i32 1\n  store ptr @{}, ptr {drop_ptr}\n",
                destroy_symbol(type_name)
            ));
        }
        let mut dependencies = Vec::new();
        for (index, dependency_type) in object.constructor_params.iter().enumerate() {
            let dependency_name = self
                .resolve_endpoint_dependency_name(dependency_type)
                .ok_or_else(|| {
                    format!(
                        "LLVM TIR backend: cannot resolve dependency {dependency_type:?} for '{type_name}'"
                    )
                })?;
            let dependency_value = self.emit_endpoint_object_allocation_inner(
                &dependency_name,
                &format!("{prefix}_dependency_{index}"),
                visiting,
            )?;
            dependencies.push((dependency_name, dependency_value));
        }
        if let Some(constructor) = &object.constructor {
            let mut args = vec![format!("ptr {value}")];
            args.extend(
                dependencies
                    .iter()
                    .map(|(_, dependency)| format!("ptr {dependency}")),
            );
            self.body.push_str(&format!(
                "  call void @{}({})\n",
                sanitize(constructor),
                args.join(", ")
            ));
        }
        for (dependency_name, dependency_value) in dependencies {
            self.body.push_str(&format!(
                "  call void @{}(ptr {dependency_value})\n",
                drop_symbol(&dependency_name)
            ));
        }
        visiting.remove(type_name);
        Ok(value)
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
            LlType::I1 | LlType::I8 | LlType::I16 => {
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
                "  {live} = load i64, ptr @glitch_live_allocations\n  {leaked} = icmp ne i64 {live}, 0\n  {exception} = load ptr, ptr @glitch_exception_pending\n  {has_exception} = icmp ne ptr {exception}, null\n  {failed} = or i1 {leaked}, {has_exception}\n  {code} = zext i1 {failed} to i32\n  {report_env} = call ptr @getenv(ptr @.env_report_leaks)\n  {should_report} = icmp ne ptr {report_env}, null\n  br i1 {should_report}, label %{report_label}, label %{return_label}\n{report_label}:\n  call i32 (ptr, ...) @printf(ptr {}, i64 {live})\n  br label %{return_label}\n{return_label}:\n  ret i32 {code}\n",
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
        if target == &LlType::Ptr && (value.ty.is_integer() || value.ty == LlType::Double) {
            return Ok(LlValue {
                value: "null".to_string(),
                ty: LlType::Ptr,
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
            "@.str.{id} = private unnamed_addr constant {{ i64, i64, [{len} x i8] }} {{ i64 1000000000, i64 {len_without_null}, [{len} x i8] c\"{bytes}\\00\" }}\n"
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

fn llvm_type(ty: &TypeSyntax) -> LlType {
    match ty {
        TypeSyntax::Scalar(ScalarType::Bool) => LlType::I1,
        TypeSyntax::Scalar(ScalarType::Byte) => LlType::I8,
        TypeSyntax::Scalar(ScalarType::Short) => LlType::I16,
        TypeSyntax::Scalar(ScalarType::I32) => LlType::I32,
        TypeSyntax::Scalar(ScalarType::I64) => LlType::I64,
        TypeSyntax::Scalar(ScalarType::U32) => LlType::I32,
        TypeSyntax::Scalar(ScalarType::F64 | ScalarType::Decimal) => LlType::Double,
        TypeSyntax::String => LlType::Ptr,
        TypeSyntax::Ref(_) => LlType::Ptr,
        TypeSyntax::GenericNamed { .. } => LlType::Ptr,
        TypeSyntax::Void => LlType::Void,
        TypeSyntax::Nullable(inner) => llvm_type(inner),
        _ => LlType::Ptr,
    }
}

fn llvm_ir_type(ty: &IrType) -> LlType {
    match ty {
        IrType::Bool => LlType::I1,
        IrType::Byte => LlType::I8,
        IrType::Short => LlType::I16,
        IrType::Int | IrType::UInt => LlType::I32,
        IrType::Long => LlType::I64,
        IrType::Double | IrType::Decimal => LlType::Double,
        IrType::Void => LlType::Void,
        IrType::String
        | IrType::Array(_)
        | IrType::Ref(_)
        | IrType::Weak(_)
        | IrType::Struct(_)
        | IrType::Class(_)
        | IrType::Interface(_)
        | IrType::List(_)
        | IrType::Dictionary(_, _)
        | IrType::Enumerable(_)
        | IrType::Thread
        | IrType::Task(_)
        | IrType::Function { .. }
        | IrType::Exception
        | IrType::Unknown(_) => LlType::Ptr,
    }
}

fn object_type_name(ty: &IrType) -> Option<&str> {
    match ty {
        IrType::Struct(name) | IrType::Class(name) | IrType::Interface(name) => Some(name),
        IrType::Ref(inner) => object_type_name(inner),
        _ => None,
    }
}

fn endpoint_parameter_supported(param: &EndpointParameterBinding, route: &str) -> bool {
    match (&param.source, &param.ty) {
        (EndpointParameterSource::Route, IrType::Int | IrType::Long | IrType::String) => {
            route_parameter_segment(route, &param.name).is_some()
        }
        (EndpointParameterSource::Body, IrType::String) => true,
        (
            EndpointParameterSource::Query,
            IrType::Bool | IrType::Int | IrType::Long | IrType::String,
        ) => true,
        _ => false,
    }
}

fn route_parameter_segment(route: &str, parameter: &str) -> Option<usize> {
    route
        .split('/')
        .filter(|segment| !segment.is_empty())
        .enumerate()
        .find_map(|(index, segment)| {
            let inner = segment.strip_prefix('{')?.strip_suffix('}')?;
            let name = inner.split(':').next().unwrap_or(inner);
            (name == parameter).then_some(index)
        })
}

fn base_type_name(name: &str) -> &str {
    name.split('<')
        .next()
        .unwrap_or(name)
        .rsplit('.')
        .next()
        .unwrap_or(name)
}

fn llvm_object_name(name: &str) -> String {
    format!("glitch.{}", sanitize(name))
}

fn retain_symbol(name: &str) -> String {
    format!("glitch_retain_{}", sanitize(name))
}

fn drop_symbol(name: &str) -> String {
    format!("glitch_drop_{}", sanitize(name))
}

fn destroy_symbol(name: &str) -> String {
    format!("glitch_destroy_{}", sanitize(name))
}

fn expr_source_name(expr: &TypedExpr) -> Option<&str> {
    match &expr.kind {
        TypedExprKind::Var(name) | TypedExprKind::Move(name) => Some(name),
        _ => None,
    }
}

fn collect_rc_instantiations_program(program: &TypedProgram, out: &mut HashSet<String>) {
    for function in &program.functions {
        collect_rc_instantiations_function(function, out);
    }
    for ty in &program.types {
        for field in &ty.fields {
            collect_rc_type(&field.ty, out);
        }
        for constructor in &ty.constructors {
            collect_rc_instantiations_function(constructor, out);
        }
        for method in &ty.methods {
            collect_rc_instantiations_function(method, out);
        }
    }
    for endpoint in &program.endpoint_handlers {
        collect_rc_type(&endpoint.return_type, out);
        collect_rc_type(&endpoint.response_type, out);
        for param in &endpoint.params {
            collect_rc_type(&param.ty, out);
        }
    }
}

fn collect_rc_instantiations_function(function: &TypedFunction, out: &mut HashSet<String>) {
    collect_rc_type(&function.return_type, out);
    for param in &function.params {
        collect_rc_type(&param.ty, out);
    }
    for local in &function.locals {
        collect_rc_type(&local.ty, out);
    }
    collect_rc_stmts(&function.body, out);
}

fn collect_rc_stmts(stmts: &[TypedStmt], out: &mut HashSet<String>) {
    for stmt in stmts {
        match &stmt.kind {
            TypedStmtKind::Let { binding, expr } => {
                collect_rc_type(&binding.ty, out);
                collect_rc_expr(expr, out);
            }
            TypedStmtKind::Assign { expr, .. } | TypedStmtKind::Print(expr) | TypedStmtKind::Expr(expr) => {
                collect_rc_expr(expr, out);
            }
            TypedStmtKind::AssignTarget { target, expr } => {
                collect_rc_expr(target, out);
                collect_rc_expr(expr, out);
            }
            TypedStmtKind::Block(body) | TypedStmtKind::While { body, .. } => {
                collect_rc_stmts(body, out);
            }
            TypedStmtKind::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_rc_expr(condition, out);
                collect_rc_stmts(then_body, out);
                collect_rc_stmts(else_body, out);
            }
            TypedStmtKind::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                collect_rc_stmts(try_body, out);
                collect_rc_stmts(catch_body, out);
                collect_rc_stmts(finally_body, out);
            }
            TypedStmtKind::Switch {
                expr,
                cases,
                default,
            } => {
                collect_rc_expr(expr, out);
                for case in cases {
                    collect_rc_expr(&case.value, out);
                    collect_rc_stmts(&case.body, out);
                }
                collect_rc_stmts(default, out);
            }
            TypedStmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                if let Some(init) = init {
                    collect_rc_stmts(std::slice::from_ref(init), out);
                }
                if let Some(condition) = condition {
                    collect_rc_expr(condition, out);
                }
                if let Some(increment) = increment {
                    collect_rc_stmts(std::slice::from_ref(increment), out);
                }
                collect_rc_stmts(body, out);
            }
            TypedStmtKind::ForEach { collection, body, .. } => {
                collect_rc_expr(collection, out);
                collect_rc_stmts(body, out);
            }
            TypedStmtKind::Return(Some(expr)) | TypedStmtKind::Throw(expr) => {
                collect_rc_expr(expr, out);
            }
            TypedStmtKind::Return(None) | TypedStmtKind::Break | TypedStmtKind::Continue => {}
        }
    }
}

fn collect_rc_expr(expr: &TypedExpr, out: &mut HashSet<String>) {
    collect_rc_type(&expr.ty, out);
    if let TypedExprKind::NewObject { type_name, .. } = &expr.kind {
        if type_name.starts_with("Rc_") {
            out.insert(type_name.clone());
        }
    }
    match &expr.kind {
        TypedExprKind::ArrayLiteral(values) => {
            for value in values {
                collect_rc_expr(value, out);
            }
        }
        TypedExprKind::NewArray { values, .. } => {
            for value in values {
                collect_rc_expr(value, out);
            }
        }
        TypedExprKind::Index { target, index } => {
            collect_rc_expr(target, out);
            collect_rc_expr(index, out);
        }
        TypedExprKind::Field { target, .. } => collect_rc_expr(target, out),
        TypedExprKind::IsPattern { expr, .. } => collect_rc_expr(expr, out),
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Method { target, .. } => collect_rc_expr(target, out),
                TypedCallKind::Function { .. } => {}
            }
            for arg in &call.args {
                collect_rc_expr(arg, out);
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_rc_expr(arg, out);
            }
            for field in fields {
                collect_rc_expr(&field.expr, out);
            }
        }
        TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_)
        | TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::Var(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::Move(_)
        | TypedExprKind::Borrow { .. } => {}
        TypedExprKind::Await(inner)
        | TypedExprKind::Unary { expr: inner, .. }
        | TypedExprKind::IncDec { target: inner, .. } => collect_rc_expr(inner, out),
        TypedExprKind::Lambda { body, .. } => collect_rc_expr(body, out),
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_rc_expr(condition, out);
            collect_rc_expr(when_true, out);
            collect_rc_expr(when_false, out);
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_rc_expr(left, out);
            collect_rc_expr(right, out);
        }
    }
}

fn collect_rc_type(ty: &IrType, out: &mut HashSet<String>) {
    match ty {
        IrType::Struct(name) if name.starts_with("Rc_") => {
            out.insert(name.clone());
        }
        IrType::Array(inner)
        | IrType::Ref(inner)
        | IrType::List(inner)
        | IrType::Enumerable(inner)
        | IrType::Task(inner)
        | IrType::Weak(inner) => collect_rc_type(inner, out),
        IrType::Dictionary(key, value) => {
            collect_rc_type(key, out);
            collect_rc_type(value, out);
        }
        IrType::Function { params, return_type } => {
            for ty in params {
                collect_rc_type(ty, out);
            }
            collect_rc_type(return_type, out);
        }
        _ => {}
    }
}

fn parse_monomorphized_rc_inner_type(
    text: &str,
    known_objects: &HashMap<String, LlObjectType>,
) -> Option<IrType> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    match text {
        "bool" => Some(IrType::Bool),
        "byte" => Some(IrType::Byte),
        "short" => Some(IrType::Short),
        "int" => Some(IrType::Int),
        "long" => Some(IrType::Long),
        "uint" => Some(IrType::UInt),
        "double" => Some(IrType::Double),
        "decimal" => Some(IrType::Decimal),
        "string" => Some(IrType::String),
        "void" => Some(IrType::Void),
        "Exception" | "System.Exception" => Some(IrType::Exception),
        _ => {
            if let Some((base, args)) = split_monomorphized_type(text) {
                let base_name = base_type_name(base);
                match base_name {
                    "List" if args.len() == 1 => Some(IrType::List(Box::new(
                        parse_monomorphized_rc_inner_type(&args[0], known_objects)?,
                    ))),
                    "Dictionary" if args.len() == 2 => Some(IrType::Dictionary(
                        Box::new(parse_monomorphized_rc_inner_type(&args[0], known_objects)?),
                        Box::new(parse_monomorphized_rc_inner_type(&args[1], known_objects)?),
                    )),
                    "IEnumerable" | "ICollection" if args.len() == 1 => Some(IrType::Enumerable(
                        Box::new(parse_monomorphized_rc_inner_type(&args[0], known_objects)?),
                    )),
                    "Task" | "ValueTask" if args.len() == 1 => Some(IrType::Task(Box::new(
                        parse_monomorphized_rc_inner_type(&args[0], known_objects)?,
                    ))),
                    "Weak" | "WeakReference" if args.len() == 1 => Some(IrType::Weak(Box::new(
                        parse_monomorphized_rc_inner_type(&args[0], known_objects)?,
                    ))),
                    _ => resolve_known_object_type(base_name, known_objects)
                        .or_else(|| Some(IrType::Unknown(text.to_string()))),
                }
            } else {
                resolve_known_object_type(base_type_name(text), known_objects)
                    .or_else(|| Some(IrType::Unknown(text.to_string())))
            }
        }
    }
}

fn resolve_known_object_type(
    name: &str,
    known_objects: &HashMap<String, LlObjectType>,
) -> Option<IrType> {
    known_objects
        .get(name)
        .or_else(|| known_objects.get(name.rsplit('.').next().unwrap_or(name)))
        .map(|object| match object.kind {
            TypeKind::Class => IrType::Class(object.name.clone()),
            TypeKind::Interface => IrType::Interface(object.name.clone()),
            TypeKind::Enum => IrType::Int,
            TypeKind::Struct | TypeKind::RefStruct => IrType::Struct(object.name.clone()),
        })
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

fn void_value() -> LlValue {
    LlValue {
        value: "0".to_string(),
        ty: LlType::I32,
    }
}

fn integer_bits(ty: &LlType) -> u8 {
    match ty {
        LlType::I1 => 1,
        LlType::I8 => 8,
        LlType::I16 => 16,
        LlType::I32 => 32,
        LlType::I64 => 64,
        _ => 0,
    }
}

fn fmt_ptr(kind: &str) -> &'static str {
    match kind {
        "i64" => "getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0)",
        "double" => "getelementptr inbounds ([4 x i8], ptr @.fmt_double, i64 0, i64 0)",
        "str" => "getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0)",
        _ => "getelementptr inbounds ([4 x i8], ptr @.fmt_i32, i64 0, i64 0)",
    }
}

fn sanitize(name: &str) -> String {
    let mut result = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            result.push(ch);
        } else {
            result.push('_');
        }
    }
    if result.is_empty() || result.chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
        result.insert(0, '_');
    }
    result
}

fn escaped_bytes(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        match byte {
            b'\\' => out.push_str("\\5C"),
            b'"' => out.push_str("\\22"),
            b'\n' => out.push_str("\\0A"),
            b'\r' => out.push_str("\\0D"),
            b'\t' => out.push_str("\\09"),
            32..=126 => out.push(byte as char),
            other => out.push_str(&format!("\\{other:02X}")),
        }
    }
    out
}

/// Walk a `TypedExpr` tree and collect all `Var` references that are:
/// - NOT in `lambda_params` (i.e. they are not local to the lambda), and
/// - ARE present in the enclosing scope's `vars` map (i.e. they are captured).
///
/// Results are appended to `out` in encounter order; duplicates are excluded
/// via the `seen` set.
fn collect_free_vars_expr(
    expr: &TypedExpr,
    lambda_params: &[String],
    scope: &HashMap<String, LlVar>,
    seen: &mut std::collections::HashSet<String>,
    out: &mut Vec<(String, LlVar)>,
) {
    match &expr.kind {
        TypedExprKind::Var(name)
        | TypedExprKind::Move(name)
        | TypedExprKind::Borrow { name, .. } => {
            if !lambda_params.contains(name) {
                if let Some(var) = scope.get(name) {
                    if seen.insert(name.clone()) {
                        out.push((name.clone(), var.clone()));
                    }
                }
            }
        }
        TypedExprKind::IncDec { target, .. } => {
            collect_free_vars_expr(target, lambda_params, scope, seen, out);
        }
        TypedExprKind::Lambda {
            params: inner_params,
            body,
        } => {
            // Don't descend into nested lambdas — their parameters shadow the
            // outer scope but their free vars are resolved when they are lifted.
            let merged_params: Vec<String> = lambda_params
                .iter()
                .chain(inner_params.iter())
                .cloned()
                .collect();
            collect_free_vars_expr(body, &merged_params, scope, seen, out);
        }
        TypedExprKind::Field { target, .. } | TypedExprKind::Await(target) => {
            collect_free_vars_expr(target, lambda_params, scope, seen, out);
        }
        TypedExprKind::IsPattern { expr, .. } => {
            collect_free_vars_expr(expr, lambda_params, scope, seen, out);
        }
        TypedExprKind::Unary { expr, .. } => {
            collect_free_vars_expr(expr, lambda_params, scope, seen, out);
        }
        TypedExprKind::Binary { left, right, .. } => {
            collect_free_vars_expr(left, lambda_params, scope, seen, out);
            collect_free_vars_expr(right, lambda_params, scope, seen, out);
        }
        TypedExprKind::Conditional {
            condition,
            when_true,
            when_false,
        } => {
            collect_free_vars_expr(condition, lambda_params, scope, seen, out);
            collect_free_vars_expr(when_true, lambda_params, scope, seen, out);
            collect_free_vars_expr(when_false, lambda_params, scope, seen, out);
        }
        TypedExprKind::Index { target, index } => {
            collect_free_vars_expr(target, lambda_params, scope, seen, out);
            collect_free_vars_expr(index, lambda_params, scope, seen, out);
        }
        TypedExprKind::Call(call) => {
            match &call.kind {
                TypedCallKind::Method { target, .. } => {
                    collect_free_vars_expr(target, lambda_params, scope, seen, out);
                }
                TypedCallKind::Function { .. } => {}
            }
            for arg in &call.args {
                collect_free_vars_expr(arg, lambda_params, scope, seen, out);
            }
        }
        TypedExprKind::NewObject { args, fields, .. } => {
            for arg in args {
                collect_free_vars_expr(arg, lambda_params, scope, seen, out);
            }
            for f in fields {
                collect_free_vars_expr(&f.expr, lambda_params, scope, seen, out);
            }
        }
        TypedExprKind::ArrayLiteral(elements) => {
            for e in elements {
                collect_free_vars_expr(e, lambda_params, scope, seen, out);
            }
        }
        TypedExprKind::NewArray { values, .. } => {
            for v in values {
                collect_free_vars_expr(v, lambda_params, scope, seen, out);
            }
        }
        // Leaf nodes with no sub-expressions.
        TypedExprKind::Int(_)
        | TypedExprKind::Float(_)
        | TypedExprKind::Bool(_)
        | TypedExprKind::Null
        | TypedExprKind::String(_)
        | TypedExprKind::FunctionSymbol(_)
        | TypedExprKind::NewCollection(_)
        | TypedExprKind::NewThread(_) => {}
    }
}
