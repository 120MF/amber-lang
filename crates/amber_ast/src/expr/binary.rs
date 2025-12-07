#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    
    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    
    // Logical
    And,
    Or,
}

