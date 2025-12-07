use crate::Expression;
mod bindings;
use crate::decl::{Function, ImplBlock, StructDef};
pub use bindings::LetBinding;

#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    Comptime,
    Runtime,
    // @section maybe...
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetBinding(LetBinding),
    ExprStatement(Expression),
    Struct(StructDef),
    Function(Function),
    Impl(ImplBlock),
    Assignment { target: String, value: Expression },
    Return(Option<Expression>),
}
