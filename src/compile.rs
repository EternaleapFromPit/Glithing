use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::*;
use crate::borrowck::BorrowChecker;
use crate::bytecode::BytecodeEmitter;
use crate::diagnostics::CompatibilityAnalyzer;
use crate::leak::LeakAnalyzer;
use crate::lexer::Lexer;
use crate::linker::{link_package_sources, strip_utf8_bom};
use crate::llvm::LlvmEmitter;
use crate::nuget::{emit_nuget_package, NugetPackageSpec};
use crate::parser::Parser;
use crate::project::{
    collect_publishable_project_files, ensure_parent_dir, load_project_spec_for_input, ProjectSpec,
};
use crate::tir::TypedProgram;
use crate::toolchain::{emit_llvm_bitcode, emit_native_executable};
use crate::workspace::{read_input_source, run_on_large_stack};

pub(crate) struct CompileRequest {
    pub(crate) bytecode_output: Option<String>,
    pub(crate) llvm_ir_output: Option<String>,
    pub(crate) llvm_bc_output: Option<String>,
    pub(crate) exe_output: Option<String>,
    pub(crate) leak_report_output: Option<String>,
    pub(crate) nuget_output: Option<String>,
    pub(crate) package_id: Option<String>,
    pub(crate) package_version: Option<String>,
    pub(crate) default_native_output: bool,
}

#[derive(Debug)]
pub(crate) struct CompileOutput {
    pub(crate) bytecode: String,
    pub(crate) llvm_ir: Option<String>,
    pub(crate) leak_report: String,
    pub(crate) diagnostics: Vec<String>,
    pub(crate) linked_source: String,
    #[allow(dead_code)]
    pub(crate) package_id: Option<String>,
}

impl CompileOutput {
    pub(crate) fn llvm_ir(&self) -> Result<&str, String> {
        self.llvm_ir
            .as_deref()
            .ok_or_else(|| "LLVM IR was not requested".to_string())
    }
}

pub(crate) fn default_package_id(input: &str) -> String {
    if let Ok(Some(project)) = load_project_spec_for_input(input) {
        return project.assembly_name_or_default();
    }
    let path = Path::new(input);
    path.file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Glitching.Package".to_string())
}

pub(crate) fn default_executable_output_path(input: &str) -> PathBuf {
    let path = Path::new(input);
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        path.with_file_name(format!("{stem}.exe"))
    } else {
        PathBuf::from("glitchc.exe")
    }
}

