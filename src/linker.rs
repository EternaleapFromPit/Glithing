use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn link_package_sources(source: &str) -> Result<String, String> {
    let mut visited = Vec::new();
    let mut linked = String::new();
    link_package_sources_inner(source, &mut visited, &mut linked)?;
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
        if visited.contains(&package_id) {
            continue;
        }
        let Some(path) = package_source_path(&package_id) else {
            continue;
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
        linked.push_str(&package_source);
        linked.push_str("\n__FILE_BOUNDARY__;\n");
    }
    Ok(())
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

pub(crate) fn find_package_native_sources(linked_source: &str) -> Vec<PathBuf> {
    let mut native_files = Vec::new();
    let packages = source_using_packages(linked_source);
    for package_id in packages {
        if package_id == "System.XUnit" {
            continue;
        }
        if let Some(source_path) = package_source_path(&package_id) {
            if let Some(parent) = source_path.parent() {
                let native_dir = parent.join("native");
                if native_dir.is_dir() {
                    if let Ok(entries) = fs::read_dir(native_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file()
                                && path.extension().and_then(|e| e.to_str()) == Some("c")
                            {
                                native_files.push(path);
                            }
                        }
                    }
                }
            }
        }
    }
    native_files
}
