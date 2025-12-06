#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetBinding {
        modifier: Option<Modifier>, // comptime / runtime / None
        is_mutable: bool,           // let = false, var = true
        name: String,
        ty: Option<Type>, // type
        value: Option<Expression>,
    },
    ExprStatement(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    Comptime,
    Runtime,
    // @section maybe...
}

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
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

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
