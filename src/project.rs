use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub(crate) struct PackageReferenceSpec {
    pub(crate) id: String,
    pub(crate) version: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ProjectSpec {
    pub(crate) path: PathBuf,
    pub(crate) project_dir: PathBuf,
    pub(crate) assembly_name: Option<String>,
    pub(crate) output_type: Option<String>,
    #[allow(dead_code)]
    pub(crate) target_framework: Option<String>,
    pub(crate) root_namespace: Option<String>,
    pub(crate) startup_object: Option<String>,
    pub(crate) is_test_project: bool,
    pub(crate) project_references: Vec<PathBuf>,
    pub(crate) package_references: Vec<PackageReferenceSpec>,
    pub(crate) compile_includes: Vec<String>,
    pub(crate) compile_removes: Vec<String>,
    pub(crate) content_includes: Vec<String>,
    pub(crate) content_removes: Vec<String>,
    pub(crate) none_includes: Vec<String>,
    pub(crate) none_removes: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BuildArtifactKind {
    NativeExe,
    LlvmBitcode,
}

impl ProjectSpec {
    pub(crate) fn assembly_name_or_default(&self) -> String {
        self.assembly_name
            .clone()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| {
                self.path
                    .file_stem()
                    .and_then(|value| value.to_str())
                    .filter(|value| !value.is_empty())
                    .unwrap_or("app")
                    .to_string()
            })
    }

    pub(crate) fn output_type_or_default(&self) -> &str {
        self.output_type
            .as_deref()
            .filter(|value| !value.is_empty())
            .unwrap_or("Exe")
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    pub(crate) fn build_artifact_kind(&self) -> BuildArtifactKind {
        if self.is_test_project {
            return BuildArtifactKind::NativeExe;
        }
        match self.output_type_or_default().to_ascii_lowercase().as_str() {
            "library" => BuildArtifactKind::LlvmBitcode,
            _ => BuildArtifactKind::NativeExe,
        }
    }

    pub(crate) fn is_runnable(&self) -> bool {
        matches!(
            self.output_type_or_default().to_ascii_lowercase().as_str(),
            "exe" | "winexe"
        )
    }

    pub(crate) fn should_run_tests(&self) -> bool {
        self.is_test_project
            || self
                .package_references
                .iter()
                .any(|package| package.id.eq_ignore_ascii_case("xunit"))
    }
}

pub(crate) fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create {}: {e}", parent.display()))?;
        }
    }
    Ok(())
}

pub(crate) fn find_single_project_in_dir(path: &Path) -> Result<Option<PathBuf>, String> {
    let mut projects = Vec::new();
    for entry in
        fs::read_dir(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?
    {
        let entry = entry.map_err(|e| format!("failed to read {} entry: {e}", path.display()))?;
        let candidate = entry.path();
        if candidate
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
        {
            projects.push(candidate);
        }
    }
    match projects.len() {
        0 => Ok(None),
        1 => Ok(projects.into_iter().next()),
        _ => Err(format!(
            "multiple .csproj files found in {}; specify the project explicitly",
            path.display()
        )),
    }
}

pub(crate) fn load_project_spec_for_input(input: &str) -> Result<Option<ProjectSpec>, String> {
    let path = Path::new(input);
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
    {
        return Ok(Some(parse_project_spec(path)?));
    }
    if path.is_dir() {
        if let Some(project_path) = find_single_project_in_dir(path)? {
            return Ok(Some(parse_project_spec(&project_path)?));
        }
    }
    Ok(None)
}

pub(crate) fn parse_project_spec(project_path: &Path) -> Result<ProjectSpec, String> {
    let canonical = fs::canonicalize(project_path)
        .map_err(|e| format!("failed to resolve project {}: {e}", project_path.display()))?;
    let project_text = fs::read_to_string(&canonical)
        .map_err(|e| format!("failed to read project {}: {e}", canonical.display()))?;
    let project_dir = canonical
        .parent()
        .ok_or_else(|| format!("project {} does not have a parent directory", canonical.display()))?
        .to_path_buf();
    let package_references = extract_package_references(&project_text);
    let is_test_project = extract_xml_tag_value(&project_text, "IsTestProject")
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
        || package_references
            .iter()
            .any(|package| package.id.eq_ignore_ascii_case("xunit"));
    let spec = ProjectSpec {
        path: canonical,
        project_dir,
        assembly_name: extract_xml_tag_value(&project_text, "AssemblyName"),
        output_type: extract_xml_tag_value(&project_text, "OutputType"),
        target_framework: extract_xml_tag_value(&project_text, "TargetFramework"),
        root_namespace: extract_xml_tag_value(&project_text, "RootNamespace"),
        startup_object: extract_xml_tag_value(&project_text, "StartupObject"),
        is_test_project,
        project_references: extract_project_references(&project_text),
        package_references,
        compile_includes: extract_item_attr_values(&project_text, "Compile", "Include"),
        compile_removes: extract_item_attr_values(&project_text, "Compile", "Remove"),
        content_includes: extract_item_attr_values(&project_text, "Content", "Include"),
        content_removes: extract_item_attr_values(&project_text, "Content", "Remove"),
        none_includes: extract_item_attr_values(&project_text, "None", "Include"),
        none_removes: extract_item_attr_values(&project_text, "None", "Remove"),
    };
    spec.validate()?;
    Ok(spec)
}

pub(crate) fn collect_project_source_members(project: &ProjectSpec) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_source_files(&project.project_dir, &mut files)?;
    files.sort();
    files.dedup();

    let mut selected = files;
    for include in &project.compile_includes {
        selected.extend(resolve_project_patterns(&project.project_dir, include, true)?);
    }
    selected.sort();
    selected.dedup();
    selected.retain(|path| {
        let relative = make_relative_project_path(&project.project_dir, path);
        !project
            .compile_removes
            .iter()
            .any(|pattern| project_pattern_matches(pattern, &relative))
    });
    Ok(selected)
}

