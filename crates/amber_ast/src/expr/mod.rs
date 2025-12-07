mod binary;

pub use binary::BinaryOp;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    Identifier(String),
    BinaryExpr {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
}
