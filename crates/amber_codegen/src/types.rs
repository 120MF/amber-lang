use amber_ast::{Modifier, Type};
use std::ops::Deref;

pub fn binding_qualifier(is_mutable: bool) -> String {
    let mut flags = Vec::new();
    if !is_mutable {
        flags.push("const");
    }
    if flags.is_empty() {
        String::new()
    } else {
        format!("{} ", flags.join(" "))
    }
}

pub fn type_to_c(ty: &Type) -> String {
    match ty {
        Type::Named(name) => name.clone(),
        Type::Pointer { inner, is_mut } => {
            let inner_type = type_to_c(inner.deref());
            format!("{}*", inner_type)
        }
        _ => builtin_type_to_c(ty),
    }
}

pub fn builtin_type_to_c(ty: &Type) -> String {
    match ty {
        Type::U8 => "uint8_t".into(),
        Type::I8 => "int8_t".into(),
        Type::U16 => "uint16_t".into(),
        Type::I16 => "int16_t".into(),
        Type::U32 => "uint32_t".into(),
        Type::I32 => "int32_t".into(),
        Type::U64 => "uint64_t".into(),
        Type::I64 => "int64_t".into(),
        Type::F32 => "float".into(),
        Type::F64 => "double".into(),
        Type::Bool => "bool".into(),
        Type::Char => "char".into(),
        Type::Void => "void".into(),
        _ => panic!("{:?} is not a  builtin type", ty),
    }
}
