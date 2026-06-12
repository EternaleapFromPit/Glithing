mod ast;
mod borrowck;
mod bytecode;
mod lexer;
mod llvm;
mod parser;
mod runtime;
mod tir;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ast::*;
use borrowck::BorrowChecker;
use bytecode::BytecodeEmitter;
use lexer::Lexer;
use llvm::LlvmEmitter;
use parser::Parser;
use tir::TypedProgram;

pub fn run_cli() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or_else(|| {
        "usage: glitchc <input.{gl,cs,pl}> [--emit-c <output.c>] [--emit-bytecode <output.gbc>] [--emit-llvm-ir <output.ll>] [--emit-llvm-bc <output.bc>] [--emit-exe <output.exe>] [--emit-leak-report <output.txt>] [--emit-nuget <output.nupkg>]"
            .to_string()
    })?;
    let mut c_output = None;
    let mut bytecode_output = None;
    let mut llvm_ir_output = None;
    let mut llvm_bc_output = None;
    let mut exe_output = None;
    let mut leak_report_output = None;
    let mut nuget_output = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--emit-c" => c_output = args.next(),
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
    let has_explicit_output = c_output.is_some()
        || bytecode_output.is_some()
        || llvm_ir_output.is_some()
        || llvm_bc_output.is_some()
        || exe_output.is_some()
        || leak_report_output.is_some()
        || nuget_output.is_some();
    let wants_c = c_output.is_some() || nuget_output.is_some() || !has_explicit_output;
    let compiled = compile_source_with_options(&source, wants_llvm, wants_c)?;
    for diagnostic in &compiled.diagnostics {
        eprintln!("{diagnostic}");
    }
    if let Some(path) = &c_output {
        fs::write(&path, compiled.c()?).map_err(|e| format!("failed to write {path}: {e}"))?;
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
        emit_native_executable(compiled.llvm_ir()?, path)?;
    }
    if let Some(path) = &leak_report_output {
        fs::write(&path, &compiled.leak_report)
            .map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if let Some(path) = &nuget_output {
        fs::write(&path, compiled.c()?).map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if c_output.is_none()
        && bytecode_output.is_none()
        && llvm_ir_output.is_none()
        && llvm_bc_output.is_none()
        && exe_output.is_none()
        && leak_report_output.is_none()
        && nuget_output.is_none()
    {
        println!("{}", compiled.c()?);
    }
    Ok(())
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
            source.push_str(strip_utf8_bom(&text));
            source.push('\n');
        }
        return Ok(source);
    }
    fs::read_to_string(path).map_err(|e| format!("failed to read {input}: {e}"))
}

fn collect_source_files(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in
        fs::read_dir(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?
    {
        let entry = entry.map_err(|e| format!("failed to read {} entry: {e}", path.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_source_files(&path, output)?;
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| matches!(ext, "gl" | "cs" | "pl"))
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
    c: Option<String>,
    bytecode: String,
    llvm_ir: Option<String>,
    leak_report: String,
    diagnostics: Vec<String>,
    #[allow(dead_code)]
    package_id: Option<String>,
}

impl CompileOutput {
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
    let linked_source = link_package_sources(strip_utf8_bom(source))?;
    let tokens = Lexer::new(&linked_source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    validate_generic_constraints(&program)?;
    BorrowChecker::check_program(&program)?;
    let cycle_warnings = check_reference_cycles(source, &program);
    let typed = TypedProgram::lower(&program)?;
    TypedProgram::check_ownership(&program)?;
    let leak_report = LeakAnalyzer::analyze_program(&program, &typed);
    let mut diagnostics = CompatibilityAnalyzer::analyze(source, &program, &typed);
    diagnostics.extend(cycle_warnings);
    let bytecode = BytecodeEmitter::emit_program(&program);
    let llvm_ir = if emit_llvm {
        Some(LlvmEmitter::emit_typed_program(&typed)?)
    } else {
        None
    };
    let c = if emit_c {
        Some(Codegen::new(&program, &typed).emit_program(&program)?)
    } else {
        None
    };
    Ok(CompileOutput {
        c,
        bytecode,
        llvm_ir,
        leak_report,
        diagnostics,
        package_id: program.package_id.clone(),
    })
}

fn check_reference_cycles(source: &str, program: &Program) -> Vec<String> {
    let classes = program
        .types
        .iter()
        .filter(|ty| matches!(ty.kind, TypeKind::Class))
        .map(|ty| ty.name.clone())
        .collect::<HashSet<_>>();
    let mut graph = HashMap::<String, Vec<String>>::new();
    for ty in &program.types {
        for field in &ty.fields {
            if let Some(target) = referenced_class(&field.ty, &classes) {
                graph
                    .entry(ty.name.clone())
                    .or_default()
                    .push(target.clone());
            }
        }
    }
    let mut warnings = Vec::new();
    for ty in &program.types {
        for field in &ty.fields {
            let Some(target) = referenced_class(&field.ty, &classes) else {
                continue;
            };
            if !has_type_path(&graph, target, &ty.name, &mut HashSet::new()) {
                continue;
            }
            let needle = format!("{} {}", type_syntax_display(&field.ty), field.name);
            let (line, col, snippet) = locate_source(source, &needle);
            let message = format!(
                "reference cycle detected: class '{}' field '{}' participates in a potential ownership cycle {} -> {}",
                ty.name, field.name, ty.name, target
            );
            let help = format!(
                "replace '{} {}' with 'Weak<{}> {}' to break the cycle and avoid memory leaks",
                type_syntax_display(&field.ty), field.name, target, field.name
            );
            warnings.push(render_diagnostic("GL3007", line, col, snippet, &message, &help));
        }
    }
    warnings
}


fn link_package_sources(source: &str) -> Result<String, String> {
    let mut visited = Vec::new();
    let mut linked = String::new();
    link_package_sources_inner(source, &mut visited, &mut linked)?;
    linked.push_str(source);
    Ok(linked)
}

fn strip_utf8_bom(source: &str) -> &str {
    source.strip_prefix('\u{feff}').unwrap_or(source)
}

fn link_package_sources_inner(
    source: &str,
    visited: &mut Vec<String>,
    linked: &mut String,
) -> Result<(), String> {
    for package_id in source_using_packages(source) {
        if visited.contains(&package_id) {
            continue;
        }
        let Some(path) = package_source_path(&package_id) else {
            continue;
        };
        visited.push(package_id.clone());
        let package_source = fs::read_to_string(&path).map_err(|e| {
            format!(
                "failed to read package {} at {}: {e}",
                package_id,
                path.display()
            )
        })?;
        link_package_sources_inner(strip_utf8_bom(&package_source), visited, linked)?;
        linked.push_str(&package_source);
        linked.push('\n');
    }
    Ok(())
}

fn source_using_packages(source: &str) -> Vec<String> {
    let mut packages = Vec::<String>::new();
    for raw_line in source.lines() {
        let line = raw_line.trim();
        let rest = line
            .strip_prefix("using ")
            .or_else(|| line.strip_prefix("global using "));
        let Some(rest) = rest else {
            continue;
        };
        if rest.starts_with("static ") || rest.contains('=') {
            continue;
        }
        let package_id = canonical_package_id(rest.trim_end_matches(';').trim());
        if package_id.is_empty() || packages.iter().any(|existing| existing == &package_id) {
            continue;
        }
        packages.push(package_id);
    }
    packages
}

fn canonical_package_id(package_id: &str) -> String {
    if package_id == "Microsoft.AspNetCore"
        || package_id.starts_with("Microsoft.AspNetCore.")
        || package_id == "Microsoft.Extensions.Hosting"
        || package_id.starts_with("Microsoft.Extensions.Hosting.")
    {
        return "Glitching.AspNetCore".to_string();
    }
    package_id.to_string()
}

fn package_source_path(package_id: &str) -> Option<PathBuf> {
    let relative = package_id.replace('.', "/");
    let direct = Path::new("packages")
        .join(package_id)
        .join(format!("{package_id}.gl"));
    if direct.exists() {
        return Some(direct);
    }
    let nested = Path::new("packages")
        .join(relative)
        .join(format!("{package_id}.gl"));
    if nested.exists() {
        return Some(nested);
    }
    None
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

fn emit_llvm_bitcode(llvm_ir: &str, output_path: &str) -> Result<(), String> {
    let ll_path = format!("{output_path}.ll.tmp");
    fs::write(&ll_path, llvm_ir).map_err(|e| format!("failed to write temporary LLVM IR: {e}"))?;
    let result = if let Some(llvm_as) = find_llvm_tool("llvm-as") {
        Command::new(llvm_as)
            .arg(&ll_path)
            .arg("-o")
            .arg(output_path)
            .output()
            .map_err(|e| format!("failed to run llvm-as: {e}"))
    } else {
        let clang = require_llvm_tool("clang")?;
        Command::new(clang)
            .arg("-x")
            .arg("ir")
            .arg("-c")
            .arg("-emit-llvm")
            .arg(&ll_path)
            .arg("-o")
            .arg(output_path)
            .output()
            .map_err(|e| format!("failed to run clang for LLVM bitcode emission: {e}"))
    };
    let _ = fs::remove_file(&ll_path);
    let output = result?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "LLVM bitcode emission failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn emit_native_executable(llvm_ir: &str, output_path: &str) -> Result<(), String> {
    let ll_path = format!("{output_path}.ll.tmp");
    fs::write(&ll_path, llvm_ir).map_err(|e| format!("failed to write temporary LLVM IR: {e}"))?;
    let clang = require_llvm_tool("clang")?;
    let mut command = Command::new(clang);
    command
        .arg("-O3")
        .arg("-x")
        .arg("ir")
        .arg(&ll_path)
        .arg("-o")
        .arg(output_path);
    let links_rust_runtime = llvm_ir.contains("call void @GlitchRestHost_Run(")
        || llvm_ir.contains("System_IO_File_ReadAllText")
        || llvm_ir.contains("System_IO_File_WriteAllText");
    if links_rust_runtime {
        command.arg("-x").arg("none").arg(find_runtime_library()?);
    }
    if cfg!(windows) {
        command.arg("-lws2_32");
        if links_rust_runtime {
            for library in ["kernel32", "ntdll", "userenv", "dbghelp"] {
                command.arg(format!("-l{library}"));
            }
            command.arg("-Xlinker").arg("/defaultlib:msvcrt");
        }
        configure_windows_linker_environment(&mut command)?;
    }
    let result = command
        .output()
        .map_err(|e| format!("failed to run clang: {e}"));
    let _ = fs::remove_file(&ll_path);
    let output = result?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "clang failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn find_runtime_library() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("GLITCH_RUNTIME_LIB").map(PathBuf::from) {
        if path.is_file() {
            return Ok(path);
        }
        return Err(format!(
            "GLITCH_RUNTIME_LIB points to missing file {}",
            path.display()
        ));
    }
    let file_name = if cfg!(windows) {
        "glitchc.lib"
    } else {
        "libglitchc.a"
    };
    let mut candidates = Vec::new();
    if let Ok(executable) = env::current_exe() {
        if let Some(directory) = executable.parent() {
            candidates.push(directory.join(file_name));
        }
    }
    candidates.push(Path::new("target-codex").join("debug").join(file_name));
    candidates.push(Path::new("target").join("debug").join(file_name));
    candidates
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| {
            format!(
                "Rust native runtime '{file_name}' was not found; build glitchc first or set GLITCH_RUNTIME_LIB"
            )
        })
}

fn require_llvm_tool(name: &str) -> Result<PathBuf, String> {
    find_llvm_tool(name).ok_or_else(|| {
        format!(
            "LLVM tool '{name}' was not found. Add LLVM's bin directory to PATH or set GLITCH_LLVM_BIN"
        )
    })
}

fn find_llvm_tool(name: &str) -> Option<PathBuf> {
    let executable = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    let mut directories = Vec::<PathBuf>::new();
    if let Some(dir) = env::var_os("GLITCH_LLVM_BIN") {
        directories.push(PathBuf::from(dir));
    }
    if let Some(path) = env::var_os("PATH") {
        directories.extend(env::split_paths(&path));
    }
    if cfg!(windows) {
        directories.push(PathBuf::from(r"C:\Program Files\LLVM\bin"));
        directories.push(PathBuf::from(r"C:\Program Files (x86)\LLVM\bin"));
        if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
            directories.push(PathBuf::from(local_app_data).join(r"Programs\LLVM\bin"));
        }
    } else {
        directories.push(PathBuf::from("/usr/bin"));
        directories.push(PathBuf::from("/usr/local/bin"));
    }
    directories
        .into_iter()
        .map(|directory| directory.join(&executable))
        .find(|candidate| candidate.is_file())
}

fn configure_windows_linker_environment(command: &mut Command) -> Result<(), String> {
    let mut library_paths = Vec::<PathBuf>::new();
    if let Some(msvc_lib) = find_msvc_library_path() {
        library_paths.push(msvc_lib);
    }
    if let Some(sdk_lib_root) =
        find_latest_subdirectory(Path::new(r"C:\Program Files (x86)\Windows Kits\10\Lib"))
    {
        for component in ["ucrt", "um"] {
            let path = sdk_lib_root.join(component).join("x64");
            if path.is_dir() {
                library_paths.push(path);
            }
        }
    }
    if library_paths.len() < 3 {
        return Err(
            "Visual Studio C++ runtime or Windows SDK libraries were not found; install the Desktop development with C++ workload"
                .to_string(),
        );
    }
    if let Some(existing) = env::var_os("LIB") {
        library_paths.extend(env::split_paths(&existing));
    }
    let joined = env::join_paths(library_paths)
        .map_err(|e| format!("failed to construct Windows linker LIB path: {e}"))?;
    command.env("LIB", joined);
    Ok(())
}

fn find_msvc_library_path() -> Option<PathBuf> {
    let visual_studio_root = Path::new(r"C:\Program Files\Microsoft Visual Studio\2022");
    for edition in ["Community", "BuildTools", "Professional", "Enterprise"] {
        let tools = visual_studio_root.join(edition).join(r"VC\Tools\MSVC");
        let Some(version) = find_latest_subdirectory(&tools) else {
            continue;
        };
        let lib = version.join(r"lib\x64");
        if lib.is_dir() {
            return Some(lib);
        }
    }
    None
}

fn find_latest_subdirectory(root: &Path) -> Option<PathBuf> {
    let mut directories = fs::read_dir(root)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    directories.sort();
    directories.pop()
}

struct CompatibilityAnalyzer<'a> {
    source: &'a str,
    known_types: HashSet<String>,
    symbols: HashSet<String>,
    interface_implementations: HashMap<String, String>,
    emitted: HashSet<String>,
    diagnostics: Vec<String>,
}

impl<'a> CompatibilityAnalyzer<'a> {
    fn analyze(source: &'a str, _program: &Program, typed: &TypedProgram) -> Vec<String> {
        let interface_names = typed
            .types
            .iter()
            .filter(|ty| matches!(ty.kind, TypeKind::Interface))
            .map(|ty| ty.name.clone())
            .collect::<HashSet<_>>();
        let mut interface_candidates = HashMap::<String, Vec<String>>::new();
        for ty in &typed.types {
            if !matches!(ty.kind, TypeKind::Class) {
                continue;
            }
            for base in &ty.bases {
                let base = base_type_name(base).to_string();
                if interface_names.contains(&base) {
                    interface_candidates
                        .entry(base)
                        .or_default()
                        .push(ty.name.clone());
                }
            }
        }
        let interface_implementations = interface_candidates
            .into_iter()
            .filter_map(|(interface, mut implementations)| {
                implementations.sort();
                implementations.dedup();
                (implementations.len() == 1).then(|| (interface, implementations.remove(0)))
            })
            .collect();
        let mut analyzer = Self {
            source,
            known_types: typed.types.iter().map(|ty| ty.name.clone()).collect(),
            symbols: typed
                .functions
                .iter()
                .map(|function| function.symbol.clone())
                .chain(typed.types.iter().flat_map(|ty| {
                    ty.constructors
                        .iter()
                        .chain(&ty.methods)
                        .map(|function| function.symbol.clone())
                }))
                .collect(),
            interface_implementations,
            emitted: HashSet::new(),
            diagnostics: Vec::new(),
        };
        analyzer.detect_native_blocks();
        analyzer.detect_unexecutable_endpoints(typed);
        for function in &typed.functions {
            analyzer.visit_stmts(&function.body);
        }
        for ty in &typed.types {
            for function in ty.constructors.iter().chain(&ty.methods) {
                analyzer.visit_stmts(&function.body);
            }
        }
        analyzer.diagnostics
    }

    fn detect_unexecutable_endpoints(&mut self, typed: &TypedProgram) {
        let types = typed
            .types
            .iter()
            .map(|ty| (ty.name.as_str(), ty))
            .collect::<HashMap<_, _>>();
        for endpoint in &typed.endpoint_handlers {
            let mut blockers = Vec::new();
            let mut proposals = Vec::new();
            let return_supported = match &endpoint.response_type {
                tir::IrType::String => true,
                tir::IrType::Class(name) | tir::IrType::Struct(name) => {
                    types.get(name.as_str()).is_some_and(|ty| {
                        !matches!(ty.kind, TypeKind::Interface)
                            && ty.fields.iter().all(|field| {
                                matches!(
                                    field.ty,
                                    tir::IrType::Bool
                                        | tir::IrType::Byte
                                        | tir::IrType::Short
                                        | tir::IrType::Int
                                        | tir::IrType::Long
                                        | tir::IrType::UInt
                                        | tir::IrType::Double
                                        | tir::IrType::Decimal
                                        | tir::IrType::String
                                )
                            })
                    })
                }
                _ => false,
            };
            if !return_supported {
                blockers.push(format!(
                    "return type {:?} with response {:?}",
                    endpoint.return_type, endpoint.response_type
                ));
                proposals.push(
                    "add Task<T>/ActionResult<T> LLVM unwrapping and JSON serialization, or temporarily return `string` containing the serialized response"
                        .to_string(),
                );
            }
            for param in &endpoint.params {
                let supported = match (&param.source, &param.ty) {
                    (
                        tir::EndpointParameterSource::Route,
                        tir::IrType::Int | tir::IrType::Long | tir::IrType::String,
                    )
                    | (
                        tir::EndpointParameterSource::Query,
                        tir::IrType::Bool
                        | tir::IrType::Int
                        | tir::IrType::Long
                        | tir::IrType::String,
                    )
                    | (tir::EndpointParameterSource::Body, tir::IrType::String) => true,
                    (
                        tir::EndpointParameterSource::Body,
                        tir::IrType::Class(name) | tir::IrType::Struct(name),
                    ) => types.get(name.as_str()).is_some_and(|ty| {
                        !matches!(ty.kind, TypeKind::Interface)
                            && (ty.constructors.is_empty()
                                || ty
                                    .constructors
                                    .iter()
                                    .any(|constructor| constructor.params.len() == 1))
                            && ty.fields.iter().all(|field| {
                                matches!(
                                    field.ty,
                                    tir::IrType::Bool
                                        | tir::IrType::Byte
                                        | tir::IrType::Short
                                        | tir::IrType::Int
                                        | tir::IrType::Long
                                        | tir::IrType::UInt
                                        | tir::IrType::Double
                                        | tir::IrType::Decimal
                                        | tir::IrType::String
                                )
                            })
                    }),
                    _ => false,
                };
                if !supported {
                    blockers.push(format!(
                        "parameter '{}' uses {:?} {:?}",
                        param.name, param.source, param.ty
                    ));
                    proposals.push(format!(
                        "rewrite `{}` temporarily as `[FromBody] string body` and deserialize it explicitly, or implement a JSON binder for {:?}",
                        param.name, param.ty
                    ));
                }
            }
            for dependency in &endpoint.constructor_params {
                let supported = diagnostic_dependency_supported(
                    dependency,
                    &types,
                    &self.interface_implementations,
                    &mut HashSet::new(),
                );
                if !supported {
                    blockers.push(format!("constructor dependency {dependency:?}"));
                    proposals.push(format!(
                        "register a concrete owned implementation for {dependency:?} and add interface vtable/drop metadata; a temporary source rewrite can inject the concrete class directly"
                    ));
                }
            }
            if blockers.is_empty() {
                continue;
            }
            let route = format!("{} {}", endpoint.http_method, endpoint.path);
            self.emit(
                "GL3101",
                endpoint
                    .controller
                    .as_deref()
                    .unwrap_or(endpoint.function.as_str()),
                format!(
                    "endpoint {route} is discoverable but currently returns 501: {}",
                    blockers.join(", ")
                ),
                proposals.join("; "),
            );
        }
    }

    fn detect_native_blocks(&mut self) {
        for (index, line) in self.source.lines().enumerate() {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("native \"") {
                continue;
            }
            let col = line.len() - trimmed.len() + 1;
            let key = format!("GL2004:{}:{col}", index + 1);
            if self.emitted.insert(key) {
                self.diagnostics.push(render_diagnostic(
                    "GL2004",
                    index + 1,
                    col,
                    line,
                    "native block allocations cannot be proven leak-free",
                    "rewrite allocation calls to `glitch_calloc`, `glitch_realloc`, and `glitch_free`, then declare borrowed/owned transfer behavior in the package wrapper",
                ));
            }
        }
    }

    fn visit_stmts(&mut self, stmts: &[tir::TypedStmt]) {
        for stmt in stmts {
            match &stmt.kind {
                tir::TypedStmtKind::Let { expr, .. }
                | tir::TypedStmtKind::Assign { expr, .. }
                | tir::TypedStmtKind::Print(expr)
                | tir::TypedStmtKind::Expr(expr)
                | tir::TypedStmtKind::Throw(expr) => self.visit_expr(expr),
                tir::TypedStmtKind::AssignTarget { target, expr } => {
                    self.visit_expr(target);
                    self.visit_expr(expr);
                }
                tir::TypedStmtKind::Block(body) | tir::TypedStmtKind::While { body, .. } => {
                    self.visit_stmts(body)
                }
                tir::TypedStmtKind::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    self.visit_expr(condition);
                    self.visit_stmts(then_body);
                    self.visit_stmts(else_body);
                }
                tir::TypedStmtKind::Try {
                    try_body,
                    catch_body,
                    finally_body,
                    ..
                } => {
                    self.visit_stmts(try_body);
                    self.visit_stmts(catch_body);
                    self.visit_stmts(finally_body);
                }
                tir::TypedStmtKind::Switch {
                    expr,
                    cases,
                    default,
                } => {
                    self.visit_expr(expr);
                    for case in cases {
                        self.visit_expr(&case.value);
                        self.visit_stmts(&case.body);
                    }
                    self.visit_stmts(default);
                }
                tir::TypedStmtKind::For {
                    init,
                    condition,
                    increment,
                    body,
                } => {
                    if let Some(init) = init {
                        self.visit_stmts(std::slice::from_ref(init));
                    }
                    if let Some(condition) = condition {
                        self.visit_expr(condition);
                    }
                    self.visit_stmts(body);
                    if let Some(increment) = increment {
                        self.visit_stmts(std::slice::from_ref(increment));
                    }
                }
                tir::TypedStmtKind::ForEach {
                    collection, body, ..
                } => {
                    self.visit_expr(collection);
                    self.visit_stmts(body);
                }
                tir::TypedStmtKind::Return(Some(expr)) => self.visit_expr(expr),
                tir::TypedStmtKind::Return(None)
                | tir::TypedStmtKind::Break
                | tir::TypedStmtKind::Continue => {}
            }
        }
    }

    fn visit_expr(&mut self, expr: &tir::TypedExpr) {
        match &expr.kind {
            tir::TypedExprKind::Call(call) => {
                match &call.kind {
                    tir::TypedCallKind::Function { name, symbol } => {
                        if !self.symbols.contains(symbol) && !is_llvm_runtime_function(symbol) {
                            self.emit(
                                "GL3002",
                                &format!("{name}("),
                                format!("function '{name}' has no linked GL or LLVM implementation"),
                                format!(
                                    "add a package function with the same signature, for example `{}`",
                                    framework_stub(name, &expr.ty)
                                ),
                            );
                        }
                    }
                    tir::TypedCallKind::Method {
                        target,
                        name,
                        symbol,
                        resolution,
                    } => {
                        if matches!(resolution, tir::CallResolution::Unknown)
                            || (matches!(resolution, tir::CallResolution::InstanceMethod)
                                && !self.symbols.contains(symbol))
                        {
                            if let tir::IrType::Interface(interface_name) = &target.ty {
                                if self
                                    .interface_implementations
                                    .get(interface_name)
                                    .is_some_and(|implementation| {
                                        let prefix = format!(
                                            "{}_{}",
                                            sanitize_ident(implementation),
                                            sanitize_ident(name)
                                        );
                                        self.symbols.iter().any(|symbol| {
                                            symbol == &prefix
                                                || symbol.starts_with(&format!("{prefix}__"))
                                        })
                                    })
                                {
                                    self.visit_expr(target);
                                    for arg in &call.args {
                                        self.visit_expr(arg);
                                    }
                                    return;
                                }
                            }
                            self.emit(
                                "GL3001",
                                &format!(".{name}"),
                                format!(
                                    "member '{name}' on {:?} has no linked implementation; LLVM emits a typed default",
                                    target.ty
                                ),
                                format!(
                                    "implement this member in a `.gl` package, for example `{}`",
                                    framework_stub(name, &expr.ty)
                                ),
                            );
                        }
                        self.visit_expr(target);
                    }
                }
                for arg in &call.args {
                    self.visit_expr(arg);
                }
            }
            tir::TypedExprKind::Field { target, name } => {
                if matches!(target.ty, tir::IrType::Unknown(_)) {
                    self.emit(
                        "GL3003",
                        &format!(".{name}"),
                        format!(
                            "static or opaque member '{name}' on {:?} has no linked implementation",
                            target.ty
                        ),
                        format!(
                            "define the containing framework type and `{name}` member in a `.gl` package; until then LLVM uses {}",
                            typed_default_description(&expr.ty)
                        ),
                    );
                }
                self.visit_expr(target);
            }
            tir::TypedExprKind::IsPattern { expr, .. } => self.visit_expr(expr),
            tir::TypedExprKind::NewObject {
                type_name,
                args,
                fields,
                ..
            } => {
                if type_name != "Exception"
                    && type_name != "System.Exception"
                    && !self.known_types.contains(type_name)
                {
                    self.emit(
                        "GL3004",
                        &format!("new {type_name}"),
                        format!("type '{type_name}' has no linked LLVM layout"),
                        format!(
                            "add `class {type_name} {{ ... }}` to the corresponding `.gl` package; until then construction produces an opaque null handle"
                        ),
                    );
                }
                for arg in args {
                    self.visit_expr(arg);
                }
                for field in fields {
                    self.visit_expr(&field.expr);
                }
            }
            tir::TypedExprKind::ArrayLiteral(values) => {
                for value in values {
                    self.visit_expr(value);
                }
            }
            tir::TypedExprKind::NewArray { values, .. } => {
                for value in values {
                    self.visit_expr(value);
                }
            }
            tir::TypedExprKind::Index { target, index } => {
                self.visit_expr(target);
                self.visit_expr(index);
            }
            tir::TypedExprKind::Await(inner) => {
                if !matches!(inner.ty, tir::IrType::Task(_)) {
                    self.emit(
                        "GL3006",
                        "await ",
                        format!(
                            "awaited expression has unresolved task type {:?}; compatibility mode evaluates it synchronously",
                            inner.ty
                        ),
                        "implement the called method with a `Task<T>` return type in a `.gl` package to enable owned async state-machine lowering".to_string(),
                    );
                }
                self.visit_expr(inner);
            }
            tir::TypedExprKind::Unary { expr: inner, .. } => self.visit_expr(inner),
            tir::TypedExprKind::Lambda { body, .. } => {
                self.emit(
                    "GL3005",
                    "=>",
                    "lambda has no executable LLVM closure or expression-tree lowering; compatibility mode emits an opaque delegate".to_string(),
                    "for framework configuration, add the receiving API to a `.gl` package; for executable code, rewrite the lambda as a named function until closure lowering is available".to_string(),
                );
                self.visit_expr(body);
            }
            tir::TypedExprKind::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.visit_expr(condition);
                self.visit_expr(when_true);
                self.visit_expr(when_false);
            }
            tir::TypedExprKind::Binary { left, right, .. } => {
                self.visit_expr(left);
                self.visit_expr(right);
            }
            _ => {}
        }
    }

    fn emit(&mut self, code: &str, needle: &str, message: String, help: String) {
        let key = format!("{code}:{message}");
        if !self.emitted.insert(key) {
            return;
        }
        let (line, col, snippet) = locate_source(self.source, needle);
        self.diagnostics
            .push(render_diagnostic(code, line, col, snippet, &message, &help));
    }
}

fn diagnostic_dependency_supported(
    dependency: &tir::IrType,
    types: &HashMap<&str, &tir::TypedType>,
    interface_implementations: &HashMap<String, String>,
    visiting: &mut HashSet<String>,
) -> bool {
    let type_name = match dependency {
        tir::IrType::Class(name) | tir::IrType::Struct(name) => name.clone(),
        tir::IrType::Interface(name) => {
            let Some(implementation) = interface_implementations.get(name) else {
                return false;
            };
            implementation.clone()
        }
        _ => return false,
    };
    if !visiting.insert(type_name.clone()) {
        return false;
    }
    let Some(ty) = types.get(type_name.as_str()) else {
        visiting.remove(&type_name);
        return false;
    };
    if matches!(ty.kind, TypeKind::Interface) {
        visiting.remove(&type_name);
        return false;
    }
    let supported = ty.constructors.first().is_none_or(|constructor| {
        constructor.params.iter().skip(1).all(|param| {
            diagnostic_dependency_supported(&param.ty, types, interface_implementations, visiting)
        })
    });
    visiting.remove(&type_name);
    supported
}

fn base_type_name(name: &str) -> &str {
    name.split('<')
        .next()
        .unwrap_or(name)
        .rsplit('.')
        .next()
        .unwrap_or(name)
}

fn is_llvm_runtime_function(symbol: &str) -> bool {
    matches!(
        symbol,
        "GlitchRestHost_Run"
            | "GlitchMiddlewareHandlers_Apply"
            | "glitch_register_attribute_routes"
            | "GlitchEndpointHandlers_Contains"
            | "GlitchEndpointHandlers_Invoke"
            | "Ok"
            | "Created"
            | "CreatedAtAction"
            | "Accepted"
            | "NoContent"
            | "NotFound"
            | "BadRequest"
    )
}

fn locate_source<'a>(source: &'a str, needle: &str) -> (usize, usize, &'a str) {
    for (index, line) in source.lines().enumerate() {
        if let Some(col) = line.find(needle) {
            return (index + 1, col + 1, line);
        }
    }
    (1, 1, source.lines().next().unwrap_or(""))
}

fn render_diagnostic(
    code: &str,
    line: usize,
    col: usize,
    snippet: &str,
    message: &str,
    help: &str,
) -> String {
    format!(
        "warning {code} at {line}:{col}: {message}\n  {line} | {}\n  help: {help}",
        snippet.trim_end()
    )
}

fn referenced_class<'a>(ty: &'a TypeSyntax, classes: &HashSet<String>) -> Option<&'a String> {
    match ty {
        TypeSyntax::Named(name) if classes.contains(name) => Some(name),
        TypeSyntax::Nullable(inner) => referenced_class(inner, classes),
        _ => None,
    }
}

fn has_type_path(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut HashSet<String>,
) -> bool {
    if current == target {
        return true;
    }
    if !visited.insert(current.to_string()) {
        return false;
    }
    graph.get(current).is_some_and(|next| {
        next.iter()
            .any(|name| has_type_path(graph, name, target, visited))
    })
}

