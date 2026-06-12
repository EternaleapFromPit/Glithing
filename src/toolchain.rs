use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn emit_llvm_bitcode(llvm_ir: &str, output_path: &str) -> Result<(), String> {
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

pub(crate) fn emit_native_executable(llvm_ir: &str, output_path: &str) -> Result<(), String> {
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
