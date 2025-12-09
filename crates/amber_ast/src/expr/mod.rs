mod binary;
mod literal;
mod unary;

pub use binary::BinaryOp;
pub use literal::{Literal, NumericLiteral};
pub use unary::{UnaryOp, Prefix, Postfix};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    UnaryExpr {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    BinaryExpr {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    TernaryExpr {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
}