pub(crate) fn collect_publishable_project_files(
    project: Option<&ProjectSpec>,
) -> Result<Vec<(String, Vec<u8>)>, String> {
    let Some(project) = project else {
        return Ok(Vec::new());
    };
    let mut files = Vec::new();
    for pattern in project
        .content_includes
        .iter()
        .chain(project.none_includes.iter())
    {
        for path in resolve_project_patterns(&project.project_dir, pattern, false)? {
            let relative = make_relative_project_path(&project.project_dir, &path);
            if project
                .content_removes
                .iter()
                .chain(project.none_removes.iter())
                .any(|remove| project_pattern_matches(remove, &relative))
            {
                continue;
            }
            let bytes = fs::read(&path)
                .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
            files.push((relative.replace('\\', "/"), bytes));
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files.dedup_by(|left, right| left.0 == right.0);
    Ok(files)
}

pub(crate) fn resolve_project_patterns(
    project_dir: &Path,
    pattern: &str,
    source_only: bool,
) -> Result<Vec<PathBuf>, String> {
    let normalized_pattern = normalize_project_pattern(pattern);
    if normalized_pattern.contains('*') || normalized_pattern.contains('?') {
        let mut all_files = Vec::new();
        collect_files(project_dir, &mut all_files)?;
        let mut matches = Vec::new();
        for file in all_files {
            if source_only
                && !file
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| matches!(ext, "gl" | "cs"))
            {
                continue;
            }
            let relative = make_relative_project_path(project_dir, &file);
            if project_pattern_matches(&normalized_pattern, &relative) {
                matches.push(file);
            }
        }
        return Ok(matches);
    }
    let path = project_dir.join(pattern);
    if !path.exists() {
        return Ok(Vec::new());
    }
    if path.is_dir() {
        let mut matches = Vec::new();
        if source_only {
            collect_source_files(&path, &mut matches)?;
        } else {
            collect_files(&path, &mut matches)?;
        }
        return Ok(matches);
    }
    if source_only
        && !path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| matches!(ext, "gl" | "cs"))
    {
        return Ok(Vec::new());
    }
    Ok(vec![path])
}

pub(crate) fn collect_source_files(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
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

pub(crate) fn collect_files(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|e| format!("failed to read {}: {e}", path.display()))? {
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
            collect_files(&path, output)?;
        } else {
            output.push(path);
        }
    }
    Ok(())
}

pub(crate) fn normalize_project_pattern(pattern: &str) -> String {
    pattern.replace('\\', "/")
}

