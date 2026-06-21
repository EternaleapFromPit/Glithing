use super::*;
use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_cli_dir(stem: &str) -> std::path::PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("gl-cli-{stem}-{stamp}"));
    fs::create_dir_all(&dir).expect("temp CLI dir should be created");
    dir
}

#[test]
fn gl_new_console_creates_project_files() {
    let cwd = temp_cli_dir("new-console");
    run_cli_with_args_from(
        vec!["new".to_string(), "console".to_string(), "DemoApp".to_string()],
        &cwd,
    )
    .expect("gl new console should succeed");

    assert!(cwd.join("DemoApp").join("DemoApp.csproj").exists());
    assert!(cwd.join("DemoApp").join("Program.cs").exists());
}

#[test]
fn gl_build_emits_debug_executable() {
    let cwd = temp_cli_dir("build");
    run_cli_with_args_from(
        vec!["new".to_string(), "console".to_string(), "DemoApp".to_string()],
        &cwd,
    )
    .expect("gl new console should succeed");

    let project_dir = cwd.join("DemoApp");
    run_cli_with_args_from(vec!["build".to_string()], &project_dir)
        .expect("gl build should emit a native executable");

    assert!(project_dir.join("bin").join("Debug").join("DemoApp.exe").exists());
}

#[test]
fn gl_clean_removes_bin_and_obj() {
    let cwd = temp_cli_dir("clean");
    let project_dir = cwd.join("DemoApp");
    fs::create_dir_all(project_dir.join("bin").join("Debug"))
        .expect("bin dir should be created");
    fs::create_dir_all(project_dir.join("obj")).expect("obj dir should be created");
    fs::write(project_dir.join("DemoApp.csproj"), "<Project />\n")
        .expect("project file should be created");

    run_cli_with_args_from(vec!["clean".to_string()], &project_dir)
        .expect("gl clean should remove generated directories");

    assert!(!project_dir.join("bin").exists());
    assert!(!project_dir.join("obj").exists());
}

#[test]
fn gl_sln_new_add_list_and_remove_manage_solution_projects() {
    let cwd = temp_cli_dir("sln");
    run_cli_with_args_from(
        vec!["new".to_string(), "console".to_string(), "App".to_string()],
        &cwd,
    )
    .expect("gl new console should succeed");

    run_cli_with_args_from(
        vec!["sln".to_string(), "new".to_string(), "Workspace".to_string()],
        &cwd,
    )
    .expect("gl sln new should succeed");
    run_cli_with_args_from(
        vec![
            "sln".to_string(),
            "add".to_string(),
            "App\\App.csproj".to_string(),
        ],
        &cwd,
    )
    .expect("gl sln add should succeed");

    let solution = fs::read_to_string(cwd.join("Workspace.sln"))
        .expect("solution file should be readable");
    assert!(solution.contains("App\\App.csproj") || solution.contains("App/App.csproj"));

    run_cli_with_args_from(
        vec![
            "sln".to_string(),
            "remove".to_string(),
            "App\\App.csproj".to_string(),
        ],
        &cwd,
    )
    .expect("gl sln remove should succeed");

    let solution = fs::read_to_string(cwd.join("Workspace.sln"))
        .expect("solution file should be readable after removal");
    assert!(!solution.contains("App\\App.csproj") && !solution.contains("App/App.csproj"));
}

#[test]
fn gl_store_emits_native_package_artifact() {
    let cwd = temp_cli_dir("store");
    run_cli_with_args_from(
        vec!["new".to_string(), "console".to_string(), "DemoApp".to_string()],
        &cwd,
    )
    .expect("gl new console should succeed");

    let project_dir = cwd.join("DemoApp");
    run_cli_with_args_from(
        vec![
            "store".to_string(),
            "DemoApp.csproj".to_string(),
            "--package-id".to_string(),
            "DemoApp".to_string(),
            "--package-version".to_string(),
            "1.2.3".to_string(),
        ],
        &project_dir,
    )
    .expect("gl store should emit a native package");

    assert!(project_dir
        .join(".gl")
        .join("store")
        .join("DemoApp")
        .join("1.2.3")
        .join("DemoApp.1.2.3.nupkg")
        .exists());
}

#[test]
fn gl_format_rewrites_source_indentation() {
    let cwd = temp_cli_dir("format");
    let source = cwd.join("sample.gl");
    fs::write(
        &source,
        "fn main(){\nprint(1);\nif (true) {\nprint(2);\n}\n}\n",
    )
    .expect("source file should be created");

    run_cli_with_args_from(vec!["format".to_string(), "sample.gl".to_string()], &cwd)
        .expect("gl format should succeed");

    let formatted = fs::read_to_string(source).expect("formatted source should be readable");
    assert!(formatted.contains("fn main(){\n    print(1);\n"));
    assert!(formatted.contains("    if (true) {\n        print(2);\n    }\n"));
}

