use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::compile::{default_package_id, emit_compile_outputs, CompileRequest};
use crate::project::{
    default_solution_name, ensure_parent_dir, expand_solution_inputs,
    load_project_spec_for_input, normalize_solution_project_path, parse_solution_command_args,
    read_solution_projects, resolve_solution_path, write_solution_file, ProjectSpec,
};
use crate::workspace::{
    collect_format_paths, collect_watch_paths, format_source_text, infer_watch_input, read_input_source,
    snapshot_watch_paths,
};

pub(crate) fn run_cli_with_args_from(args: Vec<String>, cwd: &Path) -> Result<(), String> {
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
        if let Some(project) = command_project_spec(target)? {
            if !project.package_references.is_empty() {
                println!(
                    "Restore completed for {} ({} package reference(s), source-linked/local packages only).",
                    target,
                    project.package_references.len()
                );
                continue;
            }
        }
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
        emit_target_artifact(target, &output, package_id.clone(), package_version.clone())?;
        println!("Built {}", output.display());
    }
    Ok(())
}

fn run_gl_publish(args: &[String], cwd: &Path) -> Result<(), String> {
    let (input, output_override, package_id, package_version) = parse_build_like_args(args, cwd)?;
    let targets = expand_solution_inputs(&input)?;
    for target in &targets {
        let publish_output = publish_output_for_target(target, output_override.as_deref());
        emit_target_artifact(target, &publish_output, package_id.clone(), package_version.clone())?;
        if let Some(project) = command_project_spec(target)? {
            let publish_dir = publish_output
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            for (relative, bytes) in crate::project::collect_publishable_project_files(Some(&project))? {
                let destination = publish_dir.join(relative);
                ensure_parent_dir(&destination)?;
                fs::write(&destination, bytes)
                    .map_err(|e| format!("failed to write {}: {e}", destination.display()))?;
            }
        }
        println!("Published {}", publish_output.display());
    }
    Ok(())
}

fn run_gl_run(args: &[String], cwd: &Path) -> Result<(), String> {
    let (build_args, program_args) = split_command_args(args);
    let input = resolve_command_input(&build_args, cwd)?;
    if let Some(project) = command_project_spec(&input)? {
        if !project.is_runnable() {
            return Err(format!(
                "cannot run project {} with OutputType {}; only executable projects are runnable",
                project.path.display(),
                project.output_type_or_default()
            ));
        }
    }
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
        if let Some(project) = command_project_spec(target)? {
            if !project.should_run_tests() {
                return Err(format!(
                    "project {} is not marked as a test project; set <IsTestProject>true</IsTestProject> or add an xUnit package reference",
                    project.path.display()
                ));
            }
        }
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
        let project = command_project_spec(target)?;
        let package_id = package_id.clone().unwrap_or_else(|| {
            project
                .as_ref()
                .map(ProjectSpec::assembly_name_or_default)
                .unwrap_or_else(|| default_package_id(target))
        });
        let package_version = package_version
            .clone()
            .unwrap_or_else(|| "0.1.0".to_string());
        let output =
            store_output_for_target(target, output_override.as_deref(), &package_id, &package_version);
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
    let mut previous = snapshot_watch_paths(&watch_paths)?;
    let mut iterations = 0usize;
    loop {
        run_gl_subcommand(&command, &inner_args, cwd)?;
        iterations += 1;
        if once || max_iterations.is_some_and(|limit| iterations >= limit) {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(poll_ms));
        loop {
            let current = snapshot_watch_paths(&watch_paths)?;
            if current != previous {
                previous = current;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(poll_ms));
        }
    }
}

fn run_gl_format(args: &[String], cwd: &Path) -> Result<(), String> {
    let (check_only, input) = if matches!(args.first().map(String::as_str), Some("--check")) {
        (true, resolve_command_input(&args[1..], cwd)?)
    } else {
        (false, resolve_command_input(args, cwd)?)
    };
    let mut changed = 0usize;
    for path in collect_format_paths(&input)? {
        let original = fs::read_to_string(&path)
            .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
        let formatted = format_source_text(&original);
        if formatted != original {
            if check_only {
                return Err(format!("formatting required for {}", path.display()));
            }
            fs::write(&path, formatted)
                .map_err(|e| format!("failed to write {}: {e}", path.display()))?;
            println!("Formatted {}", path.display());
            changed += 1;
        }
    }
    println!("Format complete: {} file(s) changed", changed);
    Ok(())
}

fn print_gl_help() {
    println!("{}", gl_help_text());
}