#[allow(dead_code)]
fn type_syntax_display(ty: &TypeSyntax) -> String {
    match ty {
        TypeSyntax::Named(name) => name.clone(),
        TypeSyntax::GenericNamed { name, args } => {
            let arg_strs = args
                .iter()
                .map(type_syntax_display)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}<{}>", name, arg_strs)
        }
        TypeSyntax::Nullable(inner) => format!("{}?", type_syntax_display(inner)),
        TypeSyntax::List(inner) => format!("List<{}>", type_syntax_display(inner)),
        TypeSyntax::Dictionary(key, value) => format!(
            "Dictionary<{}, {}>",
            type_syntax_display(key),
            type_syntax_display(value)
        ),
        TypeSyntax::Task(inner) => format!("Task<{}>", type_syntax_display(inner)),
        TypeSyntax::IEnumerable(inner) => format!("IEnumerable<{}>", type_syntax_display(inner)),
        other => format!("{other:?}"),
    }
}

fn framework_stub(name: &str, return_type: &tir::IrType) -> String {
    format!(
        "{} {name}(/* parameters */) {{ return {}; }}",
        ir_type_display(return_type),
        typed_default_description(return_type)
    )
}

fn ir_type_display(ty: &tir::IrType) -> String {
    match ty {
        tir::IrType::Void => "void".to_string(),
        tir::IrType::Bool => "bool".to_string(),
        tir::IrType::Int => "int".to_string(),
        tir::IrType::Long => "long".to_string(),
        tir::IrType::Double => "double".to_string(),
        tir::IrType::String => "string".to_string(),
        tir::IrType::Class(name)
        | tir::IrType::Struct(name)
        | tir::IrType::Interface(name)
        | tir::IrType::Unknown(name) => name.clone(),
        other => format!("{other:?}"),
    }
}

fn typed_default_description(ty: &tir::IrType) -> &'static str {
    match ty {
        tir::IrType::Void => "no value",
        tir::IrType::Bool => "false",
        tir::IrType::Byte
        | tir::IrType::Short
        | tir::IrType::Int
        | tir::IrType::Long
        | tir::IrType::UInt
        | tir::IrType::Double
        | tir::IrType::Decimal => "0",
        tir::IrType::String => "\"\"",
        _ => "null",
    }
}

struct LeakAnalyzer<'a> {
    functions: HashMap<&'a str, &'a TypeSyntax>,
    warnings: Vec<String>,
}

impl<'a> LeakAnalyzer<'a> {
    fn analyze_program(program: &'a Program, typed: &TypedProgram) -> String {
        let mut analyzer = Self {
            functions: program
                .functions
                .iter()
                .map(|f| (f.name.as_str(), &f.return_type))
                .collect(),
            warnings: Vec::new(),
        };
        let _ownership_summary = typed.ownership_summary();
        for function in &program.functions {
            analyzer.analyze_stmts(&format!("function {}", function.name), &function.body);
        }
        for ty in &program.types {
            for constructor in &ty.constructors {
                analyzer.analyze_stmts(&format!("constructor {}", ty.name), &constructor.body);
            }
            for method in &ty.methods {
                analyzer
                    .analyze_stmts(&format!("method {}.{}", ty.name, method.name), &method.body);
            }
        }
        if analyzer.warnings.is_empty() {
            "No obvious owned temporary leaks detected.\n".to_string()
        } else {
            let mut report = String::from("Potential leak report:\n");
            for warning in analyzer.warnings {
                report.push_str("- ");
                report.push_str(&warning);
                report.push('\n');
            }
            report
        }
    }

    fn analyze_stmts(&mut self, context: &str, stmts: &[Stmt]) {
        for stmt in stmts {
            self.analyze_stmt(context, stmt);
        }
    }

    fn analyze_stmt(&mut self, context: &str, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                if self.expr_may_create_owned_value(expr) {
                    self.warnings.push(format!(
                        "{context}: expression result is owned and discarded: {expr:?}"
                    ));
                }
                self.analyze_expr(context, expr);
            }
            Stmt::Let { expr, .. }
            | Stmt::Assign { expr, .. }
            | Stmt::AssignTarget { expr, .. }
            | Stmt::Print(expr)
            | Stmt::Return(Some(expr))
            | Stmt::Throw(expr) => self.analyze_expr(context, expr),
            Stmt::Block(body) | Stmt::While { body, .. } => self.analyze_stmts(context, body),
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr(context, condition);
                self.analyze_stmts(context, then_body);
                self.analyze_stmts(context, else_body);
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                self.analyze_stmts(context, try_body);
                if let Some(catch) = catch {
                    self.analyze_stmts(context, &catch.body);
                }
                self.analyze_stmts(context, finally_body);
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                self.analyze_expr(context, expr);
                for case in cases {
                    self.analyze_expr(context, &case.value);
                    self.analyze_stmts(context, &case.body);
                }
                self.analyze_stmts(context, default);
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                if let Some(init) = init {
                    self.analyze_stmt(context, init);
                }
                if let Some(condition) = condition {
                    self.analyze_expr(context, condition);
                }
                self.analyze_stmts(context, body);
                if let Some(increment) = increment {
                    self.analyze_stmt(context, increment);
                }
            }
            Stmt::ForEach {
                collection, body, ..
            } => {
                self.analyze_expr(context, collection);
                self.analyze_stmts(context, body);
            }
            Stmt::Return(None) | Stmt::Break | Stmt::Continue => {}
        }
    }

    fn analyze_expr(&mut self, context: &str, expr: &Expr) {
        match expr {
            Expr::ArrayLiteral(values) => {
                for value in values {
                    self.analyze_expr(context, value);
                }
            }
            Expr::NewArray { values, .. } => {
                for value in values {
                    self.analyze_expr(context, value);
                }
            }
            Expr::Index { target, index } => {
                self.analyze_expr(context, target);
                self.analyze_expr(context, index);
            }
            Expr::Field { target, .. } => self.analyze_expr(context, target),
            Expr::IsPattern { expr, .. } => self.analyze_expr(context, expr),
            Expr::MethodCall { target, args, .. } => {
                self.analyze_expr(context, target);
                for arg in args {
                    self.analyze_expr(context, arg);
                }
            }
            Expr::FunctionCall { args, .. } => {
                for arg in args {
                    self.analyze_expr(context, arg);
                }
            }
            Expr::NewObject { args, fields, .. } => {
                for arg in args {
                    self.analyze_expr(context, arg);
                }
                for field in fields {
                    self.analyze_expr(context, &field.expr);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.analyze_expr(context, left);
                self.analyze_expr(context, right);
            }
            Expr::Unary { expr, .. } => self.analyze_expr(context, expr),
            Expr::Lambda { body, .. } => self.analyze_expr(context, body),
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.analyze_expr(context, condition);
                self.analyze_expr(context, when_true);
                self.analyze_expr(context, when_false);
            }
            Expr::Await(inner) => self.analyze_expr(context, inner),
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                self.analyze_expr(context, expr)
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
            | Expr::Borrow { .. } => {
                let _ = context;
            }
        }
    }

    fn expr_may_create_owned_value(&self, expr: &Expr) -> bool {
        match expr {
            Expr::String(_)
            | Expr::NewArray { .. }
            | Expr::NewObject { .. }
            | Expr::NewCollection(_)
            | Expr::NewThread(_) => true,
            Expr::FunctionCall { name, .. } => self
                .functions
                .get(name.as_str())
                .is_some_and(|ty| type_may_own(ty)),
            Expr::MethodCall { target, name, .. } => {
                matches!(name.as_str(), "GetResult")
                    || matches!(target.as_ref(), Expr::Var(type_name) if type_name == "Task")
            }
            Expr::Await(_) => true,
            Expr::Unary { expr, .. } => self.expr_may_create_owned_value(expr),
            Expr::Lambda { .. } => false,
            Expr::Conditional {
                when_true,
                when_false,
                ..
            } => {
                self.expr_may_create_owned_value(when_true)
                    || self.expr_may_create_owned_value(when_false)
            }
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                self.expr_may_create_owned_value(expr)
            }
            _ => false,
        }
    }
}

fn type_may_own(ty: &TypeSyntax) -> bool {
    match ty {
        TypeSyntax::String
        | TypeSyntax::Array(_)
        | TypeSyntax::List(_)
        | TypeSyntax::Dictionary(_, _)
        | TypeSyntax::Thread
        | TypeSyntax::Task(_) => true,
        TypeSyntax::IEnumerable(_) => false,
        TypeSyntax::Named(_) | TypeSyntax::GenericNamed { .. } => true,
        TypeSyntax::Nullable(inner) => type_may_own(inner),
        _ => false,
    }
}

fn foreach_item_type_matches(expected: &CType, actual: &CType) -> bool {
    matches!(expected, CType::GenericPtr(name) if name == "var")
        || expected == actual
        || matches!((expected, actual), (CType::String, CType::BorrowedString))
}

fn parameter_binding_type(c_type: &CType) -> CType {
    match c_type {
        CType::String => CType::BorrowedString,
        other => other.clone(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CType {
    Scalar(ScalarType),
    Ptr(ScalarType),
    Array(ScalarType, usize),
    Null,
    String,
    BorrowedString,
    Exception,
    ExceptionPtr,
    Struct(String),
    ClassPtr(String),
    GenericPtr(String),
    ListInt,
    ListI64,
    ListBool,
    ListF64,
    ListString,
    DictStringInt,
    DictStringI64,
    DictStringBool,
    DictStringF64,
    DictStringString,
    EnumerableInt,
    EnumerableI64,
    EnumerableBool,
    EnumerableF64,
    EnumerableString,
    Thread,
    Task(Box<CType>),
    Void,
}

impl fmt::Display for CType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CType::Scalar(scalar) => write!(f, "{}", scalar.c_name()),
            CType::Ptr(scalar) => write!(f, "{} *", scalar.c_name()),
            CType::Array(scalar, _) => write!(f, "struct {}", array_runtime_name(*scalar)),
            CType::Null => write!(f, "void *"),
            CType::String => write!(f, "char *"),
            CType::BorrowedString => write!(f, "char *"),
            CType::Exception => write!(f, "struct GlitchException"),
            CType::ExceptionPtr => write!(f, "struct GlitchException *"),
            CType::Struct(name) => write!(f, "struct {}", sanitize_ident(name)),
            CType::ClassPtr(name) => write!(f, "struct {} *", sanitize_ident(name)),
            CType::GenericPtr(name) => write!(f, "struct {} *", sanitize_ident(name)),
            CType::ListInt => write!(f, "struct List_int"),
            CType::ListI64 => write!(f, "struct List_i64"),
            CType::ListBool => write!(f, "struct List_bool"),
            CType::ListF64 => write!(f, "struct List_f64"),
            CType::ListString => write!(f, "struct List_string"),
            CType::DictStringInt => write!(f, "struct Dict_string_int"),
            CType::DictStringI64 => write!(f, "struct Dict_string_i64"),
            CType::DictStringBool => write!(f, "struct Dict_string_bool"),
            CType::DictStringF64 => write!(f, "struct Dict_string_f64"),
            CType::DictStringString => write!(f, "struct Dict_string_string"),
            CType::EnumerableInt => write!(f, "struct IEnumerable_int"),
            CType::EnumerableI64 => write!(f, "struct IEnumerable_i64"),
            CType::EnumerableBool => write!(f, "struct IEnumerable_bool"),
            CType::EnumerableF64 => write!(f, "struct IEnumerable_f64"),
            CType::EnumerableString => write!(f, "struct IEnumerable_string"),
            CType::Thread => write!(f, "struct GlitchThread"),
            CType::Task(result) => write!(f, "struct {}", task_runtime_name(result)),
            CType::Void => write!(f, "void"),
        }
    }
}

impl ScalarType {
    fn c_name(self) -> &'static str {
        match self {
            ScalarType::Bool => "int",
            ScalarType::Byte => "unsigned char",
            ScalarType::Short => "short",
            ScalarType::I32 => "int",
            ScalarType::I64 => "long long",
            ScalarType::U32 => "unsigned int",
            ScalarType::F64 => "double",
            ScalarType::Decimal => "long double",
        }
    }

    fn printf_format(self) -> &'static str {
        match self {
            ScalarType::Bool => "%d",
            ScalarType::Byte => "%hhu",
            ScalarType::Short => "%hd",
            ScalarType::I32 => "%d",
            ScalarType::I64 => "%lld",
            ScalarType::U32 => "%u",
            ScalarType::F64 => "%f",
            ScalarType::Decimal => "%Lf",
        }
    }
}

#[derive(Clone)]
struct FunctionInfo {
    params: Vec<CType>,
    param_names: Vec<String>,
    param_modifiers: Vec<ParamModifier>,
    param_is_params: Vec<bool>,
    param_defaults: Vec<Option<Expr>>,
    return_type: CType,
    symbol: String,
}

struct ResolvedCall {
    info: FunctionInfo,
    args: Vec<Expr>,
}

struct CCallArg {
    c_type: CType,
    is_int_literal: bool,
    int_value: Option<i64>,
}

struct Codegen {
    types: HashMap<String, TypeKind>,
    bases: HashMap<String, Vec<String>>,
    fields: HashMap<String, Vec<(String, CType)>>,
    constructors: HashMap<String, FunctionInfo>,
    functions: HashMap<String, Vec<FunctionInfo>>,
    methods: HashMap<String, Vec<FunctionInfo>>,
    has_endpoint_handlers: bool,
    has_middleware_handlers: bool,
    endpoint_handler_symbols: Vec<String>,
    temp_counter: usize,
}

#[derive(Clone)]
struct CodegenState {
    vars: HashMap<String, CType>,
    scopes: Vec<Vec<String>>,
    moved: Vec<String>,
    return_type: CType,
    is_main: bool,
    is_async: bool,
    terminated: bool,
}

impl CodegenState {
    fn new(return_type: CType, is_main: bool, is_async: bool) -> Self {
        Self {
            vars: HashMap::new(),
            scopes: vec![Vec::new()],
            moved: Vec::new(),
            return_type,
            is_main,
            is_async,
            terminated: false,
        }
    }
}

struct EmittedExpr {
    code: String,
    c_type: CType,
}

struct ForeachInfo {
    declared_element_type: CType,
    variable_type: CType,
    length_expr: String,
    item_expr: String,
}

