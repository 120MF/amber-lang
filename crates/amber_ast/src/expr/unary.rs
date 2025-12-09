use crate::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    PrefixOp(Prefix),
    PostfixOp(Postfix),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Neg,    // -x
    Pos,    // +x
    Not,    // !x (logical not)
    BitNot, // ^x (bitwise not)
    PreInc, // ++x
    PreDec, // --x
    Deref,  // *x
}

#[derive(Debug, Clone, PartialEq)]
pub enum Postfix {
    Index { index: Box<Expression> },  // x[i]
}