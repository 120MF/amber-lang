use crate::{Block, Expression};

#[derive(Clone, Debug, PartialEq)]
pub struct IfElse{
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: Expression,
    pub block: Block,
}