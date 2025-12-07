use crate::program::Block;
use crate::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,
    pub is_extern: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    SelfParam,
    Typed { name: String, ty: Type },
}