fn print_command_help(command: Option<&str>) {
    match command {
        Some("new") => println!("gl new console|classlib|xunit [name]"),
        Some("restore") => println!("gl restore [project|solution|directory|file]"),
        Some("build") => println!("gl build [project|solution|directory|file] [--output <path>]"),
        Some("publish") => println!("gl publish [project|solution|directory|file] [--output <path>]"),
        Some("run") => println!("gl run [project|directory|file] [-- <program args...>]"),
        Some("test") => println!("gl test [project|solution|directory|file]"),
        Some("clean") => println!("gl clean [project|solution|directory|file]"),
        Some("sln") => println!("gl sln new|add|remove|list"),
        Some("store") => println!("gl store [project|solution|directory|file] [--output <path>] [--package-id <id>] [--package-version <version>]"),
        Some("watch") => println!("gl watch [--poll-ms <ms>] [--once] <build|publish|run|test|clean|restore|format> [...]"),
        Some("format") => println!("gl format [project|solution|directory|file] [--check]"),
        _ => print_gl_help(),
    }
}

fn gl_help_text() -> &'static str {
    "gl <command> [arguments]\n\nCommands:\n  gl new\n  gl restore\n  gl build\n  gl publish\n  gl run\n  gl test\n  gl clean\n  gl sln\n  gl help\n  gl store\n  gl watch\n  gl format\n\nLegacy compiler mode:\n  glitchc <input.{gl,cs}> [--emit-bytecode <output.gbc>] [--emit-llvm-ir <output.ll>] [--emit-llvm-bc <output.bc>] [--emit-exe <output.exe>] [--emit-leak-report <output.txt>] [--emit-nuget <output.nupkg>]"
}

fn legacy_usage() -> &'static str {
    "usage: glitchc <input.gl|input.cs|project.csproj|directory> [--emit-bytecode <output.gbc>] [--emit-llvm-ir <output.ll>] [--emit-llvm-bc <output.bc>] [--emit-exe <output.exe>] [--emit-leak-report <output.txt>] [--emit-nuget <output.nupkg>] [--package-id <id>] [--package-version <version>]"
}

pub(crate) fn parse_build_like_args(
    args: &[String],
    cwd: &Path,
) -> Result<(String, Option<PathBuf>, Option<String>, Option<String>), String> {
    let mut input = None;
    let mut output = None;
    let mut package_id = None;
    let mut package_version = None;
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "--output" => {
                output = Some(cwd.join(
                    args.get(index + 1)
                        .ok_or_else(|| "--output requires a value".to_string())?,
                ));
                index += 2;
            }
            "--package-id" => {
                package_id = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--package-id requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--package-version" => {
                package_version = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--package-version requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            value if value.starts_with("--") => {
                return Err(format!("unknown argument '{}'", value));
            }
            value => {
                if input.is_some() {
                    return Err(format!("unexpected extra argument '{}'", value));
                }
                input = Some(value.to_string());
                index += 1;
            }
        }
    }
    Ok((resolve_command_input_from_optional(input, cwd)?, output, package_id, package_version))
}

fn command_project_spec(input: &str) -> Result<Option<ProjectSpec>, String> {
    load_project_spec_for_input(input)
}

fn emit_target_artifact(
    input: &str,
    output: &Path,
    package_id: Option<String>,
    package_version: Option<String>,
) -> Result<(), String> {
    let project = load_project_spec_for_input(input)?;
    match project
        .as_ref()
        .map(ProjectSpec::build_artifact_kind)
        .unwrap_or(crate::project::BuildArtifactKind::NativeExe)
    {
        crate::project::BuildArtifactKind::NativeExe => emit_compile_outputs(
            input,
            CompileRequest {
                bytecode_output: None,
                llvm_ir_output: None,
                llvm_bc_output: None,
                exe_output: Some(output.to_string_lossy().to_string()),
                leak_report_output: None,
                nuget_output: None,
                package_id,
                package_version,
                default_native_output: false,
            },
        ),
        crate::project::BuildArtifactKind::LlvmBitcode => emit_compile_outputs(
            input,
            CompileRequest {
                bytecode_output: None,
                llvm_ir_output: None,
                llvm_bc_output: Some(output.to_string_lossy().to_string()),
                exe_output: None,
                leak_report_output: None,
                nuget_output: None,
                package_id,
                package_version,
                default_native_output: false,
            },
        ),
    }
}

pub(crate) fn split_command_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    if let Some(separator) = args.iter().position(|arg| arg == "--") {
        (args[..separator].to_vec(), args[separator + 1..].to_vec())
    } else {
        (args.to_vec(), Vec::new())
    }
}

pub(crate) fn resolve_command_input(args: &[String], cwd: &Path) -> Result<String, String> {
    resolve_command_input_from_optional(args.first().cloned(), cwd)
}

pub(crate) fn resolve_command_input_from_optional(
    input: Option<String>,
    cwd: &Path,
) -> Result<String, String> {
    let path = match input {
        Some(value) => cwd.join(value),
        None => cwd.to_path_buf(),
    };
    if path.is_dir() {
        if let Some(project) = crate::project::find_single_project_in_dir(&path)? {
            return Ok(project.to_string_lossy().to_string());
        }
    }
    Ok(path.to_string_lossy().to_string())
}

