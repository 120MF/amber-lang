mod binary;
mod literal;

pub use binary::BinaryOp;
pub use literal::{Literal, NumericLiteral};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryExpr {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
}
