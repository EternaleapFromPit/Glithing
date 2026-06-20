mod ast;
mod borrowck;
mod bytecode;
mod cycles;
mod diagnostics;
mod leak;
mod lexer;
mod linker;
mod nuget;
mod llvm;
mod parser;
mod runtime;
pub mod realworld_smoke;
mod xunit_runner;
mod tir;
mod toolchain;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ast::*;
use borrowck::BorrowChecker;
use bytecode::BytecodeEmitter;
use diagnostics::CompatibilityAnalyzer;
use leak::LeakAnalyzer;
use lexer::Lexer;
use linker::{link_package_sources, strip_utf8_bom};
use nuget::{emit_nuget_package, NugetPackageSpec};
use llvm::LlvmEmitter;
use parser::Parser;
use tir::TypedProgram;
use toolchain::{emit_llvm_bitcode, emit_native_executable};

pub fn run_cli() -> Result<(), String> {
    let cwd = env::current_dir().map_err(|e| format!("failed to read current directory: {e}"))?;
    let args = env::args().skip(1).collect::<Vec<_>>();
    run_cli_with_args_from(args, &cwd)
}

fn run_cli_with_args_from(args: Vec<String>, cwd: &Path) -> Result<(), String> {
    if args.is_empty() {
        print_gl_help();
        return Ok(());
    }
    let first = args[0].as_str();
    if matches!(first, "help" | "--help" | "-h") {
        print_command_help(args.get(1).map(String::as_str));
        return Ok(());
    }
    if matches!(
        first,
        "new"
            | "restore"
            | "build"
            | "publish"
            | "run"
            | "test"
            | "clean"
            | "sln"
            | "store"
            | "watch"
            | "format"
    ) {
        return run_gl_subcommand(first, &args[1..], cwd);
    }
    run_legacy_compile_cli(args)
}

fn run_legacy_compile_cli(args: Vec<String>) -> Result<(), String> {
    let mut args = args.into_iter();
    let input = args.next().ok_or_else(|| legacy_usage().to_string())?;
    let mut bytecode_output = None;
    let mut llvm_ir_output = None;
    let mut llvm_bc_output = None;
    let mut exe_output = None;
    let mut leak_report_output = None;
    let mut nuget_output = None;
    let mut package_id = None;
    let mut package_version = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--emit-bytecode" => bytecode_output = args.next(),
            "--emit-llvm-ir" => llvm_ir_output = args.next(),
            "--emit-llvm-bc" => llvm_bc_output = args.next(),
            "--emit-exe" => exe_output = args.next(),
            "--emit-leak-report" => leak_report_output = args.next(),
            "--emit-nuget" => nuget_output = args.next(),
            "--package-id" => package_id = args.next(),
            "--package-version" => package_version = args.next(),
            other => return Err(format!("unknown argument '{other}'")),
        }
    }
    emit_compile_outputs(
        &input,
        CompileRequest {
            bytecode_output,
            llvm_ir_output,
            llvm_bc_output,
            exe_output,
            leak_report_output,
            nuget_output,
            package_id,
            package_version,
            default_native_output: true,
        },
    )
}

