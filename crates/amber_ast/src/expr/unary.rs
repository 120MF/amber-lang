#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,     // -x
    Pos,     // +x
    Not,     // !x (logical not)
    BitNot,  // ^x (bitwise not)
    PreInc,  // ++x
    PreDec,  // --x
}
