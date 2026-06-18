use super::*;
use super::support::*;

pub(super) fn llvm_type(ty: &TypeSyntax) -> LlType {
    match ty {
        TypeSyntax::Scalar(ScalarType::Bool) => LlType::I1,
        TypeSyntax::Scalar(ScalarType::Byte) => LlType::I8,
        TypeSyntax::Scalar(ScalarType::Short) => LlType::I16,
        TypeSyntax::Scalar(ScalarType::I32) => LlType::I32,
        TypeSyntax::Scalar(ScalarType::I64) => LlType::I64,
        TypeSyntax::Scalar(ScalarType::U32) => LlType::I32,
        TypeSyntax::Scalar(ScalarType::F64 | ScalarType::Decimal) => LlType::Double,
        TypeSyntax::String => LlType::Ptr,
        TypeSyntax::Ref(_) => LlType::Ptr,
        TypeSyntax::GenericNamed { .. } => LlType::Ptr,
        TypeSyntax::Void => LlType::Void,
        TypeSyntax::Nullable(inner) => llvm_type(inner),
        _ => LlType::Ptr,
    }
}

pub(super) fn llvm_ir_type(ty: &IrType) -> LlType {
    match ty {
        IrType::Bool => LlType::I1,
        IrType::Byte => LlType::I8,
        IrType::Short => LlType::I16,
        IrType::Int | IrType::UInt => LlType::I32,
        IrType::Long => LlType::I64,
        IrType::Double | IrType::Decimal => LlType::Double,
        IrType::Void => LlType::Void,
        IrType::String
        | IrType::Array(_)
        | IrType::Ref(_)
        | IrType::Weak(_)
        | IrType::Struct(_)
        | IrType::Class(_)
        | IrType::Interface(_)
        | IrType::List(_)
        | IrType::Dictionary(_, _)
        | IrType::Enumerable(_)
        | IrType::Nullable(_)
        | IrType::Thread
        | IrType::Task(_)
        | IrType::Function { .. }
        | IrType::Exception
        | IrType::Unknown(_) => LlType::Ptr,
    }
}

pub(super) fn object_type_name(ty: &IrType) -> Option<&str> {
    match ty {
        IrType::Struct(name) | IrType::Class(name) | IrType::Interface(name) => Some(name),
        IrType::Ref(inner) => object_type_name(inner),
        IrType::Nullable(inner) => object_type_name(inner),
        _ => None,
    }
}

pub(super) fn endpoint_parameter_supported(param: &EndpointParameterBinding, route: &str) -> bool {
    match (&param.source, &param.ty) {
        (EndpointParameterSource::Route, IrType::Int | IrType::Long | IrType::String) => {
            route_parameter_segment(route, &param.name).is_some()
        }
        (EndpointParameterSource::Body, IrType::String) => true,
        (
            EndpointParameterSource::Query,
            IrType::Bool | IrType::Int | IrType::Long | IrType::String,
        ) => true,
        _ => false,
    }
}

pub(super) fn route_parameter_segment(route: &str, parameter: &str) -> Option<usize> {
    route
        .split('/')
        .filter(|segment| !segment.is_empty())
        .enumerate()
        .find_map(|(index, segment)| {
            let inner = segment.strip_prefix('{')?.strip_suffix('}')?;
            let name = inner.split(':').next().unwrap_or(inner);
            (name == parameter).then_some(index)
        })
}

pub(super) fn base_type_name(name: &str) -> &str {
    name.split('<')
        .next()
        .unwrap_or(name)
        .rsplit('.')
        .next()
        .unwrap_or(name)
}

pub(super) fn llvm_object_name(name: &str) -> String {
    format!("glitch.{}", sanitize(name))
}

pub(super) fn qualified_name(
    namespace: &[String],
    name: &str,
    generic_arity: usize,
    type_id: usize,
) -> String {
    if namespace.is_empty() {
        format!("{}__g{}__t{}", name, generic_arity, type_id)
    } else {
        format!("{}.{}__g{}__t{}", namespace.join("."), name, generic_arity, type_id)
    }
}

pub(super) fn retain_symbol(name: &str) -> String {
    format!("glitch_retain_{}", sanitize(name))
}

pub(super) fn drop_symbol(name: &str) -> String {
    format!("glitch_drop_{}", sanitize(name))
}

pub(super) fn destroy_symbol(name: &str) -> String {
    format!("glitch_destroy_{}", sanitize(name))
}

pub(super) fn expr_source_name(expr: &TypedExpr) -> Option<&str> {
    match &expr.kind {
        TypedExprKind::Var(name) | TypedExprKind::Move(name) => Some(name),
        _ => None,
    }
}