#[test]
fn gl_watch_once_build_delegates_to_inner_command() {
    let cwd = temp_cli_dir("watch-once");
    run_cli_with_args_from(
        vec!["new".to_string(), "console".to_string(), "DemoApp".to_string()],
        &cwd,
    )
    .expect("gl new console should succeed");

    let project_dir = cwd.join("DemoApp");
    run_cli_with_args_from(
        vec![
            "watch".to_string(),
            "--once".to_string(),
            "build".to_string(),
        ],
        &project_dir,
    )
    .expect("gl watch --once build should succeed");

    assert!(project_dir.join("bin").join("Debug").join("DemoApp.exe").exists());
}

#[test]
fn gl_watch_rebuilds_after_source_change() {
    let cwd = temp_cli_dir("watch-change");
    run_cli_with_args_from(
        vec!["new".to_string(), "console".to_string(), "DemoApp".to_string()],
        &cwd,
    )
    .expect("gl new console should succeed");
    let project_dir = cwd.join("DemoApp");
    let program_path = project_dir.join("Program.cs");

    let project_dir_for_thread = project_dir.clone();
    let program_path_for_thread = program_path.clone();
    let handle = thread::spawn(move || {
        run_cli_with_args_from(
            vec![
                "watch".to_string(),
                "--poll-ms".to_string(),
                "25".to_string(),
                "--max-iterations".to_string(),
                "2".to_string(),
                "build".to_string(),
            ],
            &project_dir_for_thread,
        )
    });

    thread::sleep(Duration::from_millis(120));
    let original = fs::read_to_string(&program_path_for_thread)
        .expect("program source should be readable");
    fs::write(&program_path_for_thread, format!("{original}\n"))
        .expect("program source should be updated to trigger watch");

    handle
        .join()
        .expect("watch thread should join")
        .expect("watch build should succeed twice");

    assert!(project_dir.join("bin").join("Debug").join("DemoApp.exe").exists());
}

#[test]
fn gl_build_uses_csproj_assembly_name_for_output() {
    let cwd = temp_cli_dir("assembly-name");
    let project_dir = cwd.join("DemoApp");
    fs::create_dir_all(&project_dir).expect("project dir should be created");
    fs::write(
        project_dir.join("DemoApp.csproj"),
        "<Project Sdk=\"Glitching\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>gl1.0</TargetFramework>\n    <AssemblyName>CustomHost</AssemblyName>\n  </PropertyGroup>\n</Project>\n",
    )
    .expect("project file should be written");
    fs::write(
        project_dir.join("Program.cs"),
        "fn main() {\n    print(\"ok\");\n}\n",
    )
    .expect("program file should be written");

    run_cli_with_args_from(vec!["build".to_string()], &project_dir)
        .expect("gl build should use AssemblyName for output");

    assert!(project_dir.join("bin").join("Debug").join("CustomHost.exe").exists());
}

