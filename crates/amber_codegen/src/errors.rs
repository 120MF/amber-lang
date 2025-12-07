use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CodegenError {
    #[error("let binding '{name}' requires an explicit type")]
    MissingType { name: String },
    #[error("function '{name}' is missing a body")]
    MissingFunctionBody { name: String },
    #[error("extern function '{name}' cannot have a body")]
    ExternFunctionWithBody { name: String },
    #[error("`self` parameter is only allowed inside impl blocks (function '{name}')")]
    SelfParamOutsideImpl { name: String },
    #[error("function '{name}' contains multiple `self` parameters")]
    MultipleSelfParams { name: String },
    #[error("impl method '{target}::{name}' cannot be declared extern")]
    ExternImplMethod { target: String, name: String },
}