impl Codegen {
    fn new(program: &Program, typed: &TypedProgram) -> Self {
        let mut types = HashMap::new();
        for enum_def in &program.enums {
            types.insert(enum_def.name.clone(), TypeKind::Enum);
        }
        for ty in &program.types {
            types.insert(ty.name.clone(), ty.kind);
        }
        let mut codegen = Self {
            types,
            bases: HashMap::new(),
            fields: HashMap::new(),
            constructors: HashMap::new(),
            functions: HashMap::new(),
            methods: HashMap::new(),
            has_endpoint_handlers: program
                .native_c
                .iter()
                .any(|native| native.contains("GlitchEndpointHandlers_RemoveApp")),
            has_middleware_handlers: program
                .native_c
                .iter()
                .any(|native| native.contains("GlitchMiddlewareHandlers_RemoveApp")),
            endpoint_handler_symbols: typed
                .endpoint_handlers
                .iter()
                .map(|handler| handler.function.clone())
                .collect(),
            temp_counter: 0,
        };
        for ty in &program.types {
            codegen.bases.insert(ty.name.clone(), ty.bases.clone());
            let fields = ty
                .fields
                .iter()
                .filter_map(|field| {
                    codegen
                        .type_syntax_to_ctype(&field.ty)
                        .ok()
                        .map(|c| (field.name.clone(), c))
                })
                .collect();
            codegen.fields.insert(ty.name.clone(), fields);
            for constructor in &ty.constructors {
                let params = constructor
                    .params
                    .iter()
                    .filter_map(|param| codegen.param_to_ctype(param).ok())
                    .collect();
                let return_type = match ty.kind {
                    TypeKind::Class => CType::ClassPtr(ty.name.clone()),
                    TypeKind::Struct | TypeKind::RefStruct => CType::Struct(ty.name.clone()),
                    TypeKind::Interface => CType::ClassPtr(ty.name.clone()),
                    TypeKind::Enum => CType::Scalar(ScalarType::I32),
                };
                codegen.constructors.insert(
                    ty.name.clone(),
                    FunctionInfo {
                        params,
                        param_names: constructor
                            .params
                            .iter()
                            .map(|param| param.name.clone())
                            .collect(),
                        param_modifiers: constructor
                            .params
                            .iter()
                            .map(|param| param.modifier)
                            .collect(),
                        param_is_params: constructor
                            .params
                            .iter()
                            .map(|param| param.is_params)
                            .collect(),
                        param_defaults: constructor
                            .params
                            .iter()
                            .map(|param| param.default.clone())
                            .collect(),
                        return_type,
                        symbol: constructor_symbol(&ty.name),
                    },
                );
            }
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
                    .filter_map(|param| codegen.param_to_ctype(param).ok())
                    .collect::<Vec<_>>();
                let return_type = codegen
                    .type_syntax_to_ctype(&method.return_type)
                    .unwrap_or(CType::Void);
                codegen
                    .methods
                    .entry(method_key(&ty.name, &method.name))
                    .or_default()
                    .push(FunctionInfo {
                        symbol: method_symbol(&ty.name, &method.name, &params, overloaded),
                        params,
                        param_names: method
                            .params
                            .iter()
                            .map(|param| param.name.clone())
                            .collect(),
                        param_modifiers: method.params.iter().map(|param| param.modifier).collect(),
                        param_is_params: method
                            .params
                            .iter()
                            .map(|param| param.is_params)
                            .collect(),
                        param_defaults: method
                            .params
                            .iter()
                            .map(|param| param.default.clone())
                            .collect(),
                        return_type,
                    });
            }
        }
        let mut function_counts = HashMap::<String, usize>::new();
        for function in &program.functions {
            *function_counts.entry(function.name.clone()).or_default() += 1;
        }
        for function in &program.functions {
            let params = function
                .params
                .iter()
                .filter_map(|param| codegen.param_to_ctype(param).ok())
                .collect::<Vec<_>>();
            let return_type = codegen
                .type_syntax_to_ctype(&function.return_type)
                .unwrap_or(CType::Void);
            let overloaded = function_counts
                .get(&function.name)
                .copied()
                .unwrap_or_default()
                > 1;
            codegen
                .functions
                .entry(function.name.clone())
                .or_default()
                .push(FunctionInfo {
                    symbol: function_symbol(&function.name, &params, overloaded),
                    params,
                    param_names: function
                        .params
                        .iter()
                        .map(|param| param.name.clone())
                        .collect(),
                    param_modifiers: function.params.iter().map(|param| param.modifier).collect(),
                    param_is_params: function
                        .params
                        .iter()
                        .map(|param| param.is_params)
                        .collect(),
                    param_defaults: function
                        .params
                        .iter()
                        .map(|param| param.default.clone())
                        .collect(),
                    return_type,
                });
        }
        codegen
    }

    fn next_temp(&mut self, prefix: &str) -> String {
        let name = format!("__glitch_{}_{}", prefix, self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn coerce_initializer(
        &self,
        emitted: &EmittedExpr,
        target_type: &CType,
    ) -> Result<String, String> {
        match (target_type, &emitted.c_type) {
            (
                CType::String | CType::BorrowedString | CType::ClassPtr(_) | CType::GenericPtr(_),
                CType::Null,
            ) => Ok("NULL".to_string()),
            (CType::Task(result), CType::Null) => Ok(match result.as_ref() {
                CType::Void => format!("(struct {}){{0}}", task_runtime_name(result)),
                CType::Scalar(ScalarType::Bool)
                | CType::Scalar(ScalarType::Byte)
                | CType::Scalar(ScalarType::Short)
                | CType::Scalar(ScalarType::I32)
                | CType::Scalar(ScalarType::I64)
                | CType::Scalar(ScalarType::U32)
                | CType::Scalar(ScalarType::F64)
                | CType::Scalar(ScalarType::Decimal) => {
                    format!(
                        "{}({})",
                        task_from_result_function(result),
                        default_c_value(result)
                    )
                }
                CType::String
                | CType::BorrowedString
                | CType::ClassPtr(_)
                | CType::GenericPtr(_)
                | CType::Null => format!("{}(NULL)", task_from_result_function(result)),
                _ => format!("(struct {}){{0}}", task_runtime_name(result)),
            }),
            (CType::String, CType::BorrowedString) => {
                Ok(format!("glitch_strdup({})", emitted.code))
            }
            (CType::EnumerableInt, CType::ListInt) => {
                Ok(format!("IEnumerable_int_from_List_int(&{})", emitted.code))
            }
            (CType::EnumerableI64, CType::ListI64) => {
                Ok(format!("IEnumerable_i64_from_List_i64(&{})", emitted.code))
            }
            (CType::EnumerableBool, CType::ListBool) => Ok(format!(
                "IEnumerable_bool_from_List_bool(&{})",
                emitted.code
            )),
            (CType::EnumerableF64, CType::ListF64) => {
                Ok(format!("IEnumerable_f64_from_List_f64(&{})", emitted.code))
            }
            (CType::EnumerableString, CType::ListString) => Ok(format!(
                "IEnumerable_string_from_List_string(&{})",
                emitted.code
            )),
            (CType::GenericPtr(_), CType::GenericPtr(_)) => Ok(emitted.code.clone()),
            _ if target_type == &emitted.c_type => Ok(emitted.code.clone()),
            _ => Ok(emitted.code.clone()),
        }
    }

    fn emit_program(mut self, program: &Program) -> Result<String, String> {
        if !program.native_c.is_empty()
            && program.functions.is_empty()
            && program.types.is_empty()
            && program.enums.is_empty()
        {
            return Ok(program.native_c.join("\n"));
        }
        let mut out = String::new();
        out.push_str(
            "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n#include <setjmp.h>\n\n",
        );
        out.push_str(RUNTIME_C);
        out.push('\n');
        for native in &program.native_c {
            out.push_str(native);
            out.push('\n');
        }
        for enum_def in &program.enums {
            self.emit_enum(enum_def, &mut out);
        }
        for ty in &program.types {
            self.emit_type(ty, &mut out)?;
        }
        for ty in &program.types {
            if ty.kind == TypeKind::Interface {
                continue;
            }
            for constructor in &ty.constructors {
                out.push_str(&format!(
                    "static {} {}({});\n",
                    self.constructor_return_type(&ty.name),
                    constructor_symbol(&ty.name),
                    self.constructor_params(&ty.name, constructor)?
                ));
            }
            for method in &ty.methods {
                out.push_str(&format!(
                    "static {} {}({});\n",
                    self.method_return_type(&ty.name, method),
                    self.method_symbol_for(&ty.name, method),
                    self.method_params(&ty.name, method)?
                ));
            }
        }
        for f in &program.functions {
            if f.name != "main" {
                out.push_str(&format!(
                    "static {} {}({});\n",
                    self.function_return_type(f),
                    self.function_symbol_for(f),
                    self.function_params(f)?
                ));
            }
        }
        if program.functions.iter().any(|f| f.name != "main")
            || program
                .types
                .iter()
                .any(|ty| !ty.methods.is_empty() || !ty.constructors.is_empty())
        {
            out.push('\n');
        }
        self.emit_attribute_route_registration(program, &mut out);
        for ty in &program.types {
            if ty.kind == TypeKind::Interface {
                continue;
            }
            for constructor in &ty.constructors {
                self.emit_constructor(ty, constructor, &mut out)?;
            }
            for method in &ty.methods {
                self.emit_method(ty, method, &mut out)?;
            }
        }
        for f in &program.functions {
            self.emit_function(f, &mut out)?;
        }
        Ok(out)
    }

    fn emit_attribute_route_registration(&self, program: &Program, out: &mut String) {
        if !program.types.iter().any(|ty| ty.name == "WebApplication") {
            return;
        }
        let routes = controller_routes(program);
        out.push_str(
            "static void glitch_register_attribute_routes(struct WebApplication * app) {\n",
        );
        for route in routes {
            match route.method.as_str() {
                "GET" => out.push_str(&format!(
                    "    WebApplication_MapGet(app, {}, {});\n",
                    c_string_literal(&route.path),
                    c_string_literal(&route.handler)
                )),
                "POST" => out.push_str(&format!(
                    "    WebApplication_MapPost(app, {}, {});\n",
                    c_string_literal(&route.path),
                    c_string_literal(&route.handler)
                )),
                _ => {}
            }
        }
        out.push_str("}\n\n");
    }

    fn emit_enum(&self, enum_def: &EnumDef, out: &mut String) {
        self.emit_metadata_comment(&enum_def.namespace, &enum_def.attributes, out);
        out.push_str(&format!("enum {} {{\n", sanitize_ident(&enum_def.name)));
        for variant in &enum_def.variants {
            out.push_str("    ");
            out.push_str(&enum_variant_symbol(&enum_def.name, &variant.name));
            if let Some(value) = variant.value {
                out.push_str(&format!(" = {value}"));
            }
            out.push_str(",\n");
        }
        out.push_str("};\n\n");
    }

    fn emit_type(&self, ty: &TypeDef, out: &mut String) -> Result<(), String> {
        self.emit_metadata_comment(&ty.namespace, &ty.attributes, out);
        if ty.kind == TypeKind::Interface {
            out.push_str(&format!("/* interface {} */\n\n", sanitize_ident(&ty.name)));
            return Ok(());
        }
        out.push_str(&format!("struct {} {{\n", sanitize_ident(&ty.name)));
        if let Some(base) = self.concrete_base(&ty.name) {
            out.push_str(&format!("    struct {} __base;\n", sanitize_ident(&base)));
        }
        for field in &ty.fields {
            let c_type = self.type_syntax_to_ctype(&field.ty)?;
            out.push_str(&emit_c_field_declaration(
                &sanitize_ident(&field.name),
                &c_type,
            ));
        }
        out.push_str("};\n\n");
        if ty.kind == TypeKind::Class {
            let c = sanitize_ident(&ty.name);
            out.push_str(&format!(
                "static struct {0} * glitch_alloc_{0}(struct {0} value) {{ struct {0} *ptr = malloc(sizeof(struct {0})); if (!ptr) {{ abort(); }} *ptr = value; return ptr; }}\n\n",
                c
            ));
        }
        out.push_str(&format!(
            "static void glitch_drop_{}(struct {} *value) {{\n    if (!value) {{ return; }}\n",
            sanitize_ident(&ty.name),
            sanitize_ident(&ty.name)
        ));
        if self.has_endpoint_handlers && ty.name == "WebApplication" {
            out.push_str("    GlitchEndpointHandlers_RemoveApp(value);\n");
        }
        if self.has_middleware_handlers && ty.name == "WebApplication" {
            out.push_str("    GlitchMiddlewareHandlers_RemoveApp(value);\n");
        }
        if let Some(fields) = self.fields.get(&ty.name) {
            for (name, c_type) in fields {
                if let Some(drop) =
                    self.emit_drop_for_expr(&format!("value->{}", sanitize_ident(name)), c_type)
                {
                    out.push_str(&drop);
                }
            }
        }
        if let Some(base) = self.concrete_base(&ty.name) {
            if let Some(drop) =
                self.emit_drop_for_expr("value->__base", &CType::Struct(base.clone()))
            {
                out.push_str(&drop);
            }
        }
        out.push_str("}\n\n");
        Ok(())
    }

    fn emit_constructor(
        &mut self,
        owner: &TypeDef,
        constructor: &Constructor,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_metadata_comment(&constructor.namespace, &constructor.attributes, out);
        let ret = self.constructor_return_type(&owner.name);
        out.push_str(&format!(
            "static {} {}({}) {{\n",
            ret,
            constructor_symbol(&owner.name),
            self.constructor_params(&owner.name, constructor)?
        ));
        let mut state = CodegenState::new(ret.clone(), false, false);
        match owner.kind {
            TypeKind::Class => {
                out.push_str(&format!(
                    "    struct {} * self = glitch_alloc_{}((struct {}){{0}});\n",
                    sanitize_ident(&owner.name),
                    sanitize_ident(&owner.name),
                    sanitize_ident(&owner.name)
                ));
            }
            TypeKind::Struct | TypeKind::RefStruct => {
                out.push_str(&format!(
                    "    struct {} __glitch_value = (struct {}){{0}};\n",
                    sanitize_ident(&owner.name),
                    sanitize_ident(&owner.name)
                ));
                out.push_str("    struct ");
                out.push_str(&sanitize_ident(&owner.name));
                out.push_str(" * self = &__glitch_value;\n");
            }
            TypeKind::Interface => {}
            TypeKind::Enum => {}
        }
        state
            .vars
            .insert("this".to_string(), CType::ClassPtr(owner.name.clone()));
        if let Some(info) = self.constructors.get(&owner.name) {
            for (param, c_type) in constructor.params.iter().zip(info.params.iter()) {
                state
                    .vars
                    .insert(param.name.clone(), parameter_binding_type(c_type));
            }
        }
        for stmt in &constructor.body {
            self.emit_stmt(stmt, &mut state, out)?;
            if state.terminated {
                break;
            }
        }
        if !state.terminated {
            self.emit_scope_drops(&mut state, out);
            match owner.kind {
                TypeKind::Class => out.push_str("    return self;\n"),
                TypeKind::Struct | TypeKind::RefStruct => {
                    out.push_str("    return __glitch_value;\n")
                }
                TypeKind::Interface => {}
                TypeKind::Enum => {}
            }
        }
        out.push_str("}\n\n");
        Ok(())
    }

    fn emit_method(
        &mut self,
        owner: &TypeDef,
        method: &Function,
        out: &mut String,
    ) -> Result<(), String> {
        self.emit_metadata_comment(&method.namespace, &method.attributes, out);
        let ret = self.method_return_type(&owner.name, method);
        out.push_str(&format!(
            "static {} {}({}) {{\n",
            ret,
            self.method_symbol_for(&owner.name, method),
            self.method_params(&owner.name, method)?
        ));
        let mut state = CodegenState::new(ret, false, method.is_async);
        state
            .vars
            .insert("this".to_string(), CType::ClassPtr(owner.name.clone()));
        if let Some(info) = self.method_info_for(&owner.name, method) {
            for (param, c_type) in method.params.iter().zip(info.params.iter()) {
                state
                    .vars
                    .insert(param.name.clone(), parameter_binding_type(c_type));
            }
        }
        for stmt in &method.body {
            self.emit_stmt(stmt, &mut state, out)?;
            if state.terminated {
                break;
            }
        }
        if !state.terminated {
            self.emit_scope_drops(&mut state, out);
        }
        out.push_str("}\n\n");
        Ok(())
    }

    fn emit_function(&mut self, f: &Function, out: &mut String) -> Result<(), String> {
        self.emit_metadata_comment(&f.namespace, &f.attributes, out);
        let ret = self.function_return_type(f);
        if f.name == "main" {
            out.push_str("int main(void) {\n");
        } else {
            out.push_str(&format!(
                "static {} {}({}) {{\n",
                ret,
                self.function_symbol_for(f),
                self.function_params(f)?
            ));
        }
        let mut state = CodegenState::new(ret.clone(), f.name == "main", f.is_async);
        if f.name == "main" {
            out.push_str("    struct string_array * args = NULL;\n");
            state.vars.insert(
                "args".to_string(),
                CType::GenericPtr("string_array".to_string()),
            );
            state.scopes[0].push("args".to_string());
        }
        if let Some(info) = self.function_info_for(f) {
            for (param, c_type) in f.params.iter().zip(info.params.iter()) {
                state
                    .vars
                    .insert(param.name.clone(), parameter_binding_type(c_type));
            }
        }
        for stmt in &f.body {
            self.emit_stmt(stmt, &mut state, out)?;
            if state.terminated {
                break;
            }
        }
        if !state.terminated {
            self.emit_scope_drops(&mut state, out);
            if f.name == "main" {
                out.push_str("    return 0;\n");
            }
        }
        out.push_str("}\n\n");
        Ok(())
    }

    fn emit_stmt(
        &mut self,
        stmt: &Stmt,
        state: &mut CodegenState,
        out: &mut String,
    ) -> Result<(), String> {
        match stmt {
            Stmt::Let {
                name,
                declared_type,
                expr,
                ..
            } => {
                let emitted = self.emit_expr(expr, &state.vars)?;
                let c_type = if let Some(ty) = declared_type {
                    self.type_syntax_to_ctype(ty)?
                } else {
                    emitted.c_type.clone()
                };
                let initializer = self.coerce_initializer(&emitted, &c_type)?;
                out.push_str(&emit_c_declaration(
                    &sanitize_ident(name),
                    c_type.clone(),
                    &initializer,
                ));
                state.vars.insert(name.clone(), c_type);
                state.scopes.last_mut().unwrap().push(name.clone());
                self.remember_moves(expr, state);
            }
            Stmt::Assign { name, expr } => {
                let (c_type, target, is_local) = if let Some(c_type) = state.vars.get(name).cloned()
                {
                    let target = if matches!(c_type, CType::Ptr(_)) {
                        format!("*{}", sanitize_ident(name))
                    } else {
                        sanitize_ident(name)
                    };
                    (c_type, target, true)
                } else if let Some(field) = self.implicit_this_field_access(state, name) {
                    (field.c_type, field.code, false)
                } else {
                    let emitted = self.emit_expr(expr, &state.vars)?;
                    let c_type = emitted.c_type.clone();
                    let initializer = self.coerce_initializer(&emitted, &c_type)?;
                    out.push_str(&emit_c_declaration(
                        &sanitize_ident(name),
                        c_type.clone(),
                        &initializer,
                    ));
                    state.vars.insert(name.clone(), c_type);
                    state.scopes.last_mut().unwrap().push(name.clone());
                    self.remember_moves(expr, state);
                    return Ok(());
                };
                if is_local && !state.moved.contains(name) && !matches!(c_type, CType::Ptr(_)) {
                    if let Some(drop) = self.emit_drop(name, &c_type) {
                        out.push_str(&drop);
                    }
                } else if !is_local {
                    if let Some(drop) = self.emit_drop_for_expr(&target, &c_type) {
                        out.push_str(&drop);
                    }
                }
                let emitted = self.emit_expr(expr, &state.vars)?;
                let target_type = match c_type {
                    CType::Ptr(scalar) => CType::Scalar(scalar),
                    ref other => other.clone(),
                };
                let assign_code = self.coerce_initializer(&emitted, &target_type)?;
                out.push_str(&format!("    {} = {};\n", target, assign_code));
                if is_local {
                    state.moved.retain(|m| m != name);
                }
                self.remember_moves(expr, state);
            }
            Stmt::AssignTarget { target, expr } => {
                let emitted_target = self.emit_expr(target, &state.vars)?;
                if let Some(drop) =
                    self.emit_drop_for_expr(&emitted_target.code, &emitted_target.c_type)
                {
                    out.push_str(&drop);
                }
                let emitted = self.emit_expr(expr, &state.vars)?;
                let assign_code = self.coerce_initializer(&emitted, &emitted_target.c_type)?;
                out.push_str(&format!("    {} = {};\n", emitted_target.code, assign_code));
                self.remember_moves(expr, state);
            }
            Stmt::Block(body) => self.emit_block(body, state, out)?,
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.declare_out_vars(condition, state, out);
                let emitted_condition = self.emit_expr(condition, &state.vars)?;
                out.push_str(&format!("    if ({}) {{\n", emitted_condition.code));
                let mut then_state = state.clone();
                then_state.scopes.push(Vec::new());
                if let Expr::IsPattern {
                    ty,
                    name: Some(name),
                    ..
                } = condition
                {
                    then_state
                        .vars
                        .insert(name.clone(), CType::GenericPtr(type_syntax_symbol(ty)));
                    then_state.scopes.last_mut().unwrap().push(name.clone());
                }
                for s in then_body {
                    self.emit_stmt(s, &mut then_state, out)?;
                    if then_state.terminated {
                        break;
                    }
                }
                if !then_state.terminated {
                    self.emit_scope_drops(&mut then_state, out);
                }
                out.push_str("    }");
                if !else_body.is_empty() {
                    out.push_str(" else {\n");
                    let mut else_state = state.clone();
                    else_state.scopes.push(Vec::new());
                    for s in else_body {
                        self.emit_stmt(s, &mut else_state, out)?;
                        if else_state.terminated {
                            break;
                        }
                    }
                    if !else_state.terminated {
                        self.emit_scope_drops(&mut else_state, out);
                    }
                    out.push_str("    }");
                }
                out.push('\n');
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                out.push_str("    {\n");
                out.push_str("    struct GlitchExceptionFrame __glitch_frame;\n");
                out.push_str("    int __glitch_uncaught = 0;\n");
                out.push_str("    glitch_exception_push(&__glitch_frame);\n");
                out.push_str("    if (setjmp(__glitch_frame.env) == 0) {\n");
                {
                    let mut try_state = state.clone();
                    try_state.scopes.push(Vec::new());
                    for stmt in try_body {
                        self.emit_stmt(stmt, &mut try_state, out)?;
                        if try_state.terminated {
                            break;
                        }
                    }
                    if !try_state.terminated {
                        self.emit_scope_drops(&mut try_state, out);
                    }
                }
                out.push_str("    } else {\n");
                if let Some(catch) = catch {
                    let mut catch_state = state.clone();
                    catch_state.scopes.push(Vec::new());
                    if let Some(name) = &catch.name {
                        out.push_str(&format!(
                            "    struct GlitchException * {} = &__glitch_frame.exception;\n",
                            sanitize_ident(name)
                        ));
                        catch_state.vars.insert(name.clone(), CType::ExceptionPtr);
                    }
                    if let Some(exception_type) = &catch.exception_type {
                        let _ = self.type_syntax_to_ctype(exception_type)?;
                    }
                    for stmt in &catch.body {
                        self.emit_stmt(stmt, &mut catch_state, out)?;
                        if catch_state.terminated {
                            break;
                        }
                    }
                    if !catch_state.terminated {
                        self.emit_scope_drops(&mut catch_state, out);
                    }
                } else {
                    out.push_str("    __glitch_uncaught = 1;\n");
                }
                out.push_str("    }\n");
                out.push_str("    glitch_exception_pop(&__glitch_frame);\n");
                let mut finally_state = state.clone();
                finally_state.scopes.push(Vec::new());
                for stmt in finally_body {
                    self.emit_stmt(stmt, &mut finally_state, out)?;
                    if finally_state.terminated {
                        break;
                    }
                }
                if !finally_state.terminated {
                    self.emit_scope_drops(&mut finally_state, out);
                }
                out.push_str(
                    "    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }\n",
                );
                out.push_str("    glitch_exception_free(&__glitch_frame.exception);\n");
                out.push_str("    }\n");
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                let expr = self.emit_expr(expr, &state.vars)?;
                out.push_str(&format!("    switch ({}) {{\n", expr.code));
                for case in cases {
                    let value = self.emit_expr(&case.value, &state.vars)?;
                    out.push_str(&format!("    case {}:\n", value.code));
                    let mut case_state = state.clone();
                    case_state.scopes.push(Vec::new());
                    for stmt in &case.body {
                        self.emit_stmt(stmt, &mut case_state, out)?;
                        if case_state.terminated {
                            break;
                        }
                    }
                    if !case_state.terminated {
                        self.emit_scope_drops(&mut case_state, out);
                    }
                }
                if !default.is_empty() {
                    out.push_str("    default:\n");
                    let mut default_state = state.clone();
                    default_state.scopes.push(Vec::new());
                    for stmt in default {
                        self.emit_stmt(stmt, &mut default_state, out)?;
                        if default_state.terminated {
                            break;
                        }
                    }
                    if !default_state.terminated {
                        self.emit_scope_drops(&mut default_state, out);
                    }
                }
                out.push_str("    }\n");
            }
            Stmt::While { condition, body } => {
                let condition = self.emit_expr(condition, &state.vars)?;
                out.push_str(&format!("    while ({}) {{\n", condition.code));
                let mut body_state = state.clone();
                body_state.scopes.push(Vec::new());
                for s in body {
                    self.emit_stmt(s, &mut body_state, out)?;
                }
                self.emit_scope_drops(&mut body_state, out);
                out.push_str("    }\n");
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                out.push_str("    {\n");
                state.scopes.push(Vec::new());
                if let Some(init) = init {
                    self.emit_stmt(init, state, out)?;
                }
                let condition = if let Some(condition) = condition {
                    self.emit_expr(condition, &state.vars)?.code
                } else {
                    "1".to_string()
                };
                out.push_str(&format!("    while ({}) {{\n", condition));
                for s in body {
                    self.emit_stmt(s, state, out)?;
                }
                if let Some(inc) = increment {
                    self.emit_stmt(inc, state, out)?;
                }
                out.push_str("    }\n");
                self.emit_scope_drops(state, out);
                out.push_str("    }\n");
            }
            Stmt::ForEach {
                item_type,
                item_name,
                collection,
                body,
            } => {
                let collection = self.emit_expr(collection, &state.vars)?;
                let expected = self.type_syntax_to_ctype(item_type)?;
                let index_name = self.next_temp("foreach_i");
                let info = self.foreach_info(&collection, &index_name)?;
                if !foreach_item_type_matches(&expected, &info.declared_element_type) {
                    return Err(format!(
                        "foreach item type '{}' does not match collection element type '{}' for collection '{}'",
                        expected, info.declared_element_type, collection.c_type
                    ));
                }
                out.push_str("    {\n");
                out.push_str(&format!(
                    "    for (int {0} = 0; {0} < {1}; {0}++) {{\n",
                    index_name, info.length_expr
                ));
                state.scopes.push(Vec::new());
                state
                    .vars
                    .insert(item_name.clone(), info.variable_type.clone());
                state.scopes.last_mut().unwrap().push(item_name.clone());
                out.push_str(&emit_c_declaration(
                    &sanitize_ident(item_name),
                    info.variable_type,
                    &info.item_expr,
                ));
                for stmt in body {
                    self.emit_stmt(stmt, state, out)?;
                }
                self.emit_scope_drops(state, out);
                out.push_str("    }\n");
                out.push_str("    }\n");
            }
            Stmt::Print(expr) => {
                let e = self.emit_expr(expr, &state.vars)?;
                match e.c_type {
                    CType::Scalar(s) => out.push_str(&format!(
                        "    printf(\"{}\\n\", {});\n",
                        s.printf_format(),
                        e.code
                    )),
                    CType::Ptr(s) => out.push_str(&format!(
                        "    printf(\"{}\\n\", *{});\n",
                        s.printf_format(),
                        e.code
                    )),
                    CType::String => out.push_str(&format!("    printf(\"%s\\n\", {});\n", e.code)),
                    CType::BorrowedString => {
                        out.push_str(&format!("    printf(\"%s\\n\", {});\n", e.code))
                    }
                    _ => out.push_str(&format!("    (void)({});\n", e.code)),
                }
            }
            Stmt::Expr(expr) => {
                let e = self.emit_expr(expr, &state.vars)?;
                out.push_str(&format!("    {};\n", e.code));
                self.remember_moves(expr, state);
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    let e = self.emit_expr(expr, &state.vars)?;
                    let ret = if state.is_main {
                        CType::Scalar(ScalarType::I32)
                    } else {
                        state.return_type.clone()
                    };
                    let return_code = if state.is_async {
                        if let CType::Task(result) = &ret {
                            if result.as_ref() == &CType::String
                                && e.c_type == CType::BorrowedString
                            {
                                format!("GlitchTask_string_from_result({})", e.code)
                            } else {
                                format!("{}({})", task_from_result_function(result), e.code)
                            }
                        } else {
                            return Err("async functions must return Task or Task<T>".to_string());
                        }
                    } else {
                        self.coerce_initializer(&e, &ret)?
                    };
                    out.push_str(&emit_c_declaration("__glitch_return", ret, &return_code));
                    self.remember_moves(expr, state);
                    self.emit_all_scope_drops(state, out);
                    out.push_str("    return __glitch_return;\n");
                } else {
                    self.emit_all_scope_drops(state, out);
                    if state.is_main {
                        out.push_str("    return 0;\n");
                    } else {
                        out.push_str("    return;\n");
                    }
                }
                state.terminated = true;
            }
            Stmt::Throw(expr) => {
                let e = self.emit_expr(expr, &state.vars)?;
                match e.c_type {
                    CType::Exception => out.push_str(&format!("    glitch_throw({});\n", e.code)),
                    CType::ExceptionPtr => out.push_str(&format!(
                        "    glitch_throw(glitch_exception_clone({}));\n",
                        e.code
                    )),
                    CType::String => out.push_str(&format!(
                        "    glitch_throw(glitch_exception_from_owned({}));\n",
                        e.code
                    )),
                    CType::BorrowedString => out.push_str(&format!(
                        "    glitch_throw(glitch_exception_new({}));\n",
                        e.code
                    )),
                    CType::ClassPtr(_) | CType::GenericPtr(_) => {
                        out.push_str("    glitch_throw(glitch_exception_new(\"\"));\n")
                    }
                    other => return Err(format!("throw expects Exception or string, got {other}")),
                }
                state.terminated = true;
            }
            Stmt::Break => {
                out.push_str("    break;\n");
            }
            Stmt::Continue => {
                out.push_str("    continue;\n");
            }
        }
        Ok(())
    }

    fn emit_block(
        &mut self,
        body: &[Stmt],
        state: &mut CodegenState,
        out: &mut String,
    ) -> Result<(), String> {
        out.push_str("    {\n");
        state.scopes.push(Vec::new());
        for stmt in body {
            self.emit_stmt(stmt, state, out)?;
        }
        self.emit_scope_drops(state, out);
        out.push_str("    }\n");
        Ok(())
    }

    fn declare_out_vars(&self, expr: &Expr, state: &mut CodegenState, out: &mut String) {
        match expr {
            Expr::RefArg {
                modifier: ParamModifier::Out,
                expr,
            } => {
                if let Expr::Var(name) = expr.as_ref() {
                    if !state.vars.contains_key(name) {
                        let c_type = CType::GenericPtr("out_var".to_string());
                        out.push_str(&emit_c_declaration(
                            &sanitize_ident(name),
                            c_type.clone(),
                            "NULL",
                        ));
                        state.vars.insert(name.clone(), c_type);
                        state.scopes.last_mut().unwrap().push(name.clone());
                    }
                }
            }
            Expr::FunctionCall { args, .. } | Expr::MethodCall { args, .. } => {
                for arg in args {
                    self.declare_out_vars(arg, state, out);
                }
            }
            Expr::NamedArg { expr, .. }
            | Expr::RefArg { expr, .. }
            | Expr::Await(expr)
            | Expr::Unary { expr, .. }
            | Expr::IsPattern { expr, .. } => self.declare_out_vars(expr, state, out),
            Expr::Binary { left, right, .. } => {
                self.declare_out_vars(left, state, out);
                self.declare_out_vars(right, state, out);
            }
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.declare_out_vars(condition, state, out);
                self.declare_out_vars(when_true, state, out);
                self.declare_out_vars(when_false, state, out);
            }
            Expr::Index { target, index } => {
                self.declare_out_vars(target, state, out);
                self.declare_out_vars(index, state, out);
            }
            Expr::Field { target, .. } => self.declare_out_vars(target, state, out),
            Expr::ArrayLiteral(values) | Expr::NewArray { values, .. } => {
                for value in values {
                    self.declare_out_vars(value, state, out);
                }
            }
            Expr::NewObject { args, fields, .. } => {
                for arg in args {
                    self.declare_out_vars(arg, state, out);
                }
                for field in fields {
                    self.declare_out_vars(&field.expr, state, out);
                }
            }
            Expr::Lambda { body, .. } => self.declare_out_vars(body, state, out),
            _ => {}
        }
    }

    fn emit_expr(
        &mut self,
        expr: &Expr,
        vars: &HashMap<String, CType>,
    ) -> Result<EmittedExpr, String> {
        match expr {
            Expr::Int(v) => Ok(EmittedExpr {
                code: v.to_string(),
                c_type: CType::Scalar(ScalarType::I64),
            }),
            Expr::Float(v) => Ok(EmittedExpr {
                code: v.to_string(),
                c_type: CType::Scalar(ScalarType::F64),
            }),
            Expr::Bool(v) => Ok(EmittedExpr {
                code: if *v { "1" } else { "0" }.to_string(),
                c_type: CType::Scalar(ScalarType::Bool),
            }),
            Expr::Null => Ok(EmittedExpr {
                code: "NULL".to_string(),
                c_type: CType::Null,
            }),
            Expr::String(s) => Ok(EmittedExpr {
                code: format!("glitch_strdup({})", c_string_literal(s)),
                c_type: CType::String,
            }),
            Expr::Var(name) | Expr::Move(name) => {
                let c_type = if let Some(c_type) = vars.get(name).cloned() {
                    c_type
                } else if let Some(this_type) = vars.get("this") {
                    match this_type {
                        CType::ClassPtr(type_name) => {
                            if let Some(field) = self.field_access(type_name, "self", true, name) {
                                return Ok(field);
                            }
                            return Err(format!("unknown variable '{name}'"));
                        }
                        CType::Struct(type_name) => {
                            if let Some(field) = self.field_access(type_name, "self", false, name) {
                                return Ok(field);
                            }
                            return Err(format!("unknown variable '{name}'"));
                        }
                        _ => return Err(format!("unknown variable '{name}'")),
                    }
                } else {
                    return Err(format!("unknown variable '{name}'"));
                };
                match c_type {
                    CType::Ptr(scalar) => Ok(EmittedExpr {
                        code: format!("*{}", sanitize_ident(name)),
                        c_type: CType::Scalar(scalar),
                    }),
                    other => Ok(EmittedExpr {
                        code: sanitize_ident(name),
                        c_type: other,
                    }),
                }
            }
            Expr::ArrayLiteral(values) => {
                let values = values
                    .iter()
                    .map(|v| self.emit_expr(v, vars).map(|e| e.code))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(EmittedExpr {
                    code: array_literal_code(ScalarType::I64, &values),
                    c_type: CType::Array(ScalarType::I64, values.len()),
                })
            }
            Expr::NewArray {
                element_type,
                values,
            } => {
                let values = values
                    .iter()
                    .map(|v| self.emit_expr(v, vars).map(|e| e.code))
                    .collect::<Result<Vec<_>, _>>()?;
                let TypeSyntax::Scalar(element_type) = element_type else {
                    return Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::GenericPtr(format!(
                            "{}_array",
                            type_syntax_symbol(element_type)
                        )),
                    });
                };
                Ok(EmittedExpr {
                    code: array_literal_code(*element_type, &values),
                    c_type: CType::Array(*element_type, values.len()),
                })
            }
            Expr::Index { target, index } => {
                let t = self.emit_expr(target, vars)?;
                let i = self.emit_expr(index, vars)?;
                match t.c_type {
                    CType::ListInt => Ok(EmittedExpr {
                        code: format!("List_int_get(&{}, {})", t.code, i.code),
                        c_type: CType::Scalar(ScalarType::I32),
                    }),
                    CType::ListI64 => Ok(EmittedExpr {
                        code: format!("List_i64_get(&{}, {})", t.code, i.code),
                        c_type: CType::Scalar(ScalarType::I64),
                    }),
                    CType::ListBool => Ok(EmittedExpr {
                        code: format!("List_bool_get(&{}, {})", t.code, i.code),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    CType::ListF64 => Ok(EmittedExpr {
                        code: format!("List_f64_get(&{}, {})", t.code, i.code),
                        c_type: CType::Scalar(ScalarType::F64),
                    }),
                    CType::ListString => Ok(EmittedExpr {
                        code: format!("List_string_get(&{}, {})", t.code, i.code),
                        c_type: CType::BorrowedString,
                    }),
                    CType::DictStringInt => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_int_get(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(index, vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::I32),
                    }),
                    CType::DictStringI64 => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_i64_get(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(index, vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::I64),
                    }),
                    CType::DictStringBool => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_bool_get(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(index, vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    CType::DictStringF64 => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_f64_get(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(index, vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::F64),
                    }),
                    CType::DictStringString => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_string_get(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(index, vars)?
                        ),
                        c_type: CType::BorrowedString,
                    }),
                    CType::Array(s, _) => Ok(EmittedExpr {
                        code: format!("{}.data[{}]", t.code, i.code),
                        c_type: CType::Scalar(s),
                    }),
                    CType::String | CType::BorrowedString => Ok(EmittedExpr {
                        code: "\"\"".to_string(),
                        c_type: CType::BorrowedString,
                    }),
                    CType::GenericPtr(type_name) => Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::GenericPtr(format!("{}_item", type_name)),
                    }),
                    other => Err(format!("index target is not indexable: {other}")),
                }
            }
            Expr::Field { target, name } => {
                if let Some(path) = expr_to_type_path(target) {
                    let type_name = path.join(".");
                    if self.types.get(&type_name) == Some(&TypeKind::Enum) {
                        return Ok(EmittedExpr {
                            code: enum_variant_symbol(&type_name, name),
                            c_type: CType::Scalar(ScalarType::I32),
                        });
                    }
                    if path.first().is_some_and(|first| !vars.contains_key(first)) {
                        return Ok(EmittedExpr {
                            code: "NULL".to_string(),
                            c_type: CType::GenericPtr(format!("{}_{}", type_name, name)),
                        });
                    }
                }
                let t = self.emit_expr(target, vars)?;
                match t.c_type {
                    CType::ExceptionPtr if name == "Message" => Ok(EmittedExpr {
                        code: format!("{}->message", t.code),
                        c_type: CType::BorrowedString,
                    }),
                    CType::ListInt
                    | CType::ListI64
                    | CType::ListBool
                    | CType::ListF64
                    | CType::ListString
                        if name == "Count" =>
                    {
                        Ok(EmittedExpr {
                            code: format!("{}.len", t.code),
                            c_type: CType::Scalar(ScalarType::I32),
                        })
                    }
                    CType::DictStringInt
                    | CType::DictStringI64
                    | CType::DictStringBool
                    | CType::DictStringF64
                    | CType::DictStringString
                        if name == "Count" =>
                    {
                        Ok(EmittedExpr {
                            code: format!("{}.len", t.code),
                            c_type: CType::Scalar(ScalarType::I32),
                        })
                    }
                    CType::Array(_, _) if name == "Length" || name == "Count" => Ok(EmittedExpr {
                        code: format!("{}.len", t.code),
                        c_type: CType::Scalar(ScalarType::I32),
                    }),
                    CType::String | CType::BorrowedString if name == "Length" => Ok(EmittedExpr {
                        code: "0".to_string(),
                        c_type: CType::Scalar(ScalarType::I32),
                    }),
                    CType::Struct(type_name) => {
                        if let Some(access) = self.field_access(&type_name, &t.code, false, name) {
                            Ok(access)
                        } else {
                            self.emit_instance_property_getter(
                                &type_name,
                                &format!("&{}", t.code),
                                name,
                            )
                        }
                    }
                    CType::ClassPtr(type_name) => {
                        if let Some(access) = self.field_access(&type_name, &t.code, true, name) {
                            Ok(access)
                        } else {
                            self.emit_instance_property_getter(&type_name, &t.code, name)
                        }
                    }
                    CType::Task(result) if name == "Result" => Ok(EmittedExpr {
                        code: format!("{}(&{})", task_result_function(&result), t.code),
                        c_type: *result,
                    }),
                    CType::Task(_) if name == "IsCompleted" => Ok(EmittedExpr {
                        code: "1".to_string(),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    CType::GenericPtr(type_name) => Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::GenericPtr(format!("{}_{}", type_name, name)),
                    }),
                    other => Err(format!(
                        "field access target for '{name}' is not a struct/class/task: {other}"
                    )),
                }
            }
            Expr::IsPattern { expr, .. } => {
                let code = match self.emit_expr(expr, vars) {
                    Ok(value)
                        if matches!(
                            value.c_type,
                            CType::ClassPtr(_)
                                | CType::GenericPtr(_)
                                | CType::String
                                | CType::BorrowedString
                                | CType::Null
                        ) =>
                    {
                        format!("({} != NULL)", value.code)
                    }
                    Ok(_) | Err(_) => "1".to_string(),
                };
                Ok(EmittedExpr {
                    code,
                    c_type: CType::Scalar(ScalarType::Bool),
                })
            }
            Expr::MethodCall { target, name, args } => {
                if let Some(path) = expr_to_type_path(target) {
                    if is_task_type_path(&path) && name == "Run" {
                        let Some(Expr::Var(entry)) = args.first() else {
                            return Err("Task.Run expects a function name".to_string());
                        };
                        let ret = self
                            .functions
                            .get(entry)
                            .and_then(|functions| functions.first())
                            .map(|f| f.return_type.clone())
                            .ok_or_else(|| {
                                format!("Task entry function '{entry}' does not exist")
                            })?;
                        return Ok(EmittedExpr {
                            code: format!("{}({})", task_run_function(&ret), sanitize_ident(entry)),
                            c_type: CType::Task(Box::new(ret)),
                        });
                    }
                    if is_task_type_path(&path) && name == "FromResult" {
                        let Some(arg) = args.first() else {
                            return Err("Task.FromResult expects a value".to_string());
                        };
                        let emitted = self.emit_expr(arg, vars)?;
                        let was_borrowed_string = emitted.c_type == CType::BorrowedString;
                        let task_result = match emitted.c_type {
                            CType::Scalar(ScalarType::I64) if matches!(arg, Expr::Int(_)) => {
                                CType::Scalar(ScalarType::I32)
                            }
                            CType::BorrowedString => CType::String,
                            other => other,
                        };
                        let function = if was_borrowed_string {
                            "GlitchTask_string_from_result"
                        } else {
                            task_from_result_function(&task_result)
                        };
                        return Ok(EmittedExpr {
                            code: format!("{}({})", function, emitted.code),
                            c_type: CType::Task(Box::new(task_result)),
                        });
                    }
                    if is_file_type_path(&path) && (name == "WriteAllText" || name == "ReadAllText") {
                        let mut arg_codes = Vec::new();
                        for arg in args {
                            arg_codes.push(self.emit_expr(arg, vars)?.code);
                        }
                        if name == "WriteAllText" {
                            return Ok(EmittedExpr {
                                code: format!("System_IO_File_WriteAllText({})", arg_codes.join(", ")),
                                c_type: CType::Void,
                            });
                        } else {
                            return Ok(EmittedExpr {
                                code: format!("System_IO_File_ReadAllText({})", arg_codes.join(", ")),
                                c_type: CType::String,
                            });
                        }
                    }
                    if path.first().is_some_and(|first| !vars.contains_key(first)) {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        return Ok(EmittedExpr {
                            code: "NULL".to_string(),
                            c_type: CType::GenericPtr(format!("{}_{}", path.join("."), name)),
                        });
                    }
                }
                if let Expr::Var(type_name) = target.as_ref() {
                    if !vars.contains_key(type_name) && self.types.get(type_name).is_none() {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        return Ok(EmittedExpr {
                            code: "NULL".to_string(),
                            c_type: CType::GenericPtr(format!("{}_{}", type_name, name)),
                        });
                    }
                }
                let t = self.emit_expr(target, vars)?;
                match (t.c_type, name.as_str()) {
                    (CType::ListInt, "Add") => Ok(EmittedExpr {
                        code: format!(
                            "List_int_add(&{}, {})",
                            t.code,
                            self.emit_expr(&args[0], vars)?.code
                        ),
                        c_type: CType::Void,
                    }),
                    (CType::ListI64, "Add") => Ok(EmittedExpr {
                        code: format!(
                            "List_i64_add(&{}, {})",
                            t.code,
                            self.emit_expr(&args[0], vars)?.code
                        ),
                        c_type: CType::Void,
                    }),
                    (CType::ListBool, "Add") => Ok(EmittedExpr {
                        code: format!(
                            "List_bool_add(&{}, {})",
                            t.code,
                            self.emit_expr(&args[0], vars)?.code
                        ),
                        c_type: CType::Void,
                    }),
                    (CType::ListF64, "Add") => Ok(EmittedExpr {
                        code: format!(
                            "List_f64_add(&{}, {})",
                            t.code,
                            self.emit_expr(&args[0], vars)?.code
                        ),
                        c_type: CType::Void,
                    }),
                    (CType::ListString, "Add") => Ok(EmittedExpr {
                        code: self.emit_list_string_add(&t.code, &args[0], vars)?,
                        c_type: CType::Void,
                    }),
                    (CType::DictStringInt, "Add") => Ok(EmittedExpr {
                        code: self.emit_dict_string_scalar_add(
                            "Dict_string_int_add",
                            &t.code,
                            &args[0],
                            &args[1],
                            vars,
                        )?,
                        c_type: CType::Void,
                    }),
                    (CType::DictStringI64, "Add") => Ok(EmittedExpr {
                        code: self.emit_dict_string_scalar_add(
                            "Dict_string_i64_add",
                            &t.code,
                            &args[0],
                            &args[1],
                            vars,
                        )?,
                        c_type: CType::Void,
                    }),
                    (CType::DictStringBool, "Add") => Ok(EmittedExpr {
                        code: self.emit_dict_string_scalar_add(
                            "Dict_string_bool_add",
                            &t.code,
                            &args[0],
                            &args[1],
                            vars,
                        )?,
                        c_type: CType::Void,
                    }),
                    (CType::DictStringF64, "Add") => Ok(EmittedExpr {
                        code: self.emit_dict_string_scalar_add(
                            "Dict_string_f64_add",
                            &t.code,
                            &args[0],
                            &args[1],
                            vars,
                        )?,
                        c_type: CType::Void,
                    }),
                    (CType::DictStringString, "Add") => Ok(EmittedExpr {
                        code: self
                            .emit_dict_string_string_add(&t.code, &args[0], &args[1], vars)?,
                        c_type: CType::Void,
                    }),
                    (CType::ListInt, "Clear") => Ok(EmittedExpr {
                        code: format!("List_int_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::ListI64, "Clear") => Ok(EmittedExpr {
                        code: format!("List_i64_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::ListBool, "Clear") => Ok(EmittedExpr {
                        code: format!("List_bool_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::ListF64, "Clear") => Ok(EmittedExpr {
                        code: format!("List_f64_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::ListString, "Clear") => Ok(EmittedExpr {
                        code: format!("List_string_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::ListInt, "Contains") => Ok(EmittedExpr {
                        code: format!(
                            "List_int_contains(&{}, {})",
                            t.code,
                            self.emit_expr(&args[0], vars)?.code
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::ListI64, "Contains") => Ok(EmittedExpr {
                        code: format!(
                            "List_i64_contains(&{}, {})",
                            t.code,
                            self.emit_expr(&args[0], vars)?.code
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::ListBool, "Contains") => Ok(EmittedExpr {
                        code: format!(
                            "List_bool_contains(&{}, {})",
                            t.code,
                            self.emit_expr(&args[0], vars)?.code
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::ListF64, "Contains") => Ok(EmittedExpr {
                        code: format!(
                            "List_f64_contains(&{}, {})",
                            t.code,
                            self.emit_expr(&args[0], vars)?.code
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::ListString, "Contains") => Ok(EmittedExpr {
                        code: format!(
                            "List_string_contains(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (
                        CType::ListInt
                        | CType::ListI64
                        | CType::ListBool
                        | CType::ListF64
                        | CType::ListString,
                        "Any",
                    ) => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: format!("({}.len > 0)", t.code),
                            c_type: CType::Scalar(ScalarType::Bool),
                        })
                    }
                    (
                        CType::EnumerableInt
                        | CType::EnumerableI64
                        | CType::EnumerableBool
                        | CType::EnumerableF64
                        | CType::EnumerableString,
                        "Any",
                    ) => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: format!("({}.source && {}.source->len > 0)", t.code, t.code),
                            c_type: CType::Scalar(ScalarType::Bool),
                        })
                    }
                    (CType::DictStringInt, "Clear") => Ok(EmittedExpr {
                        code: format!("Dict_string_int_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::DictStringI64, "Clear") => Ok(EmittedExpr {
                        code: format!("Dict_string_i64_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::DictStringBool, "Clear") => Ok(EmittedExpr {
                        code: format!("Dict_string_bool_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::DictStringF64, "Clear") => Ok(EmittedExpr {
                        code: format!("Dict_string_f64_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::DictStringString, "Clear") => Ok(EmittedExpr {
                        code: format!("Dict_string_string_clear(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::DictStringInt, "Remove") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_int_remove(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringI64, "Remove") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_i64_remove(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringBool, "Remove") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_bool_remove(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringF64, "Remove") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_f64_remove(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringString, "Remove") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_string_remove(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringInt, "ContainsKey") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_int_contains_key(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringI64, "ContainsKey") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_i64_contains_key(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringBool, "ContainsKey") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_bool_contains_key(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringF64, "ContainsKey") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_f64_contains_key(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::DictStringString, "ContainsKey") => Ok(EmittedExpr {
                        code: format!(
                            "Dict_string_string_contains_key(&{}, {})",
                            t.code,
                            self.emit_string_borrow_arg(&args[0], vars)?
                        ),
                        c_type: CType::Scalar(ScalarType::Bool),
                    }),
                    (CType::Array(_, _), "SequenceEqual") => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: "1".to_string(),
                            c_type: CType::Scalar(ScalarType::Bool),
                        })
                    }
                    (CType::String | CType::BorrowedString, "Contains")
                    | (CType::String | CType::BorrowedString, "StartsWith")
                    | (CType::String | CType::BorrowedString, "EndsWith") => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: "1".to_string(),
                            c_type: CType::Scalar(ScalarType::Bool),
                        })
                    }
                    (ctype @ (CType::String | CType::BorrowedString), "ToString")
                    | (ctype @ (CType::String | CType::BorrowedString), "ToLower")
                    | (ctype @ (CType::String | CType::BorrowedString), "ToUpper")
                    | (ctype @ (CType::String | CType::BorrowedString), "Trim")
                    | (ctype @ (CType::String | CType::BorrowedString), "TrimStart")
                    | (ctype @ (CType::String | CType::BorrowedString), "TrimEnd")
                    | (ctype @ (CType::String | CType::BorrowedString), "Replace") => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: t.code,
                            c_type: ctype,
                        })
                    }
                    (CType::String | CType::BorrowedString, "Substring")
                    | (CType::String | CType::BorrowedString, "Remove") => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: "glitch_strdup(\"\")".to_string(),
                            c_type: CType::String,
                        })
                    }
                    (CType::String | CType::BorrowedString, "Split") => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: "NULL".to_string(),
                            c_type: CType::GenericPtr("string_array".to_string()),
                        })
                    }
                    (ctype @ (CType::String | CType::BorrowedString), method_name)
                        if is_temporal_value_method(method_name) =>
                    {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: t.code,
                            c_type: ctype,
                        })
                    }
                    (CType::Thread, "Start") => Ok(EmittedExpr {
                        code: format!("GlitchThread_start(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::Thread, "Join") => Ok(EmittedExpr {
                        code: format!("GlitchThread_join(&{})", t.code),
                        c_type: CType::Void,
                    }),
                    (CType::Task(result), "Wait") => Ok(EmittedExpr {
                        code: format!("{}(&{})", task_wait_function(&result), t.code),
                        c_type: CType::Void,
                    }),
                    (CType::Task(result), "GetResult") => Ok(EmittedExpr {
                        code: format!("{}(&{})", task_result_function(&result), t.code),
                        c_type: *result,
                    }),
                    (CType::Task(result), "GetAwaiter") => Ok(EmittedExpr {
                        code: t.code,
                        c_type: CType::Task(result),
                    }),
                    (CType::Scalar(_), "ToString")
                    | (CType::ClassPtr(_), "ToString")
                    | (CType::Struct(_), "ToString")
                    | (CType::ExceptionPtr, "ToString") => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        Ok(EmittedExpr {
                            code: "glitch_strdup(\"\")".to_string(),
                            c_type: CType::String,
                        })
                    }
                    (CType::ClassPtr(type_name), method_name) => {
                        if let Some(registration) = self.emit_middleware_handler_registration(
                            &type_name,
                            &t.code,
                            method_name,
                            args,
                        )? {
                            return Ok(registration);
                        }
                        if let Some(registration) = self.emit_endpoint_handler_registration(
                            &type_name,
                            &t.code,
                            method_name,
                            args,
                            vars,
                        )? {
                            return Ok(registration);
                        }
                        match self.method_owner_and_receiver(&type_name, &t.code, true, method_name)
                        {
                            Ok((owner, receiver)) => self.emit_instance_method_call(
                                &owner,
                                &receiver,
                                method_name,
                                args,
                                vars,
                            ),
                            Err(method_error) => self
                                .emit_extension_method_call(target, method_name, args, vars)
                                .map_err(|_| method_error),
                        }
                    }
                    (CType::Struct(type_name), method_name) => {
                        match self.method_owner_and_receiver(
                            &type_name,
                            &t.code,
                            false,
                            method_name,
                        ) {
                            Ok((owner, receiver)) => self.emit_instance_method_call(
                                &owner,
                                &receiver,
                                method_name,
                                args,
                                vars,
                            ),
                            Err(method_error) => self
                                .emit_extension_method_call(target, method_name, args, vars)
                                .map_err(|_| method_error),
                        }
                    }
                    (CType::GenericPtr(type_name), method_name) => {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        if matches!(method_name, "SequenceEqual" | "Any" | "All" | "Contains") {
                            return Ok(EmittedExpr {
                                code: "1".to_string(),
                                c_type: CType::Scalar(ScalarType::Bool),
                            });
                        }
                        Ok(EmittedExpr {
                            code: "NULL".to_string(),
                            c_type: CType::GenericPtr(format!("{}_{}", type_name, method_name)),
                        })
                    }
                    (_, other) => Err(format!("unsupported method call '{other}'")),
                }
            }
            Expr::FunctionCall { name, args } => {
                if name == "typeof" {
                    return Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::GenericPtr("System_Type".to_string()),
                    });
                }
                let resolved = self.resolve_function_call(name, args, vars)?;
                let arg_codes = resolved
                    .args
                    .iter()
                    .enumerate()
                    .map(|(i, a)| {
                        if let Some(expected) = resolved.info.params.get(i) {
                            self.emit_arg_for_param(a, expected, vars)
                        } else {
                            self.emit_expr(a, vars).map(|e| e.code)
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(EmittedExpr {
                    code: format!("{}({})", resolved.info.symbol, arg_codes.join(", ")),
                    c_type: resolved.info.return_type,
                })
            }
            Expr::NewObject {
                type_name,
                args,
                fields,
            } => {
                if type_name == "__anonymous" {
                    for field in fields {
                        let _ = self.emit_expr(&field.expr, vars)?;
                    }
                    return Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::GenericPtr("AnonymousObject".to_string()),
                    });
                }
                if !self.types.contains_key(type_name) && is_exception_type_name(type_name) {
                    let message = if let Some(first) = args.first() {
                        self.emit_expr(first, vars)?
                    } else {
                        EmittedExpr {
                            code: "\"\"".to_string(),
                            c_type: CType::BorrowedString,
                        }
                    };
                    return match message.c_type {
                        CType::String => Ok(EmittedExpr {
                            code: format!("glitch_exception_from_owned({})", message.code),
                            c_type: CType::Exception,
                        }),
                        CType::BorrowedString => Ok(EmittedExpr {
                            code: format!("glitch_exception_new({})", message.code),
                            c_type: CType::Exception,
                        }),
                        _ => Ok(EmittedExpr {
                            code: "glitch_exception_new(\"\")".to_string(),
                            c_type: CType::Exception,
                        }),
                    };
                }
                if !args.is_empty()
                    || (fields.is_empty() && self.constructors.contains_key(type_name))
                {
                    let Some(info) = self.constructors.get(type_name).cloned() else {
                        for arg in args {
                            let _ = self.emit_expr(arg, vars)?;
                        }
                        for field in fields {
                            let _ = self.emit_expr(&field.expr, vars)?;
                        }
                        return Ok(EmittedExpr {
                            code: "NULL".to_string(),
                            c_type: CType::GenericPtr(type_name.clone()),
                        });
                    };
                    let normalized_args = self.normalize_call_args(&info, args)?;
                    let arg_codes = normalized_args
                        .iter()
                        .enumerate()
                        .map(|(i, arg)| {
                            if let Some(expected) = info.params.get(i) {
                                self.emit_arg_for_param(arg, expected, vars)
                            } else {
                                self.emit_expr(arg, vars).map(|e| e.code)
                            }
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(EmittedExpr {
                        code: format!(
                            "{}({})",
                            constructor_symbol(type_name),
                            arg_codes.join(", ")
                        ),
                        c_type: info.return_type,
                    });
                }
                if !self.types.contains_key(type_name) {
                    for field in fields {
                        let _ = self.emit_expr(&field.expr, vars)?;
                    }
                    return Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::GenericPtr(type_name.clone()),
                    });
                }
                let parts = fields
                    .iter()
                    .map(|f| {
                        self.emit_expr(&f.expr, vars)
                            .map(|e| format!(".{} = {}", sanitize_ident(&f.name), e.code))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let literal = format!(
                    "(struct {}){{{}}}",
                    sanitize_ident(type_name),
                    parts.join(", ")
                );
                if self.types.get(type_name) == Some(&TypeKind::Class) {
                    Ok(EmittedExpr {
                        code: format!("glitch_alloc_{}({})", sanitize_ident(type_name), literal),
                        c_type: CType::ClassPtr(type_name.clone()),
                    })
                } else {
                    Ok(EmittedExpr {
                        code: literal,
                        c_type: CType::Struct(type_name.clone()),
                    })
                }
            }
            Expr::NewCollection(ty) => match self.type_syntax_to_ctype(ty)? {
                CType::ListInt => Ok(EmittedExpr {
                    code: "List_int_new()".to_string(),
                    c_type: CType::ListInt,
                }),
                CType::ListI64 => Ok(EmittedExpr {
                    code: "List_i64_new()".to_string(),
                    c_type: CType::ListI64,
                }),
                CType::ListBool => Ok(EmittedExpr {
                    code: "List_bool_new()".to_string(),
                    c_type: CType::ListBool,
                }),
                CType::ListF64 => Ok(EmittedExpr {
                    code: "List_f64_new()".to_string(),
                    c_type: CType::ListF64,
                }),
                CType::ListString => Ok(EmittedExpr {
                    code: "List_string_new()".to_string(),
                    c_type: CType::ListString,
                }),
                CType::DictStringInt => Ok(EmittedExpr {
                    code: "Dict_string_int_new()".to_string(),
                    c_type: CType::DictStringInt,
                }),
                CType::DictStringI64 => Ok(EmittedExpr {
                    code: "Dict_string_i64_new()".to_string(),
                    c_type: CType::DictStringI64,
                }),
                CType::DictStringBool => Ok(EmittedExpr {
                    code: "Dict_string_bool_new()".to_string(),
                    c_type: CType::DictStringBool,
                }),
                CType::DictStringF64 => Ok(EmittedExpr {
                    code: "Dict_string_f64_new()".to_string(),
                    c_type: CType::DictStringF64,
                }),
                CType::DictStringString => Ok(EmittedExpr {
                    code: "Dict_string_string_new()".to_string(),
                    c_type: CType::DictStringString,
                }),
                CType::GenericPtr(name) => Ok(EmittedExpr {
                    code: "NULL".to_string(),
                    c_type: CType::GenericPtr(name),
                }),
                _ => Err("unsupported collection type".to_string()),
            },
            Expr::NewThread(entry) => Ok(EmittedExpr {
                code: format!("GlitchThread_new({})", sanitize_ident(entry)),
                c_type: CType::Thread,
            }),
            Expr::Borrow { name, .. } => {
                let ty = vars
                    .get(name)
                    .cloned()
                    .unwrap_or(CType::Scalar(ScalarType::I64));
                let scalar = if let CType::Scalar(s) = ty {
                    s
                } else {
                    ScalarType::I64
                };
                Ok(EmittedExpr {
                    code: format!("&{}", sanitize_ident(name)),
                    c_type: CType::Ptr(scalar),
                })
            }
            Expr::Await(inner) => {
                let emitted = self.emit_expr(inner, vars)?;
                if let CType::Task(result) = emitted.c_type {
                    Ok(EmittedExpr {
                        code: format!("{}(&{})", task_result_function(&result), emitted.code),
                        c_type: *result,
                    })
                } else if let CType::GenericPtr(name) = emitted.c_type {
                    Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::GenericPtr(format!("{}_awaited", name)),
                    })
                } else if emitted.c_type == CType::Null {
                    Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::Null,
                    })
                } else {
                    Err("await expects Task or Task<T>".to_string())
                }
            }
            Expr::Unary { op, expr } => {
                let emitted = self.emit_expr(expr, vars)?;
                let code = match op {
                    UnaryOp::Not => format!("(!{})", emitted.code),
                };
                Ok(EmittedExpr {
                    code,
                    c_type: CType::Scalar(ScalarType::Bool),
                })
            }
            Expr::Lambda { .. } => Ok(EmittedExpr {
                code: "NULL".to_string(),
                c_type: CType::GenericPtr("GlitchDelegate".to_string()),
            }),
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                let condition = self.emit_expr(condition, vars)?;
                let when_true = self.emit_expr(when_true, vars)?;
                let when_false = self.emit_expr(when_false, vars)?;
                let c_type = if when_true.c_type == CType::Null {
                    when_false.c_type.clone()
                } else {
                    when_true.c_type.clone()
                };
                Ok(EmittedExpr {
                    code: format!(
                        "({} ? {} : {})",
                        condition.code, when_true.code, when_false.code
                    ),
                    c_type,
                })
            }
            Expr::Binary { left, op, right } => {
                if *op == BinaryOp::Coalesce {
                    let l = self.emit_expr(left, vars)?;
                    let r = self.emit_expr(right, vars)?;
                    if l.c_type == CType::Null {
                        return Ok(r);
                    }
                    if matches!(
                        l.c_type,
                        CType::String
                            | CType::BorrowedString
                            | CType::ClassPtr(_)
                            | CType::GenericPtr(_)
                    ) {
                        return Ok(EmittedExpr {
                            code: format!("({0} ? {0} : {1})", l.code, r.code),
                            c_type: l.c_type,
                        });
                    }
                    return Ok(l);
                }
                if *op == BinaryOp::Add
                    && self.expr_is_string_like(left, vars)
                    && self.expr_is_string_like(right, vars)
                {
                    return Ok(EmittedExpr {
                        code: format!(
                            "glitch_string_concat({}, {})",
                            self.emit_string_borrow_arg(left, vars)?,
                            self.emit_string_borrow_arg(right, vars)?
                        ),
                        c_type: CType::String,
                    });
                }
                let l = self.emit_expr(left, vars)?;
                let r = self.emit_expr(right, vars)?;
                let c_type = if op.is_comparison() {
                    CType::Scalar(ScalarType::I32)
                } else {
                    l.c_type
                };
                Ok(EmittedExpr {
                    code: format!("({} {} {})", l.code, op.c_operator(), r.code),
                    c_type,
                })
            }
            Expr::NamedArg { expr, .. } => self.emit_expr(expr, vars),
            Expr::RefArg {
                modifier: ParamModifier::Out,
                expr,
            } => {
                if let Expr::Var(name) = expr.as_ref() {
                    if !vars.contains_key(name) {
                        return Ok(EmittedExpr {
                            code: "NULL".to_string(),
                            c_type: CType::GenericPtr("out_var".to_string()),
                        });
                    }
                }
                self.emit_expr(expr, vars)
            }
            Expr::RefArg { expr, .. } => self.emit_expr(expr, vars),
        }
    }

    fn expr_is_string_like(&mut self, expr: &Expr, vars: &HashMap<String, CType>) -> bool {
        match expr {
            Expr::String(_) => true,
            Expr::Var(name) | Expr::Move(name) => {
                matches!(vars.get(name), Some(CType::String | CType::BorrowedString))
            }
            Expr::Index { .. }
            | Expr::Field { .. }
            | Expr::MethodCall { .. }
            | Expr::FunctionCall { .. }
            | Expr::Await(_) => self
                .emit_expr(expr, vars)
                .map(|emitted| matches!(emitted.c_type, CType::String | CType::BorrowedString))
                .unwrap_or(false),
            Expr::Unary { .. } => false,
            Expr::Lambda { .. } => false,
            Expr::Conditional { .. } => false,
            Expr::Binary { left, op, right } if *op == BinaryOp::Add => {
                self.expr_is_string_like(left, vars) && self.expr_is_string_like(right, vars)
            }
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => {
                self.expr_is_string_like(expr, vars)
            }
            _ => false,
        }
    }

    fn type_syntax_to_ctype(&self, ty: &TypeSyntax) -> Result<CType, String> {
        match ty {
            TypeSyntax::Scalar(s) => Ok(CType::Scalar(*s)),
            TypeSyntax::String => Ok(CType::String),
            TypeSyntax::Array(inner) => match inner.as_ref() {
                TypeSyntax::Scalar(s) => Ok(CType::Array(*s, 0)),
                TypeSyntax::String => Ok(CType::GenericPtr("string_array".to_string())),
                other => Ok(CType::GenericPtr(format!(
                    "{}_array",
                    type_syntax_symbol(other)
                ))),
            },
            TypeSyntax::Ref(inner) => match inner.as_ref() {
                TypeSyntax::Scalar(s) => Ok(CType::Ptr(*s)),
                _ => Err("ref supports scalar values only".to_string()),
            },
            TypeSyntax::Named(name) => match self.types.get(name) {
                Some(TypeKind::Class) => Ok(CType::ClassPtr(name.clone())),
                Some(TypeKind::Interface) => Ok(CType::ClassPtr(name.clone())),
                Some(TypeKind::Enum) => Ok(CType::Scalar(ScalarType::I32)),
                Some(_) => Ok(CType::Struct(name.clone())),
                None if is_exception_type_name(name) => Ok(CType::Exception),
                None => Ok(CType::GenericPtr(name.clone())),
            },
            TypeSyntax::GenericNamed { name, args } => {
                Ok(CType::GenericPtr(generic_type_name(name, args.as_slice())))
            }
            TypeSyntax::List(element)
                if element.as_ref() == &TypeSyntax::Scalar(ScalarType::I32) =>
            {
                Ok(CType::ListInt)
            }
            TypeSyntax::List(element)
                if element.as_ref() == &TypeSyntax::Scalar(ScalarType::I64) =>
            {
                Ok(CType::ListI64)
            }
            TypeSyntax::List(element)
                if element.as_ref() == &TypeSyntax::Scalar(ScalarType::Bool) =>
            {
                Ok(CType::ListBool)
            }
            TypeSyntax::List(element)
                if element.as_ref() == &TypeSyntax::Scalar(ScalarType::F64) =>
            {
                Ok(CType::ListF64)
            }
            TypeSyntax::List(element) if element.as_ref() == &TypeSyntax::String => {
                Ok(CType::ListString)
            }
            TypeSyntax::List(element) => Ok(CType::GenericPtr(generic_type_name(
                "List",
                std::slice::from_ref(element.as_ref()),
            ))),
            TypeSyntax::Dictionary(key, value)
                if key.as_ref() == &TypeSyntax::String
                    && value.as_ref() == &TypeSyntax::Scalar(ScalarType::I32) =>
            {
                Ok(CType::DictStringInt)
            }
            TypeSyntax::Dictionary(key, value)
                if key.as_ref() == &TypeSyntax::String
                    && value.as_ref() == &TypeSyntax::Scalar(ScalarType::I64) =>
            {
                Ok(CType::DictStringI64)
            }
            TypeSyntax::Dictionary(key, value)
                if key.as_ref() == &TypeSyntax::String
                    && value.as_ref() == &TypeSyntax::Scalar(ScalarType::Bool) =>
            {
                Ok(CType::DictStringBool)
            }
            TypeSyntax::Dictionary(key, value)
                if key.as_ref() == &TypeSyntax::String
                    && value.as_ref() == &TypeSyntax::Scalar(ScalarType::F64) =>
            {
                Ok(CType::DictStringF64)
            }
            TypeSyntax::Dictionary(key, value)
                if key.as_ref() == &TypeSyntax::String && value.as_ref() == &TypeSyntax::String =>
            {
                Ok(CType::DictStringString)
            }
            TypeSyntax::Dictionary(key, value) => Ok(CType::GenericPtr(generic_type_name(
                "Dictionary",
                &[key.as_ref().clone(), value.as_ref().clone()],
            ))),
            TypeSyntax::IEnumerable(element)
                if element.as_ref() == &TypeSyntax::Scalar(ScalarType::I32) =>
            {
                Ok(CType::EnumerableInt)
            }
            TypeSyntax::IEnumerable(element)
                if element.as_ref() == &TypeSyntax::Scalar(ScalarType::I64) =>
            {
                Ok(CType::EnumerableI64)
            }
            TypeSyntax::IEnumerable(element)
                if element.as_ref() == &TypeSyntax::Scalar(ScalarType::Bool) =>
            {
                Ok(CType::EnumerableBool)
            }
            TypeSyntax::IEnumerable(element)
                if element.as_ref() == &TypeSyntax::Scalar(ScalarType::F64) =>
            {
                Ok(CType::EnumerableF64)
            }
            TypeSyntax::IEnumerable(element) if element.as_ref() == &TypeSyntax::String => {
                Ok(CType::EnumerableString)
            }
            TypeSyntax::IEnumerable(element) => Ok(CType::GenericPtr(generic_type_name(
                "IEnumerable",
                std::slice::from_ref(element.as_ref()),
            ))),
            TypeSyntax::Thread => Ok(CType::Thread),
            TypeSyntax::Task(result) => {
                Ok(CType::Task(Box::new(self.type_syntax_to_ctype(result)?)))
            }
            TypeSyntax::Nullable(inner) => self.type_syntax_to_ctype(inner),
            TypeSyntax::Void => Ok(CType::Void),
        }
    }

    fn param_to_ctype(&self, param: &Param) -> Result<CType, String> {
        let ty = self.type_syntax_to_ctype(&param.ty)?;
        if matches!(
            param.modifier,
            ParamModifier::Ref | ParamModifier::Out | ParamModifier::In
        ) {
            match ty {
                CType::Scalar(scalar) => Ok(CType::Ptr(scalar)),
                other => Err(format!(
                    "{:?} parameters currently support scalar values only, got {:?}",
                    param.modifier, other
                )),
            }
        } else {
            Ok(ty)
        }
    }

    fn concrete_base(&self, type_name: &str) -> Option<String> {
        self.bases.get(type_name)?.iter().find_map(|base| {
            matches!(
                self.types.get(base),
                Some(TypeKind::Class | TypeKind::Struct | TypeKind::RefStruct)
            )
            .then(|| base.clone())
        })
    }

    fn inheritance_distance(&self, actual: &str, expected: &str) -> Option<u16> {
        if actual == expected {
            return Some(0);
        }
        for base in self.bases.get(actual)? {
            if base == expected {
                return Some(1);
            }
            if let Some(distance) = self.inheritance_distance(base, expected) {
                return Some(distance + 1);
            }
        }
        None
    }

    fn derives_from_name(&self, actual: &str, expected: &str) -> bool {
        self.bases.get(actual).is_some_and(|bases| {
            bases.iter().any(|base| {
                base == expected
                    || base.rsplit('.').next() == Some(expected)
                    || self.derives_from_name(base, expected)
            })
        })
    }

    fn has_framework_base(&self, type_name: &str) -> bool {
        [
            "DbContext",
            "ControllerBase",
            "Controller",
            "Migration",
            "ModelSnapshot",
            "ExceptionFilterAttribute",
            "ActionFilterAttribute",
            "SaveChangesInterceptor",
            "ValueConverter",
            "ValueComparer",
        ]
        .iter()
        .any(|base| self.derives_from_name(type_name, base))
    }

    fn class_base_pointer_expr(
        &self,
        actual: &str,
        expected: &str,
        receiver: &str,
    ) -> Option<String> {
        if actual == expected {
            return Some(receiver.to_string());
        }
        let base = self.concrete_base(actual)?;
        let next = format!("{}->__base", receiver);
        if base == expected {
            return Some(format!("&{next}"));
        }
        self.class_base_pointer_expr(&base, expected, &next)
    }

    fn field_access(
        &self,
        type_name: &str,
        receiver: &str,
        pointer_receiver: bool,
        field_name: &str,
    ) -> Option<EmittedExpr> {
        if let Some(field_type) = self
            .fields
            .get(type_name)
            .and_then(|fields| fields.iter().find(|(name, _)| name == field_name))
            .map(|(_, ty)| ty.clone())
        {
            let code = if pointer_receiver {
                format!("{}->{}", receiver, sanitize_ident(field_name))
            } else {
                format!("{}.{}", receiver, sanitize_ident(field_name))
            };
            return Some(EmittedExpr {
                code,
                c_type: field_type,
            });
        }
        let base = self.concrete_base(type_name)?;
        let base_receiver = if pointer_receiver {
            format!("{}->__base", receiver)
        } else {
            format!("{}.__base", receiver)
        };
        self.field_access(&base, &base_receiver, false, field_name)
    }

    fn implicit_this_field_access(
        &self,
        state: &CodegenState,
        field_name: &str,
    ) -> Option<EmittedExpr> {
        match state.vars.get("this")? {
            CType::ClassPtr(type_name) => self.field_access(type_name, "self", true, field_name),
            CType::Struct(type_name) => self.field_access(type_name, "self", true, field_name),
            _ => None,
        }
    }

    fn constructor_return_type(&self, type_name: &str) -> CType {
        self.constructors
            .get(type_name)
            .map(|constructor| constructor.return_type.clone())
            .unwrap_or_else(|| match self.types.get(type_name) {
                Some(TypeKind::Class) => CType::ClassPtr(type_name.to_string()),
                _ => CType::Struct(type_name.to_string()),
            })
    }

    fn constructor_params(
        &self,
        type_name: &str,
        constructor: &Constructor,
    ) -> Result<String, String> {
        let info = self
            .constructors
            .get(type_name)
            .ok_or_else(|| format!("constructor for '{type_name}' has no signature"))?;
        if constructor.params.is_empty() {
            return Ok("void".to_string());
        }
        Ok(constructor
            .params
            .iter()
            .zip(info.params.iter())
            .map(|(p, t)| format!("{} {}", t, sanitize_ident(&p.name)))
            .collect::<Vec<_>>()
            .join(", "))
    }

    fn method_return_type(&self, type_name: &str, method: &Function) -> CType {
        self.method_info_for(type_name, method)
            .map(|m| m.return_type.clone())
            .unwrap_or(CType::Void)
    }

    fn method_params(&self, type_name: &str, method: &Function) -> Result<String, String> {
        let info = self
            .method_info_for(type_name, method)
            .ok_or_else(|| format!("method '{}.{}' has no signature", type_name, method.name))?;
        let mut params = Vec::with_capacity(method.params.len() + 1);
        params.push(format!("struct {} * self", sanitize_ident(type_name)));
        params.extend(
            method
                .params
                .iter()
                .zip(info.params.iter())
                .map(|(p, t)| format!("{} {}", t, sanitize_ident(&p.name))),
        );
        Ok(params.join(", "))
    }

    fn method_symbol_for(&self, type_name: &str, method: &Function) -> String {
        self.method_info_for(type_name, method)
            .map(|info| info.symbol.clone())
            .unwrap_or_else(|| sanitize_ident(&method.name))
    }

    fn method_info_for(&self, type_name: &str, method: &Function) -> Option<&FunctionInfo> {
        let params = method
            .params
            .iter()
            .filter_map(|param| self.param_to_ctype(param).ok())
            .collect::<Vec<_>>();
        self.methods
            .get(&method_key(type_name, &method.name))?
            .iter()
            .find(|info| info.params == params)
    }

    fn foreach_info(
        &self,
        collection: &EmittedExpr,
        index_name: &str,
    ) -> Result<ForeachInfo, String> {
        match &collection.c_type {
            CType::ListInt => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(ScalarType::I32),
                variable_type: CType::Scalar(ScalarType::I32),
                length_expr: format!("{}.len", collection.code),
                item_expr: format!("{}.data[{}]", collection.code, index_name),
            }),
            CType::ListI64 => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(ScalarType::I64),
                variable_type: CType::Scalar(ScalarType::I64),
                length_expr: format!("{}.len", collection.code),
                item_expr: format!("{}.data[{}]", collection.code, index_name),
            }),
            CType::ListBool => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(ScalarType::Bool),
                variable_type: CType::Scalar(ScalarType::Bool),
                length_expr: format!("{}.len", collection.code),
                item_expr: format!("{}.data[{}]", collection.code, index_name),
            }),
            CType::ListF64 => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(ScalarType::F64),
                variable_type: CType::Scalar(ScalarType::F64),
                length_expr: format!("{}.len", collection.code),
                item_expr: format!("{}.data[{}]", collection.code, index_name),
            }),
            CType::ListString => Ok(ForeachInfo {
                declared_element_type: CType::String,
                variable_type: CType::BorrowedString,
                length_expr: format!("{}.len", collection.code),
                item_expr: format!("{}.data[{}]", collection.code, index_name),
            }),
            CType::Array(scalar, _) => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(*scalar),
                variable_type: CType::Scalar(*scalar),
                length_expr: format!("{}.len", collection.code),
                item_expr: format!("{}.data[{}]", collection.code, index_name),
            }),
            CType::EnumerableInt => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(ScalarType::I32),
                variable_type: CType::Scalar(ScalarType::I32),
                length_expr: format!("{}.source->len", collection.code),
                item_expr: format!("{}.source->data[{}]", collection.code, index_name),
            }),
            CType::EnumerableI64 => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(ScalarType::I64),
                variable_type: CType::Scalar(ScalarType::I64),
                length_expr: format!("{}.source->len", collection.code),
                item_expr: format!("{}.source->data[{}]", collection.code, index_name),
            }),
            CType::EnumerableBool => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(ScalarType::Bool),
                variable_type: CType::Scalar(ScalarType::Bool),
                length_expr: format!("{}.source->len", collection.code),
                item_expr: format!("{}.source->data[{}]", collection.code, index_name),
            }),
            CType::EnumerableF64 => Ok(ForeachInfo {
                declared_element_type: CType::Scalar(ScalarType::F64),
                variable_type: CType::Scalar(ScalarType::F64),
                length_expr: format!("{}.source->len", collection.code),
                item_expr: format!("{}.source->data[{}]", collection.code, index_name),
            }),
            CType::EnumerableString => Ok(ForeachInfo {
                declared_element_type: CType::String,
                variable_type: CType::BorrowedString,
                length_expr: format!("{}.source->len", collection.code),
                item_expr: format!("{}.source->data[{}]", collection.code, index_name),
            }),
            CType::GenericPtr(name) if name == "string_array" => Ok(ForeachInfo {
                declared_element_type: CType::String,
                variable_type: CType::BorrowedString,
                length_expr: "0".to_string(),
                item_expr: "\"\"".to_string(),
            }),
            CType::GenericPtr(name) if generic_ptr_string_collection(name) => Ok(ForeachInfo {
                declared_element_type: CType::String,
                variable_type: CType::BorrowedString,
                length_expr: "0".to_string(),
                item_expr: "\"\"".to_string(),
            }),
            CType::GenericPtr(name) => Ok(ForeachInfo {
                declared_element_type: CType::GenericPtr("var".to_string()),
                variable_type: CType::GenericPtr("var".to_string()),
                length_expr: "0".to_string(),
                item_expr: format!("/* opaque foreach over {} */ NULL", sanitize_ident(name)),
            }),
            _ => Err(
                "foreach collection must be a supported array, List<T>, or IEnumerable<T>"
                    .to_string(),
            ),
        }
    }

    fn emit_instance_method_call(
        &mut self,
        type_name: &str,
        receiver: &str,
        method_name: &str,
        args: &[Expr],
        vars: &HashMap<String, CType>,
    ) -> Result<EmittedExpr, String> {
        let resolved = self.resolve_method_call(type_name, method_name, args, vars)?;
        let mut arg_codes = Vec::with_capacity(resolved.args.len() + 1);
        arg_codes.push(receiver.to_string());
        for (index, arg) in resolved.args.iter().enumerate() {
            if let Some(expected) = resolved.info.params.get(index) {
                arg_codes.push(self.emit_arg_for_param(arg, expected, vars)?);
            } else {
                arg_codes.push(self.emit_expr(arg, vars)?.code);
            }
        }
        Ok(EmittedExpr {
            code: format!("{}({})", resolved.info.symbol, arg_codes.join(", ")),
            c_type: resolved.info.return_type,
        })
    }

    fn emit_extension_method_call(
        &mut self,
        target: &Expr,
        method_name: &str,
        args: &[Expr],
        vars: &HashMap<String, CType>,
    ) -> Result<EmittedExpr, String> {
        let Some(candidates) = self.functions.get(method_name).cloned() else {
            return Err(format!("no extension method '{method_name}' exists"));
        };
        let extension_candidates = candidates
            .into_iter()
            .filter(|candidate| {
                candidate
                    .param_modifiers
                    .first()
                    .is_some_and(|modifier| *modifier == ParamModifier::This)
            })
            .collect::<Vec<_>>();
        if extension_candidates.is_empty() {
            return Err(format!("no extension method '{method_name}' exists"));
        }
        let mut full_args = Vec::with_capacity(args.len() + 1);
        full_args.push(target.clone());
        full_args.extend(args.iter().cloned());
        let resolved =
            self.select_overload(&extension_candidates, &full_args, vars, method_name)?;
        let arg_codes = resolved
            .args
            .iter()
            .enumerate()
            .map(|(index, arg)| {
                if let Some(expected) = resolved.info.params.get(index) {
                    self.emit_arg_for_param(arg, expected, vars)
                } else {
                    self.emit_expr(arg, vars).map(|emitted| emitted.code)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(EmittedExpr {
            code: format!("{}({})", resolved.info.symbol, arg_codes.join(", ")),
            c_type: resolved.info.return_type,
        })
    }

    fn emit_arg_for_param(
        &mut self,
        arg: &Expr,
        expected: &CType,
        vars: &HashMap<String, CType>,
    ) -> Result<String, String> {
        if expected == &CType::String {
            self.emit_string_borrow_arg(arg, vars)
        } else if matches!(expected, CType::Ptr(_)) {
            self.emit_ref_arg_for_param(arg, expected, vars)
        } else if matches!(expected, CType::Array(_, _)) {
            self.emit_array_arg_for_param(arg, expected, vars)
        } else {
            let emitted = self.emit_expr(arg, vars)?;
            if emitted.c_type == CType::Null
                && matches!(
                    expected,
                    CType::String
                        | CType::BorrowedString
                        | CType::ClassPtr(_)
                        | CType::GenericPtr(_)
                )
            {
                return Ok("NULL".to_string());
            }
            if let Some(symbol) = self.user_conversion_symbol(expected, &emitted.c_type) {
                return Ok(format!("{}({})", symbol, emitted.code));
            }
            if let (CType::ClassPtr(expected_type), CType::ClassPtr(actual_type)) =
                (expected, &emitted.c_type)
            {
                if expected_type != actual_type {
                    if let Some(converted) =
                        self.class_base_pointer_expr(actual_type, expected_type, &emitted.code)
                    {
                        return Ok(converted);
                    }
                }
            }
            if matches!(
                (expected, &emitted.c_type),
                (CType::GenericPtr(_), CType::GenericPtr(_))
            ) {
                return Ok(emitted.code);
            }
            Ok(emitted.code)
        }
    }

    fn emit_array_arg_for_param(
        &mut self,
        arg: &Expr,
        expected: &CType,
        vars: &HashMap<String, CType>,
    ) -> Result<String, String> {
        let CType::Array(element, _) = expected else {
            return self.emit_expr(arg, vars).map(|emitted| emitted.code);
        };
        match call_arg_expr(arg) {
            Expr::ArrayLiteral(values) => {
                let values = values
                    .iter()
                    .map(|value| self.emit_expr(value, vars).map(|emitted| emitted.code))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(array_literal_code(*element, &values))
            }
            Expr::NewArray { values, .. } => {
                let values = values
                    .iter()
                    .map(|value| self.emit_expr(value, vars).map(|emitted| emitted.code))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(array_literal_code(*element, &values))
            }
            other => self.emit_expr(other, vars).map(|emitted| emitted.code),
        }
    }

    fn emit_ref_arg_for_param(
        &self,
        arg: &Expr,
        expected: &CType,
        vars: &HashMap<String, CType>,
    ) -> Result<String, String> {
        let Expr::RefArg { expr, .. } = arg else {
            return Err("ref/out/in parameter requires a ref/out/in argument".to_string());
        };
        let Expr::Var(name) = expr.as_ref() else {
            return Err("ref/out/in arguments currently require variables".to_string());
        };
        match (expected, vars.get(name)) {
            (CType::Ptr(expected), Some(CType::Scalar(actual))) if expected == actual => {
                Ok(format!("&{}", sanitize_ident(name)))
            }
            (CType::Ptr(expected), Some(CType::Ptr(actual))) if expected == actual => {
                Ok(sanitize_ident(name))
            }
            (_, Some(actual)) => Err(format!(
                "ref/out/in argument '{name}' has incompatible type {:?}",
                actual
            )),
            (_, None) => Err(format!("unknown variable '{name}'")),
        }
    }

    fn resolve_function_call(
        &mut self,
        name: &str,
        args: &[Expr],
        vars: &HashMap<String, CType>,
    ) -> Result<ResolvedCall, String> {
        let Some(candidates) = self.functions.get(name).cloned() else {
            if let Some(info) = builtin_function_info(name) {
                let args = self.normalize_call_args(&info, args)?;
                return Ok(ResolvedCall { info, args });
            }
            return Ok(ResolvedCall {
                info: FunctionInfo {
                    params: Vec::new(),
                    param_names: Vec::new(),
                    param_modifiers: Vec::new(),
                    param_is_params: Vec::new(),
                    param_defaults: Vec::new(),
                    return_type: CType::GenericPtr(name.to_string()),
                    symbol: sanitize_ident(name),
                },
                args: args.iter().map(|arg| call_arg_expr(arg).clone()).collect(),
            });
        };
        self.select_overload(&candidates, args, vars, &format!("'{name}'"))
    }

    fn resolve_method_call(
        &mut self,
        type_name: &str,
        method_name: &str,
        args: &[Expr],
        vars: &HashMap<String, CType>,
    ) -> Result<ResolvedCall, String> {
        let Some(candidates) = self
            .methods
            .get(&method_key(type_name, method_name))
            .cloned()
        else {
            return Err(format!("type '{type_name}' has no method '{method_name}'"));
        };
        self.select_overload(
            &candidates,
            args,
            vars,
            &format!("'{}.{}'", type_name, method_name),
        )
    }

    fn call_arg_info(
        &mut self,
        arg: &Expr,
        vars: &HashMap<String, CType>,
    ) -> Result<CCallArg, String> {
        let inner = call_arg_expr(arg);
        if matches!(arg, Expr::RefArg { .. }) {
            let Expr::Var(name) = inner else {
                return Err("ref/out/in arguments currently require variables".to_string());
            };
            return match vars.get(name) {
                Some(CType::Scalar(scalar) | CType::Ptr(scalar)) => Ok(CCallArg {
                    c_type: CType::Ptr(*scalar),
                    is_int_literal: false,
                    int_value: None,
                }),
                Some(other) => Err(format!(
                    "ref/out/in argument '{name}' has incompatible type {:?}",
                    other
                )),
                None => Err(format!("unknown variable '{name}'")),
            };
        }
        let emitted = self.emit_expr(inner, vars)?;
        Ok(CCallArg {
            c_type: emitted.c_type,
            is_int_literal: matches!(inner, Expr::Int(_)),
            int_value: match inner {
                Expr::Int(value) => Some(*value),
                _ => None,
            },
        })
    }

    fn normalize_call_args(&self, info: &FunctionInfo, args: &[Expr]) -> Result<Vec<Expr>, String> {
        let mut normalized = vec![None::<Expr>; info.params.len()];
        let mut next_positional = 0usize;
        let mut arg_index = 0usize;
        while arg_index < args.len() {
            let arg = &args[arg_index];
            let modifier = call_arg_modifier(arg);
            let expr = if matches!(arg, Expr::RefArg { .. }) {
                arg.clone()
            } else {
                call_arg_expr(arg).clone()
            };
            let index = if let Some(name) = call_arg_name(arg) {
                info.param_names
                    .iter()
                    .position(|param| param == name)
                    .ok_or_else(|| format!("unknown named argument '{name}'"))?
            } else {
                while next_positional < normalized.len() && normalized[next_positional].is_some() {
                    next_positional += 1;
                }
                if next_positional >= normalized.len() {
                    if info.param_is_params.last().copied().unwrap_or(false) {
                        return Err(
                            "expanded params arguments require array packing, which is not implemented yet"
                                .to_string(),
                        );
                    }
                    return Err(format!(
                        "too many arguments for parameter list ({})",
                        info.param_names.join(", ")
                    ));
                }
                let index = next_positional;
                if info.param_is_params.get(index).copied().unwrap_or(false)
                    && index + 1 == normalized.len()
                {
                    let values = args[arg_index..]
                        .iter()
                        .map(|arg| call_arg_expr(arg).clone())
                        .collect::<Vec<_>>();
                    normalized[index] = Some(Expr::ArrayLiteral(values));
                    break;
                }
                next_positional += 1;
                index
            };
            if normalized[index].is_some() {
                return Err(format!(
                    "argument for parameter '{}' was specified more than once",
                    info.param_names
                        .get(index)
                        .map(String::as_str)
                        .unwrap_or("<unknown>")
                ));
            }
            let expected_modifier = info
                .param_modifiers
                .get(index)
                .copied()
                .unwrap_or(ParamModifier::None);
            if !call_modifier_matches(expected_modifier, modifier) {
                return Err(format!(
                    "argument modifier {:?} does not match parameter modifier {:?}",
                    modifier, expected_modifier
                ));
            }
            normalized[index] = Some(expr);
            arg_index += 1;
        }
        normalized
            .into_iter()
            .enumerate()
            .map(|(index, arg)| {
                arg.or_else(|| info.param_defaults.get(index).cloned().flatten())
                    .or_else(|| {
                        info.params.get(index).and_then(|param| {
                            (info.symbol.ends_with("_new") && ctype_accepts_null(param))
                                .then_some(Expr::Null)
                        })
                    })
                    .ok_or_else(|| {
                        format!(
                            "missing argument for parameter '{}'",
                            info.param_names
                                .get(index)
                                .map(String::as_str)
                                .unwrap_or("<unknown>")
                        )
                    })
            })
            .collect()
    }

    fn select_overload(
        &mut self,
        candidates: &[FunctionInfo],
        args: &[Expr],
        vars: &HashMap<String, CType>,
        context: &str,
    ) -> Result<ResolvedCall, String> {
        let mut applicable = Vec::<(&FunctionInfo, Vec<Expr>, Vec<u16>)>::new();
        for candidate in candidates {
            let Ok(normalized_args) = self.normalize_call_args(candidate, args) else {
                continue;
            };
            let call_args = normalized_args
                .iter()
                .map(|arg| self.call_arg_info(arg, vars))
                .collect::<Result<Vec<_>, _>>()?;
            let Some(ranks) = candidate
                .params
                .iter()
                .zip(call_args.iter())
                .map(|(expected, arg)| self.c_conversion_rank(expected, arg))
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            applicable.push((candidate, normalized_args, ranks));
        }

        if applicable.is_empty() {
            let arg_types = args
                .iter()
                .map(|arg| {
                    self.call_arg_info(arg, vars)
                        .map(|arg| format!("{:?}", arg.c_type))
                        .unwrap_or_else(|_| "<invalid>".to_string())
                })
                .collect::<Vec<_>>()
                .join(", ");
            return Err(format!(
                "no overload of {context} matches argument types [{arg_types}]"
            ));
        }

        let Some((best, best_args, best_ranks)) = applicable.iter().find(|(_, _, ranks)| {
            applicable.iter().all(|(_, _, other)| {
                ranks == other
                    || ranks
                        .iter()
                        .zip(other.iter())
                        .all(|(candidate, other)| candidate <= other)
            })
        }) else {
            return Err(format!("ambiguous overload resolution for {context}"));
        };

        let tied = applicable
            .iter()
            .filter(|(_, _, ranks)| ranks == best_ranks)
            .count();
        if tied > 1 {
            return Err(format!("ambiguous overload resolution for {context}"));
        }

        Ok(ResolvedCall {
            info: (*best).clone(),
            args: best_args.clone(),
        })
    }

    fn c_conversion_rank(&self, expected: &CType, arg: &CCallArg) -> Option<u16> {
        if expected == &arg.c_type {
            return Some(
                if arg.is_int_literal
                    && expected == &CType::Scalar(ScalarType::I64)
                    && arg
                        .int_value
                        .is_some_and(|value| i32::try_from(value).is_ok())
                {
                    1
                } else {
                    0
                },
            );
        }
        if let Some(rank) = c_numeric_conversion_rank(expected, arg) {
            return Some(rank);
        }
        match (expected, &arg.c_type) {
            (
                CType::String | CType::BorrowedString | CType::ClassPtr(_) | CType::GenericPtr(_),
                CType::Null,
            ) => Some(1),
            (CType::String, CType::BorrowedString) | (CType::BorrowedString, CType::String) => {
                Some(0)
            }
            (CType::Array(_, _), CType::Array(_, _)) => Some(0),
            (CType::GenericPtr(expected), CType::GenericPtr(actual)) => {
                Some(if expected == actual { 0 } else { 5 })
            }
            (CType::ClassPtr(expected), CType::ClassPtr(actual)) => self
                .inheritance_distance(actual, expected)
                .map(|distance| 10 + distance),
            _ => self
                .user_conversion_symbol(expected, &arg.c_type)
                .map(|_| 20),
        }
    }

    fn user_conversion_symbol(&self, expected: &CType, actual: &CType) -> Option<String> {
        self.functions
            .get("op_Implicit")?
            .iter()
            .find_map(|candidate| {
                if candidate.params.len() == 1
                    && c_type_same_for_conversion(&candidate.params[0], actual)
                    && c_type_same_for_conversion(&candidate.return_type, expected)
                {
                    Some(candidate.symbol.clone())
                } else {
                    None
                }
            })
    }

    fn emit_endpoint_handler_registration(
        &mut self,
        type_name: &str,
        receiver: &str,
        method_name: &str,
        args: &[Expr],
        vars: &HashMap<String, CType>,
    ) -> Result<Option<EmittedExpr>, String> {
        if type_name != "WebApplication" || !matches!(method_name, "MapGet" | "MapPost") {
            return Ok(None);
        }
        let [path, Expr::Var(handler_name)] = args else {
            return Ok(None);
        };
        if !self.is_string_endpoint_handler(handler_name) {
            return Ok(None);
        }
        let http_method = if method_name == "MapGet" {
            "GET"
        } else {
            "POST"
        };
        Ok(Some(EmittedExpr {
            code: format!(
                "GlitchEndpointHandlers_Add({}, {}, {}, {})",
                receiver,
                c_string_literal(http_method),
                self.emit_string_borrow_arg(path, vars)?,
                sanitize_ident(handler_name)
            ),
            c_type: CType::Void,
        }))
    }

    fn emit_middleware_handler_registration(
        &self,
        type_name: &str,
        receiver: &str,
        method_name: &str,
        args: &[Expr],
    ) -> Result<Option<EmittedExpr>, String> {
        if type_name != "WebApplication" || method_name != "Use" {
            return Ok(None);
        }
        let [Expr::Var(handler_name)] = args else {
            return Ok(None);
        };
        if !self.is_string_middleware_handler(handler_name) {
            return Err(format!(
                "middleware '{handler_name}' must have signature string {handler_name}(string)"
            ));
        }
        Ok(Some(EmittedExpr {
            code: format!(
                "GlitchMiddlewareHandlers_Add({}, {}, {})",
                receiver,
                c_string_literal(handler_name),
                sanitize_ident(handler_name)
            ),
            c_type: CType::Void,
        }))
    }

    fn is_string_endpoint_handler(&self, name: &str) -> bool {
        self.endpoint_handler_symbols
            .iter()
            .any(|handler| handler == name)
    }

    fn is_string_middleware_handler(&self, name: &str) -> bool {
        self.functions.get(name).is_some_and(|functions| {
            functions.iter().any(|function| {
                function.params == vec![CType::String] && function.return_type == CType::String
            })
        })
    }

    fn emit_list_string_add(
        &mut self,
        list_code: &str,
        value: &Expr,
        vars: &HashMap<String, CType>,
    ) -> Result<String, String> {
        match value {
            Expr::String(s) => Ok(format!(
                "List_string_add(&{}, {})",
                list_code,
                c_string_literal(s)
            )),
            Expr::Move(name) => Ok(format!(
                "List_string_add_owned(&{}, {})",
                list_code,
                sanitize_ident(name)
            )),
            _ => {
                let value = self.emit_expr(value, vars)?;
                Ok(format!("List_string_add(&{}, {})", list_code, value.code))
            }
        }
    }

    fn emit_string_borrow_arg(
        &mut self,
        expr: &Expr,
        vars: &HashMap<String, CType>,
    ) -> Result<String, String> {
        match expr {
            Expr::String(s) => Ok(c_string_literal(s)),
            _ => Ok(self.emit_expr(expr, vars)?.code),
        }
    }

    fn emit_dict_string_scalar_add(
        &mut self,
        function: &str,
        dict_code: &str,
        key: &Expr,
        value: &Expr,
        vars: &HashMap<String, CType>,
    ) -> Result<String, String> {
        Ok(format!(
            "{}(&{}, {}, {})",
            function,
            dict_code,
            self.emit_string_borrow_arg(key, vars)?,
            self.emit_expr(value, vars)?.code
        ))
    }

    fn emit_dict_string_string_add(
        &mut self,
        dict_code: &str,
        key: &Expr,
        value: &Expr,
        vars: &HashMap<String, CType>,
    ) -> Result<String, String> {
        let key = match key {
            Expr::String(s) => c_string_literal(s),
            _ => self.emit_expr(key, vars)?.code,
        };
        let value = match value {
            Expr::String(s) => c_string_literal(s),
            Expr::Move(name) => {
                return Ok(format!(
                    "Dict_string_string_add_owned(&{}, {}, {})",
                    dict_code,
                    key,
                    sanitize_ident(name)
                ));
            }
            _ => self.emit_expr(value, vars)?.code,
        };
        Ok(format!(
            "Dict_string_string_add(&{}, {}, {})",
            dict_code, key, value
        ))
    }

    fn method_owner_and_receiver(
        &self,
        type_name: &str,
        receiver: &str,
        pointer_receiver: bool,
        method_name: &str,
    ) -> Result<(String, String), String> {
        if self
            .methods
            .contains_key(&method_key(type_name, method_name))
        {
            let receiver = if pointer_receiver {
                receiver.to_string()
            } else {
                format!("&{}", receiver)
            };
            return Ok((type_name.to_string(), receiver));
        }
        if let Some(base) = self.concrete_base(type_name) {
            let base_receiver = if pointer_receiver {
                format!("{}->__base", receiver)
            } else {
                format!("{}.__base", receiver)
            };
            return self.method_owner_and_receiver(&base, &base_receiver, false, method_name);
        }
        Err(format!("type '{type_name}' has no method '{method_name}'"))
    }

    fn emit_instance_property_getter(
        &self,
        type_name: &str,
        receiver: &str,
        property_name: &str,
    ) -> Result<EmittedExpr, String> {
        let getter = property_getter_name(property_name);
        let (owner, receiver) =
            match self.method_owner_and_receiver(type_name, receiver, true, &getter) {
                Ok(owner_receiver) => owner_receiver,
                Err(_) if self.has_framework_base(type_name) => {
                    return Ok(EmittedExpr {
                        code: "NULL".to_string(),
                        c_type: CType::GenericPtr(format!("{}_{}", type_name, property_name)),
                    });
                }
                Err(err) => return Err(err),
            };
        let info = self
            .methods
            .get(&method_key(&owner, &getter))
            .and_then(|methods| methods.first())
            .cloned()
            .ok_or_else(|| {
                format!("type '{type_name}' has no field or property '{property_name}'")
            })?;
        Ok(EmittedExpr {
            code: format!("{}({})", info.symbol, receiver),
            c_type: info.return_type,
        })
    }

    fn function_return_type(&self, f: &Function) -> CType {
        if f.name == "main" {
            CType::Scalar(ScalarType::I32)
        } else {
            self.function_info_for(f)
                .map(|info| info.return_type.clone())
                .unwrap_or(CType::Void)
        }
    }

    fn function_symbol_for(&self, f: &Function) -> String {
        self.function_info_for(f)
            .map(|info| info.symbol.clone())
            .unwrap_or_else(|| sanitize_ident(&f.name))
    }

    fn function_info_for(&self, f: &Function) -> Option<&FunctionInfo> {
        let params = f
            .params
            .iter()
            .filter_map(|param| self.param_to_ctype(param).ok())
            .collect::<Vec<_>>();
        self.functions
            .get(&f.name)?
            .iter()
            .find(|info| info.params == params)
    }

    fn function_params(&self, f: &Function) -> Result<String, String> {
        if f.params.is_empty() {
            return Ok("void".to_string());
        }
        let info = self
            .function_info_for(f)
            .ok_or_else(|| format!("function '{}' has no signature", f.name))?;
        Ok(f.params
            .iter()
            .zip(info.params.iter())
            .map(|(p, t)| format!("{} {}", t, sanitize_ident(&p.name)))
            .collect::<Vec<_>>()
            .join(", "))
    }

    fn emit_scope_drops(&self, state: &mut CodegenState, out: &mut String) {
        if let Some(scope) = state.scopes.pop() {
            for name in scope.iter().rev() {
                if state.moved.contains(name) {
                    continue;
                }
                if let Some(ty) = state.vars.get(name) {
                    if let Some(drop) = self.emit_drop(name, ty) {
                        out.push_str(&drop);
                    }
                }
            }
            for name in scope {
                state.vars.remove(&name);
            }
        }
    }

    fn emit_all_scope_drops(&self, state: &mut CodegenState, out: &mut String) {
        while !state.scopes.is_empty() {
            self.emit_scope_drops(state, out);
        }
    }

    fn emit_drop(&self, name: &str, ty: &CType) -> Option<String> {
        self.emit_drop_for_expr(&sanitize_ident(name), ty)
    }

    fn emit_drop_for_expr(&self, expr: &str, ty: &CType) -> Option<String> {
        match ty {
            CType::String => Some(format!("    free({});\n", expr)),
            CType::Exception => Some(format!("    glitch_exception_free(&{});\n", expr)),
            CType::ClassPtr(name) => Some(format!(
                "    if ({0}) {{ glitch_drop_{1}({0}); free({0}); }}\n",
                expr,
                sanitize_ident(name)
            )),
            CType::Struct(name) => Some(format!(
                "    glitch_drop_{}(&{});\n",
                sanitize_ident(name),
                expr
            )),
            CType::ListInt => Some(format!("    List_int_free(&{});\n", expr)),
            CType::ListI64 => Some(format!("    List_i64_free(&{});\n", expr)),
            CType::ListBool => Some(format!("    List_bool_free(&{});\n", expr)),
            CType::ListF64 => Some(format!("    List_f64_free(&{});\n", expr)),
            CType::ListString => Some(format!("    List_string_free(&{});\n", expr)),
            CType::DictStringInt => Some(format!("    Dict_string_int_free(&{});\n", expr)),
            CType::DictStringI64 => Some(format!("    Dict_string_i64_free(&{});\n", expr)),
            CType::DictStringBool => Some(format!("    Dict_string_bool_free(&{});\n", expr)),
            CType::DictStringF64 => Some(format!("    Dict_string_f64_free(&{});\n", expr)),
            CType::DictStringString => Some(format!("    Dict_string_string_free(&{});\n", expr)),
            CType::Task(result) => {
                Some(format!("    {}(&{});\n", task_free_function(result), expr))
            }
            _ => None,
        }
    }

    fn remember_moves(&self, expr: &Expr, state: &mut CodegenState) {
        match expr {
            Expr::Move(name) => state.moved.push(name.clone()),
            Expr::ArrayLiteral(values) => {
                for value in values {
                    self.remember_moves(value, state);
                }
            }
            Expr::NewArray { values, .. } => {
                for value in values {
                    self.remember_moves(value, state);
                }
            }
            Expr::FunctionCall { args, .. } | Expr::MethodCall { args, .. } => {
                for arg in args {
                    self.remember_moves(arg, state);
                }
            }
            Expr::NewObject { args, fields, .. } => {
                for arg in args {
                    self.remember_moves(arg, state);
                }
                for field in fields {
                    self.remember_moves(&field.expr, state);
                }
            }
            Expr::Index { target, index }
            | Expr::Binary {
                left: target,
                right: index,
                ..
            } => {
                self.remember_moves(target, state);
                self.remember_moves(index, state);
            }
            Expr::Field { target, .. } | Expr::Await(target) => {
                self.remember_moves(target, state);
            }
            _ => {}
        }
    }

    fn emit_metadata_comment(
        &self,
        namespace: &[String],
        attributes: &[Attribute],
        out: &mut String,
    ) {
        if namespace.is_empty() && attributes.is_empty() {
            return;
        }
        out.push_str("/* metadata:");
        if !namespace.is_empty() {
            out.push_str(" namespace=");
            out.push_str(&namespace.join("."));
        }
        if !attributes.is_empty() {
            out.push_str(" attributes=");
            let rendered = attributes
                .iter()
                .map(render_attribute)
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&rendered);
        }
        out.push_str(" */\n");
    }
}

fn emit_c_declaration(name: &str, c_type: CType, initializer: &str) -> String {
    match c_type {
        CType::Scalar(s) => format!("    {} {} = {};\n", s.c_name(), name, initializer),
        CType::Ptr(s) => format!("    {} * {} = {};\n", s.c_name(), name, initializer),
        CType::Array(s, _) => format!(
            "    struct {} {} = {};\n",
            array_runtime_name(s),
            name,
            initializer
        ),
        CType::Null => format!("    void * {} = {};\n", name, initializer),
        CType::String => format!("    char * {} = {};\n", name, initializer),
        CType::BorrowedString => format!("    char * {} = {};\n", name, initializer),
        CType::Exception => format!("    struct GlitchException {} = {};\n", name, initializer),
        CType::ExceptionPtr => {
            format!("    struct GlitchException * {} = {};\n", name, initializer)
        }
        CType::Struct(type_name) => format!(
            "    struct {} {} = {};\n",
            sanitize_ident(&type_name),
            name,
            initializer
        ),
        CType::ClassPtr(type_name) => format!(
            "    struct {} * {} = {};\n",
            sanitize_ident(&type_name),
            name,
            initializer
        ),
        CType::GenericPtr(type_name) => format!(
            "    struct {} * {} = {};\n",
            sanitize_ident(&type_name),
            name,
            initializer
        ),
        CType::ListInt => format!("    struct List_int {} = {};\n", name, initializer),
        CType::ListI64 => format!("    struct List_i64 {} = {};\n", name, initializer),
        CType::ListBool => format!("    struct List_bool {} = {};\n", name, initializer),
        CType::ListF64 => format!("    struct List_f64 {} = {};\n", name, initializer),
        CType::ListString => format!("    struct List_string {} = {};\n", name, initializer),
        CType::DictStringInt => format!("    struct Dict_string_int {} = {};\n", name, initializer),
        CType::DictStringI64 => {
            format!("    struct Dict_string_i64 {} = {};\n", name, initializer)
        }
        CType::DictStringBool => {
            format!("    struct Dict_string_bool {} = {};\n", name, initializer)
        }
        CType::DictStringF64 => {
            format!("    struct Dict_string_f64 {} = {};\n", name, initializer)
        }
        CType::DictStringString => {
            format!(
                "    struct Dict_string_string {} = {};\n",
                name, initializer
            )
        }
        CType::EnumerableInt => format!("    struct IEnumerable_int {} = {};\n", name, initializer),
        CType::EnumerableI64 => {
            format!("    struct IEnumerable_i64 {} = {};\n", name, initializer)
        }
        CType::EnumerableBool => {
            format!("    struct IEnumerable_bool {} = {};\n", name, initializer)
        }
        CType::EnumerableF64 => {
            format!("    struct IEnumerable_f64 {} = {};\n", name, initializer)
        }
        CType::EnumerableString => {
            format!(
                "    struct IEnumerable_string {} = {};\n",
                name, initializer
            )
        }
        CType::Thread => format!("    struct GlitchThread {} = {};\n", name, initializer),
        CType::Task(result) => format!(
            "    struct {} {} = {};\n",
            task_runtime_name(&result),
            name,
            initializer
        ),
        CType::Void => format!("    {};\n", initializer),
    }
}

fn default_c_value(ty: &CType) -> &'static str {
    match ty {
        CType::Scalar(ScalarType::F64) => "0.0",
        CType::Scalar(_) => "0",
        CType::String
        | CType::BorrowedString
        | CType::ClassPtr(_)
        | CType::GenericPtr(_)
        | CType::Null => "NULL",
        _ => "0",
    }
}

