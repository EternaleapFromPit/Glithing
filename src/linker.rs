use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn link_package_sources(source: &str) -> Result<String, String> {
    let mut visited = Vec::new();
    let mut linked = String::new();
    link_package_sources_inner(source, &mut visited, &mut linked)?;
    linked.push_str(&source_file_marker("linked input"));
    linked.push_str(source);
    linked.push_str("\n__FILE_BOUNDARY__;\n");
    Ok(linked)
}

pub(crate) fn strip_utf8_bom(source: &str) -> &str {
    source.strip_prefix('\u{feff}').unwrap_or(source)
}

fn link_package_sources_inner(
    source: &str,
    visited: &mut Vec<String>,
    linked: &mut String,
) -> Result<(), String> {
    for package_id in source_using_packages(source) {
        if source_declares_namespace(source, &package_id) {
            continue;
        }
        if visited.contains(&package_id) {
            continue;
        }
        let Some(path) = package_source_path(&package_id) else {
            return Err(format!(
                "package import '{package_id}' could not be resolved to packages/{package_id}/{package_id}.gl or packages/{}/{}.gl",
                package_id.replace('.', "/"),
                package_id
            ));
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
        linked.push_str(&source_file_marker(&path.display().to_string()));
        linked.push_str(&package_source);
        linked.push_str("\n__FILE_BOUNDARY__;\n");
    }
    Ok(())
}

fn source_declares_namespace(source: &str, package_id: &str) -> bool {
    let namespace = format!("namespace {package_id}");
    let package = format!("package {package_id}");
    source.contains(&namespace) || source.contains(&package)
}

fn source_file_marker(path: &str) -> String {
    format!("// __FILE_PATH__: {path}\n")
}

pub(crate) fn source_using_packages(source: &str) -> Vec<String> {
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

pub(crate) fn package_source_path(package_id: &str) -> Option<PathBuf> {
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
