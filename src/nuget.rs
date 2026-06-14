use std::fs;
use std::path::Path;

pub(crate) struct NugetPackageSpec<'a> {
    pub(crate) package_id: &'a str,
    pub(crate) version: &'a str,
    pub(crate) linked_source: &'a str,
    pub(crate) llvm_ir: &'a str,
}

pub(crate) fn emit_nuget_package(
    spec: NugetPackageSpec<'_>,
    output_path: &str,
) -> Result<(), String> {
    let output = Path::new(output_path);
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create {}: {e}", parent.display()))?;
        }
    }

    let mut entries = Vec::new();
    entries.push((
        format!("{}.nuspec", spec.package_id),
        render_nuspec(&spec).into_bytes(),
    ));
    entries.push((
        String::from("[Content_Types].xml"),
        render_content_types().into_bytes(),
    ));
    entries.push((String::from("_rels/.rels"), render_rels(&spec).into_bytes()));
    entries.push((
        format!("build/native/{}.ll", spec.package_id),
        spec.llvm_ir.as_bytes().to_vec(),
    ));
    entries.push((
        format!("contentFiles/any/any/{}.gl", spec.package_id),
        spec.linked_source.as_bytes().to_vec(),
    ));

    let mut archive = Vec::new();
    write_zip(&mut archive, &entries)?;
    fs::write(output, archive)
        .map_err(|e| format!("failed to write {}: {e}", output.display()))
}

fn render_nuspec(spec: &NugetPackageSpec<'_>) -> String {
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2013/05/nuspec.xsd">
  <metadata>
    <id>{}</id>
    <version>{}</version>
    <authors>Glitching</authors>
    <owners>Glitching</owners>
    <description>Native LLVM package emitted by Glitching.</description>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
  </metadata>
</package>
"#,
        spec.package_id, spec.version
    )
}

fn render_content_types() -> String {
    r#"<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml" />
  <Default Extension="nuspec" ContentType="application/octet-stream" />
  <Default Extension="ll" ContentType="text/plain" />
  <Default Extension="gl" ContentType="text/plain" />
</Types>
"#
    .to_string()
}

fn render_rels(spec: &NugetPackageSpec<'_>) -> String {
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="Package" Type="http://schemas.microsoft.com/packaging/2010/07/manifest" Target="/{}.nuspec" />
</Relationships>
"#,
        spec.package_id
    )
}

fn write_zip(output: &mut Vec<u8>, entries: &[(String, Vec<u8>)]) -> Result<(), String> {
    let mut central_directory = Vec::new();
    let mut local_header_offsets = Vec::new();

    for (name, data) in entries {
        let name_bytes = name.as_bytes();
        let crc32 = crc32(data);
        let offset = output.len() as u32;
        local_header_offsets.push(offset);

        write_u32(output, 0x0403_4B50);
        write_u16(output, 20);
        write_u16(output, 0);
        write_u16(output, 0);
        write_u16(output, 0);
        write_u16(output, 0);
        write_u32(output, crc32);
        write_u32(output, data.len() as u32);
        write_u32(output, data.len() as u32);
        write_u16(output, name_bytes.len() as u16);
        write_u16(output, 0);
        output.extend_from_slice(name_bytes);
        output.extend_from_slice(data);
    }

    let central_directory_offset = output.len() as u32;
    for ((name, data), offset) in entries.iter().zip(local_header_offsets.iter()) {
        let name_bytes = name.as_bytes();
        let crc32 = crc32(data);
        write_u32(&mut central_directory, 0x0201_4B50);
        write_u16(&mut central_directory, 20);
        write_u16(&mut central_directory, 20);
        write_u16(&mut central_directory, 0);
        write_u16(&mut central_directory, 0);
        write_u16(&mut central_directory, 0);
        write_u16(&mut central_directory, 0);
        write_u32(&mut central_directory, crc32);
        write_u32(&mut central_directory, data.len() as u32);
        write_u32(&mut central_directory, data.len() as u32);
        write_u16(&mut central_directory, name_bytes.len() as u16);
        write_u16(&mut central_directory, 0);
        write_u16(&mut central_directory, 0);
        write_u16(&mut central_directory, 0);
        write_u16(&mut central_directory, 0);
        write_u32(&mut central_directory, 0);
        write_u32(&mut central_directory, *offset);
        central_directory.extend_from_slice(name_bytes);
    }
    let central_directory_size = central_directory.len() as u32;
    output.extend_from_slice(&central_directory);

    write_u32(output, 0x0605_4B50);
    write_u16(output, 0);
    write_u16(output, 0);
    write_u16(output, entries.len() as u16);
    write_u16(output, entries.len() as u16);
    write_u32(output, central_directory_size);
    write_u32(output, central_directory_offset);
    write_u16(output, 0);
    Ok(())
}

fn write_u16(buffer: &mut Vec<u8>, value: u16) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn write_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}