fn emit_c_field_declaration(name: &str, c_type: &CType) -> String {
    match c_type {
        CType::Scalar(s) => format!("    {} {};\n", s.c_name(), name),
        CType::String => format!("    char * {};\n", name),
        CType::BorrowedString => format!("    char * {};\n", name),
        CType::Exception => format!("    struct GlitchException {};\n", name),
        CType::ExceptionPtr => format!("    struct GlitchException * {};\n", name),
        CType::Struct(type_name) => format!("    struct {} {};\n", sanitize_ident(type_name), name),
        CType::ClassPtr(type_name) => {
            format!("    struct {} * {};\n", sanitize_ident(type_name), name)
        }
        CType::GenericPtr(type_name) => {
            format!("    struct {} * {};\n", sanitize_ident(type_name), name)
        }
        CType::Array(s, _) => format!("    struct {} {};\n", array_runtime_name(*s), name),
        CType::Null => format!("    void * {};\n", name),
        CType::ListInt => format!("    struct List_int {};\n", name),
        CType::ListI64 => format!("    struct List_i64 {};\n", name),
        CType::ListBool => format!("    struct List_bool {};\n", name),
        CType::ListF64 => format!("    struct List_f64 {};\n", name),
        CType::ListString => format!("    struct List_string {};\n", name),
        CType::DictStringInt => format!("    struct Dict_string_int {};\n", name),
        CType::DictStringI64 => format!("    struct Dict_string_i64 {};\n", name),
        CType::DictStringBool => format!("    struct Dict_string_bool {};\n", name),
        CType::DictStringF64 => format!("    struct Dict_string_f64 {};\n", name),
        CType::DictStringString => format!("    struct Dict_string_string {};\n", name),
        CType::EnumerableInt => format!("    struct IEnumerable_int {};\n", name),
        CType::EnumerableI64 => format!("    struct IEnumerable_i64 {};\n", name),
        CType::EnumerableBool => format!("    struct IEnumerable_bool {};\n", name),
        CType::EnumerableF64 => format!("    struct IEnumerable_f64 {};\n", name),
        CType::EnumerableString => format!("    struct IEnumerable_string {};\n", name),
        CType::Thread => format!("    struct GlitchThread {};\n", name),
        CType::Task(result) => format!("    struct {} {};\n", task_runtime_name(result), name),
        CType::Ptr(s) => format!("    {} * {};\n", s.c_name(), name),
        CType::Void => String::new(),
    }
}

