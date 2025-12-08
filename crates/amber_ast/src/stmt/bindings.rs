use crate::stmt::Modifier;
use crate::{Expression, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableBinding {
    pub modifier: Option<Modifier>, // comptime / runtime / None
    pub is_mutable: bool,           // const = false, var = true
    pub name: String,
    pub ty: Option<Type>, // type
    pub value: Option<Expression>,
}
