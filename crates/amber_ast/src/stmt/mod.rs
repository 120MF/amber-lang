mod bindings;
mod control;

pub use bindings::LetBinding;
pub use control::IfElse;
use crate::{Expression, Function, ImplBlock, StructDef};

#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    Comptime,
    Runtime,
    // @section maybe...
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetBinding(LetBinding),
    IfElse(IfElse),
    ExprStatement(Expression),
    Struct(StructDef),
    Function(Function),
    Impl(ImplBlock),
    Assignment { target: String, value: Expression },
    Return(Option<Expression>),
}
