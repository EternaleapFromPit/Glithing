use super::*;
use std::fs;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn emit_native_executable_from_source(stem: &str, source: &str) -> std::path::PathBuf {
    let compiled = compile_source_with_options(source, true, false)
        .expect("source should compile to LLVM IR");
    emit_native_executable_from_ir(stem, compiled.llvm_ir().expect("LLVM IR should be available"))
}

fn emit_native_executable_from_path(stem: &str, input: &str) -> std::path::PathBuf {
    let source = read_input_source(input).expect("input source should be readable");
    let compiled = run_on_large_stack(move || compile_source_with_options(&source, true, false))
        .expect("input path should compile to LLVM IR");
    emit_native_executable_from_ir(stem, compiled.llvm_ir().expect("LLVM IR should be available"))
}

fn emit_native_executable_from_ir(stem: &str, llvm_ir: &str) -> std::path::PathBuf {
    let output_exe = temp_smoke_executable(stem);
    let _ = fs::remove_file(&output_exe);
    crate::toolchain::emit_native_executable(
        llvm_ir,
        output_exe
            .to_str()
            .expect("temporary smoke path should be valid UTF-8"),
    )
    .expect("native executable should be emitted");
    output_exe
}

fn run_native_executable(path: &std::path::Path) -> std::process::Output {
    Command::new(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("native executable should run")
}

fn run_native_executable_with_leak_report(path: &std::path::Path) -> std::process::Output {
    Command::new(path)
        .env("GLITCH_REPORT_LEAKS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("native executable should run with leak reporting enabled")
}

fn temp_smoke_executable(stem: &str) -> std::path::PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("glitching-{stem}-{stamp}.exe"))
}

mod async_and_basics;
mod calls_and_types;
mod system_collections;
mod llvm_and_cycles;
mod packages_and_generics;
mod framework_and_xunit;
mod borrow_checker;