fn sanitize_ident(name: &str) -> String {
    if name == "this" {
        return "self".to_string();
    }
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

fn method_key(type_name: &str, method_name: &str) -> String {
    format!("{type_name}.{method_name}")
}

fn function_symbol(name: &str, params: &[CType], overloaded: bool) -> String {
    if !overloaded {
        return sanitize_ident(name);
    }
    let suffix = params
        .iter()
        .map(c_type_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    if suffix.is_empty() {
        format!("{}__overload", sanitize_ident(name))
    } else {
        format!("{}__{}", sanitize_ident(name), suffix)
    }
}

fn c_type_symbol_suffix(ty: &CType) -> String {
    match ty {
        CType::Scalar(ScalarType::Bool) => "bool".to_string(),
        CType::Scalar(ScalarType::Byte) => "byte".to_string(),
        CType::Scalar(ScalarType::Short) => "short".to_string(),
        CType::Scalar(ScalarType::I32) => "int".to_string(),
        CType::Scalar(ScalarType::I64) => "long".to_string(),
        CType::Scalar(ScalarType::U32) => "uint".to_string(),
        CType::Scalar(ScalarType::F64) => "double".to_string(),
        CType::Scalar(ScalarType::Decimal) => "decimal".to_string(),
        CType::String | CType::BorrowedString => "string".to_string(),
        CType::Null => "null".to_string(),
        CType::Void => "void".to_string(),
        CType::ClassPtr(name) | CType::Struct(name) | CType::GenericPtr(name) => {
            sanitize_ident(name)
        }
        CType::Task(inner) => format!("task_{}", c_type_symbol_suffix(inner)),
        other => sanitize_ident(&format!("{other:?}")),
    }
}

fn array_runtime_name(scalar: ScalarType) -> &'static str {
    match scalar {
        ScalarType::Bool => "GlitchArray_bool",
        ScalarType::Byte => "GlitchArray_byte",
        ScalarType::Short => "GlitchArray_short",
        ScalarType::I32 => "GlitchArray_int",
        ScalarType::I64 => "GlitchArray_i64",
        ScalarType::U32 => "GlitchArray_uint",
        ScalarType::F64 => "GlitchArray_f64",
        ScalarType::Decimal => "GlitchArray_decimal",
    }
}

fn array_literal_code(element: ScalarType, values: &[String]) -> String {
    format!(
        "(struct {}){{({}[]){{{}}}, {}}}",
        array_runtime_name(element),
        element.c_name(),
        values.join(", "),
        values.len()
    )
}

fn call_arg_expr(arg: &Expr) -> &Expr {
    match arg {
        Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => expr,
        other => other,
    }
}

fn call_arg_name(arg: &Expr) -> Option<&str> {
    match arg {
        Expr::NamedArg { name, .. } => Some(name.as_str()),
        _ => None,
    }
}

fn call_arg_modifier(arg: &Expr) -> ParamModifier {
    match arg {
        Expr::RefArg { modifier, .. } => *modifier,
        _ => ParamModifier::None,
    }
}

fn call_modifier_matches(expected: ParamModifier, actual: ParamModifier) -> bool {
    match expected {
        ParamModifier::None | ParamModifier::This => actual == ParamModifier::None,
        ParamModifier::In => matches!(actual, ParamModifier::None | ParamModifier::In),
        ParamModifier::Ref => actual == ParamModifier::Ref,
        ParamModifier::Out => actual == ParamModifier::Out,
    }
}

fn c_type_same_for_conversion(left: &CType, right: &CType) -> bool {
    left == right
        || matches!(
            (left, right),
            (CType::String, CType::BorrowedString)
                | (CType::BorrowedString, CType::String)
                | (CType::GenericPtr(_), CType::GenericPtr(_))
        )
}

fn c_numeric_conversion_rank(expected: &CType, arg: &CCallArg) -> Option<u16> {
    if let Some(value) = arg.int_value {
        return int_literal_c_conversion_rank(expected, value);
    }
    let (CType::Scalar(actual), CType::Scalar(expected)) = (&arg.c_type, expected) else {
        return None;
    };
    match (actual, expected) {
        (ScalarType::Byte, ScalarType::Short) => Some(1),
        (ScalarType::Byte, ScalarType::I32) => Some(2),
        (ScalarType::Byte, ScalarType::U32) => Some(2),
        (ScalarType::Byte, ScalarType::I64) => Some(3),
        (ScalarType::Byte, ScalarType::F64) | (ScalarType::Byte, ScalarType::Decimal) => Some(4),
        (ScalarType::Short, ScalarType::I32) => Some(1),
        (ScalarType::Short, ScalarType::I64) => Some(2),
        (ScalarType::Short, ScalarType::F64) | (ScalarType::Short, ScalarType::Decimal) => Some(3),
        (ScalarType::I32, ScalarType::I64) => Some(1),
        (ScalarType::I32, ScalarType::F64) | (ScalarType::I32, ScalarType::Decimal) => Some(2),
        (ScalarType::U32, ScalarType::I64) => Some(1),
        (ScalarType::U32, ScalarType::F64) | (ScalarType::U32, ScalarType::Decimal) => Some(2),
        (ScalarType::I64, ScalarType::F64) | (ScalarType::I64, ScalarType::Decimal) => Some(1),
        _ => None,
    }
}

fn int_literal_c_conversion_rank(expected: &CType, value: i64) -> Option<u16> {
    match expected {
        CType::Scalar(ScalarType::I32) if i32::try_from(value).is_ok() => Some(0),
        CType::Scalar(ScalarType::I64) => Some(1),
        CType::Scalar(ScalarType::Short) if i16::try_from(value).is_ok() => Some(2),
        CType::Scalar(ScalarType::Byte) if u8::try_from(value).is_ok() => Some(3),
        CType::Scalar(ScalarType::U32) if u32::try_from(value).is_ok() => Some(4),
        CType::Scalar(ScalarType::F64 | ScalarType::Decimal) => Some(5),
        _ => None,
    }
}

fn method_symbol(type_name: &str, method_name: &str, params: &[CType], overloaded: bool) -> String {
    let base = format!(
        "{}_{}",
        sanitize_ident(type_name),
        sanitize_ident(method_name)
    );
    if !overloaded {
        return base;
    }
    let suffix = params
        .iter()
        .map(c_type_symbol_suffix)
        .collect::<Vec<_>>()
        .join("_");
    if suffix.is_empty() {
        format!("{base}__overload")
    } else {
        format!("{base}__{suffix}")
    }
}

fn enum_variant_symbol(type_name: &str, variant_name: &str) -> String {
    format!(
        "{}_{}",
        sanitize_ident(type_name),
        sanitize_ident(variant_name)
    )
}

fn constructor_symbol(type_name: &str) -> String {
    format!("{}_new", sanitize_ident(type_name))
}

fn generic_type_name(name: &str, args: &[TypeSyntax]) -> String {
    let suffix = args
        .iter()
        .map(type_syntax_symbol)
        .collect::<Vec<_>>()
        .join("_");
    if suffix.is_empty() {
        name.to_string()
    } else {
        format!("{}_{}", name, suffix)
    }
}

fn type_syntax_symbol(ty: &TypeSyntax) -> String {
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
        TypeSyntax::Array(inner) => format!("array_{}", type_syntax_symbol(inner)),
        TypeSyntax::Ref(inner) => format!("ref_{}", type_syntax_symbol(inner)),
        TypeSyntax::Named(name) => sanitize_ident(name),
        TypeSyntax::GenericNamed { name, args } => sanitize_ident(&generic_type_name(name, args)),
        TypeSyntax::List(inner) => sanitize_ident(&generic_type_name(
            "List",
            std::slice::from_ref(inner.as_ref()),
        )),
        TypeSyntax::Dictionary(key, value) => sanitize_ident(&generic_type_name(
            "Dictionary",
            &[key.as_ref().clone(), value.as_ref().clone()],
        )),
        TypeSyntax::IEnumerable(inner) => sanitize_ident(&generic_type_name(
            "IEnumerable",
            std::slice::from_ref(inner.as_ref()),
        )),
        TypeSyntax::Thread => "Thread".to_string(),
        TypeSyntax::Task(inner) => sanitize_ident(&generic_type_name(
            "Task",
            std::slice::from_ref(inner.as_ref()),
        )),
        TypeSyntax::Nullable(inner) => format!("nullable_{}", type_syntax_symbol(inner)),
        TypeSyntax::Void => "void".to_string(),
    }
}