fn default_package_id(input: &str) -> String {
    let path = Path::new(input);
    path.file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Glitching.Package".to_string())
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
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
    {
        return read_project_source(path);
    }
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

struct CompileRequest {
    bytecode_output: Option<String>,
    llvm_ir_output: Option<String>,
    llvm_bc_output: Option<String>,
    exe_output: Option<String>,
    leak_report_output: Option<String>,
    nuget_output: Option<String>,
    package_id: Option<String>,
    package_version: Option<String>,
    default_native_output: bool,
}

fn emit_compile_outputs(input: &str, request: CompileRequest) -> Result<(), String> {
    let source = read_input_source(input)?;
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
    let compiled =
        run_on_large_stack(move || compile_source_with_options(&source, wants_llvm, false))?;
    for diagnostic in &compiled.diagnostics {
        eprintln!("{diagnostic}");
    }
    if let Some(path) = &request.bytecode_output {
        ensure_parent_dir(Path::new(path))?;
        fs::write(path, &compiled.bytecode).map_err(|e| format!("failed to write {path}: {e}"))?;
    }
    if let Some(path) = &request.llvm_ir_output {
        ensure_parent_dir(Path::new(path))?;
        fs::write(path, compiled.llvm_ir()?)
            .map_err(|e| format!("failed to write {path}: {e}"))?;
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
        let package_version = request
            .package_version
            .unwrap_or_else(|| "0.1.0".to_string());
        emit_nuget_package(
            NugetPackageSpec {
                package_id: &package_id,
                version: &package_version,
                linked_source: &compiled.linked_source,
                llvm_ir: compiled.llvm_ir()?,
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

fn run_gl_subcommand(command: &str, args: &[String], cwd: &Path) -> Result<(), String> {
    match command {
        "new" => run_gl_new(args, cwd),
        "restore" => run_gl_restore(args, cwd),
        "build" => run_gl_build(args, cwd),
        "publish" => run_gl_publish(args, cwd),
        "run" => run_gl_run(args, cwd),
        "test" => run_gl_test(args, cwd),
        "clean" => run_gl_clean(args, cwd),
        "sln" => run_gl_sln(args, cwd),
        "store" => run_gl_store(args, cwd),
        "watch" => run_gl_watch(args, cwd),
        "format" => run_gl_format(args, cwd),
        other => Err(format!("unknown gl command '{other}'")),
    }
}

fn run_gl_new(args: &[String], cwd: &Path) -> Result<(), String> {
    if args.is_empty() {
        print_command_help(Some("new"));
        return Ok(());
    }
    let template = args[0].as_str();
    let name = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| default_template_name(template));
    let project_dir = cwd.join(&name);
    if project_dir.exists() {
        return Err(format!(
            "cannot create project '{}': destination already exists",
            project_dir.display()
        ));
    }
    fs::create_dir_all(&project_dir)
        .map_err(|e| format!("failed to create {}: {e}", project_dir.display()))?;
    match template {
        "console" => write_console_template(&project_dir, &name)?,
        "classlib" => write_classlib_template(&project_dir, &name)?,
        "xunit" => write_xunit_template(&project_dir, &name)?,
        other => {
            return Err(format!(
                "unknown template '{other}'; supported templates: console, classlib, xunit"
            ))
        }
    }
    println!("Created {} project at {}", template, project_dir.display());
    Ok(())
}

fn run_gl_restore(args: &[String], cwd: &Path) -> Result<(), String> {
    let input = resolve_command_input(args, cwd)?;
    let targets = expand_solution_inputs(&input)?;
    for target in &targets {
        let _ = read_input_source(target)?;
        println!(
            "Restore completed for {} (source-linked packages only; no remote feed restore is required).",
            target
        );
    }
    Ok(())
}

fn run_gl_build(args: &[String], cwd: &Path) -> Result<(), String> {
    let (input, output_override, package_id, package_version) = parse_build_like_args(args, cwd)?;
    let targets = expand_solution_inputs(&input)?;
    for target in &targets {
        let output = build_output_for_target(target, output_override.as_deref(), "Debug");
        emit_compile_outputs(
            target,
            CompileRequest {
                bytecode_output: None,
                llvm_ir_output: None,
                llvm_bc_output: None,
                exe_output: Some(output.to_string_lossy().to_string()),
                leak_report_output: None,
                nuget_output: None,
                package_id: package_id.clone(),
                package_version: package_version.clone(),
                default_native_output: false,
            },
        )?;
        println!("Built {}", output.display());
    }
    Ok(())
}

fn run_gl_publish(args: &[String], cwd: &Path) -> Result<(), String> {
    let (input, output_override, package_id, package_version) = parse_build_like_args(args, cwd)?;
    let targets = expand_solution_inputs(&input)?;
    for target in &targets {
        let publish_output = publish_output_for_target(target, output_override.as_deref());
        emit_compile_outputs(
            target,
            CompileRequest {
                bytecode_output: None,
                llvm_ir_output: None,
                llvm_bc_output: None,
                exe_output: Some(publish_output.to_string_lossy().to_string()),
                leak_report_output: None,
                nuget_output: None,
                package_id: package_id.clone(),
                package_version: package_version.clone(),
                default_native_output: false,
            },
        )?;
        println!("Published {}", publish_output.display());
    }
    Ok(())
}

fn run_gl_run(args: &[String], cwd: &Path) -> Result<(), String> {
    let (build_args, program_args) = split_command_args(args);
    let input = resolve_command_input(&build_args, cwd)?;
    let temp_exe = temp_cli_executable(&input, "run");
    emit_compile_outputs(
        &input,
        CompileRequest {
            bytecode_output: None,
            llvm_ir_output: None,
            llvm_bc_output: None,
            exe_output: Some(temp_exe.to_string_lossy().to_string()),
            leak_report_output: None,
            nuget_output: None,
            package_id: None,
            package_version: None,
            default_native_output: false,
        },
    )?;
    let status = Command::new(&temp_exe)
        .args(program_args)
        .status()
        .map_err(|e| format!("failed to run {}: {e}", temp_exe.display()))?;
    if !status.success() {
        return Err(format!(
            "program exited with status {}",
            status.code().unwrap_or(1)
        ));
    }
    Ok(())
}

fn run_gl_test(args: &[String], cwd: &Path) -> Result<(), String> {
    let input = resolve_command_input(args, cwd)?;
    let targets = expand_solution_inputs(&input)?;
    for target in &targets {
        let temp_exe = temp_cli_executable(target, "test");
        emit_compile_outputs(
            target,
            CompileRequest {
                bytecode_output: None,
                llvm_ir_output: None,
                llvm_bc_output: None,
                exe_output: Some(temp_exe.to_string_lossy().to_string()),
                leak_report_output: None,
                nuget_output: None,
                package_id: None,
                package_version: None,
                default_native_output: false,
            },
        )?;
        let status = Command::new(&temp_exe)
            .status()
            .map_err(|e| format!("failed to run tests in {}: {e}", temp_exe.display()))?;
        if !status.success() {
            return Err(format!(
                "tests failed with status {}",
                status.code().unwrap_or(1)
            ));
        }
    }
    Ok(())
}

fn run_gl_clean(args: &[String], cwd: &Path) -> Result<(), String> {
    let input = resolve_command_input(args, cwd)?;
    for target in expand_solution_inputs(&input)? {
        let base_dir = project_root_for_input(&target);
        for path in [base_dir.join("bin"), base_dir.join("obj")] {
            if path.exists() {
                fs::remove_dir_all(&path)
                    .map_err(|e| format!("failed to remove {}: {e}", path.display()))?;
            }
        }
        println!("Cleaned {}", base_dir.display());
    }
    Ok(())
}

fn run_gl_sln(args: &[String], cwd: &Path) -> Result<(), String> {
    if args.is_empty() {
        print_command_help(Some("sln"));
        return Ok(());
    }
    match args[0].as_str() {
        "new" => {
            let name = args
                .get(1)
                .cloned()
                .unwrap_or_else(|| default_solution_name(cwd));
            let solution_path = cwd.join(format!("{name}.sln"));
            if solution_path.exists() {
                return Err(format!(
                    "cannot create solution '{}': destination already exists",
                    solution_path.display()
                ));
            }
            write_solution_file(&solution_path, &[])?;
            println!("Created solution {}", solution_path.display());
            Ok(())
        }
        "add" => {
            let (solution_path, project_args) = parse_solution_command_args(&args[1..], cwd)?;
            if project_args.is_empty() {
                return Err("gl sln add requires at least one project path".to_string());
            }
            let solution_dir = solution_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            let mut projects = read_solution_projects(&solution_path)?;
            for project in project_args {
                let project_path = cwd.join(project);
                let project_path = normalize_solution_project_path(&solution_dir, &project_path)?;
                if !projects.iter().any(|existing| existing == &project_path) {
                    projects.push(project_path);
                }
            }
            projects.sort();
            write_solution_file(&solution_path, &projects)?;
            println!("Updated solution {}", solution_path.display());
            Ok(())
        }
        "remove" => {
            let (solution_path, project_args) = parse_solution_command_args(&args[1..], cwd)?;
            if project_args.is_empty() {
                return Err("gl sln remove requires at least one project path".to_string());
            }
            let solution_dir = solution_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            let removals = project_args
                .iter()
                .map(|project| normalize_solution_project_path(&solution_dir, &cwd.join(project)))
                .collect::<Result<Vec<_>, _>>()?;
            let mut projects = read_solution_projects(&solution_path)?;
            projects.retain(|project| !removals.iter().any(|remove| remove == project));
            write_solution_file(&solution_path, &projects)?;
            println!("Updated solution {}", solution_path.display());
            Ok(())
        }
        "list" => {
            let solution_path = resolve_solution_path(None, cwd)?;
            for project in read_solution_projects(&solution_path)? {
                println!("{}", project.display());
            }
            Ok(())
        }
        other => Err(format!(
            "unknown gl sln command '{other}'; supported commands: new, add, remove, list"
        )),
    }
}

fn run_gl_store(args: &[String], cwd: &Path) -> Result<(), String> {
    let (input, output_override, package_id, package_version) = parse_build_like_args(args, cwd)?;
    let targets = expand_solution_inputs(&input)?;
    for target in &targets {
        let package_id = package_id
            .clone()
            .unwrap_or_else(|| default_package_id(target));
        let package_version = package_version
            .clone()
            .unwrap_or_else(|| "0.1.0".to_string());
        let output = store_output_for_target(target, output_override.as_deref(), &package_id, &package_version);
        emit_compile_outputs(
            target,
            CompileRequest {
                bytecode_output: None,
                llvm_ir_output: None,
                llvm_bc_output: None,
                exe_output: None,
                leak_report_output: None,
                nuget_output: Some(output.to_string_lossy().to_string()),
                package_id: Some(package_id.clone()),
                package_version: Some(package_version.clone()),
                default_native_output: false,
            },
        )?;
        println!("Stored {}", output.display());
    }
    Ok(())
}

fn run_gl_watch(args: &[String], cwd: &Path) -> Result<(), String> {
    if args.is_empty() {
        return Err("gl watch requires an inner command such as build, test, run, publish, clean, restore, or format".to_string());
    }
    let mut once = false;
    let mut poll_ms = 500u64;
    let mut max_iterations = None;
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "--once" => {
                once = true;
                index += 1;
            }
            "--poll-ms" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "gl watch --poll-ms requires a value".to_string())?;
                poll_ms = value
                    .parse::<u64>()
                    .map_err(|_| format!("invalid poll interval '{}'", value))?;
                index += 2;
            }
            "--max-iterations" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "gl watch --max-iterations requires a value".to_string())?;
                max_iterations = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| format!("invalid max-iterations '{}'", value))?,
                );
                index += 2;
            }
            _ => break,
        }
    }
    let command = args
        .get(index)
        .ok_or_else(|| "gl watch requires an inner command".to_string())?
        .clone();
    if matches!(command.as_str(), "watch" | "help" | "new" | "sln" | "store") {
        return Err(format!("gl watch does not support wrapping '{}'", command));
    }
    let inner_args = args[index + 1..].to_vec();
    let watch_input = infer_watch_input(&command, &inner_args, cwd)?;
    let watch_paths = collect_watch_paths(&watch_input)?;
    if watch_paths.is_empty() {
        return Err(format!("no watchable files found for {}", watch_input));
    }
    let mut snapshot = snapshot_watch_paths(&watch_paths)?;
    let mut iterations = 0usize;
    loop {
        run_gl_subcommand(&command, &inner_args, cwd)?;
        iterations += 1;
        if once || max_iterations.is_some_and(|limit| iterations >= limit) {
            break;
        }
        loop {
            std::thread::sleep(std::time::Duration::from_millis(poll_ms));
            let next = snapshot_watch_paths(&watch_paths)?;
            if next != snapshot {
                snapshot = next;
                break;
            }
        }
    }
    Ok(())
}