#[test]
fn gl_build_accepts_standard_dotnet_targetframework_metadata() {
    let cwd = temp_cli_dir("net-targetframework");
    let project_dir = cwd.join("DemoApp");
    fs::create_dir_all(&project_dir).expect("project dir should be created");
    fs::write(
        project_dir.join("DemoApp.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>net7.0</TargetFramework>\n    <AssemblyName>CustomHost</AssemblyName>\n  </PropertyGroup>\n</Project>\n",
    )
    .expect("project file should be written");
    fs::write(
        project_dir.join("Program.cs"),
        "fn main() {\n    print(\"ok\");\n}\n",
    )
    .expect("program file should be written");

    run_cli_with_args_from(vec!["build".to_string()], &project_dir)
        .expect("gl build should tolerate standard .NET TargetFramework metadata");

    assert!(project_dir.join("bin").join("Debug").join("CustomHost.exe").exists());
}

#[test]
fn gl_build_library_project_emits_bitcode_and_run_rejects_it() {
    let cwd = temp_cli_dir("library-build");
    run_cli_with_args_from(
        vec!["new".to_string(), "classlib".to_string(), "DemoLib".to_string()],
        &cwd,
    )
    .expect("gl new classlib should succeed");

    let project_dir = cwd.join("DemoLib");
    run_cli_with_args_from(vec!["build".to_string()], &project_dir)
        .expect("gl build should emit bitcode for library projects");

    assert!(project_dir.join("bin").join("Debug").join("DemoLib.bc").exists());
    let error = run_cli_with_args_from(vec!["run".to_string()], &project_dir)
        .expect_err("gl run should reject library projects");
    assert!(error.contains("cannot run project"));
}

#[test]
fn gl_build_honors_compile_remove_items() {
    let cwd = temp_cli_dir("compile-remove");
    let project_dir = cwd.join("DemoApp");
    fs::create_dir_all(&project_dir).expect("project dir should be created");
    fs::write(
        project_dir.join("DemoApp.csproj"),
        "<Project Sdk=\"Glitching\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>gl1.0</TargetFramework>\n  </PropertyGroup>\n  <ItemGroup>\n    <Compile Remove=\"Broken.cs\" />\n  </ItemGroup>\n</Project>\n",
    )
    .expect("project file should be written");
    fs::write(
        project_dir.join("Program.cs"),
        "fn main() {\n    print(\"ok\");\n}\n",
    )
    .expect("program file should be written");
    fs::write(project_dir.join("Broken.cs"), "this is not valid syntax\n")
        .expect("broken file should be written");

    run_cli_with_args_from(vec!["build".to_string()], &project_dir)
        .expect("gl build should ignore removed compile items");

    assert!(project_dir.join("bin").join("Debug").join("DemoApp.exe").exists());
}

#[test]
fn gl_build_honors_root_namespace_and_startup_object() {
    let cwd = temp_cli_dir("startup-object");
    let project_dir = cwd.join("DemoApp");
    fs::create_dir_all(&project_dir).expect("project dir should be created");
    fs::write(
        project_dir.join("DemoApp.csproj"),
        "<Project Sdk=\"Glitching\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>gl1.0</TargetFramework>\n    <AssemblyName>DemoApp</AssemblyName>\n    <RootNamespace>Demo.App</RootNamespace>\n    <StartupObject>Demo.App.Program</StartupObject>\n  </PropertyGroup>\n</Project>\n",
    )
    .expect("project file should be written");
    fs::write(
        project_dir.join("Program.cs"),
        "public class Program {\n    public static int Main(string[] args) {\n        print(args[0]);\n        return 7;\n    }\n}\n",
    )
    .expect("program file should be written");

    run_cli_with_args_from(vec!["build".to_string()], &project_dir)
        .expect("gl build should honor RootNamespace and StartupObject");

    let output_exe = project_dir.join("bin").join("Debug").join("DemoApp.exe");
    assert!(output_exe.exists());

    let output = Command::new(&output_exe)
        .arg("hello")
        .output()
        .expect("startup-object executable should run");
    assert_eq!(output.status.code(), Some(7));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\r\n");
}

#[test]
fn gl_publish_and_store_include_project_content_files() {
    let cwd = temp_cli_dir("content-files");
    let project_dir = cwd.join("DemoApp");
    fs::create_dir_all(project_dir.join("assets")).expect("assets dir should be created");
    fs::write(
        project_dir.join("DemoApp.csproj"),
        "<Project Sdk=\"Glitching\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>gl1.0</TargetFramework>\n    <AssemblyName>DemoApp</AssemblyName>\n  </PropertyGroup>\n  <ItemGroup>\n    <Content Include=\"assets\\seed.json\" />\n    <PackageReference Include=\"xunit\" Version=\"2.0.0\" />\n  </ItemGroup>\n</Project>\n",
    )
    .expect("project file should be written");
    fs::write(
        project_dir.join("Program.cs"),
        "fn main() {\n    print(\"ok\");\n}\n",
    )
    .expect("program file should be written");
    fs::write(project_dir.join("assets").join("seed.json"), "{\"ok\":true}\n")
        .expect("content file should be written");

    run_cli_with_args_from(vec!["publish".to_string()], &project_dir)
        .expect("gl publish should copy content files");
    assert!(project_dir
        .join("bin")
        .join("Release")
        .join("publish")
        .join("assets")
        .join("seed.json")
        .exists());

    run_cli_with_args_from(vec!["store".to_string()], &project_dir)
        .expect("gl store should include content files and dependencies");
    let package_path = project_dir
        .join(".gl")
        .join("store")
        .join("DemoApp")
        .join("0.1.0")
        .join("DemoApp.0.1.0.nupkg");
    let package_text =
        String::from_utf8_lossy(&fs::read(package_path).expect("package should be readable"))
            .to_string();
    assert!(package_text.contains("contentFiles/any/any/assets/seed.json"));
    assert!(package_text.contains("dependency id=\"xunit\" version=\"2.0.0\""));
}

#[test]
fn gl_test_honors_is_test_project_metadata() {
    let cwd = temp_cli_dir("test-project");
    run_cli_with_args_from(
        vec!["new".to_string(), "xunit".to_string(), "DemoTests".to_string()],
        &cwd,
    )
    .expect("gl new xunit should succeed");

    let project_dir = cwd.join("DemoTests");
    run_cli_with_args_from(vec!["test".to_string()], &project_dir)
        .expect("gl test should run a project marked as IsTestProject");
}

#[test]
fn gl_test_rejects_non_test_project_metadata() {
    let cwd = temp_cli_dir("non-test-project");
    run_cli_with_args_from(
        vec!["new".to_string(), "console".to_string(), "DemoApp".to_string()],
        &cwd,
    )
    .expect("gl new console should succeed");

    let project_dir = cwd.join("DemoApp");
    let error = run_cli_with_args_from(vec!["test".to_string()], &project_dir)
        .expect_err("gl test should reject non-test projects");
    assert!(error.contains("is not marked as a test project"));
}
