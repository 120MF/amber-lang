#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    pub target: String,
    pub methods: Vec<super::Function>,
}
