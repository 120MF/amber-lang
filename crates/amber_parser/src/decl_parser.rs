use pest::iterators::Pair;

use amber_ast::{Function, ImplBlock, Param, StructDef, StructField};

use crate::stmt_parser::parse_block;
use crate::utils::parse_type;
use crate::Rule;

/// Parse a struct definition
pub fn parse_struct(pair: Pair<Rule>) -> StructDef {
    let mut inner = pair.into_inner();
    let name = inner
        .next()
        .expect("struct must have a name")
        .as_str()
        .to_string();

    let mut fields = Vec::new();
    for part in inner {
        if part.as_rule() == Rule::struct_fields {
            fields = part.into_inner().map(parse_struct_field).collect();
        }
    }

    StructDef { name, fields }
}

/// Parse a single struct field
fn parse_struct_field(pair: Pair<Rule>) -> StructField {
    let mut inner = pair.into_inner();
    let name = inner
        .next()
        .expect("struct field needs name")
        .as_str()
        .to_string();
    let ty_pair = inner.next().expect("struct field needs type");

    StructField {
        name,
        ty: parse_type(ty_pair),
    }
}

/// Parse a function definition
pub fn parse_function(pair: Pair<Rule>) -> Function {
    let mut name = String::new();
    let mut params = Vec::new();
    let mut return_type = None;
    let mut body = None;
    let mut is_extern = false;

    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::extern_modifier => is_extern = true,
            Rule::ident => name = part.as_str().to_string(),
            Rule::parameter_list => {
                params = part.into_inner().map(parse_param).collect();
            }
            Rule::return_type => {
                let mut inner = part.into_inner();
                let ty_pair = inner.next().expect("return type must contain a type");
                return_type = Some(parse_type(ty_pair));
            }
            Rule::function_body => {
                if let Some(block_pair) = part.into_inner().next() {
                    if block_pair.as_rule() == Rule::block {
                        body = Some(parse_block(block_pair));
                    }
                }
            }
            _ => {}
        }
    }

    Function {
        name,
        params,
        return_type,
        body,
        is_extern,
    }
}

/// Parse a function parameter
fn parse_param(pair: Pair<Rule>) -> Param {
    match pair.as_rule() {
        Rule::param => {
            let inner = pair.into_inner().next().expect("param must have inner");
            parse_param(inner)
        }
        Rule::param_self => Param::SelfParam,
        Rule::param_typed => parse_typed_param(pair),
        _ => panic!("Unexpected parameter {:?}", pair.as_rule()),
    }
}

/// Parse a typed parameter
fn parse_typed_param(pair: Pair<Rule>) -> Param {
    let mut inner = pair.into_inner();
    let name = inner
        .next()
        .expect("param needs a name")
        .as_str()
        .to_string();
    let ty_pair = inner.next().expect("param needs a type");
    Param::Typed {
        name,
        ty: parse_type(ty_pair),
    }
}

/// Parse an impl block
pub fn parse_impl(pair: Pair<Rule>) -> ImplBlock {
    let mut inner = pair.into_inner();
    let target = inner
        .next()
        .expect("impl block needs target")
        .as_str()
        .to_string();
    let methods = inner
        .filter(|p| p.as_rule() == Rule::function_def)
        .map(parse_function)
        .collect();

    ImplBlock { target, methods }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_ast;
    use amber_ast::{Type, Statement};

    #[test]
    fn test_struct_definition() {
        let code = r#"
            struct Point {
                x: i32,
                y: i32,
            }
        "#;
        let program = build_ast(code).unwrap();
        match &program.statements[0] {
            Statement::Struct(def) => {
                assert_eq!(def.name, "Point");
                assert_eq!(def.fields.len(), 2);
                assert_eq!(def.fields[0].name, "x");
                assert_eq!(def.fields[1].name, "y");
            }
            _ => panic!("Expected struct definition"),
        }
    }

    #[test]
    fn test_function_definition() {
        let code = r#"
            fn add(a: i32, b: i32) -> i32 {
                return a + b;
            }

            extern fn HAL_Delay(ms: u32);
        "#;

        let program = build_ast(code).unwrap();
        assert_eq!(program.statements.len(), 2);

        match &program.statements[0] {
            Statement::Function(func) => {
                assert_eq!(func.name, "add");
                assert!(!func.is_extern);
                assert_eq!(func.params.len(), 2);
                assert_eq!(func.return_type, Some(Type::I32));
                assert!(
                    func.body
                        .as_ref()
                        .is_some_and(|body| matches!(body.statements[0], Statement::Return(_)))
                );
            }
            _ => panic!("Expected function definition"),
        }

        match &program.statements[1] {
            Statement::Function(func) => {
                assert!(func.is_extern);
                assert!(func.body.is_none());
                assert_eq!(func.params.len(), 1);
            }
            _ => panic!("Expected extern function definition"),
        }
    }

    #[test]
    fn test_impl_block() {
        let code = r#"
            impl Point {
                fn new(x: i32, y: i32) -> i32 {
                    return x + y;
                }

                fn translate(self, dx: i32) {
                    var delta: i32 = dx;
                    return;
                }
            }
        "#;

        let program = build_ast(code).unwrap();
        match &program.statements[0] {
            Statement::Impl(block) => {
                assert_eq!(block.target, "Point");
                assert_eq!(block.methods.len(), 2);

                let first = &block.methods[0];
                assert_eq!(first.name, "new");
                assert_eq!(first.return_type, Some(Type::I32));

                let second = &block.methods[1];
                assert_eq!(second.name, "translate");
                assert!(matches!(second.params.first(), Some(Param::SelfParam)));
            }
            _ => panic!("Expected impl block"),
        }
    }
}
