use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::linker::strip_utf8_bom;
use crate::project::{
    collect_project_source_members, collect_source_files, expand_solution_inputs,
    find_single_project_in_dir, is_solution_path, parse_project_spec,
};

pub(crate) fn read_input_source(input: &str) -> Result<String, String> {
    let path = Path::new(input);
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
    {
        return read_project_source(path);
    }
    if path.is_dir() {
        if let Some(project_path) = find_single_project_in_dir(path)? {
            return read_project_source(&project_path);
        }
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

pub(crate) fn run_on_large_stack<T, F>(f: F) -> Result<T, String>
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

pub(crate) fn read_project_source(project_path: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    let mut visited_projects = HashSet::<PathBuf>::new();
    collect_project_source_units(project_path, &mut visited_projects, &mut files)?;
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files.dedup_by(|left, right| left.0 == right.0);
    let mut source = String::new();
    for (file, root_namespace) in files {
        let text = fs::read_to_string(&file)
            .map_err(|e| format!("failed to read {}: {e}", file.display()))?;
        source.push_str(&format!("// __FILE_PATH__: {}\n", file.display()));
        let file_text = apply_project_root_namespace(strip_utf8_bom(&text), root_namespace.as_deref());
        source.push_str(&file_text);
        source.push_str("\n__FILE_BOUNDARY__;\n");
    }
    Ok(source)
}

pub(crate) fn collect_project_source_files(
    project_path: &Path,
    visited_projects: &mut HashSet<PathBuf>,
    output: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let project = parse_project_spec(project_path)?;
    if !visited_projects.insert(project.path.clone()) {
        return Ok(());
    }
    output.extend(collect_project_source_members(&project)?);
    for reference in &project.project_references {
        let reference_path = project.project_dir.join(reference);
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

pub(crate) fn collect_project_source_units(
    project_path: &Path,
    visited_projects: &mut HashSet<PathBuf>,
    output: &mut Vec<(PathBuf, Option<String>)>,
) -> Result<(), String> {
    let project = parse_project_spec(project_path)?;
    if !visited_projects.insert(project.path.clone()) {
        return Ok(());
    }
    for file in collect_project_source_members(&project)? {
        output.push((file, project.root_namespace.clone()));
    }
    for reference in &project.project_references {
        let reference_path = project.project_dir.join(reference);
        if reference_path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
        {
            collect_project_source_units(&reference_path, visited_projects, output)?;
        }
    }
    Ok(())
}

pub(crate) fn apply_project_root_namespace(source: &str, root_namespace: Option<&str>) -> String {
    let Some(root_namespace) = root_namespace.map(str::trim).filter(|value| !value.is_empty()) else {
        return source.to_string();
    };
    if source_declares_any_namespace_or_package(source) {
        return source.to_string();
    }
    format!("namespace {root_namespace} {{\n{source}\n}}\n")
}

pub(crate) fn source_declares_any_namespace_or_package(source: &str) -> bool {
    source.lines().any(|line| {
        let line = line.trim_start();
        line.starts_with("namespace ") || line.starts_with("package ")
    })
}

pub(crate) fn infer_watch_input(command: &str, args: &[String], cwd: &Path) -> Result<String, String> {
    match command {
        "build" | "publish" | "restore" | "clean" | "store" => {
            Ok(crate::cli::parse_build_like_args(args, cwd)?.0)
        }
        "test" | "format" => crate::cli::resolve_command_input(args, cwd),
        "run" => {
            let (build_args, _) = crate::cli::split_command_args(args);
            crate::cli::resolve_command_input(&build_args, cwd)
        }
        other => Err(format!("gl watch does not support '{}'", other)),
    }
}

pub(crate) fn collect_watch_paths(input: &str) -> Result<Vec<PathBuf>, String> {
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

pub(crate) fn collect_watch_paths_into(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_dir() {
        collect_source_files(path, output)?;
        for entry in fs::read_dir(path).map_err(|e| format!("failed to read {}: {e}", path.display()))? {
            let entry = entry.map_err(|e| format!("failed to read {}: {e}", path.display()))?;
            let candidate = entry.path();
            if candidate
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
            {
                output.push(candidate);
            }
        }
    } else if is_solution_path(path)
        || path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| matches!(ext, "csproj" | "gl" | "cs"))
    {
        output.push(path.to_path_buf());
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
        {
            let mut files = Vec::new();
            let mut visited = HashSet::new();
            collect_project_source_files(path, &mut visited, &mut files)?;
            output.extend(files);
        }
    }
    Ok(())
}

pub(crate) fn snapshot_watch_paths(paths: &[PathBuf]) -> Result<Vec<(PathBuf, u64, u128)>, String> {
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

pub(crate) fn collect_format_paths(input: &str) -> Result<Vec<PathBuf>, String> {
    let path = Path::new(input);
    let mut files = Vec::new();
    if is_solution_path(path) {
        for project in expand_solution_inputs(input)? {
            files.extend(collect_format_paths(&project)?);
        }
    } else if path.is_dir() {
        collect_source_files(path, &mut files)?;
    } else if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
    {
        let project = parse_project_spec(path)?;
        files.extend(collect_project_source_members(&project)?);
    } else if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext, "gl" | "cs"))
    {
        files.push(path.to_path_buf());
    }
    files.sort();
    files.dedup();
    Ok(files)
}

pub(crate) fn format_source_text(source: &str) -> String {
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

pub(crate) fn count_leading_closers(line: &str) -> usize {
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

pub(crate) fn count_braces(line: &str) -> (usize, usize) {
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
