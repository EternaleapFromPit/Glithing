use super::*;
use std::fs;
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
