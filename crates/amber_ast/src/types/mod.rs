#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Char,
    Void,
    Named(String),

    Pointer { inner: Box<Type>, is_mut: bool },
    Array { inner: Box<Type>, len: usize },
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::F32
                | Type::F64
        )
    }

    pub fn is_floating(&self) -> bool {
        matches!(self, Type::F32 | Type::F64)
    }
}