fn run_gl_format(args: &[String], cwd: &Path) -> Result<(), String> {
    let mut check = false;
    let mut input = None;
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            other if other.starts_with("--") => return Err(format!("unknown argument '{other}'")),
            other if input.is_none() => input = Some(other.to_string()),
            other => return Err(format!("unexpected argument '{other}'")),
        }
    }
    let input = resolve_command_input_from_optional(input, cwd)?;
    let files = collect_format_paths(&input)?;
    if files.is_empty() {
        return Err(format!("no source files found to format for {}", input));
    }
    let mut changed = 0usize;
    for file in &files {
        let original = fs::read_to_string(file)
            .map_err(|e| format!("failed to read {}: {e}", file.display()))?;
        let formatted = format_source_text(&original);
        if formatted != original {
            changed += 1;
            if check {
                println!("Would format {}", file.display());
            } else {
                fs::write(file, formatted)
                    .map_err(|e| format!("failed to write {}: {e}", file.display()))?;
                println!("Formatted {}", file.display());
            }
        }
    }
    if check && changed > 0 {
        return Err(format!("{changed} files would be reformatted"));
    }
    if !check {
        println!("Format complete: {} file(s) changed", changed);
    }
    Ok(())
}

fn print_gl_help() {
    println!("{}", gl_help_text());
}