fn property_getter_name(name: &str) -> String {
    format!("get_{name}")
}

fn c_string_literal(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

fn render_attribute(attribute: &Attribute) -> String {
    if attribute.args.is_empty() {
        return attribute.name.clone();
    }
    let args = attribute
        .args
        .iter()
        .map(render_attribute_arg)
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}({})", attribute.name, args)
}

fn render_attribute_arg(expr: &Expr) -> String {
    match expr {
        Expr::String(value) => c_string_literal(value),
        Expr::Int(value) => value.to_string(),
        Expr::Float(value) => value.to_string(),
        Expr::Bool(value) => value.to_string(),
        Expr::Var(name) => name.clone(),
        Expr::NamedArg { name, expr } => format!("{} = {}", name, render_attribute_arg(expr)),
        Expr::Await(_) => "<await>".to_string(),
        _ => "<expr>".to_string(),
    }
}

struct ControllerRoute {
    method: String,
    path: String,
    handler: String,
}

fn controller_routes(program: &Program) -> Vec<ControllerRoute> {
    let mut routes = Vec::new();
    for ty in &program.types {
        if !has_attribute(&ty.attributes, "ApiController") {
            continue;
        }
        let prefix =
            attribute_string_arg(&ty.attributes, "Route").unwrap_or_else(|| "/".to_string());
        for method in &ty.methods {
            if let Some(path) = attribute_string_arg(&method.attributes, "HttpGet") {
                routes.push(ControllerRoute {
                    method: "GET".to_string(),
                    path: join_route_path(&prefix, &path),
                    handler: format!("{}.{}", ty.name, method.name),
                });
            }
            if let Some(path) = attribute_string_arg(&method.attributes, "HttpPost") {
                routes.push(ControllerRoute {
                    method: "POST".to_string(),
                    path: join_route_path(&prefix, &path),
                    handler: format!("{}.{}", ty.name, method.name),
                });
            }
        }
    }
    routes
}

fn has_attribute(attributes: &[Attribute], name: &str) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute_name_matches(&attribute.name, name))
}

fn attribute_string_arg(attributes: &[Attribute], name: &str) -> Option<String> {
    attributes
        .iter()
        .find(|attribute| attribute_name_matches(&attribute.name, name))
        .and_then(|attribute| match attribute.args.first() {
            Some(Expr::String(value)) => Some(value.clone()),
            None => Some(String::new()),
            _ => None,
        })
}

fn attribute_name_matches(actual: &str, expected: &str) -> bool {
    let short = actual.rsplit('.').next().unwrap_or(actual);
    short == expected || short.strip_suffix("Attribute") == Some(expected)
}

fn join_route_path(prefix: &str, path: &str) -> String {
    let normalized_prefix = if prefix.is_empty() { "/" } else { prefix };
    let normalized_path = if path.is_empty() { "/" } else { path };
    if normalized_path == "/" {
        return if normalized_prefix == "/" {
            "/".to_string()
        } else if normalized_prefix.starts_with('/') {
            normalized_prefix.trim_end_matches('/').to_string()
        } else {
            format!("/{}", normalized_prefix.trim_end_matches('/'))
        };
    }
    let left = normalized_prefix.trim_end_matches('/');
    let right = normalized_path.trim_start_matches('/');
    if left.is_empty() || left == "/" {
        format!("/{right}")
    } else if left.starts_with('/') {
        format!("{left}/{right}")
    } else {
        format!("/{left}/{right}")
    }
}

fn expr_to_type_path(expr: &Expr) -> Option<Vec<String>> {
    match expr {
        Expr::Var(name) => Some(vec![name.clone()]),
        Expr::Field { target, name } => {
            let mut path = expr_to_type_path(target)?;
            path.push(name.clone());
            Some(path)
        }
        _ => None,
    }
}

fn is_task_type_path(parts: &[String]) -> bool {
    parts == ["Task"]
        || parts == ["System", "Threading", "Tasks", "Task"]
        || parts == ["ValueTask"]
        || parts == ["System", "Threading", "Tasks", "ValueTask"]
}

fn is_file_type_path(parts: &[String]) -> bool {
    parts == ["File"]
        || parts == ["System", "IO", "File"]
}

fn is_temporal_value_method(name: &str) -> bool {
    matches!(
        name,
        "AddTicks"
            | "AddDays"
            | "AddHours"
            | "AddMinutes"
            | "AddSeconds"
            | "AddMilliseconds"
            | "AddMonths"
            | "AddYears"
            | "Subtract"
            | "ToDateTime"
    )
}

fn generic_ptr_string_collection(name: &str) -> bool {
    name == "HashSet_string"
        || name == "ICollection_string"
        || name == "IReadOnlyCollection_string"
        || name == "IEnumerable_string"
        || name == "IQueryable_string"
        || name == "string_array"
}

fn ctype_accepts_null(ty: &CType) -> bool {
    matches!(
        ty,
        CType::String
            | CType::BorrowedString
            | CType::ClassPtr(_)
            | CType::GenericPtr(_)
            | CType::Exception
            | CType::ExceptionPtr
    )
}

fn is_exception_type_name(name: &str) -> bool {
    name == "Exception" || name == "System.Exception" || name.ends_with("Exception")
}

fn task_runtime_name(result: &CType) -> &'static str {
    match result {
        CType::Void => "GlitchTask",
        CType::Scalar(ScalarType::Bool) => "GlitchTask_bool",
        CType::Scalar(ScalarType::I32) => "GlitchTask_i32",
        CType::Scalar(ScalarType::I64) => "GlitchTask_i64",
        CType::Scalar(ScalarType::F64) => "GlitchTask_f64",
        CType::String => "GlitchTask_string",
        CType::ClassPtr(_) | CType::GenericPtr(_) | CType::Null => "GlitchTask_ptr",
        _ => "GlitchTask_unsupported",
    }
}

fn task_run_function(result: &CType) -> &'static str {
    match result {
        CType::Void => "GlitchTask_run",
        CType::Scalar(ScalarType::Bool) => "GlitchTask_bool_run",
        CType::Scalar(ScalarType::I32) => "GlitchTask_i32_run",
        CType::Scalar(ScalarType::I64) => "GlitchTask_i64_run",
        CType::Scalar(ScalarType::F64) => "GlitchTask_f64_run",
        CType::String => "GlitchTask_string_run",
        CType::ClassPtr(_) | CType::GenericPtr(_) | CType::Null => "GlitchTask_ptr_run",
        _ => "GlitchTask_unsupported_run",
    }
}

fn task_from_result_function(result: &CType) -> &'static str {
    match result {
        CType::Scalar(ScalarType::Bool) => "GlitchTask_bool_from_result",
        CType::Scalar(ScalarType::I32) => "GlitchTask_i32_from_result",
        CType::Scalar(ScalarType::I64) => "GlitchTask_i64_from_result",
        CType::Scalar(ScalarType::F64) => "GlitchTask_f64_from_result",
        CType::String => "GlitchTask_string_from_owned",
        CType::ClassPtr(_) | CType::GenericPtr(_) | CType::Null => "GlitchTask_ptr_from_result",
        _ => "GlitchTask_unsupported_from_result",
    }
}

fn task_wait_function(result: &CType) -> &'static str {
    match result {
        CType::Void => "GlitchTask_wait",
        CType::Scalar(ScalarType::Bool) => "GlitchTask_bool_wait",
        CType::Scalar(ScalarType::I32) => "GlitchTask_i32_wait",
        CType::Scalar(ScalarType::I64) => "GlitchTask_i64_wait",
        CType::Scalar(ScalarType::F64) => "GlitchTask_f64_wait",
        CType::String => "GlitchTask_string_wait",
        CType::ClassPtr(_) | CType::GenericPtr(_) | CType::Null => "GlitchTask_ptr_wait",
        _ => "GlitchTask_unsupported_wait",
    }
}

fn task_free_function(result: &CType) -> &'static str {
    match result {
        CType::Void => "GlitchTask_wait",
        CType::Scalar(ScalarType::Bool) => "GlitchTask_bool_free",
        CType::Scalar(ScalarType::I32) => "GlitchTask_i32_free",
        CType::Scalar(ScalarType::I64) => "GlitchTask_i64_free",
        CType::Scalar(ScalarType::F64) => "GlitchTask_f64_free",
        CType::String => "GlitchTask_string_free",
        CType::ClassPtr(_) | CType::GenericPtr(_) | CType::Null => "GlitchTask_ptr_free",
        _ => "GlitchTask_unsupported_free",
    }
}

fn task_result_function(result: &CType) -> &'static str {
    match result {
        CType::Scalar(ScalarType::Bool) => "GlitchTask_bool_result",
        CType::Scalar(ScalarType::I32) => "GlitchTask_i32_result",
        CType::Scalar(ScalarType::I64) => "GlitchTask_i64_result",
        CType::Scalar(ScalarType::F64) => "GlitchTask_f64_result",
        CType::String => "GlitchTask_string_result",
        CType::ClassPtr(_) | CType::GenericPtr(_) | CType::Null => "GlitchTask_ptr_result",
        _ => "GlitchTask_unsupported_result",
    }
}

fn builtin_function_info(name: &str) -> Option<FunctionInfo> {
    match name {
        "SystemTextJson_SerializeString"
        | "SystemTextJson_DeserializeString"
        | "JsonSerializer_SerializeString"
        | "JsonSerializer_DeserializeString" => Some(FunctionInfo {
            params: vec![CType::String],
            param_names: vec!["value".to_string()],
            param_modifiers: vec![ParamModifier::None],
            param_is_params: vec![false],
            param_defaults: vec![None],
            return_type: CType::String,
            symbol: sanitize_ident(name),
        }),
        "GlitchEndpointHandlers_Contains" => Some(FunctionInfo {
            params: vec![
                CType::ClassPtr("WebApplication".to_string()),
                CType::String,
                CType::String,
            ],
            param_names: vec!["app".to_string(), "method".to_string(), "path".to_string()],
            param_modifiers: vec![ParamModifier::None; 3],
            param_is_params: vec![false; 3],
            param_defaults: vec![None; 3],
            return_type: CType::Scalar(ScalarType::Bool),
            symbol: sanitize_ident(name),
        }),
        "GlitchEndpointHandlers_Invoke" => Some(FunctionInfo {
            params: vec![
                CType::ClassPtr("WebApplication".to_string()),
                CType::String,
                CType::String,
                CType::String,
            ],
            param_names: vec![
                "app".to_string(),
                "method".to_string(),
                "path".to_string(),
                "body".to_string(),
            ],
            param_modifiers: vec![ParamModifier::None; 4],
            param_is_params: vec![false; 4],
            param_defaults: vec![None; 4],
            return_type: CType::String,
            symbol: sanitize_ident(name),
        }),
        "GlitchMiddlewareHandlers_Apply" => Some(FunctionInfo {
            params: vec![CType::ClassPtr("WebApplication".to_string()), CType::String],
            param_names: vec!["app".to_string(), "input".to_string()],
            param_modifiers: vec![ParamModifier::None; 2],
            param_is_params: vec![false; 2],
            param_defaults: vec![None; 2],
            return_type: CType::String,
            symbol: sanitize_ident(name),
        }),
        _ => None,
    }
}

const RUNTIME_C: &str = r#"
struct GlitchArray_bool { int *data; int len; };
struct GlitchArray_byte { unsigned char *data; int len; };
struct GlitchArray_short { short *data; int len; };
struct GlitchArray_int { int *data; int len; };
struct GlitchArray_i64 { long long *data; int len; };
struct GlitchArray_uint { unsigned int *data; int len; };
struct GlitchArray_f64 { double *data; int len; };
struct GlitchArray_decimal { long double *data; int len; };

