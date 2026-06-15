use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub(crate) fn emit_llvm_bitcode(llvm_ir: &str, output_path: &str) -> Result<(), String> {
    let ll_path = format!("{output_path}.ll.tmp");
    fs::write(&ll_path, llvm_ir).map_err(|e| format!("failed to write temporary LLVM IR: {e}"))?;
    let result = if let Some(llvm_as) = find_llvm_tool("llvm-as") {
        run_llvm_tool(
            &llvm_as,
            &[
                OsString::from(&ll_path),
                OsString::from("-o"),
                OsString::from(output_path),
            ],
            "llvm-as",
        )
    } else {
        let clang = require_llvm_tool("clang")?;
        run_llvm_tool(
            &clang,
            &[
                OsString::from("-x"),
                OsString::from("ir"),
                OsString::from("-c"),
                OsString::from("-emit-llvm"),
                OsString::from(&ll_path),
                OsString::from("-o"),
                OsString::from(output_path),
            ],
            "clang for LLVM bitcode emission",
        )
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

pub(crate) fn emit_native_executable(
    llvm_ir: &str,
    output_path: &str,
) -> Result<(), String> {
    let ll_path = format!("{output_path}.ll.tmp");
    fs::write(&ll_path, llvm_ir).map_err(|e| format!("failed to write temporary LLVM IR: {e}"))?;
    let clang = require_llvm_tool("clang")?;
    let mut args = vec![
        OsString::from("-O3"),
        OsString::from("-x"),
        OsString::from("ir"),
        OsString::from(&ll_path),
    ];
    args.push(OsString::from("-o"));
    args.push(OsString::from(output_path));
    let links_rust_runtime = llvm_ir.contains("call void @GlitchRestHost_Run(")
        || llvm_ir.contains("System_IO_File_ReadAllText")
        || llvm_ir.contains("System_IO_File_WriteAllText")
        || llvm_ir.contains("System_String_Substring_Native")
        || llvm_ir.contains("System_String_TrimEnd_Native")
        || llvm_ir.contains("System_String_ToLower_Native")
        || llvm_ir.contains("System_String_ToLowerInvariant_Native")
        || llvm_ir.contains("System_String_Replace_Native")
        || llvm_ir.contains("System_String_Trim_Native")
        || llvm_ir.contains("System_String_Split_Native")
        || llvm_ir.contains("System_String_Contains_Native")
        || llvm_ir.contains("System_String_TrimStart_Native")
        || llvm_ir.contains("System_Array_Empty_Native");
    if links_rust_runtime {
        args.push(OsString::from("-x"));
        args.push(OsString::from("none"));
        args.push(find_runtime_library()?.into_os_string());
    }
    if cfg!(windows) {
        args.push(OsString::from("-lws2_32"));
        if links_rust_runtime {
            for library in ["kernel32", "ntdll", "userenv", "dbghelp"] {
                args.push(OsString::from(format!("-l{library}")));
            }
            args.push(OsString::from("-Xlinker"));
            args.push(OsString::from("/defaultlib:msvcrt"));
        }
        let mut command = Command::new(&clang);
        command.args(&args);
        configure_windows_linker_environment(&mut command)?;
        configure_windows_compiler_environment(&mut command)?;
        let result = run_command_with_fallback(command, &clang, &args, "clang");
        let _ = fs::remove_file(&ll_path);
        let output = result?;
        if output.status.success() {
            return Ok(());
        }
        return Err(format!(
            "clang failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let _ = fs::remove_file(&ll_path);
    let output = run_llvm_tool(&clang, &args, "clang")?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "clang failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn run_llvm_tool(
    tool: &Path,
    args: &[OsString],
    context: &str,
) -> Result<Output, String> {
    let mut command = Command::new(tool);
    command.args(args);
    match command.output() {
        Ok(output) => Ok(output),
        Err(err) if cfg!(windows) && err.raw_os_error() == Some(5) => {
            run_windows_shell_fallback(tool, args, context)
        }
        Err(err) => Err(format!("failed to run {context}: {err}")),
    }
}

fn run_command_with_fallback(
    mut command: Command,
    tool: &Path,
    args: &[OsString],
    context: &str,
) -> Result<Output, String> {
    match command.output() {
        Ok(output) => Ok(output),
        Err(err) if cfg!(windows) && err.raw_os_error() == Some(5) => {
            run_windows_shell_fallback(tool, args, context)
        }
        Err(err) => Err(format!("failed to run {context}: {err}")),
    }
}

fn run_windows_shell_fallback(
    tool: &Path,
    args: &[OsString],
    context: &str,
) -> Result<Output, String> {
    let command_line = build_powershell_command(tool, args);
    Command::new("powershell.exe")
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(command_line)
        .output()
        .map_err(|err| format!("failed to run {context} via PowerShell fallback: {err}"))
}

fn build_powershell_command(tool: &Path, args: &[OsString]) -> String {
    let mut script = String::from("& ");
    script.push_str(&quote_powershell_single(tool));
    for arg in args {
        script.push(' ');
        script.push_str(&quote_powershell_single_os(arg));
    }
    script
}

fn quote_powershell_single(value: &Path) -> String {
    quote_powershell_single_os(value.as_os_str())
}

fn quote_powershell_single_os(value: &std::ffi::OsStr) -> String {
    let text = value.to_string_lossy().replace('\'', "''");
    format!("'{}'", text)
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

fn configure_windows_compiler_environment(command: &mut Command) -> Result<(), String> {
    let mut include_paths = Vec::<PathBuf>::new();
    if let Some(msvc_lib) = find_msvc_library_path() {
        if let Some(msvc_tool_root) = msvc_lib.parent().and_then(|p| p.parent()) {
            let msvc_include = msvc_tool_root.join("include");
            if msvc_include.is_dir() {
                include_paths.push(msvc_include);
            }
        }
    }
    if let Some(sdk_include_root) =
        find_latest_subdirectory(Path::new(r"C:\Program Files (x86)\Windows Kits\10\Include"))
    {
        for component in ["ucrt", "shared", "um"] {
            let path = sdk_include_root.join(component);
            if path.is_dir() {
                include_paths.push(path);
            }
        }
    }
    if let Some(existing) = env::var_os("INCLUDE") {
        include_paths.extend(env::split_paths(&existing));
    }
    if !include_paths.is_empty() {
        let joined = env::join_paths(include_paths)
            .map_err(|e| format!("failed to construct Windows compiler INCLUDE path: {e}"))?;
        command.env("INCLUDE", joined);
    }
    Ok(())
}