fn print_command_help(command: Option<&str>) {
    match command {
        Some("new") => println!("gl new <console|classlib|xunit> [name]"),
        Some("restore") => println!("gl restore [project|directory|file]"),
        Some("build") => println!("gl build [project|directory|file] [--output <path>]"),
        Some("publish") => println!("gl publish [project|directory|file] [--output <path>]"),
        Some("run") => println!("gl run [project|directory|file] [-- <program args...>]"),
        Some("test") => println!("gl test [project|directory|file]"),
        Some("clean") => println!("gl clean [project|directory|file]"),
        Some("sln") => println!("gl sln <new|add|remove|list> [...]"),
        Some("store") => println!("gl store [project|solution|directory|file] [--output <path>] [--package-id <id>] [--package-version <version>]"),
        Some("watch") => println!("gl watch [--poll-ms <milliseconds>] [--once] <build|publish|run|test|clean|restore|format> [...]"),
        Some("format") => println!("gl format [project|solution|directory|file] [--check]"),
        _ => print_gl_help(),
    }
}

fn gl_help_text() -> &'static str {
    "gl <command> [arguments]\n\nCommands:\n  gl new\n  gl restore\n  gl build\n  gl publish\n  gl run\n  gl test\n  gl clean\n  gl sln\n  gl help\n  gl store\n  gl watch\n  gl format\n\nLegacy compiler mode:\n  glitchc <input.{gl,cs}> [--emit-bytecode <output.gbc>] [--emit-llvm-ir <output.ll>] [--emit-llvm-bc <output.bc>] [--emit-exe <output.exe>] [--emit-leak-report <output.txt>] [--emit-nuget <output.nupkg>]"
}

fn legacy_usage() -> &'static str {
    "usage: glitchc <input.{gl,cs}> [--emit-bytecode <output.gbc>] [--emit-llvm-ir <output.ll>] [--emit-llvm-bc <output.bc>] [--emit-exe <output.exe>] [--emit-leak-report <output.txt>] [--emit-nuget <output.nupkg>]"
}

