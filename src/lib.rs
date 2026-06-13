mod ast;
mod borrowck;
mod bytecode;
#[cfg(test)]
mod codegen;
mod cycles;
mod diagnostics;
mod leak;
mod lexer;
mod linker;
mod llvm;
mod parser;
mod runtime;
mod xunit_runner;
mod tir;
mod toolchain;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use ast::*;
use borrowck::BorrowChecker;
use bytecode::BytecodeEmitter;
#[cfg(test)]
use codegen::Codegen;
use diagnostics::CompatibilityAnalyzer;
use leak::LeakAnalyzer;
use lexer::Lexer;
use linker::{link_package_sources, strip_utf8_bom};
use llvm::LlvmEmitter;
use parser::Parser;
use tir::TypedProgram;
use toolchain::{emit_llvm_bitcode, emit_native_executable};

pub fn run_cli() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or_else(|| {
        "usage: glitchc <input.{gl,cs}> [--emit-bytecode <output.gbc>] [--emit-llvm-ir <output.ll>] [--emit-llvm-bc <output.bc>] [--emit-exe <output.exe>] [--emit-leak-report <output.txt>] [--emit-nuget <output.nupkg>]"
            .to_string()
    })?;
    let mut bytecode_output = None;
    let mut llvm_ir_output = None;
    let mut llvm_bc_output = None;
    let mut exe_output = None;
    let mut leak_report_output = None;
    let mut nuget_output = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--emit-bytecode" => bytecode_output = args.next(),
            "--emit-llvm-ir" => llvm_ir_output = args.next(),
            "--emit-llvm-bc" => llvm_bc_output = args.next(),
            "--emit-exe" => exe_output = args.next(),
            "--emit-leak-report" => leak_report_output = args.next(),
            "--emit-nuget" => nuget_output = args.next(),
            "--package-id" | "--package-version" => {
                args.next();
            }
            other => return Err(format!("unknown argument '{other}'")),
        }
    }
    let source = read_input_source(&input)?;
    let wants_llvm = llvm_ir_output.is_some() || llvm_bc_output.is_some() || exe_output.is_some();
    let has_explicit_output = bytecode_output.is_some()
        || llvm_ir_output.is_some()
        || llvm_bc_output.is_some()
        || exe_output.is_some()
        || leak_report_output.is_some()
        || nuget_output.is_some();
    let default_exe_output = if has_explicit_output {
        None
    } else {
        Some(default_executable_output_path(&input))
    };
    let wants_llvm = wants_llvm || default_exe_output.is_some();
    let compiled = compile_source_with_options(&source, wants_llvm, false)?;
    for diagnostic in &compiled.diagnostics {
        eprintln!("{diagnostic}");
    }
    if let Some(path) = &bytecode_output {
        fs::write(&path, &compiled.bytecode).map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if let Some(path) = &llvm_ir_output {
        fs::write(&path, compiled.llvm_ir()?)
            .map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if let Some(path) = &llvm_bc_output {
        emit_llvm_bitcode(compiled.llvm_ir()?, path)?;
    }
    if let Some(path) = &exe_output {
        emit_native_executable(compiled.llvm_ir()?, &compiled.native_sources, path)?;
    }
    if let Some(path) = &leak_report_output {
        fs::write(&path, &compiled.leak_report)
            .map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if let Some(path) = &nuget_output {
        return Err(format!(
            "NuGet package emission is temporarily disabled until the package format is redesigned without the legacy C backend: {path}"
        ));
    }
    if let Some(path) = default_exe_output {
        emit_native_executable(compiled.llvm_ir()?, &compiled.native_sources, path.to_string_lossy().as_ref())?;
    }
    Ok(())
}

fn default_executable_output_path(input: &str) -> PathBuf {
    let path = Path::new(input);
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        path.with_file_name(format!("{stem}.exe"))
    } else {
        PathBuf::from("glitchc.exe")
    }
}

fn read_input_source(input: &str) -> Result<String, String> {
    let path = Path::new(input);
    if path.is_dir() {
        let mut files = Vec::new();
        collect_source_files(path, &mut files)?;
        files.sort();
        let mut source = String::new();
        for file in files {
            let text = fs::read_to_string(&file)
                .map_err(|e| format!("failed to read {}: {e}", file.display()))?;
            source.push_str(&format!("// __FILE_PATH__: {}\n", file.display()));
            source.push_str(strip_utf8_bom(&text));
            source.push_str("\n__FILE_BOUNDARY__;\n");
        }
        return Ok(source);
    }
    let text = fs::read_to_string(path).map_err(|e| format!("failed to read {input}: {e}"))?;
    Ok(format!(
        "// __FILE_PATH__: {}\n{}\n__FILE_BOUNDARY__;\n",
        path.display(),
        strip_utf8_bom(&text)
    ))
}

fn collect_source_files(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in
        fs::read_dir(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?
    {
        let entry = entry.map_err(|e| format!("failed to read {} entry: {e}", path.display()))?;
        let path = entry.path();
        if path.is_dir() {
            let ignored = path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| matches!(name, "obj" | "bin" | "target" | ".git"));
            if ignored {
                continue;
            }
            collect_source_files(&path, output)?;
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| matches!(ext, "gl" | "cs"))
        {
            output.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
fn compile_source(source: &str) -> Result<String, String> {
    Ok(compile_source_with_metadata(source)?.c()?.to_string())
}

#[cfg(test)]
fn compile_bytecode(source: &str) -> Result<String, String> {
    Ok(compile_source_with_metadata(source)?.bytecode)
}

#[cfg(test)]
fn compile_llvm_ir(source: &str) -> Result<String, String> {
    Ok(compile_source_with_options(source, true, false)?
        .llvm_ir()?
        .to_string())
}

#[cfg(test)]
fn compile_leak_report(source: &str) -> Result<String, String> {
    Ok(compile_source_with_metadata(source)?.leak_report)
}

#[cfg(test)]
fn compile_ownership_summary(source: &str) -> Result<String, String> {
    let linked_source = link_package_sources(strip_utf8_bom(source))?;
    let tokens = Lexer::new(&linked_source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    let typed = TypedProgram::lower(&program)?;
    Ok(typed.ownership_summary())
}

struct CompileOutput {
    #[cfg(test)]
    c: Option<String>,
    bytecode: String,
    llvm_ir: Option<String>,
    leak_report: String,
    diagnostics: Vec<String>,
    native_sources: Vec<PathBuf>,
    #[allow(dead_code)]
    package_id: Option<String>,
}

impl CompileOutput {
    #[cfg(test)]
    fn c(&self) -> Result<&str, String> {
        self.c
            .as_deref()
            .ok_or_else(|| "C output was not requested".to_string())
    }

    fn llvm_ir(&self) -> Result<&str, String> {
        self.llvm_ir
            .as_deref()
            .ok_or_else(|| "LLVM IR was not requested".to_string())
    }
}

#[cfg(test)]
fn compile_source_with_metadata(source: &str) -> Result<CompileOutput, String> {
    compile_source_with_options(source, false, true)
}

fn compile_source_with_options(
    source: &str,
    emit_llvm: bool,
    emit_c: bool,
) -> Result<CompileOutput, String> {
    let _ = emit_c;
    let linked_source = link_package_sources(strip_utf8_bom(source))?;
    let _ = fs::write("target/linked_source.gl", &linked_source);
    let tokens = Lexer::new(&linked_source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    validate_generic_constraints(&program)?;
    BorrowChecker::check_program(&program)?;
    let cycle_warnings = cycles::check_reference_cycles(source, &program);
    let typed = TypedProgram::lower(&program)?;
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
    #[cfg(test)]
    let c = if emit_c {
        Some(Codegen::new(&program, &typed).emit_program(&program)?)
    } else {
        None
    };
    let native_sources = linker::find_package_native_sources(&linked_source);
    Ok(CompileOutput {
        #[cfg(test)]
        c,
        bytecode,
        llvm_ir,
        leak_report,
        diagnostics,
        native_sources,
        package_id: program.package_id.clone(),
    })
}

fn validate_generic_constraints(program: &Program) -> Result<(), String> {
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

fn validate_generic_param_constraints(
    context: &str,
    params: &[GenericParam],
    known_types: &HashMap<String, TypeKind>,
) -> Result<(), String> {
    for param in params {
        let has_class = param
            .constraints
            .iter()
            .any(|constraint| constraint == "class");
        let has_struct = param
            .constraints
            .iter()
            .any(|constraint| constraint == "struct");
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

#[cfg(test)]
mod tests;
