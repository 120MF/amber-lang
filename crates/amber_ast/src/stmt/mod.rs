mod bindings;
mod control;

pub use bindings::VariableBinding;
pub use control::{IfElse, WhileLoop};
use crate::{Expression, Function, ImplBlock, StructDef};

#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    Comptime,
    Runtime,
    // @section maybe...
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Binding(VariableBinding),
    IfElse(IfElse),
    WhileLoop(WhileLoop),
    ExprStatement(Expression),
    Struct(StructDef),
    Function(Function),
    Impl(ImplBlock),
    Assignment { target: Expression, value: Expression },
    Return(Option<Expression>),
}