fn parse_build_like_args(
    args: &[String],
    cwd: &Path,
) -> Result<(String, Option<PathBuf>, Option<String>, Option<String>), String> {
    let mut input = None;
    let mut output = None;
    let mut package_id = None;
    let mut package_version = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--output" => output = iter.next().map(PathBuf::from),
            "--package-id" => package_id = iter.next().cloned(),
            "--package-version" => package_version = iter.next().cloned(),
            other if other.starts_with("--") => return Err(format!("unknown argument '{other}'")),
            other if input.is_none() => input = Some(other.to_string()),
            other => return Err(format!("unexpected argument '{other}'")),
        }
    }
    let input = resolve_command_input_from_optional(input, cwd)?;
    Ok((input, output, package_id, package_version))
}

fn split_command_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    if let Some(index) = args.iter().position(|arg| arg == "--") {
        (args[..index].to_vec(), args[index + 1..].to_vec())
    } else {
        (args.to_vec(), Vec::new())
    }
}

fn resolve_command_input(args: &[String], cwd: &Path) -> Result<String, String> {
    let mut input = None;
    for arg in args {
        if arg.starts_with("--") {
            return Err(format!("unknown argument '{arg}'"));
        }
        if input.is_some() {
            return Err(format!("unexpected argument '{arg}'"));
        }
        input = Some(arg.clone());
    }
    resolve_command_input_from_optional(input, cwd)
}

fn resolve_command_input_from_optional(input: Option<String>, cwd: &Path) -> Result<String, String> {
    if let Some(input) = input {
        return Ok(cwd.join(input).to_string_lossy().to_string());
    }
    let mut solutions = Vec::new();
    let mut projects = Vec::new();
    let mut sources = Vec::new();
    for entry in fs::read_dir(cwd).map_err(|e| format!("failed to read {}: {e}", cwd.display()))? {
        let entry = entry.map_err(|e| format!("failed to read {}: {e}", cwd.display()))?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("sln"))
        {
            solutions.push(path);
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
        {
            projects.push(path);
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("gl") || ext.eq_ignore_ascii_case("cs"))
        {
            sources.push(path);
        }
    }
    if projects.len() == 1 {
        return Ok(projects.remove(0).to_string_lossy().to_string());
    }
    if projects.len() > 1 {
        return Err("multiple .csproj files found; specify the project explicitly".to_string());
    }
    if solutions.len() == 1 {
        return Ok(solutions.remove(0).to_string_lossy().to_string());
    }
    if solutions.len() > 1 {
        return Err("multiple .sln files found; specify the solution explicitly".to_string());
    }
    let program_cs = cwd.join("Program.cs");
    let main_gl = cwd.join("main.gl");
    if program_cs.exists() || main_gl.exists() {
        return Ok(cwd.to_string_lossy().to_string());
    }
    if sources.len() == 1 {
        return Ok(sources.remove(0).to_string_lossy().to_string());
    }
    Err("could not infer input from the current directory; specify a .csproj, directory, .cs, or .gl file".to_string())
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create {}: {e}", parent.display()))?;
        }
    }
    Ok(())
}

fn project_root_for_input(input: &str) -> PathBuf {
    let path = Path::new(input);
    if path.is_dir() {
        path.to_path_buf()
    } else if is_solution_path(path) {
        path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf()
    } else {
        path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf()
    }
}

fn artifact_name_for_input(input: &str) -> String {
    let path = Path::new(input);
    if path.is_dir() {
        path.file_name()
            .and_then(|s| s.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("app")
            .to_string()
    } else {
        path.file_stem()
            .and_then(|s| s.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("app")
            .to_string()
    }
}

fn default_build_output_path(input: &str, configuration: &str) -> PathBuf {
    let base = project_root_for_input(input);
    base.join("bin")
        .join(configuration)
        .join(format!("{}.exe", artifact_name_for_input(input)))
}

fn default_publish_output_path(input: &str) -> PathBuf {
    let base = project_root_for_input(input);
    base.join("bin")
        .join("Release")
        .join("publish")
        .join(format!("{}.exe", artifact_name_for_input(input)))
}

fn build_output_for_target(target: &str, output_override: Option<&Path>, configuration: &str) -> PathBuf {
    match output_override {
        Some(path) if path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("exe")) => path.to_path_buf(),
        Some(path) => path.join(format!("{}.exe", artifact_name_for_input(target))),
        None => default_build_output_path(target, configuration),
    }
}

fn normalize_publish_output_path(output: &Path, input: &str) -> PathBuf {
    if output.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("exe")) {
        output.to_path_buf()
    } else {
        output.join(format!("{}.exe", artifact_name_for_input(input)))
    }
}

fn publish_output_for_target(target: &str, output_override: Option<&Path>) -> PathBuf {
    output_override
        .map(|path| normalize_publish_output_path(path, target))
        .unwrap_or_else(|| default_publish_output_path(target))
}

fn store_output_for_target(
    target: &str,
    output_override: Option<&Path>,
    package_id: &str,
    package_version: &str,
) -> PathBuf {
    let file_name = format!("{package_id}.{package_version}.nupkg");
    match output_override {
        Some(path) if path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("nupkg")) => path.to_path_buf(),
        Some(path) => path.join(package_id).join(package_version).join(file_name),
        None => project_root_for_input(target)
            .join(".gl")
            .join("store")
            .join(package_id)
            .join(package_version)
            .join(file_name),
    }
}

