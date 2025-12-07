use amber_ast::{Modifier, Type};

pub fn binding_qualifier(modifier: Option<Modifier>, is_mutable: bool) -> String {
    let mut flags = Vec::new();
    if matches!(modifier, Some(Modifier::Comptime)) {
        flags.push("const");
    }
    if !is_mutable && !flags.contains(&"const") {
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
        Type::Custom(name) => name.clone(),
    }
}

pub fn type_to_c_opt(ty: Option<&Type>) -> String {
    ty.map(type_to_c)
        .unwrap_or_else(|| "void".into())
}
