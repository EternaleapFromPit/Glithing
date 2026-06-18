use super::*;

pub(super) fn void_value() -> LlValue {
    LlValue {
        value: "0".to_string(),
        ty: LlType::I32,
    }
}

pub(super) fn integer_bits(ty: &LlType) -> u8 {
    match ty {
        LlType::I1 => 1,
        LlType::I8 => 8,
        LlType::I16 => 16,
        LlType::I32 => 32,
        LlType::I64 => 64,
        _ => 0,
    }
}

pub(super) fn fmt_ptr(kind: &str) -> &'static str {
    match kind {
        "i64" => "getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0)",
        "double" => "getelementptr inbounds ([4 x i8], ptr @.fmt_double, i64 0, i64 0)",
        "str" => "getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0)",
        _ => "getelementptr inbounds ([4 x i8], ptr @.fmt_i32, i64 0, i64 0)",
    }
}

pub(super) fn sanitize(name: &str) -> String {
    let mut result = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            result.push(ch);
        } else {
            result.push('_');
        }
    }
    if result.is_empty() || result.chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
        result.insert(0, '_');
    }
    result
}

pub(super) fn escaped_bytes(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        match byte {
            b'\\' => out.push_str("\\5C"),
            b'"' => out.push_str("\\22"),
            b'\n' => out.push_str("\\0A"),
            b'\r' => out.push_str("\\0D"),
            b'\t' => out.push_str("\\09"),
            32..=126 => out.push(byte as char),
            other => out.push_str(&format!("\\{other:02X}")),
        }
    }
    out
}