fn temp_cli_executable(input: &str, suffix: &str) -> PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    env::temp_dir().join(format!("gl-{}-{suffix}-{stamp}.exe", artifact_name_for_input(input)))
}

fn default_template_name(template: &str) -> String {
    match template {
        "console" => "GlitchingApp".to_string(),
        "classlib" => "GlitchingLibrary".to_string(),
        "xunit" => "GlitchingTests".to_string(),
        other => other.to_string(),
    }
}

fn write_console_template(project_dir: &Path, name: &str) -> Result<(), String> {
    fs::write(project_dir.join(format!("{name}.csproj")), simple_project_file(name, true))
        .map_err(|e| format!("failed to write project file: {e}"))?;
    fs::write(
        project_dir.join("Program.cs"),
        "using System;\n\nfn main() {\n    print(\"Hello from Glitching\");\n}\n",
    )
    .map_err(|e| format!("failed to write Program.cs: {e}"))?;
    Ok(())
}

fn write_classlib_template(project_dir: &Path, name: &str) -> Result<(), String> {
    fs::write(project_dir.join(format!("{name}.csproj")), simple_project_file(name, false))
        .map_err(|e| format!("failed to write project file: {e}"))?;
    fs::write(
        project_dir.join("Library.cs"),
        "public class Library {\n    public string Message() {\n        return \"Hello from Glitching\";\n    }\n}\n",
    )
    .map_err(|e| format!("failed to write Library.cs: {e}"))?;
    Ok(())
}

fn write_xunit_template(project_dir: &Path, name: &str) -> Result<(), String> {
    fs::write(project_dir.join(format!("{name}.csproj")), simple_project_file(name, false))
        .map_err(|e| format!("failed to write project file: {e}"))?;
    fs::write(
        project_dir.join("SampleTests.cs"),
        "using Xunit;\n\nclass SampleTests {\n    [Fact]\n    void TruthIsTrue() {\n        Assert.True(true);\n    }\n}\n",
    )
    .map_err(|e| format!("failed to write SampleTests.cs: {e}"))?;
    Ok(())
}

fn simple_project_file(name: &str, executable: bool) -> String {
    let output_type = if executable { "Exe" } else { "Library" };
    format!(
        "<Project Sdk=\"Glitching\">\n  <PropertyGroup>\n    <OutputType>{output_type}</OutputType>\n    <TargetFramework>gl1.0</TargetFramework>\n    <AssemblyName>{name}</AssemblyName>\n  </PropertyGroup>\n</Project>\n"
    )
}

fn default_solution_name(cwd: &Path) -> String {
    cwd.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("GlitchingSolution")
        .to_string()
}

fn is_solution_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("sln"))
}

fn parse_solution_command_args(args: &[String], cwd: &Path) -> Result<(PathBuf, Vec<String>), String> {
    if let Some(first) = args.first() {
        if first.ends_with(".sln") {
            return Ok((cwd.join(first), args[1..].to_vec()));
        }
    }
    Ok((resolve_solution_path(None, cwd)?, args.to_vec()))
}

fn resolve_solution_path(explicit: Option<String>, cwd: &Path) -> Result<PathBuf, String> {
    if let Some(path) = explicit {
        return Ok(cwd.join(path));
    }
    let mut solutions = Vec::new();
    for entry in fs::read_dir(cwd).map_err(|e| format!("failed to read {}: {e}", cwd.display()))? {
        let entry = entry.map_err(|e| format!("failed to read {}: {e}", cwd.display()))?;
        let path = entry.path();
        if is_solution_path(&path) {
            solutions.push(path);
        }
    }
    if solutions.len() == 1 {
        return Ok(solutions.remove(0));
    }
    if solutions.is_empty() {
        Err("no .sln file found in the current directory".to_string())
    } else {
        Err("multiple .sln files found; specify the solution explicitly".to_string())
    }
}

fn normalize_solution_project_path(solution_dir: &Path, project_path: &Path) -> Result<PathBuf, String> {
    let absolute = if project_path.is_absolute() {
        project_path.to_path_buf()
    } else {
        fs::canonicalize(project_path)
            .map_err(|e| format!("failed to resolve {}: {e}", project_path.display()))?
    };
    if let Ok(relative) = absolute.strip_prefix(solution_dir) {
        Ok(relative.to_path_buf())
    } else {
        Ok(absolute)
    }
}