pub(crate) fn make_relative_project_path(project_dir: &Path, path: &Path) -> String {
    path.strip_prefix(project_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub(crate) fn project_pattern_matches(pattern: &str, path: &str) -> bool {
    let normalized_pattern = normalize_project_pattern(pattern);
    let normalized_path = path.replace('\\', "/");
    let pattern_segments = normalized_pattern
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let path_segments = normalized_path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    match_path_segments(&pattern_segments, &path_segments)
}

pub(crate) fn match_path_segments(pattern: &[&str], path: &[&str]) -> bool {
    if pattern.is_empty() {
        return path.is_empty();
    }
    if pattern[0] == "**" {
        for skip in 0..=path.len() {
            if match_path_segments(&pattern[1..], &path[skip..]) {
                return true;
            }
        }
        return false;
    }
    if path.is_empty() {
        return false;
    }
    if !match_segment(pattern[0], path[0]) {
        return false;
    }
    match_path_segments(&pattern[1..], &path[1..])
}

pub(crate) fn match_segment(pattern: &str, text: &str) -> bool {
    let pattern = pattern.as_bytes();
    let text = text.as_bytes();
    let (mut p, mut t) = (0usize, 0usize);
    let (mut star, mut star_match) = (None, 0usize);
    while t < text.len() {
        if p < pattern.len() && (pattern[p] == b'?' || pattern[p] == text[t]) {
            p += 1;
            t += 1;
        } else if p < pattern.len() && pattern[p] == b'*' {
            star = Some(p);
            p += 1;
            star_match = t;
        } else if let Some(star_index) = star {
            p = star_index + 1;
            star_match += 1;
            t = star_match;
        } else {
            return false;
        }
    }
    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}

pub(crate) fn extract_xml_tag_value(text: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)?;
    let rest = &text[start + open.len()..];
    let end = rest.find(&close)?;
    Some(rest[..end].trim().to_string())
}

pub(crate) fn extract_item_attr_values(text: &str, tag: &str, attr: &str) -> Vec<String> {
    extract_xml_start_tags(text, tag)
        .into_iter()
        .filter_map(|segment| extract_xml_attr(&segment, attr))
        .collect()
}

pub(crate) fn extract_package_references(text: &str) -> Vec<PackageReferenceSpec> {
    extract_xml_start_tags(text, "PackageReference")
        .into_iter()
        .filter_map(|segment| {
            let id = extract_xml_attr(&segment, "Include")?;
            let version = extract_xml_attr(&segment, "Version");
            Some(PackageReferenceSpec { id, version })
        })
        .collect()
}

pub(crate) fn extract_xml_start_tags(text: &str, tag: &str) -> Vec<String> {
    let needle = format!("<{tag}");
    let mut values = Vec::new();
    let mut cursor = 0usize;
    while let Some(offset) = text[cursor..].find(&needle) {
        let start = cursor + offset;
        let Some(end) = text[start..].find('>') else {
            break;
        };
        values.push(text[start..start + end + 1].to_string());
        cursor = start + end + 1;
    }
    values
}

pub(crate) fn extract_xml_attr(segment: &str, attr: &str) -> Option<String> {
    for prefix in [format!(r#"{attr}=""#), format!(r#"{attr}='"#)] {
        if let Some(index) = segment.find(&prefix) {
            let rest = &segment[index + prefix.len()..];
            let end = rest.find(['"', '\'']).unwrap_or(rest.len());
            return Some(rest[..end].trim().to_string());
        }
    }
    None
}

pub(crate) fn default_solution_name(cwd: &Path) -> String {
    cwd.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("GlitchingSolution")
        .to_string()
}

pub(crate) fn is_solution_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("sln"))
}

pub(crate) fn parse_solution_command_args(
    args: &[String],
    cwd: &Path,
) -> Result<(PathBuf, Vec<String>), String> {
    if let Some(first) = args.first() {
        if first.ends_with(".sln") {
            return Ok((cwd.join(first), args[1..].to_vec()));
        }
    }
    Ok((resolve_solution_path(None, cwd)?, args.to_vec()))
}

pub(crate) fn resolve_solution_path(
    explicit: Option<String>,
    cwd: &Path,
) -> Result<PathBuf, String> {
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

pub(crate) fn normalize_solution_project_path(
    solution_dir: &Path,
    project_path: &Path,
) -> Result<PathBuf, String> {
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

pub(crate) fn write_solution_file(solution_path: &Path, projects: &[PathBuf]) -> Result<(), String> {
    ensure_parent_dir(solution_path)?;
    let solution_dir = solution_path.parent().unwrap_or_else(|| Path::new("."));
    let mut text =
        String::from("Microsoft Visual Studio Solution File, Format Version 12.00\n# Glitching Solution\n");
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

pub(crate) fn read_solution_projects(solution_path: &Path) -> Result<Vec<PathBuf>, String> {
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

pub(crate) fn expand_solution_inputs(input: &str) -> Result<Vec<String>, String> {
    let input_path = Path::new(input);
    if !is_solution_path(input_path) {
        return Ok(vec![input.to_string()]);
    }
    let solution_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
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
        return Err(format!(
            "solution {} does not contain any projects",
            input_path.display()
        ));
    }
    Ok(projects)
}

pub(crate) fn extract_project_references(project_text: &str) -> Vec<PathBuf> {
    extract_item_attr_values(project_text, "ProjectReference", "Include")
        .into_iter()
        .map(PathBuf::from)
        .collect()
}