struct List_int { int *data; int len; int cap; };
static char * glitch_strdup(const char *source);
static struct List_int List_int_new(void) { struct List_int list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(int) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_int_add(struct List_int *list, int value) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(int) * (size_t)list->cap); if (!list->data) { abort(); } } list->data[list->len++] = value; }
static int List_int_get(struct List_int *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_int_contains(struct List_int *list, int value) { for (int i = 0; i < list->len; i++) { if (list->data[i] == value) { return 1; } } return 0; }
static void List_int_clear(struct List_int *list) { list->len = 0; }
static void List_int_free(struct List_int *list) { free(list->data); list->data = NULL; list->len = 0; list->cap = 0; }

struct List_i64 { long long *data; int len; int cap; };
static struct List_i64 List_i64_new(void) { struct List_i64 list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(long long) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_i64_add(struct List_i64 *list, long long value) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(long long) * (size_t)list->cap); if (!list->data) { abort(); } } list->data[list->len++] = value; }
static long long List_i64_get(struct List_i64 *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_i64_contains(struct List_i64 *list, long long value) { for (int i = 0; i < list->len; i++) { if (list->data[i] == value) { return 1; } } return 0; }
static void List_i64_clear(struct List_i64 *list) { list->len = 0; }
static void List_i64_free(struct List_i64 *list) { free(list->data); list->data = NULL; list->len = 0; list->cap = 0; }

struct List_bool { int *data; int len; int cap; };
static struct List_bool List_bool_new(void) { struct List_bool list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(int) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_bool_add(struct List_bool *list, int value) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(int) * (size_t)list->cap); if (!list->data) { abort(); } } list->data[list->len++] = value; }
static int List_bool_get(struct List_bool *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_bool_contains(struct List_bool *list, int value) { for (int i = 0; i < list->len; i++) { if (list->data[i] == value) { return 1; } } return 0; }
static void List_bool_clear(struct List_bool *list) { list->len = 0; }
static void List_bool_free(struct List_bool *list) { free(list->data); list->data = NULL; list->len = 0; list->cap = 0; }

struct List_f64 { double *data; int len; int cap; };
static struct List_f64 List_f64_new(void) { struct List_f64 list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(double) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_f64_add(struct List_f64 *list, double value) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(double) * (size_t)list->cap); if (!list->data) { abort(); } } list->data[list->len++] = value; }
static double List_f64_get(struct List_f64 *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_f64_contains(struct List_f64 *list, double value) { for (int i = 0; i < list->len; i++) { if (list->data[i] == value) { return 1; } } return 0; }
static void List_f64_clear(struct List_f64 *list) { list->len = 0; }
static void List_f64_free(struct List_f64 *list) { free(list->data); list->data = NULL; list->len = 0; list->cap = 0; }

struct List_string { char **data; int len; int cap; };
static struct List_string List_string_new(void) { struct List_string list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(char *) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_string_reserve_one(struct List_string *list) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(char *) * (size_t)list->cap); if (!list->data) { abort(); } } }
static void List_string_add_owned(struct List_string *list, char *value) { List_string_reserve_one(list); list->data[list->len++] = value ? value : glitch_strdup(""); }
static void List_string_add(struct List_string *list, const char *value) { List_string_add_owned(list, glitch_strdup(value ? value : "")); }
static char * List_string_get(struct List_string *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_string_contains(struct List_string *list, const char *value) { for (int i = 0; i < list->len; i++) { if (strcmp(list->data[i], value) == 0) { return 1; } } return 0; }
static void List_string_clear(struct List_string *list) { for (int i = 0; i < list->len; i++) { free(list->data[i]); } list->len = 0; }
static void List_string_free(struct List_string *list) { List_string_clear(list); free(list->data); list->data = NULL; list->cap = 0; }

struct IEnumerable_int { struct List_int *source; };
struct IEnumerable_i64 { struct List_i64 *source; };
struct IEnumerable_bool { struct List_bool *source; };
struct IEnumerable_f64 { struct List_f64 *source; };
struct IEnumerable_string { struct List_string *source; };
static struct IEnumerable_int IEnumerable_int_from_List_int(struct List_int *source) { struct IEnumerable_int enumerable; enumerable.source = source; return enumerable; }
static struct IEnumerable_i64 IEnumerable_i64_from_List_i64(struct List_i64 *source) { struct IEnumerable_i64 enumerable; enumerable.source = source; return enumerable; }
static struct IEnumerable_bool IEnumerable_bool_from_List_bool(struct List_bool *source) { struct IEnumerable_bool enumerable; enumerable.source = source; return enumerable; }
static struct IEnumerable_f64 IEnumerable_f64_from_List_f64(struct List_f64 *source) { struct IEnumerable_f64 enumerable; enumerable.source = source; return enumerable; }
static struct IEnumerable_string IEnumerable_string_from_List_string(struct List_string *source) { struct IEnumerable_string enumerable; enumerable.source = source; return enumerable; }

struct Dict_string_int_entry { char *key; int value; };
struct Dict_string_int { struct Dict_string_int_entry *entries; int len; int cap; };
static char * glitch_strdup(const char *source) { size_t len = strlen(source) + 1; char *copy = malloc(len); if (!copy) { abort(); } memcpy(copy, source, len); return copy; }
static char * glitch_string_concat(const char *left, const char *right) { left = left ? left : ""; right = right ? right : ""; size_t left_len = strlen(left); size_t right_len = strlen(right); char *copy = malloc(left_len + right_len + 1); if (!copy) { abort(); } memcpy(copy, left, left_len); memcpy(copy + left_len, right, right_len + 1); return copy; }
struct GlitchException { char *message; };
struct GlitchExceptionFrame { jmp_buf env; struct GlitchExceptionFrame *prev; struct GlitchException exception; };
static struct GlitchExceptionFrame *glitch_exception_stack = NULL;
static struct GlitchException glitch_exception_from_owned(char *message) { struct GlitchException ex; ex.message = message ? message : glitch_strdup(""); return ex; }
static struct GlitchException glitch_exception_new(const char *message) { return glitch_exception_from_owned(glitch_strdup(message ? message : "")); }
static struct GlitchException glitch_exception_clone(struct GlitchException *source) { return glitch_exception_new(source && source->message ? source->message : ""); }
static void glitch_exception_free(struct GlitchException *exception) { if (!exception) { return; } free(exception->message); exception->message = NULL; }
static void glitch_exception_push(struct GlitchExceptionFrame *frame) { frame->prev = glitch_exception_stack; frame->exception.message = NULL; glitch_exception_stack = frame; }
static void glitch_exception_pop(struct GlitchExceptionFrame *frame) { if (glitch_exception_stack == frame) { glitch_exception_stack = frame->prev; } }
static void glitch_throw(struct GlitchException exception) { if (!glitch_exception_stack) { fprintf(stderr, "Unhandled exception: %s\n", exception.message ? exception.message : ""); glitch_exception_free(&exception); abort(); } glitch_exception_stack->exception = exception; longjmp(glitch_exception_stack->env, 1); }
static struct Dict_string_int Dict_string_int_new(void) { struct Dict_string_int dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_int_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_int_add(struct Dict_string_int *dict, const char *key, int value) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_int_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } dict->entries[dict->len].key = glitch_strdup(key); dict->entries[dict->len].value = value; dict->len++; }
static int Dict_string_int_contains_key(struct Dict_string_int *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static int Dict_string_int_get(struct Dict_string_int *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_int_remove(struct Dict_string_int *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_int_clear(struct Dict_string_int *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); } dict->len = 0; }
static void Dict_string_int_free(struct Dict_string_int *dict) { Dict_string_int_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

struct Dict_string_i64_entry { char *key; long long value; };
struct Dict_string_i64 { struct Dict_string_i64_entry *entries; int len; int cap; };
static struct Dict_string_i64 Dict_string_i64_new(void) { struct Dict_string_i64 dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_i64_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_i64_add(struct Dict_string_i64 *dict, const char *key, long long value) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_i64_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } dict->entries[dict->len].key = glitch_strdup(key); dict->entries[dict->len].value = value; dict->len++; }
static int Dict_string_i64_contains_key(struct Dict_string_i64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static long long Dict_string_i64_get(struct Dict_string_i64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_i64_remove(struct Dict_string_i64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_i64_clear(struct Dict_string_i64 *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); } dict->len = 0; }
static void Dict_string_i64_free(struct Dict_string_i64 *dict) { Dict_string_i64_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

struct Dict_string_bool_entry { char *key; int value; };
struct Dict_string_bool { struct Dict_string_bool_entry *entries; int len; int cap; };
static struct Dict_string_bool Dict_string_bool_new(void) { struct Dict_string_bool dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_bool_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_bool_add(struct Dict_string_bool *dict, const char *key, int value) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_bool_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } dict->entries[dict->len].key = glitch_strdup(key); dict->entries[dict->len].value = value; dict->len++; }
static int Dict_string_bool_contains_key(struct Dict_string_bool *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static int Dict_string_bool_get(struct Dict_string_bool *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_bool_remove(struct Dict_string_bool *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_bool_clear(struct Dict_string_bool *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); } dict->len = 0; }
static void Dict_string_bool_free(struct Dict_string_bool *dict) { Dict_string_bool_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

struct Dict_string_f64_entry { char *key; double value; };
struct Dict_string_f64 { struct Dict_string_f64_entry *entries; int len; int cap; };
static struct Dict_string_f64 Dict_string_f64_new(void) { struct Dict_string_f64 dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_f64_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_f64_add(struct Dict_string_f64 *dict, const char *key, double value) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_f64_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } dict->entries[dict->len].key = glitch_strdup(key); dict->entries[dict->len].value = value; dict->len++; }
static int Dict_string_f64_contains_key(struct Dict_string_f64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static double Dict_string_f64_get(struct Dict_string_f64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_f64_remove(struct Dict_string_f64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_f64_clear(struct Dict_string_f64 *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); } dict->len = 0; }
static void Dict_string_f64_free(struct Dict_string_f64 *dict) { Dict_string_f64_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

struct Dict_string_string_entry { char *key; char *value; };
struct Dict_string_string { struct Dict_string_string_entry *entries; int len; int cap; };
static struct Dict_string_string Dict_string_string_new(void) { struct Dict_string_string dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_string_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_string_reserve_one(struct Dict_string_string *dict) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_string_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } }
static void Dict_string_string_add_owned(struct Dict_string_string *dict, const char *key, char *value) { const char *safe_key = key ? key : ""; for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, safe_key) == 0) { free(dict->entries[i].value); dict->entries[i].value = value ? value : glitch_strdup(""); return; } } Dict_string_string_reserve_one(dict); dict->entries[dict->len].key = glitch_strdup(safe_key); dict->entries[dict->len].value = value ? value : glitch_strdup(""); dict->len++; }
static void Dict_string_string_add(struct Dict_string_string *dict, const char *key, const char *value) { Dict_string_string_add_owned(dict, key, glitch_strdup(value ? value : "")); }
static int Dict_string_string_contains_key(struct Dict_string_string *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static char * Dict_string_string_get(struct Dict_string_string *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_string_remove(struct Dict_string_string *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); free(dict->entries[i].value); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_string_clear(struct Dict_string_string *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); free(dict->entries[i].value); } dict->len = 0; }
static void Dict_string_string_free(struct Dict_string_string *dict) { Dict_string_string_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

typedef void (*glitch_thread_entry)(void);
struct GlitchThread { glitch_thread_entry entry; };
static struct GlitchThread GlitchThread_new(glitch_thread_entry entry) { struct GlitchThread thread; thread.entry = entry; return thread; }
static void GlitchThread_start(struct GlitchThread *thread) { thread->entry(); }
static void GlitchThread_join(struct GlitchThread *thread) { (void)thread; }
struct GlitchTask { struct GlitchThread thread; };
static struct GlitchTask GlitchTask_run(glitch_thread_entry entry) { struct GlitchTask task; task.thread = GlitchThread_new(entry); GlitchThread_start(&task.thread); return task; }
static void GlitchTask_wait(struct GlitchTask *task) { GlitchThread_join(&task->thread); }

typedef int (*glitch_task_i32_entry)(void);
typedef long long (*glitch_task_i64_entry)(void);
typedef int (*glitch_task_bool_entry)(void);
typedef double (*glitch_task_f64_entry)(void);
typedef char * (*glitch_task_string_entry)(void);
typedef void * (*glitch_task_ptr_entry)(void);
struct GlitchTask_bool { int result; };
struct GlitchTask_i32 { int result; };
struct GlitchTask_i64 { long long result; };
struct GlitchTask_f64 { double result; };
struct GlitchTask_string { char *result; };
struct GlitchTask_ptr { void *result; };
static struct GlitchTask_bool GlitchTask_bool_run(glitch_task_bool_entry entry) { struct GlitchTask_bool task; task.result = entry(); return task; }
static struct GlitchTask_i32 GlitchTask_i32_run(glitch_task_i32_entry entry) { struct GlitchTask_i32 task; task.result = entry(); return task; }
static struct GlitchTask_i64 GlitchTask_i64_run(glitch_task_i64_entry entry) { struct GlitchTask_i64 task; task.result = entry(); return task; }
static struct GlitchTask_f64 GlitchTask_f64_run(glitch_task_f64_entry entry) { struct GlitchTask_f64 task; task.result = entry(); return task; }
static struct GlitchTask_string GlitchTask_string_run(glitch_task_string_entry entry) { struct GlitchTask_string task; task.result = entry(); return task; }
static struct GlitchTask_ptr GlitchTask_ptr_run(glitch_task_ptr_entry entry) { struct GlitchTask_ptr task; task.result = entry(); return task; }
static struct GlitchTask_bool GlitchTask_bool_from_result(int result) { struct GlitchTask_bool task; task.result = result; return task; }
static struct GlitchTask_i32 GlitchTask_i32_from_result(int result) { struct GlitchTask_i32 task; task.result = result; return task; }
static struct GlitchTask_i64 GlitchTask_i64_from_result(long long result) { struct GlitchTask_i64 task; task.result = result; return task; }
static struct GlitchTask_f64 GlitchTask_f64_from_result(double result) { struct GlitchTask_f64 task; task.result = result; return task; }
static struct GlitchTask_string GlitchTask_string_from_result(const char *result) { struct GlitchTask_string task; task.result = glitch_strdup(result ? result : ""); return task; }
static struct GlitchTask_string GlitchTask_string_from_owned(char *result) { struct GlitchTask_string task; task.result = result; return task; }
static struct GlitchTask_ptr GlitchTask_ptr_from_result(void *result) { struct GlitchTask_ptr task; task.result = result; return task; }
static void GlitchTask_bool_wait(struct GlitchTask_bool *task) { (void)task; }
static void GlitchTask_i32_wait(struct GlitchTask_i32 *task) { (void)task; }
static void GlitchTask_i64_wait(struct GlitchTask_i64 *task) { (void)task; }
static void GlitchTask_f64_wait(struct GlitchTask_f64 *task) { (void)task; }
static void GlitchTask_string_wait(struct GlitchTask_string *task) { (void)task; }
static void GlitchTask_ptr_wait(struct GlitchTask_ptr *task) { (void)task; }
static int GlitchTask_bool_result(struct GlitchTask_bool *task) { return task->result; }
static int GlitchTask_i32_result(struct GlitchTask_i32 *task) { return task->result; }
static long long GlitchTask_i64_result(struct GlitchTask_i64 *task) { return task->result; }
static double GlitchTask_f64_result(struct GlitchTask_f64 *task) { return task->result; }
static char * GlitchTask_string_result(struct GlitchTask_string *task) { char *result = task->result; task->result = NULL; return result; }
static void * GlitchTask_ptr_result(struct GlitchTask_ptr *task) { return task->result; }
static void GlitchTask_bool_free(struct GlitchTask_bool *task) { (void)task; }
static void GlitchTask_i32_free(struct GlitchTask_i32 *task) { (void)task; }
static void GlitchTask_i64_free(struct GlitchTask_i64 *task) { (void)task; }
static void GlitchTask_f64_free(struct GlitchTask_f64 *task) { (void)task; }
static void GlitchTask_string_free(struct GlitchTask_string *task) { free(task->result); task->result = NULL; }
static void GlitchTask_ptr_free(struct GlitchTask_ptr *task) { task->result = NULL; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_csharp_control_flow_smoke() {
        let source = r#"
            int ClampSmall(int value) {
                if (value > 10) {
                    return 10;
                } else {
                    return value;
                }
            }

            fn main() {
                int total = 0;
                for (int i = 0; i < 4; i = i + 1) {
                    total = total + ClampSmall(i);
                }
                while (total < 20) {
                    total = total + 1;
                }
                print(total);
            }
        "#;

        let c = compile_source(source).expect("control flow should compile");

        assert!(c.contains("static int ClampSmall(int value);"));
        assert!(c.contains("while ((i < 4))"));
        assert!(c.contains("while ((total < 20))"));
    }

    #[test]
    fn compiles_task_generic_smoke() {
        let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 42;
            }

            string LoadName() {
                return "Ada";
            }

            bool IsReady() {
                return true;
            }

            double LoadRatio() {
                return 1.5;
            }

            fn main() {
                Task<int> numberTask = Task.Run(Compute);
                int value = numberTask.Result;
                print(value);

                Task<string> nameTask = Task.Run(LoadName);
                string name = nameTask.GetResult();
                print(name);

                Task<bool> readyTask = Task.Run(IsReady);
                bool ready = readyTask.Result;
                print(ready);

                Task<double> ratioTask = Task.Run(LoadRatio);
                double ratio = ratioTask.GetAwaiter().GetResult();
                print(ratio);

                Task<bool> completed = Task.FromResult(true);
                print(completed.IsCompleted);
                print(completed.GetResult());

                Task<double> fromRatio = Task.FromResult(2.5);
                print(fromRatio.Result);
            }
        "#;

        let c = compile_source(source).expect("Task<T> should compile");

        assert!(c.contains("struct GlitchTask_i32 numberTask = GlitchTask_i32_run(Compute);"));
        assert!(c.contains("int value = GlitchTask_i32_result(&numberTask);"));
        assert!(c.contains("char * name = GlitchTask_string_result(&nameTask);"));
        assert!(c.contains("struct GlitchTask_bool readyTask = GlitchTask_bool_run(IsReady);"));
        assert!(c.contains("int ready = GlitchTask_bool_result(&readyTask);"));
        assert!(c.contains("struct GlitchTask_f64 ratioTask = GlitchTask_f64_run(LoadRatio);"));
        assert!(c.contains("double ratio = GlitchTask_f64_result(&ratioTask);"));
        assert!(c.contains("struct GlitchTask_bool completed = GlitchTask_bool_from_result(1);"));
        assert!(c.contains("printf(\"%d\\n\", 1);"));
        assert!(c.contains("struct GlitchTask_f64 fromRatio = GlitchTask_f64_from_result(2.5);"));
    }

    #[test]
    fn compiles_async_await_task_lowering() {
        let source = r#"
            using System.Threading.Tasks;

            async Task<int> LoadNumber() {
                return 42;
            }

            async Task<string> LoadName() {
                return "Ada";
            }

            fn main() {
                Task<int> numberTask = LoadNumber();
                int value = await numberTask;
                print(value);

                Task<string> nameTask = LoadName();
                string name = await nameTask;
                print(name);
            }
        "#;

        let c = compile_source(source).expect("async/await should compile");
        let bytecode = compile_bytecode(source).expect("async/await bytecode should compile");

        assert!(c.contains("static struct GlitchTask_i32 LoadNumber(void);"));
        assert!(
            c.contains("struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result(42);")
        );
        assert!(c.contains("struct GlitchTask_string __glitch_return = GlitchTask_string_from_owned(glitch_strdup(\"Ada\"));"));
        assert!(c.contains("int value = GlitchTask_i32_result(&numberTask);"));
        assert!(c.contains("char * name = GlitchTask_string_result(&nameTask);"));
        assert!(bytecode.contains("  await"));
    }

    #[test]
    fn compiles_tiny_aspnet_like_supported_subset() {
        let source = include_str!("../examples/tiny_aspnet_subset.cs");

        let c = compile_source(source).expect("tiny ASP.NET-like supported subset should compile");
        let report = compile_leak_report(source).expect("leak report should compile");

        assert!(c.contains("/* metadata: attributes=ApiController, Route(\"/hello\") */"));
        assert!(c.contains("/* metadata: attributes=HttpGet(\"/\") */"));
        assert!(c.contains("static struct GlitchTask_string HelloController_Get"));
        assert!(c.contains("GlitchTask_string_from_owned(ServiceProvider_GetRequiredService"));
        assert!(c.contains("char * controllerText = GlitchTask_string_result(&controllerTask);"));
        assert!(
            c.contains("static void glitch_register_attribute_routes(struct WebApplication * app)")
        );
        assert!(c.contains("WebApplication_MapGet(app, \"/hello\", \"HelloController.Get\");"));
        assert!(c.contains("WebApplication_MapGet(app"));
        assert!(c.contains("WebApplication_MapPost(app"));
        assert!(c.contains("WebApplication_Handle(app"));
        assert!(c.contains("static char * HealthEndpoint(void);"));
        assert!(
            c.contains("GlitchEndpointHandlers_Add(app, \"GET\", \"/health\", HealthEndpoint);")
        );
        assert!(c.contains("GlitchEndpointHandlers_Contains(self, method, path)"));
        assert!(c.contains("GlitchEndpointHandlers_Invoke(self, method, path, body)"));
        assert!(c.contains("GlitchRestHost_Run"));
        assert!(c.contains("SystemTextJson_SerializeString"));
        assert!(c.contains("JsonSerializer_SerializeString(controllerText)"));
        assert!(report.contains("No obvious owned temporary leaks detected."));
    }

    #[test]
    fn compiles_aspnet_endpoint_handler_function_pointer() {
        let source = r#"
            using Glitching.AspNetCore;

            string Ping() {
                return "{\"pong\":true}";
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.MapGet("/ping", Ping);
                string response = app.Handle("GET", "/ping", "");
                print(response);
            }
        "#;

        let c = compile_source(source).expect("endpoint handler should compile");
        let report =
            compile_leak_report(source).expect("endpoint handler leak report should compile");

        assert!(c.contains("typedef char *(*GlitchEndpointHandler)(void);"));
        assert!(c.contains("static char * Ping(void);"));
        assert!(c.contains("GlitchEndpointHandlers_Add(app, \"GET\", \"/ping\", Ping);"));
        assert!(c.contains(
            "char * handlerResponse = GlitchEndpointHandlers_Invoke(self, method, path, body);"
        ));
        assert!(c.contains("free(handlerResponse);"));
        assert!(
            c.contains("static void GlitchEndpointHandlers_RemoveApp(struct WebApplication *app)")
        );
        assert!(c.contains("GlitchEndpointHandlers_RemoveApp(value);"));
        let summary =
            compile_ownership_summary(source).expect("endpoint typed IR summary should compile");
        assert!(summary.contains("endpoint GET /ping -> Ping returns Owned String"));
        assert!(summary.contains(
            "tir call method MapGet symbol=MapGet resolution=EndpointHandlerRegistration"
        ));
        assert!(report.contains("No obvious owned temporary leaks detected."));
    }

    #[test]
    fn compiles_aspnet_named_middleware_pipeline() {
        let source = r#"
            using Glitching.AspNetCore;

            fn main() {
                WebApplication app = new WebApplication();
                app.UseTrace();
                app.MapGet("/ping", "pong");
                string response = app.Handle("GET", "/ping", "");
                print(response);
            }
        "#;

        let c = compile_source(source).expect("ASP.NET-like middleware pipeline should compile");
        let report = compile_leak_report(source).expect("middleware leak report should compile");

        assert!(c.contains("static void WebApplication_UseTrace(struct WebApplication * self);"));
        assert!(c.contains("WebApplication_UseTrace(app);"));
        assert!(c.contains("List_string_contains(&self->Middleware, \"trace\")"));
        assert!(c.contains("glitch_string_concat(prefix, current)"));
        assert!(report.contains("No obvious owned temporary leaks detected."));
    }

    #[test]
    fn compiles_aspnet_delegate_middleware_pipeline() {
        let source = r#"
            using Glitching.AspNetCore;

            string AddPrefix(string text) {
                string prefix = "mw:";
                return prefix + text;
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.Use(AddPrefix);
                app.MapGet("/ping", "pong");
                string response = app.Handle("GET", "/ping", "");
                print(response);
            }
        "#;

        let c = compile_source(source).expect("delegate middleware should compile");
        let report =
            compile_leak_report(source).expect("delegate middleware leak report should compile");

        assert!(c.contains("typedef char *(*GlitchMiddlewareHandler)(char *);"));
        assert!(c.contains("static char * AddPrefix(char * text);"));
        assert!(c.contains("GlitchMiddlewareHandlers_Add(app, \"AddPrefix\", AddPrefix);"));
        assert!(c.contains("char * current = GlitchMiddlewareHandlers_Apply(self, text);"));
        assert!(c.contains("GlitchMiddlewareHandlers_RemoveApp(value);"));
        assert!(report.contains("No obvious owned temporary leaks detected."));
    }

    #[test]
    fn compiles_efcore_groundwork_package() {
        let source = r#"
            using Microsoft.EntityFrameworkCore;

            fn main() {
                DbContext db = new DbContext("Server=:memory:");
                DbSetString users = SetString(db, "Users");
                IQueryableString tracked = users.AsQueryable();
                List<string> rows = tracked.ToList();
                IQueryableString noTracking = users.AsNoTracking();
                string sql = noTracking.ToQueryString();
                print(rows[0]);
                print(sql);
                print(db.TrackedCount);
                db.Dispose();
            }
        "#;

        let c = compile_source(source).expect("EF Core groundwork package should compile");
        let report = compile_leak_report(source).expect("EF leak report should compile");

        assert!(c.contains("struct DbContext * db = DbContext_new(\"Server=:memory:\");"));
        assert!(c.contains("struct DbSetString * users = SetString(db, \"Users\");"));
        assert!(c.contains("static struct IQueryableString * DbSetString_AsNoTracking"));
        assert!(c.contains("SqlProvider_BuildSelectAll(provider, self->Table)"));
        assert!(c.contains("List_string_add(&values, glitch_string_concat(prefix, query))"));
        assert!(c.contains("DbContext_Dispose(db);"));
        assert!(report.contains("No obvious owned temporary leaks detected."));
    }

    #[test]
    fn compiles_framework_base_opaque_property_chains() {
        let source = r#"
            class ApplicationDbContext : DbContext {
            }

            public static class LinqHelpers {
                public static Expression<Func<T, bool>> BuildWherePredicate<T>(
                    PropertyInfo propertyInfo,
                    string filterQuery)
                {
                    return null;
                }
            }

            class InvalidUserException {
                public InvalidUserException() : this(string.Empty) {}
                public InvalidUserException(string? message) {}
            }

            void ExecuteParameterizedQuery(string sql, object[] sqlParametersObjects) {
                ApplicationDbContext context = new ApplicationDbContext {};
                context.Database.ExecuteSqlRaw(sql, sqlParametersObjects);
            }

            fn main() {
                string firstArg = args[0].ToString();
                ApplicationDbContext context = new ApplicationDbContext {};
                var database = context.Database;
                database.Migrate();
                database.ExecuteSqlRaw("delete from Books");
                var predicate = LinqHelpers.BuildWherePredicate<Book>(null, "name");
                byte[] left = new byte[] { 1, 2 };
                byte[] right = new byte[] { 1, 2 };
                bool same = left.SequenceEqual(right);
                bool hasAny = predicate.Any();
                List<int> ids = new List<int>();
                bool hasIds = ids.Any();
                foreach (var id in ids) {
                    print(id);
                }
                int number = 42;
                string idText = number.ToString();
                var invalid = new InvalidUserException();
                throw invalid;
            }
        "#;

        let c = compile_source(source)
            .expect("framework-derived opaque property chains should compile");

        assert!(c.contains("struct ApplicationDbContext_Database *"));
        assert!(c.contains("struct string_array * args = NULL;"));
        assert!(c.contains("struct object_array * sqlParametersObjects"));
        assert!(c.contains("struct LinqHelpers_BuildWherePredicate * predicate = NULL;"));
        assert!(c.contains("int same = 1;"));
        assert!(c.contains("int hasAny = 1;"));
        assert!(c.contains("int hasIds = (ids.len > 0);"));
        assert!(c.contains("int id = ids.data["));
        assert!(c.contains("char * idText = glitch_strdup(\"\");"));
        assert!(
            c.contains("struct InvalidUserException * invalid = InvalidUserException_new(NULL);")
        );
        assert!(c.contains("glitch_throw(glitch_exception_new(\"\"));"));
    }

    #[test]
    fn typed_ir_represents_function_symbols() {
        let source = r#"
            string Ping() {
                return "pong";
            }

            fn main() {
                var handler = Ping;
            }
        "#;

        let summary =
            compile_ownership_summary(source).expect("function symbol typed IR should compile");

        assert!(summary.contains("local handler: Copy Function"));
        assert!(summary.contains("return_type: String"));
        assert!(summary.contains("tir let handler: Copy Function"));
    }

    #[test]
    fn resolves_top_level_function_overloads_in_tir_and_llvm() {
        let source = r#"
            long Pick(long value) {
                return value + 1;
            }

            string Pick(string value) {
                return value;
            }

            fn main() {
                long n = Pick(41);
                string s = Pick("ok");
                print(n);
                print(s);
            }
        "#;

        let c = compile_source(source).expect("overloaded functions should compile to C");
        let llvm_ir = compile_llvm_ir(source).expect("overloaded functions should compile to LLVM");
        let summary =
            compile_ownership_summary(source).expect("overloaded functions should lower to TIR");

        assert!(c.contains("static long long Pick__long(long long value);"));
        assert!(c.contains("static char * Pick__string(char * value);"));
        assert!(c.contains("long long n = Pick__long(41);"));
        assert!(c.contains("char * s = Pick__string(\"ok\");"));
        assert!(llvm_ir.contains("define i64 @Pick__long(i64 %value)"));
        assert!(llvm_ir.contains("define ptr @Pick__string(ptr %value)"));
        assert!(llvm_ir.contains("call i64 @Pick__long(i64 41)"));
        assert!(llvm_ir.contains("call ptr @Pick__string("));
        assert!(summary.contains("tir call function Pick symbol=Pick__long"));
        assert!(summary.contains("tir call function Pick symbol=Pick__string"));
    }

    #[test]
    fn resolves_instance_method_overloads_in_codegen_and_tir() {
        let source = r#"
            class Greeter {
                long Say(long value) {
                    return value + 1;
                }

                string Say(string value) {
                    return value;
                }
            }

            fn main() {
                Greeter greeter = new Greeter {};
                long n = greeter.Say(41);
                string s = greeter.Say("ok");
                print(n);
                print(s);
            }
        "#;

        let c = compile_source(source).expect("overloaded instance methods should compile to C");
        let summary =
            compile_ownership_summary(source).expect("overloaded instance methods should lower");

        assert!(c.contains(
            "static long long Greeter_Say__long(struct Greeter * self, long long value);"
        ));
        assert!(
            c.contains("static char * Greeter_Say__string(struct Greeter * self, char * value);")
        );
        assert!(c.contains("long long n = Greeter_Say__long(greeter, 41);"));
        assert!(c.contains("char * s = Greeter_Say__string(greeter, \"ok\");"));
        assert!(summary.contains("tir call method Say symbol=Greeter_Say__long"));
        assert!(summary.contains("tir call method Say symbol=Greeter_Say__string"));
    }

    #[test]
    fn ranks_numeric_overloads_like_csharp_positional_calls() {
        let source = r#"
            int Pick(int value) {
                return 1;
            }

            long Pick(long value) {
                return 2;
            }

            double Pick(double value) {
                return 3.0;
            }

            fn main() {
                int i = 7;
                long l = 8;
                int literalResult = Pick(1);
                int intResult = Pick(i);
                long longResult = Pick(l);
                int expressionResult = Pick(i + 1);
                print(literalResult);
                print(intResult);
                print(longResult);
                print(expressionResult);
            }
        "#;

        let c = compile_source(source).expect("numeric overload ranking should compile");
        let summary =
            compile_ownership_summary(source).expect("numeric overload ranking should lower");

        assert!(c.contains("int literalResult = Pick__int(1);"));
        assert!(c.contains("int intResult = Pick__int(i);"));
        assert!(c.contains("long long longResult = Pick__long(l);"));
        assert!(c.contains("int expressionResult = Pick__int((i + 1));"));
        assert!(summary.contains("tir call function Pick symbol=Pick__int"));
        assert!(summary.contains("tir call function Pick symbol=Pick__long"));
    }

    #[test]
    fn ranks_reference_overloads_and_converts_derived_to_base() {
        let source = r#"
            class Base {
            }

            class Derived : Base {
            }

            int Pick(Base value) {
                return 1;
            }

            int Pick(Derived value) {
                return 2;
            }

            int ReadBase(Base value) {
                return 3;
            }

            fn main() {
                Derived derived = new Derived {};
                int exact = Pick(derived);
                int baseOnly = ReadBase(derived);
                print(exact);
                print(baseOnly);
            }
        "#;

        let c = compile_source(source).expect("reference overload ranking should compile");
        let summary =
            compile_ownership_summary(source).expect("reference overload ranking should lower");

        assert!(c.contains("int exact = Pick__Derived(derived);"));
        assert!(c.contains("int baseOnly = ReadBase(&derived->__base);"));
        assert!(summary.contains("tir call function Pick symbol=Pick__Derived"));
        assert!(summary.contains("tir call function ReadBase symbol=ReadBase"));
    }

    #[test]
    fn rejects_ambiguous_reference_overload_resolution() {
        let source = r#"
            interface IA {
            }

            interface IB {
            }

            class Both : IA, IB {
            }

            int Pick(IA value) {
                return 1;
            }

            int Pick(IB value) {
                return 2;
            }

            fn main() {
                Both both = new Both {};
                int result = Pick(both);
                print(result);
            }
        "#;

        let error = compile_source(source).expect_err("ambiguous overload should fail");

        assert!(error.contains("ambiguous overload resolution for call to 'Pick'"));
    }

    #[test]
    fn compiles_named_and_default_arguments() {
        let source = r#"
            int Add(int left, int right = 10) {
                return left + right;
            }

            class Pair {
                public int Sum;

                public Pair(int left, int right = 4) {
                    this.Sum = left + right;
                }
            }

            fn main() {
                int named = Add(right: 2, left: 1);
                int defaulted = Add(5);
                Pair pair = new Pair(right: 3, left: 2);
                print(named);
                print(defaulted);
                print(pair.Sum);
            }
        "#;

        let c = compile_source(source).expect("named/default arguments should compile");

        assert!(c.contains("int named = Add(1, 2);"));
        assert!(c.contains("int defaulted = Add(5, 10);"));
        assert!(c.contains("struct Pair * pair = Pair_new(2, 3);"));
    }

    #[test]
    fn compiles_extension_method_calls() {
        let source = r#"
            class Counter {
                public int Value;
            }

            int ScorePlus(this Counter counter, int bonus = 1) {
                return counter.Value + bonus;
            }

            fn main() {
                Counter counter = new Counter { Value = 7 };
                int score = counter.ScorePlus(bonus: 5);
                int defaulted = counter.ScorePlus();
                print(score);
                print(defaulted);
            }
        "#;

        let c = compile_source(source).expect("extension methods should compile");

        assert!(c.contains("static int ScorePlus(struct Counter * counter, int bonus);"));
        assert!(c.contains("int score = ScorePlus(counter, 5);"));
        assert!(c.contains("int defaulted = ScorePlus(counter, 1);"));
    }

    #[test]
    fn rejects_missing_ref_argument_modifier() {
        let source = r#"
            int Read(ref int value) {
                return value;
            }

            fn main() {
                int value = 1;
                int result = Read(value);
                print(result);
            }
        "#;

        let error = compile_source(source).expect_err("missing ref modifier should fail");

        assert!(error.contains("no overload of 'Read' matches argument types"));
    }

    #[test]
    fn compiles_scalar_ref_and_out_arguments() {
        let source = r#"
            void Increment(ref int value) {
                value = value + 1;
            }

            void SetSeven(out int value) {
                value = 7;
            }

            fn main() {
                int current = 1;
                Increment(ref current);
                SetSeven(out current);
                print(current);
            }
        "#;

        let c = compile_source(source).expect("scalar ref/out arguments should compile");

        assert!(c.contains("static void Increment(int * value);"));
        assert!(c.contains("static void SetSeven(int * value);"));
        assert!(c.contains("*value = (*value + 1);"));
        assert!(c.contains("*value = 7;"));
        assert!(c.contains("Increment(&current);"));
        assert!(c.contains("SetSeven(&current);"));
    }

    #[test]
    fn compiles_expanded_params_scalar_arrays() {
        let source = r#"
            int First(params int[] values) {
                print(values.Length);
                return values[0];
            }

            fn main() {
                int value = First(10, 20, 30);
                print(value);
            }
        "#;

        let c = compile_source(source).expect("expanded params arguments should compile");

        assert!(c.contains("static int First(struct GlitchArray_int values);"));
        assert!(c.contains("printf(\"%d\\n\", values.len);"));
        assert!(c.contains("int __glitch_return = values.data[0];"));
        assert!(c.contains("int value = First((struct GlitchArray_int){(int[]){10, 20, 30}, 3});"));
    }

    #[test]
    fn validates_generic_constraints() {
        let source = r#"
            fn Use<T>() where T : MissingType {
            }
        "#;

        let error = compile_source(source).expect_err("unknown generic constraint should fail");

        assert!(error.contains("generic constraint error in function Use"));
        assert!(error.contains("unknown constraint type 'MissingType'"));
    }

    #[test]
    fn applies_user_defined_implicit_conversion_for_overloads() {
        let source = r#"
            struct Meter {
                public int Value;
            }

            int op_Implicit(Meter value) {
                return value.Value;
            }

            int Read(int value) {
                return value;
            }

            fn main() {
                Meter meter = new Meter { Value = 12 };
                int result = Read(meter);
                print(result);
            }
        "#;

        let c = compile_source(source).expect("implicit conversion should compile");

        assert!(c.contains("static int op_Implicit(struct Meter value);"));
        assert!(c.contains("int result = Read(op_Implicit(meter));"));
    }

    #[test]
    fn compiles_null_literal_for_nullable_reference_overloads() {
        let source = r#"
            string Pick(string value) {
                return "string";
            }

            fn main() {
                string? maybe = null;
                string result = Pick(null);
                print(result);
            }
        "#;

        let c = compile_source(source).expect("null reference overload should compile");

        assert!(c.contains("char * maybe = NULL;"));
        assert!(c.contains("char * result = Pick(NULL);"));
    }

    #[test]
    fn compiles_system_text_json_package_helpers() {
        let source = r#"
            using System.Text.Json;

            fn main() {
                string value = "hello";
                string json = JsonSerializer_SerializeString(value);
                string plain = JsonSerializer_DeserializeString(json);
                print(json);
                print(plain);
            }
        "#;

        let c = compile_source(source).expect("System.Text.Json package should compile");
        let report = compile_leak_report(source).expect("JSON leak report should compile");

        assert!(c.contains("static char * SystemTextJson_SerializeString(char *value)"));
        assert!(c.contains("char * json = JsonSerializer_SerializeString(value);"));
        assert!(c.contains("char * plain = JsonSerializer_DeserializeString(json);"));
        assert!(report.contains("No obvious owned temporary leaks detected."));
    }

    #[test]
    fn compiles_system_io_file_operations() {
        let source = r#"
            using System.IO;

            fn main() {
                string path = "test_file.txt";
                File.WriteAllText(path, "Hello from Glitching!");
                string content = File.ReadAllText(path);
                print(content);
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("System.IO.File operations should compile to LLVM IR");
        assert!(llvm_ir.contains("call void @System_IO_File_WriteAllText"));
        assert!(llvm_ir.contains("call ptr @System_IO_File_ReadAllText"));

        let c = compile_source(source).expect("System.IO.File operations should compile to C");
        assert!(c.contains("System_IO_File_WriteAllText(path, glitch_strdup(\"Hello from Glitching!\"));"));
        assert!(c.contains("char * content = System_IO_File_ReadAllText(path);"));
    }

    #[test]
    fn compiles_collections_smoke() {
        let source = r#"
            fn main() {
                List<int> xs = new List<int>();
                xs.Add(10);
                print(xs[0]);

                Dictionary<string, int> scores = new Dictionary<string, int>();
                scores.Add("hp", 100);
                print(scores["hp"]);
            }
        "#;

        let c = compile_source(source).expect("collections should compile");

        assert!(c.contains("struct List_int xs = List_int_new();"));
        assert!(c.contains("Dict_string_int_add(&scores"));
    }

    #[test]
    fn preserves_namespace_and_attribute_metadata() {
        let source = r#"
            namespace Demo.Api {
                [ApiController]
                [Route("/users")]
                class UsersController {
                    public string Name;
                }

                [HttpGet("/health")]
                string Health() {
                    return "ok";
                }
            }
        "#;

        let c = compile_source(source).expect("attributes and namespace should compile");

        assert!(c.contains(
            "/* metadata: namespace=Demo.Api attributes=ApiController, Route(\"/users\") */"
        ));
        assert!(c.contains("/* metadata: namespace=Demo.Api attributes=HttpGet(\"/health\") */"));
    }

    #[test]
    fn compiles_class_method_and_this_access() {
        let source = r#"
            class Counter {
                public int Value;

                int GetValue() {
                    return this.Value;
                }
            }

            fn main() {
                Counter counter = new Counter { Value = 7 };
                int value = counter.GetValue();
                print(value);
            }
        "#;

        let c = compile_source(source).expect("class methods should compile");

        assert!(c.contains("static int Counter_GetValue(struct Counter * self);"));
        assert!(c.contains("static int Counter_GetValue(struct Counter * self) {"));
        assert!(c.contains("int __glitch_return = self->Value;"));
        assert!(c.contains("int value = Counter_GetValue(counter);"));
    }

    #[test]
    fn preserves_method_attribute_metadata() {
        let source = r#"
            namespace Demo.Api {
                [ApiController]
                class UsersController {
                    public int Count;

                    [HttpGet("/count")]
                    int GetCount() {
                        return this.Count;
                    }
                }
            }
        "#;

        let c = compile_source(source).expect("method attributes should compile");

        assert!(c.contains("/* metadata: namespace=Demo.Api attributes=HttpGet(\"/count\") */"));
        assert!(c.contains("static int UsersController_GetCount(struct UsersController * self) {"));
    }

    #[test]
    fn compiles_constructor_auto_property_and_expression_property() {
        let source = r#"
            class Counter {
                public int Count { get; set; }

                public Counter(int count) {
                    this.Count = count;
                }

                int Twice => this.Count + this.Count;
            }

            fn main() {
                Counter counter = new Counter(5);
                print(counter.Twice);
            }
        "#;

        let c = compile_source(source).expect("constructors and properties should compile");

        assert!(c.contains("static struct Counter * Counter_new(int count);"));
        assert!(c.contains("static struct Counter * Counter_new(int count) {"));
        assert!(c.contains("self->Count = count;"));
        assert!(c.contains("static int Counter_get_Twice(struct Counter * self) {"));
        assert!(c.contains("struct Counter * counter = Counter_new(5);"));
        assert!(c.contains("printf(\"%d\\n\", Counter_get_Twice(counter));"));
    }

    #[test]
    fn compiles_common_scalars_nullable_using_static_and_enums() {
        let source = r#"
            using static System.Math;

            namespace Demo {
                enum Status {
                    Unknown,
                    Active = 2,
                }

                class Model {
                    public Status State;
                    public bool Enabled;
                    public double Score;

                    public Model(Status state) {
                        this.State = state;
                        this.Enabled = true;
                        this.Score = 12.5;
                    }

                    int StateCode => this.State;
                }

                static int ReadConst() {
                    const int value = 3;
                    return value;
                }

                int ReadStatus(Status status) {
                    int code = 0;
                    switch (status) {
                        case Status.Active:
                            code = 1;
                            break;
                        default:
                            code = 9;
                            break;
                    }
                    return code;
                }

                fn main() {
                    Model model = new Model(Status.Active);
                    string? title = "ready";
                    DateTime now = "2026-05-31";
                    Guid id = "00000000-0000-0000-0000-000000000000";
                    byte b = 1;
                    short s = 2;
                    uint u = 3;
                    decimal money = 4.5;
                    print(model.StateCode);
                    print(model.Enabled);
                    print(model.Score);
                    print(ReadConst());
                    print(ReadStatus(model.State));
                }
            }
        "#;

        let c = compile_source(source).expect("common C# scalar surface should compile");

        assert!(c.contains("enum Status {"));
        assert!(c.contains("Status_Active = 2"));
        assert!(c.contains("static struct Model * Model_new(int state);"));
        assert!(c.contains("struct Model * model = Model_new(Status_Active);"));
        assert!(c.contains("char * title = glitch_strdup(\"ready\");"));
        assert!(c.contains("unsigned char b = 1;"));
        assert!(c.contains("short s = 2;"));
        assert!(c.contains("unsigned int u = 3;"));
        assert!(c.contains("long double money = 4.5;"));
        assert!(c.contains("switch (status) {"));
        assert!(c.contains("case Status_Active:"));
        assert!(c.contains("printf(\"%f\\n\", model->Score);"));
    }

    #[test]
    fn compiles_interfaces_inheritance_and_try_finally_surface() {
        let source = r#"
            interface IScored {
                int Score();
            }

            class BaseCounter {
                public int Seed;

                int GetSeed() {
                    return this.Seed;
                }
            }

            class DerivedCounter : BaseCounter, IScored {
                public int Bonus;

                public DerivedCounter(int seed, int bonus) {
                    this.Seed = seed;
                    this.Bonus = bonus;
                }

                int Score() {
                    int result = 0;
                    try {
                        result = this.GetSeed() + this.Bonus;
                    } catch (Exception ex) {
                        result = 0;
                    } finally {
                        result = result + 1;
                    }
                    return result;
                }
            }

            fn main() {
                DerivedCounter counter = new DerivedCounter(4, 5);
                print(counter.Score());
            }
        "#;

        let c = compile_source(source).expect("inheritance and try/finally surface should compile");

        assert!(c.contains("/* interface IScored */"));
        assert!(c.contains("struct BaseCounter __base;"));
        assert!(c.contains("self->__base.Seed = seed;"));
        assert!(c.contains("BaseCounter_GetSeed(&self->__base)"));
        assert!(c.contains("if (setjmp(__glitch_frame.env) == 0)"));
        assert!(c.contains("struct GlitchException * ex = &__glitch_frame.exception;"));
        assert!(c.contains("int __glitch_return = result;"));
    }

    #[test]
    fn compiles_throw_catch_finally_runtime_path() {
        let source = r#"
            fn main() {
                int code = 0;
                try {
                    throw new Exception("boom");
                } catch (Exception ex) {
                    print(ex.Message);
                    code = 7;
                } finally {
                    code = code + 1;
                }
                print(code);
            }
        "#;

        let c = compile_source(source).expect("throw/catch/finally should compile");

        assert!(c.contains("glitch_exception_push(&__glitch_frame);"));
        assert!(c.contains("glitch_throw(glitch_exception_from_owned(glitch_strdup(\"boom\")));"));
        assert!(c.contains("struct GlitchException * ex = &__glitch_frame.exception;"));
        assert!(c.contains("printf(\"%s\\n\", ex->message);"));
        assert!(c.contains("code = (code + 1);"));
        assert!(c.contains("glitch_exception_free(&__glitch_frame.exception);"));
    }

    #[test]
    fn emits_stack_bytecode_backend() {
        let source = r#"
            fn main() {
                int x = 1;
                print(x);
            }
        "#;

        let bytecode = compile_bytecode(source).expect("bytecode should compile");

        assert!(bytecode.contains(".glitch_bytecode 0.1"));
        assert!(bytecode.contains(".function main"));
        assert!(bytecode.contains("  const.i64 1"));
        assert!(bytecode.contains("  store x"));
        assert!(bytecode.contains("  load x"));
        assert!(bytecode.contains("  print"));
    }

    #[test]
    fn emits_llvm_ir_backend_after_borrow_check() {
        let source = r#"
            long Add(long left, long right) {
                return left + right;
            }

            fn main() {
                long value = Add(40, 2);
                print(value);
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("LLVM IR should compile");

        assert!(llvm_ir.contains("; ModuleID = 'glitching'"));
        assert!(llvm_ir.contains("declare i32 @printf(ptr, ...)"));
        assert!(llvm_ir.contains("define i64 @Add(i64 %left, i64 %right)"));
        assert!(llvm_ir.contains(" = add i64 "));
        assert!(llvm_ir.contains("define i32 @main()"));
        assert!(llvm_ir.contains(" = call i64 @Add(i64 40, i64 2)"));
        assert!(llvm_ir.contains("@printf"));
    }

    #[test]
    fn emits_llvm_class_layout_constructor_methods_fields_and_drop_glue() {
        let source = r#"
            class Counter {
                public int Value;

                public Counter(int value) {
                    this.Value = value;
                }

                int Increment() {
                    this.Value = this.Value + 1;
                    return this.Value;
                }
            }

            fn main() {
                Counter counter = new Counter(41);
                int value = counter.Increment();
                print(value);
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("LLVM class object model should compile");

        assert!(llvm_ir.contains("%glitch.Counter = type { i64, ptr, i32 }"));
        assert!(llvm_ir.contains("define void @Counter_ctor(ptr %this, i32 %value)"));
        assert!(llvm_ir.contains("define i32 @Counter_Increment(ptr %this)"));
        assert!(llvm_ir.contains("call ptr @glitch_calloc(i64 1, i64"));
        assert!(llvm_ir.contains("call void @Counter_ctor(ptr"));
        assert!(llvm_ir.contains("call i32 @Counter_Increment(ptr"));
        assert!(llvm_ir.contains("getelementptr inbounds %glitch.Counter"));
        assert!(llvm_ir.contains("define void @glitch_retain_Counter(ptr %object)"));
        assert!(llvm_ir.contains("define void @glitch_drop_Counter(ptr %object)"));
        assert!(llvm_ir.contains("define void @glitch_destroy_Counter(ptr %object)"));
        assert!(llvm_ir.contains("call void @glitch_drop_Counter(ptr"));
    }

    #[test]
    fn emits_llvm_owned_generic_collections_and_counted_cleanup() {
        let source = r#"
            fn main() {
                List<int> values = new List<int>();
                values.Add(10);
                print(values[0]);

                Dictionary<string, int> scores = new Dictionary<string, int>();
                scores.Add("hp", 100);
                print(scores["hp"]);
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("LLVM collections should compile");

        assert!(llvm_ir.contains("%glitch.list = type { i64, i64, ptr }"));
        assert!(llvm_ir.contains("%glitch.dict = type { i64, i64, ptr, ptr }"));
        assert!(llvm_ir.contains("call ptr @glitch_realloc"));
        assert!(llvm_ir.contains("call void @glitch_free"));
        assert!(llvm_ir.contains("@glitch_live_allocations"));
    }

    #[test]
    fn compiles_nested_collection_drops_to_llvm() {
        let source = r#"
            fn main() {
                List<string[]> nested = new List<string[]>();
                string[] arr = new string[2];
                arr[0] = "hello";
                arr[1] = "world";
                nested.Add(arr);
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("Nested collections should compile to LLVM IR");
        assert!(llvm_ir.contains("call void @glitch_string_release(ptr"));
        assert!(llvm_ir.contains("element_drop_loop"));
    }

    #[test]
    fn lowers_aspnet_string_routes_and_rust_socket_host_to_llvm() {
        let source = r#"
            using Glitching.AspNetCore;

            fn main() {
                WebApplication app = new WebApplication();
                app.MapGet("/health", "{\"status\":\"ok\"}");
                app.RunOnce(5099);
            }
        "#;
        let llvm = compile_llvm_ir(source).expect("ASP.NET socket subset should lower");
        assert!(llvm.contains("call void @WebApplication_MapGet"));
        assert!(llvm.contains("call void @GlitchRestHost_Run(ptr"));
        assert!(llvm.contains("ptr @WebApplication_Handle"));
        assert!(llvm.contains("ptr @glitch_string_release"));
    }

    #[test]
    fn lowers_attribute_controller_routes_to_owned_llvm_thunks() {
        let source = r#"
            using Glitching.AspNetCore;

            [ApiController]
            [Route("api/[controller]")]
            class StatusController {
                [HttpGet("ready")]
                string Ready() {
                    return "{\"status\":\"ready\"}";
                }
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.RunOnce(5101);
            }
        "#;

        let llvm = compile_llvm_ir(source).expect("attribute controller route should lower");
        assert!(llvm.contains("define ptr @glitch_endpoint_handler_0(ptr %path, ptr %body)"));
        assert!(llvm.contains("call ptr @StatusController_Ready(ptr %controller)"));
        assert!(llvm.contains("call void @glitch_drop_StatusController(ptr %controller)"));
        assert!(llvm.contains("define i1 @glitch_endpoint_handlers_contains"));
        assert!(llvm.contains("define ptr @glitch_endpoint_handlers_invoke"));
        assert!(llvm.contains("c\"/api/Status/ready\\00\""));
    }

    #[test]
    fn lowers_inherited_generic_controller_routes_and_templates() {
        let source = r#"
            using Glitching.AspNetCore;

            [ApiController]
            [Route("api/[controller]")]
            class CrudController<T> {
                [HttpGet("{id}")]
                string Get(int id) {
                    return "item";
                }
            }

            class BookController : CrudController<int> {
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.RunOnce(5103);
            }
        "#;

        let llvm = compile_llvm_ir(source).expect("inherited controller route should lower");
        assert!(llvm.contains("c\"/api/Book/{id}\\00\""));
        assert!(llvm.contains("define i1 @glitch_route_match"));
        assert!(llvm.contains("call i1 @glitch_route_match"));
        assert!(llvm.contains("c\"501 Not Implemented\\00\""));
    }

    #[test]
    fn emits_llvm_exception_propagation_catch_finally_and_cleanup() {
        let source = r#"
            void Fail() {
                throw new Exception("boom");
            }

            fn main() {
                int code = 0;
                try {
                    Fail();
                } catch (Exception ex) {
                    print(ex.Message);
                    code = 7;
                } finally {
                    code = code + 1;
                }
                print(code);
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("LLVM exceptions should compile");

        assert!(llvm_ir.contains("@glitch_exception_pending"));
        assert!(llvm_ir.contains("try_catch"));
        assert!(llvm_ir.contains("try_finally"));
        assert!(llvm_ir.contains("exception_unwind"));
        assert!(llvm_ir.contains("store ptr null, ptr @glitch_exception_pending"));
    }

    #[test]
    fn warns_on_reference_cycle_statically() {
        let source = r#"
            class Node {
                public Node Parent;
            }

            fn main() {
            }
        "#;

        let output = compile_source_with_options(source, true, false).expect("should compile with warning");
        let diagnostics = output.diagnostics.join("\n");
        assert!(diagnostics.contains("warning GL3007"));
        assert!(diagnostics.contains("reference cycle detected: class 'Node' field 'Parent' participates in a potential ownership cycle Node -> Node"));
    }

    #[test]
    fn compiles_weak_reference_cycles() {
        let source = r#"
            class Node {
                public Weak<Node> Parent;
                public Node Child;
            }

            fn main() {
                var parent = new Node();
                var child = new Node();
                parent.Child = child;
                child.Parent = new Weak<Node>(parent);
                
                Node target;
                if (child.Parent.TryGetTarget(out target)) {
                    print("target found");
                }
                
                var parentTarget = child.Parent.Target;
                if (parentTarget != null) {
                    print("target property found");
                }
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("Weak reference cycle code should compile to LLVM IR");
        assert!(llvm_ir.contains("phi i1 [ true, %tryget_not_null"));
        assert!(llvm_ir.contains("phi i1"));
        assert!(llvm_ir.contains("store ptr"));
    }

    #[test]
    fn compiles_lambdas_with_captures() {
        let source = r#"
            class Runner {
                public Func<int, int> Worker;

                public Runner(Func<int, int> worker) {
                    this.Worker = worker;
                }

                public int Run(int x) {
                    var f = this.Worker;
                    return f(x);
                }
            }

            fn main() {
                int factor = 3;
                var runner = new Runner(x => x * factor);
                int res = runner.Run(5);
                print(res);
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("Lambda with captures should compile to LLVM IR");
        println!("LLVM IR:\n{}", llvm_ir);
        
        // Assert that the environment struct is defined
        assert!(llvm_ir.contains("%glitch.lambda.0.env = type { i32 }"));
        // Assert that the delegate struct is used
        assert!(llvm_ir.contains("%glitch.delegate = type { ptr, ptr }"));
        // Assert that the environment is stack-allocated and stored
        assert!(llvm_ir.contains("alloca %glitch.lambda.0.env"));
        // Assert that the delegate function signature matches and it loads the capture
        assert!(llvm_ir.contains("define ptr @glitch_lambda_0(ptr %env, ptr %x)"));
        assert!(llvm_ir.contains("getelementptr inbounds %glitch.lambda.0.env, ptr %env"));
        // Assert that calling the delegate loads fields and calls the function pointer
        assert!(llvm_ir.contains("getelementptr inbounds { ptr, ptr }"));
    }

    #[test]
    fn emits_compatibility_warning_for_missing_members() {
        let source = r#"
            fn main() {
                var value = ExternalFramework.Create();
            }
        "#;

        let output = compile_source_with_options(source, true, false)
            .expect("compatibility warnings must not reject source");
        let diagnostics = output.diagnostics.join("\n");

        assert!(diagnostics.contains("warning GL3001"));
        assert!(diagnostics.contains("implement this member in a `.gl` package"));
    }

    #[test]
    fn warns_with_route_specific_endpoint_rewrite_proposals() {
        let source = r#"
            using Glitching.AspNetCore;

            class CreateRequest {
                public List<string> Names;
            }

            [ApiController]
            [Route("/books")]
            class BooksController {
                [HttpPost]
                string Create([FromBody] CreateRequest request) {
                    return "created";
                }
            }
        "#;

        let output = compile_source_with_options(source, true, false)
            .expect("unsupported endpoint binding should remain a warning");
        let diagnostics = output.diagnostics.join("\n");
        assert!(diagnostics.contains("warning GL3101"));
        assert!(diagnostics.contains("endpoint POST /books is discoverable"));
        assert!(diagnostics.contains("[FromBody] string body"));
        assert!(diagnostics.contains("JSON binder for Class(\"CreateRequest\")"));
    }

    #[test]
    fn lowers_unqualified_instance_fields_to_this_in_llvm() {
        let source = r#"
            class Options {
                public int Value;

                public Options(int value) {
                    Value = value;
                }

                int Get() {
                    return Value;
                }
            }

            fn main() {
                Options options = new Options(42);
                print(options.Get());
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("unqualified instance fields should compile");

        assert!(llvm_ir.contains("define void @Options_ctor(ptr %this, i32 %value)"));
        assert!(llvm_ir.contains("define i32 @Options_Get(ptr %this)"));
        assert!(llvm_ir.contains("getelementptr inbounds %glitch.Options"));
    }

    #[test]
    fn lowers_csharp_pattern_variable_binding_in_llvm() {
        let source = r#"
            class Entity {
                public int Id;
            }

            fn main() {
                Entity entity = new Entity { Id = 42 };
                if (entity is Entity typed && typed.Id == 42) {
                    print(typed.Id);
                }
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("pattern binding should compile");

        assert!(llvm_ir.contains("icmp ne ptr"));
        assert!(llvm_ir.contains("%glitch.Entity"));
        assert!(llvm_ir.contains("store ptr"));
    }

    #[test]
    fn compiles_more_generic_collection_instantiations() {
        let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<string> names = new List<string>();
                names.Add("Ada");
                print(names[0]);
                print(names.Contains("Ada"));
                print(names.Count);

                List<long> ids = new List<long>();
                ids.Add(42);
                print(ids[0]);
                print(ids.Contains(42));

                Dictionary<string, string> map = new Dictionary<string, string>();
                map.Add("lang", "glitching");
                print(map["lang"]);
                print(map.Remove("lang"));
                print(map.Count);

                List<bool> flags = new List<bool>();
                flags.Add(true);
                print(flags[0]);
                print(flags.Contains(true));
                flags.Clear();
                print(flags.Count);

                List<double> ratios = new List<double>();
                ratios.Add(1.5);
                print(ratios[0]);
                print(ratios.Contains(1.5));

                Dictionary<string, long> longMap = new Dictionary<string, long>();
                longMap.Add("id", 42);
                print(longMap["id"]);
                print(longMap.ContainsKey("id"));

                Dictionary<string, bool> boolMap = new Dictionary<string, bool>();
                boolMap.Add("ok", true);
                print(boolMap["ok"]);

                Dictionary<string, double> doubleMap = new Dictionary<string, double>();
                doubleMap.Add("pi", 3.14);
                print(doubleMap["pi"]);
                print(doubleMap.Remove("pi"));
                doubleMap.Clear();
                print(doubleMap.Count);
            }
        "#;

        let c = compile_source(source).expect("generic collection instantiations should compile");

        assert!(c.contains("struct List_string names = List_string_new();"));
        assert!(c.contains("List_string_add(&names, \"Ada\")"));
        assert!(c.contains("printf(\"%s\\n\", List_string_get(&names, 0));"));
        assert!(c.contains("List_string_contains(&names, \"Ada\")"));
        assert!(c.contains("struct List_i64 ids = List_i64_new();"));
        assert!(c.contains("List_i64_add(&ids, 42)"));
        assert!(c.contains("List_i64_contains(&ids, 42)"));
        assert!(c.contains("struct Dict_string_string map = Dict_string_string_new();"));
        assert!(c.contains("Dict_string_string_add(&map, \"lang\", \"glitching\")"));
        assert!(c.contains("Dict_string_string_remove(&map, \"lang\")"));
        assert!(c.contains("struct List_bool flags = List_bool_new();"));
        assert!(c.contains("List_bool_add(&flags, 1)"));
        assert!(c.contains("List_bool_contains(&flags, 1)"));
        assert!(c.contains("List_bool_clear(&flags);"));
        assert!(c.contains("struct List_f64 ratios = List_f64_new();"));
        assert!(c.contains("List_f64_add(&ratios, 1.5)"));
        assert!(c.contains("List_f64_contains(&ratios, 1.5)"));
        assert!(c.contains("struct Dict_string_i64 longMap = Dict_string_i64_new();"));
        assert!(c.contains("Dict_string_i64_add(&longMap, \"id\", 42)"));
        assert!(c.contains("Dict_string_i64_contains_key(&longMap, \"id\")"));
        assert!(c.contains("struct Dict_string_bool boolMap = Dict_string_bool_new();"));
        assert!(c.contains("Dict_string_bool_add(&boolMap, \"ok\", 1)"));
        assert!(c.contains("struct Dict_string_f64 doubleMap = Dict_string_f64_new();"));
        assert!(c.contains("Dict_string_f64_add(&doubleMap, \"pi\", 3.14)"));
        assert!(c.contains("Dict_string_f64_remove(&doubleMap, \"pi\")"));
        assert!(c.contains("Dict_string_f64_clear(&doubleMap);"));
        assert!(c.contains("Dict_string_string_free(&map);"));
    }

    #[test]
    fn compiles_foreach_and_ienumerable_views() {
        let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);
                int total = 0;
                foreach (int value in numbers) {
                    total = total + value;
                }
                print(total);

                List<string> names = new List<string>();
                names.Add("Ada");
                IEnumerable<string> view = names;
                foreach (string name in view) {
                    print(name);
                }
            }
        "#;

        let c = compile_source(source).expect("foreach and IEnumerable<T> should compile");
        let bytecode = compile_bytecode(source).expect("foreach bytecode should compile");

        assert!(c.contains(
            "struct IEnumerable_string view = IEnumerable_string_from_List_string(&names);"
        ));
        assert!(c.contains("for (int __glitch_foreach_i_"));
        assert!(c.contains("int value = numbers.data[__glitch_foreach_i_"));
        assert!(c.contains("char * name = view.source->data[__glitch_foreach_i_"));
        assert!(c.contains("List_string_free(&names);"));
        assert!(c.contains("List_int_free(&numbers);"));
        assert!(bytecode.contains("foreach_start value"));
        assert!(bytecode.contains("foreach_next name"));
    }

    #[test]
    fn compiles_library_service_generic_mvc_shapes() {
        let source = r#"
            using System.Collections.Generic;
            using System.Threading.Tasks;

            namespace Microsoft.AspNetCore.Mvc {
                public class ControllerBase {}
                public class ActionResult<T> {}
            }

            class BaseEntity {
                public int Id;
            }

            interface IBaseRepo<T> {}
            class DbSet<T> {}

            class Book : BaseEntity {
                public ICollection<Book> Related { get; set; } = new List<Book>();
            }

            class BookResponseDTO {}

            class BaseCrudController<TEntity, TResponseDto> : ControllerBase
                where TEntity : BaseEntity, new()
                where TResponseDto : new()
            {
                protected IBaseRepo<TEntity> _repo;

                public async virtual Task<ActionResult<IEnumerable<TResponseDto>>> GetAll() {
                    return null;
                }
            }

            class BookController : BaseCrudController<Book, BookResponseDTO> {
                public DbSet<Book> Books { get; set; }

                public async override Task<ActionResult<BookResponseDTO>> GetOneAsync(int id) {
                    return null;
                }
            }

            fn main() {
                BookController controller = new BookController {};
                print(1);
            }
        "#;

        let c = compile_source(source).expect("Library-style MVC generics should compile");
        let bytecode =
            compile_bytecode(source).expect("Library-style MVC generics should emit bytecode");

        assert!(c.contains("struct IEnumerable_Book * Related;"));
        assert!(c.contains("struct DbSet_Book * Books;"));
        assert!(c.contains("static struct GlitchTask_ptr BaseCrudController_GetAll"));
        assert!(c.contains("static struct GlitchTask_ptr BookController_GetOneAsync"));
        assert!(c.contains("GlitchTask_ptr_from_result(NULL)"));
        assert!(bytecode.contains(".base BaseCrudController<Book,BookResponseDTO>"));
        assert!(bytecode.contains("Task<ActionResult<IEnumerable<TResponseDto>>>"));
    }

    #[test]
    fn compiles_csharp_di_fields_and_null_coalescing_assignment() {
        let source = r#"
            class ApiVersion {}
            class IServiceCollection {}
            class ControllerBase {}
            class JwtOptions {}
            interface IUserDataService {}

            public static class ApiVersionConfiguration {
                public static IServiceCollection AddLibraryApiVersionConfiguration(
                    this IServiceCollection services, ApiVersion defaultVersion = null)
                {
                    defaultVersion ??= new ApiVersion {};
                    return services;
                }
            }

            public class AuthController : ControllerBase {
                private readonly JwtOptions _jwtOptions;
                private readonly IUserDataService _userDataService;

                public AuthController(JwtOptions jwtOptions, IUserDataService userDataService)
                {
                    _jwtOptions = jwtOptions;
                    _userDataService = userDataService;
                }

                public JwtOptions Options()
                {
                    return _jwtOptions;
                }
            }
        "#;

        let c = compile_source(source).expect("C# DI field initialization and ??= should compile");

        assert!(c.contains("if ((defaultVersion == NULL))"));
        assert!(c.contains("self->_jwtOptions = jwtOptions;"));
        assert!(c.contains("self->_userDataService = userDataService;"));
        assert!(!c.contains("glitch_drop_JwtOptions(jwtOptions)"));
        assert!(!c.contains("glitch_drop_IUserDataService(userDataService)"));
    }

    #[test]
    fn compiles_valuetask_with_default_literal_parameter() {
        let source = r#"
            using System.Threading.Tasks;

            class DbContextEventData {}
            class CancellationToken {}
            class InterceptionResult<T> {}

            class ExceptionInterceptor {
                public override ValueTask<InterceptionResult<int>> SavingChangesAsync(
                    DbContextEventData eventData,
                    InterceptionResult<int> result,
                    CancellationToken cancellationToken = default)
                {
                    return null;
                }
            }
        "#;

        let c = compile_source(source)
            .expect("ValueTask<T> and default literal parameter should compile");

        assert!(c.contains("static struct GlitchTask_ptr ExceptionInterceptor_SavingChangesAsync"));
        assert!(c.contains("GlitchTask_ptr_from_result(NULL)"));
    }

    #[test]
    fn compiles_external_framework_object_initializer_as_opaque_handle() {
        let source = r#"
            class WebException {
                public string Type;
                public string Title;
                public int Status;
            }

            fn main() {
                var problemDetails = new ProblemDetails {
                    Type = "https://example.com/problems/internal-server-error",
                    Title = "Internal Server Error",
                    Status = 500,
                };
                WebException ex = new WebException {
                    Type = "conflict",
                    Title = "Conflict",
                    Status = 409,
                };
                problemDetails.Type = ex.Type;
                IActionResult result = new ObjectResult(problemDetails) {
                    StatusCode = ex.Status,
                };
            }
        "#;

        let c = compile_source(source)
            .expect("unknown ASP.NET-style framework DTOs should compile as opaque handles");

        assert!(c.contains("struct ProblemDetails * problemDetails = NULL;"));
        assert!(c.contains("struct IActionResult * result = NULL;"));
    }

    #[test]
    fn compiles_datetime_constructor_temporal_methods_in_framework_calls() {
        let source = r#"
            fn main() {
                migrationBuilder.AlterColumn<DateTime>(
                    name: "DateOut",
                    defaultValue: new DateTime(2024, 3, 29, 16, 16, 58, 968, DateTimeKind.Local).AddTicks(1996),
                    oldDefaultValue: new DateTime(2024, 3, 29).AddDays(1));
            }
        "#;

        let c = compile_source(source)
            .expect("DateTime constructor temporal methods should compile in framework calls");

        assert!(c.contains("glitch_strdup(\"\")"));
    }

    #[test]
    fn compiles_csharp_string_index_split_and_string_methods() {
        let source = r#"
            fn main() {
                string filterQuery = ">5~10";
                List<string> operators = new List<string>();
                if (operators.Contains(filterQuery[0].ToString()) || filterQuery.Contains("~")) {
                    var parts = filterQuery.Split("~");
                    string[] explicitParts = filterQuery.Split(".");
                    string first = parts[0].ToString();
                    string explicitFirst = explicitParts[0].ToString();
                    string lowered = first.ToLower().Replace("a", "b");
                    foreach (string part in parts) {
                        print(part);
                    }
                    HashSet<string> values = new HashSet<string> { "a", "b" };
                    foreach (string value in values) {
                        print(value);
                    }
                    string path = filterQuery.TrimEnd('/').Substring(1);
                    int length = path.Length;
                    print(lowered);
                    print(path);
                    print(length);
                }
            }
        "#;

        let c = compile_source(source)
            .expect("C# string indexing, Split, and common string methods should compile");

        assert!(c.contains("List_string_contains(&operators, \"\")"));
        assert!(c.contains("struct string_array * parts = NULL;"));
        assert!(c.contains("struct string_array * explicitParts = NULL;"));
        assert!(c.contains("char * part = \"\";"));
        assert!(c.contains("struct HashSet_string * values = NULL;"));
        assert!(c.contains("char * value = \"\";"));
        assert!(c.contains("char * path = glitch_strdup(\"\");"));
        assert!(c.contains("int length = 0;"));
    }

    #[test]
    fn compiles_task_from_result_and_leak_report() {
        let source = r#"
            using System.Threading.Tasks;

            fn main() {
                Task<int> number = Task.FromResult(42);
                print(number.Result);
                new List<int>();
            }
        "#;

        let c = compile_source(source).expect("Task.FromResult should compile");
        let report = compile_leak_report(source).expect("leak report should be emitted");

        assert!(c.contains("struct GlitchTask_i32 number = GlitchTask_i32_from_result(42);"));
        assert!(report.contains("expression result is owned and discarded"));
    }

    #[test]
    fn borrow_checker_rejects_use_after_move() {
        let source = r#"
            fn main() {
                string name = "Ada";
                string moved = move name;
                print(name);
            }
        "#;

        let error = compile_source(source).expect_err("use after move should fail");

        assert!(error.contains("borrow checker: use of moved value 'name'"));
    }

    #[test]
    fn borrow_checker_rejects_assignment_while_borrowed() {
        let source = r#"
            fn main() {
                int value = 1;
                ref int borrowed = borrow value;
                value = 2;
                print(value);
            }
        "#;

        let error = compile_source(source).expect_err("assignment while borrowed should fail");

        assert!(error.contains("borrow checker: cannot assign to 'value' while it is borrowed"));
    }

    #[test]
    fn ownership_checker_allows_class_transfer_without_move() {
        let source = r#"
            class Service {
                public string Name;
            }

            fn main() {
                Service first = new Service { Name = "one" };
                Service second = first;
                print(second.Name);
            }
        "#;

        let llvm_ir = compile_llvm_ir(source).expect("class transfer without move should succeed");
        assert!(llvm_ir.contains("call void @glitch_retain"));
    }

    #[test]
    fn ownership_checker_rejects_view_outliving_source() {
        let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> outer = new List<int>();
                IEnumerable<int> view = outer;
                {
                    List<int> inner = new List<int>();
                    view = inner;
                }
                foreach (int value in view) {
                    print(value);
                }
            }
        "#;

        let error = compile_source(source).expect_err("view outliving source should fail");

        assert!(error.contains(
            "ownership checker: function main: 'view' would outlive borrowed/view source 'inner'"
        ));
    }
}