fn write_solution_file(solution_path: &Path, projects: &[PathBuf]) -> Result<(), String> {
    ensure_parent_dir(solution_path)?;
    let solution_dir = solution_path
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let mut text = String::from("Microsoft Visual Studio Solution File, Format Version 12.00\n# Glitching Solution\n");
    for project in projects {
        let name = project
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("Project");
        let path = if project.is_absolute() {
            normalize_solution_project_path(solution_dir, project)?
        } else {
            project.clone()
        };
        text.push_str(&format!(
            "Project(\"Glitching\") = \"{name}\", \"{}\"\nEndProject\n",
            path.display()
        ));
    }
    fs::write(solution_path, text)
        .map_err(|e| format!("failed to write {}: {e}", solution_path.display()))?;
    Ok(())
}

fn read_solution_projects(solution_path: &Path) -> Result<Vec<PathBuf>, String> {
    let text = fs::read_to_string(solution_path)
        .map_err(|e| format!("failed to read {}: {e}", solution_path.display()))?;
    let mut projects = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with("Project(") {
            continue;
        }
        let Some(comma_index) = line.find(", ") else {
            continue;
        };
        let Some(path_start) = line[comma_index + 2..].find('"') else {
            continue;
        };
        let rest = &line[comma_index + 2 + path_start + 1..];
        let Some(path_end) = rest.find('"') else {
            continue;
        };
        projects.push(PathBuf::from(&rest[..path_end]));
    }
    Ok(projects)
}

fn expand_solution_inputs(input: &str) -> Result<Vec<String>, String> {
    let input_path = Path::new(input);
    if !is_solution_path(input_path) {
        return Ok(vec![input.to_string()]);
    }
    let solution_dir = input_path
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let mut projects = Vec::new();
    for project in read_solution_projects(input_path)? {
        let path = if project.is_absolute() {
            project
        } else {
            solution_dir.join(project)
        };
        projects.push(path.to_string_lossy().to_string());
    }
    if projects.is_empty() {
        return Err(format!("solution {} does not contain any projects", input_path.display()));
    }
    Ok(projects)
}

fn infer_watch_input(command: &str, args: &[String], cwd: &Path) -> Result<String, String> {
    match command {
        "build" | "publish" | "restore" | "clean" | "store" => {
            Ok(parse_build_like_args(args, cwd)?.0)
        }
        "test" | "format" => resolve_command_input(args, cwd),
        "run" => {
            let (build_args, _) = split_command_args(args);
            resolve_command_input(&build_args, cwd)
        }
        other => Err(format!("gl watch does not support '{}'", other)),
    }
}

fn collect_watch_paths(input: &str) -> Result<Vec<PathBuf>, String> {
    let path = Path::new(input);
    let mut files = Vec::new();
    if is_solution_path(path) {
        files.push(path.to_path_buf());
        for project in expand_solution_inputs(input)? {
            collect_watch_paths_into(Path::new(&project), &mut files)?;
        }
    } else {
        collect_watch_paths_into(path, &mut files)?;
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_watch_paths_into(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_dir() {
        collect_source_files(path, output)?;
        for entry in fs::read_dir(path).map_err(|e| format!("failed to read {}: {e}", path.display()))? {
            let entry = entry.map_err(|e| format!("failed to read {}: {e}", path.display()))?;
            let candidate = entry.path();
            if candidate.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("csproj")) {
                output.push(candidate);
            }
        }
    } else if is_solution_path(path) || path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| matches!(ext, "csproj" | "gl" | "cs")) {
        output.push(path.to_path_buf());
        if path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("csproj")) {
            let mut files = Vec::new();
            let mut visited = HashSet::new();
            collect_project_source_files(path, &mut visited, &mut files)?;
            output.extend(files);
        }
    }
    Ok(())
}

fn snapshot_watch_paths(paths: &[PathBuf]) -> Result<Vec<(PathBuf, u64, u128)>, String> {
    let mut snapshot = Vec::new();
    for path in paths {
        let metadata = fs::metadata(path)
            .map_err(|e| format!("failed to read metadata for {}: {e}", path.display()))?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        snapshot.push((path.clone(), metadata.len(), modified));
    }
    snapshot.sort();
    Ok(snapshot)
}

fn collect_format_paths(input: &str) -> Result<Vec<PathBuf>, String> {
    let path = Path::new(input);
    let mut files = Vec::new();
    if is_solution_path(path) {
        for project in expand_solution_inputs(input)? {
            files.extend(collect_format_paths(&project)?);
        }
    } else if path.is_dir() {
        collect_source_files(path, &mut files)?;
    } else if path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("csproj")) {
        let mut visited = HashSet::new();
        collect_project_source_files(path, &mut visited, &mut files)?;
    } else if path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| matches!(ext, "gl" | "cs")) {
        files.push(path.to_path_buf());
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn format_source_text(source: &str) -> String {
    let normalized = source.replace("\r\n", "\n");
    let mut output = String::new();
    let mut indent = 0usize;
    let mut previous_blank = false;
    for raw_line in normalized.lines() {
        let line = raw_line.trim_end();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !previous_blank && !output.is_empty() {
                output.push('\n');
            }
            previous_blank = true;
            continue;
        }
        previous_blank = false;
        let leading_closers = count_leading_closers(trimmed);
        let current_indent = indent.saturating_sub(leading_closers);
        output.push_str(&"    ".repeat(current_indent));
        output.push_str(trimmed);
        output.push('\n');
        let (opens, closes) = count_braces(trimmed);
        let closes_after_leading = closes.saturating_sub(leading_closers);
        indent = current_indent + opens.saturating_sub(closes_after_leading);
    }
    if output.is_empty() {
        String::from("\n")
    } else {
        output
    }
}