fn project_root_for_input(input: &str) -> PathBuf {
    let path = Path::new(input);
    if path.is_dir() {
        return path.to_path_buf();
    }
    path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf()
}

fn artifact_name_for_input(input: &str) -> String {
    if let Ok(Some(project)) = load_project_spec_for_input(input) {
        return project.assembly_name_or_default();
    }
    let path = Path::new(input);
    path.file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("app")
        .to_string()
}

fn artifact_extension_for_input(input: &str) -> &'static str {
    if let Ok(Some(project)) = load_project_spec_for_input(input) {
        return match project.build_artifact_kind() {
            crate::project::BuildArtifactKind::NativeExe => "exe",
            crate::project::BuildArtifactKind::LlvmBitcode => "bc",
        };
    }
    "exe"
}

fn default_build_output_path(input: &str, configuration: &str) -> PathBuf {
    let root = project_root_for_input(input);
    let name = artifact_name_for_input(input);
    let extension = artifact_extension_for_input(input);
    root.join("bin")
        .join(configuration)
        .join(format!("{name}.{extension}"))
}

fn default_publish_output_path(input: &str) -> PathBuf {
    let root = project_root_for_input(input);
    let name = artifact_name_for_input(input);
    let extension = artifact_extension_for_input(input);
    root.join("bin")
        .join("Release")
        .join("publish")
        .join(format!("{name}.{extension}"))
}

fn build_output_for_target(target: &str, output_override: Option<&Path>, configuration: &str) -> PathBuf {
    output_override
        .map(|path| {
            if path.extension().is_none() {
                path.join(format!(
                    "{}.{}",
                    artifact_name_for_input(target),
                    artifact_extension_for_input(target)
                ))
            } else {
                path.to_path_buf()
            }
        })
        .unwrap_or_else(|| default_build_output_path(target, configuration))
}

fn normalize_publish_output_path(output: &Path, input: &str) -> PathBuf {
    if output.extension().is_none() {
        output.join(format!(
            "{}.{}",
            artifact_name_for_input(input),
            artifact_extension_for_input(input)
        ))
    } else {
        output.to_path_buf()
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
    output_override
        .map(|path| {
            if path.extension().is_none() {
                path.join(format!("{package_id}.{package_version}.nupkg"))
            } else {
                path.to_path_buf()
            }
        })
        .unwrap_or_else(|| {
            project_root_for_input(target)
                .join(".gl")
                .join("store")
                .join(package_id)
                .join(package_version)
                .join(format!("{package_id}.{package_version}.nupkg"))
        })
}

pub(crate) fn temp_cli_executable(input: &str, suffix: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let name = artifact_name_for_input(input);
    crate::toolchain::native_host_temp_dir().join(format!("gl-{name}-{suffix}-{stamp}.exe"))
}

fn default_template_name(template: &str) -> String {
    match template {
        "console" => "ConsoleApp".to_string(),
        "classlib" => "ClassLibrary".to_string(),
        "xunit" => "Tests".to_string(),
        _ => "App".to_string(),
    }
}

fn write_console_template(project_dir: &Path, name: &str) -> Result<(), String> {
    fs::write(project_dir.join(format!("{name}.csproj")), simple_project_file(name, true))
        .map_err(|e| format!("failed to write project file: {e}"))?;
    fs::write(
        project_dir.join("Program.cs"),
        "fn main() {\n    print(\"Hello from Glitching\");\n}\n",
    )
    .map_err(|e| format!("failed to write Program.cs: {e}"))?;
    Ok(())
}

fn write_classlib_template(project_dir: &Path, name: &str) -> Result<(), String> {
    fs::write(project_dir.join(format!("{name}.csproj")), simple_project_file(name, false))
        .map_err(|e| format!("failed to write project file: {e}"))?;
    fs::write(
        project_dir.join("Class1.cs"),
        "public class Class1 {\n}\n",
    )
    .map_err(|e| format!("failed to write Class1.cs: {e}"))?;
    Ok(())
}

fn write_xunit_template(project_dir: &Path, name: &str) -> Result<(), String> {
    fs::write(project_dir.join(format!("{name}.csproj")), xunit_project_file(name))
        .map_err(|e| format!("failed to write project file: {e}"))?;
    fs::write(
        project_dir.join("SampleTests.cs"),
        "using Xunit;\n\npublic class SampleTests {\n    [Fact]\n    public void TruthIsTrue() {\n        Assert.True(true);\n    }\n}\n\npublic static void main() {\n    RunAllTests();\n}\n",
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

fn xunit_project_file(name: &str) -> String {
    format!(
        "<Project Sdk=\"Glitching\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>gl1.0</TargetFramework>\n    <AssemblyName>{name}</AssemblyName>\n    <IsTestProject>true</IsTestProject>\n  </PropertyGroup>\n  <ItemGroup>\n    <PackageReference Include=\"xunit\" Version=\"2.0.0\" />\n  </ItemGroup>\n</Project>\n"
    )
}