pub(crate) fn emit_compile_outputs(input: &str, request: CompileRequest) -> Result<(), String> {
    let source = read_input_source(input)?;
    let project_spec = load_project_spec_for_input(input)?;
    let wants_llvm = request.llvm_ir_output.is_some()
        || request.llvm_bc_output.is_some()
        || request.exe_output.is_some()
        || request.nuget_output.is_some()
        || request.default_native_output;
    let has_explicit_output = request.bytecode_output.is_some()
        || request.llvm_ir_output.is_some()
        || request.llvm_bc_output.is_some()
        || request.exe_output.is_some()
        || request.leak_report_output.is_some()
        || request.nuget_output.is_some();
    let default_exe_output = if request.default_native_output && !has_explicit_output {
        Some(default_executable_output_path(input))
    } else {
        None
    };
    let project_for_compile = project_spec.clone();
    let compiled = run_on_large_stack(move || {
        compile_source_with_project(&source, wants_llvm, project_for_compile.as_ref())
    })?;
    for diagnostic in &compiled.diagnostics {
        eprintln!("{diagnostic}");
    }
    if let Some(path) = &request.bytecode_output {
        ensure_parent_dir(Path::new(path))?;
        fs::write(path, &compiled.bytecode).map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if let Some(path) = &request.llvm_ir_output {
        ensure_parent_dir(Path::new(path))?;
        fs::write(path, compiled.llvm_ir()?).map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if let Some(path) = &request.llvm_bc_output {
        ensure_parent_dir(Path::new(path))?;
        emit_llvm_bitcode(compiled.llvm_ir()?, path)?;
    }
    if let Some(path) = &request.exe_output {
        ensure_parent_dir(Path::new(path))?;
        emit_native_executable(compiled.llvm_ir()?, path)?;
    }
    if let Some(path) = &request.leak_report_output {
        ensure_parent_dir(Path::new(path))?;
        fs::write(path, &compiled.leak_report)
            .map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if let Some(path) = &request.nuget_output {
        ensure_parent_dir(Path::new(path))?;
        let package_id = request.package_id.unwrap_or_else(|| default_package_id(input));
        let package_version = request.package_version.unwrap_or_else(|| "0.1.0".to_string());
        let publishable_files = collect_publishable_project_files(project_spec.as_ref())?;
        let dependencies = project_spec
            .as_ref()
            .map(|project| project.package_references.as_slice())
            .unwrap_or(&[]);
        emit_nuget_package(
            NugetPackageSpec {
                package_id: &package_id,
                version: &package_version,
                linked_source: &compiled.linked_source,
                llvm_ir: compiled.llvm_ir()?,
                dependencies,
                content_files: &publishable_files,
            },
            path,
        )?;
    }
    if let Some(path) = default_exe_output {
        ensure_parent_dir(&path)?;
        emit_native_executable(compiled.llvm_ir()?, path.to_string_lossy().as_ref())?;
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn compile_bytecode(source: &str) -> Result<String, String> {
    Ok(compile_source_with_metadata(source)?.bytecode)
}

#[cfg(test)]
pub(crate) fn compile_llvm_ir(source: &str) -> Result<String, String> {
    Ok(compile_source_with_options(source, true, false)?
        .llvm_ir()?
        .to_string())
}

#[cfg(test)]
pub(crate) fn compile_llvm_ir_from_path(input: &str) -> Result<String, String> {
    let source = read_input_source(input)?;
    let project = load_project_spec_for_input(input)?;
    run_on_large_stack(move || {
        Ok(compile_source_with_project(&source, true, project.as_ref())?
            .llvm_ir()?
            .to_string())
    })
}

#[cfg(test)]
pub(crate) fn compile_leak_report(source: &str) -> Result<String, String> {
    Ok(compile_source_with_metadata(source)?.leak_report)
}

#[cfg(test)]
pub(crate) fn compile_ownership_summary(source: &str) -> Result<String, String> {
    let linked_source = link_package_sources(strip_utf8_bom(source))?;
    let tokens = Lexer::new(&linked_source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    let typed = TypedProgram::lower(&program, None)?;
    Ok(typed.ownership_summary())
}

#[cfg(test)]
pub(crate) fn compile_source_with_metadata(source: &str) -> Result<CompileOutput, String> {
    compile_source_with_options(source, false, false)
}

pub(crate) fn compile_source_with_options(
    source: &str,
    emit_llvm: bool,
    _emit_c: bool,
) -> Result<CompileOutput, String> {
    compile_source_with_project(source, emit_llvm, None)
}

pub(crate) fn compile_source_with_project(
    source: &str,
    emit_llvm: bool,
    project: Option<&ProjectSpec>,
) -> Result<CompileOutput, String> {
    let linked_source = link_package_sources(strip_utf8_bom(source))?;
    let _ = fs::write("target/linked_source.gl", &linked_source);
    let tokens = Lexer::new(&linked_source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    validate_generic_constraints(&program)?;
    BorrowChecker::check_program(&program)?;
    let cycle_warnings = crate::cycles::check_reference_cycles(source, &program);
    let typed = TypedProgram::lower(&program, project.and_then(|spec| spec.startup_object.as_deref()))?;
    TypedProgram::check_ownership(&program)?;
    let leak_report = LeakAnalyzer::analyze_program(&program, &typed);
    let mut diagnostics = CompatibilityAnalyzer::analyze(source, &program, &typed, emit_llvm);
    diagnostics.extend(cycle_warnings);
    let bytecode = BytecodeEmitter::emit_program(&program);
    let llvm_ir = if emit_llvm {
        Some(LlvmEmitter::emit_typed_program(&typed)?)
    } else {
        None
    };
    Ok(CompileOutput {
        bytecode,
        llvm_ir,
        leak_report,
        diagnostics,
        linked_source,
        package_id: program.package_id.clone(),
    })
}

pub(crate) fn validate_generic_constraints(program: &Program) -> Result<(), String> {
    let mut known_types = HashMap::<String, TypeKind>::new();
    for enum_def in &program.enums {
        known_types.insert(enum_def.name.clone(), TypeKind::Enum);
    }
    for ty in &program.types {
        known_types.insert(ty.name.clone(), ty.kind);
    }
    for ty in &program.types {
        validate_generic_param_constraints(
            &format!("type {}", ty.name),
            &ty.generic_params,
            &known_types,
        )?;
        for method in &ty.methods {
            validate_generic_param_constraints(
                &format!("method {}.{}", ty.name, method.name),
                &method.generic_params,
                &known_types,
            )?;
        }
    }
    for function in &program.functions {
        validate_generic_param_constraints(
            &format!("function {}", function.name),
            &function.generic_params,
            &known_types,
        )?;
    }
    Ok(())
}

pub(crate) fn validate_generic_param_constraints(
    context: &str,
    params: &[GenericParam],
    known_types: &HashMap<String, TypeKind>,
) -> Result<(), String> {
    for param in params {
        let has_class = param.constraints.iter().any(|constraint| constraint == "class");
        let has_struct = param.constraints.iter().any(|constraint| constraint == "struct");
        let has_unmanaged = param
            .constraints
            .iter()
            .any(|constraint| constraint == "unmanaged");
        if has_class && (has_struct || has_unmanaged) {
            return Err(format!(
                "generic constraint error in {context}: '{}' cannot combine class with struct/unmanaged",
                param.name
            ));
        }
        for constraint in &param.constraints {
            if matches!(
                constraint.as_str(),
                "class" | "struct" | "notnull" | "unmanaged" | "new()"
            ) {
                continue;
            }
            if !known_types.contains_key(constraint) {
                return Err(format!(
                    "generic constraint error in {context}: unknown constraint type '{}'",
                    constraint
                ));
            }
        }
    }
    Ok(())
}