fn count_leading_closers(line: &str) -> usize {
    let mut count = 0usize;
    for ch in line.chars() {
        match ch {
            '}' => count += 1,
            ' ' | '\t' => continue,
            _ => break,
        }
    }
    count
}

fn count_braces(line: &str) -> (usize, usize) {
    let mut opens = 0usize;
    let mut closes = 0usize;
    let mut in_string = false;
    let mut escape = false;
    for ch in line.chars() {
        if in_string {
            if escape {
                escape = false;
                continue;
            }
            match ch {
                '\\' => escape = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' => opens += 1,
            '}' => closes += 1,
            _ => {}
        }
    }
    (opens, closes)
}

fn run_on_large_stack<T, F>(f: F) -> Result<T, String>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, String> + Send + 'static,
{
    let builder = std::thread::Builder::new().stack_size(1024 * 1024 * 1024);
    let handle = builder
        .spawn(f)
        .map_err(|e| format!("failed to spawn compilation thread: {e}"))?;
    handle
        .join()
        .map_err(|_| "compilation thread panicked".to_string())?
}

fn read_project_source(project_path: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    let mut visited_projects = HashSet::<PathBuf>::new();
    collect_project_source_files(project_path, &mut visited_projects, &mut files)?;
    files.sort();
    files.dedup();
    let mut source = String::new();
    for file in files {
        let text = fs::read_to_string(&file)
            .map_err(|e| format!("failed to read {}: {e}", file.display()))?;
        source.push_str(&format!("// __FILE_PATH__: {}\n", file.display()));
        source.push_str(strip_utf8_bom(&text));
        source.push_str("\n__FILE_BOUNDARY__;\n");
    }
    Ok(source)
}

fn collect_project_source_files(
    project_path: &Path,
    visited_projects: &mut HashSet<PathBuf>,
    output: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let canonical = fs::canonicalize(project_path)
        .map_err(|e| format!("failed to resolve project {}: {e}", project_path.display()))?;
    if !visited_projects.insert(canonical.clone()) {
        return Ok(());
    }

    let project_dir = canonical.parent().ok_or_else(|| {
        format!(
            "project {} does not have a parent directory",
            canonical.display()
        )
    })?;
    collect_source_files(project_dir, output)?;

    let project_text = fs::read_to_string(&canonical)
        .map_err(|e| format!("failed to read project {}: {e}", canonical.display()))?;
    for reference in extract_project_references(&project_text) {
        let reference_path = project_dir.join(reference);
        if reference_path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
        {
            collect_project_source_files(&reference_path, visited_projects, output)?;
        }
    }
    Ok(())
}

fn extract_project_references(project_text: &str) -> Vec<PathBuf> {
    let mut references = Vec::new();
    for line in project_text.lines() {
        let line = line.trim();
        if !line.contains("<ProjectReference") {
            continue;
        }
        if let Some(include) = extract_xml_include_attr(line) {
            references.push(PathBuf::from(include));
        }
    }
    references
}

fn extract_xml_include_attr(line: &str) -> Option<String> {
    for key in ["Include=\"", "Include='"] {
        if let Some(start) = line.find(key) {
            let rest = &line[start + key.len()..];
            let end = rest.find(['"', '\'']).unwrap_or(rest.len());
            return Some(rest[..end].to_string());
        }
    }
    None
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
fn compile_llvm_ir_from_path(input: &str) -> Result<String, String> {
    let source = read_input_source(input)?;
    run_on_large_stack(move || {
        Ok(compile_source_with_options(&source, true, false)?
            .llvm_ir()?
            .to_string())
    })
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

#[derive(Debug)]
struct CompileOutput {
    bytecode: String,
    llvm_ir: Option<String>,
    leak_report: String,
    diagnostics: Vec<String>,
    linked_source: String,
    #[allow(dead_code)]
    package_id: Option<String>,
}

impl CompileOutput {
    fn llvm_ir(&self) -> Result<&str, String> {
        self.llvm_ir
            .as_deref()
            .ok_or_else(|| "LLVM IR was not requested".to_string())
    }
}

#[cfg(test)]
fn compile_source_with_metadata(source: &str) -> Result<CompileOutput, String> {
    compile_source_with_options(source, false, false)
}

fn compile_source_with_options(
    source: &str,
    emit_llvm: bool,
    _emit_c: bool,
) -> Result<CompileOutput, String> {
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
    Ok(CompileOutput {
        bytecode,
        llvm_ir,
        leak_report,
        diagnostics,
        linked_source,
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
