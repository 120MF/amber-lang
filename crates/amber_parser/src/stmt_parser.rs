use pest::iterators::Pair;

use amber_ast::{Block, IfElse, LetBinding, Modifier, Statement, WhileLoop};

use crate::Rule;
use crate::expr_parser::parse_expr;

/// Parse a declaration (const/var binding)
pub fn parse_declaration(pair: Pair<Rule>) -> Statement {
    let inner = pair.into_inner();

    let mut modifier = None;
    let mut is_mutable = false;
    let mut name = String::new();
    let mut ty = None;
    let mut value = None;

    for part in inner {
        match part.as_rule() {
            Rule::modifier => {
                modifier = match part.as_str() {
                    "comptime" => Some(Modifier::Comptime),
                    "runtime" => Some(Modifier::Runtime),
                    _ => None,
                };
            }
            Rule::keyword => {
                is_mutable = part.as_str() == "var";
            }
            Rule::ident => {
                name = part.as_str().to_string();
            }
            Rule::type_def => {
                ty = Some(crate::utils::parse_type(part));
            }
            Rule::expr => {
                value = Some(parse_expr(part));
            }
            _ => {}
        }
    }

    Statement::LetBinding(LetBinding {
        modifier,
        is_mutable,
        name,
        ty,
        value,
    })
}

/// Parse an assignment statement
pub fn parse_assignment(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();
    let target = inner
        .next()
        .expect("assignment must have a target")
        .as_str()
        .to_string();
    let expr_pair = inner.next().expect("assignment must have value");
    Statement::Assignment {
        target,
        value: parse_expr(expr_pair),
    }
}

/// Parse a return statement
pub fn parse_return(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();
    let expr = inner.next().map(parse_expr);
    Statement::Return(expr)
}

/// Parse an expression statement
pub fn parse_expr_stmt(pair: Pair<Rule>) -> Statement {
    let expr_pair = pair.into_inner().next().unwrap();
    Statement::ExprStatement(parse_expr(expr_pair))
}

/// Parse an if-else statement
pub fn parse_if_stmt(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    let condition = parse_expr(inner.next().expect("if must have condition"));
    let then_block = parse_block(inner.next().expect("if must have then block"));

    let else_block = if let Some(else_part) = inner.next() {
        match else_part.as_rule() {
            Rule::block => Some(parse_block(else_part)),
            Rule::if_stmt => {
                // else if case - wrap in block containing if statement
                let else_if_stmt = parse_if_stmt(else_part);
                Some(Block {
                    statements: vec![else_if_stmt],
                })
            }
            _ => None,
        }
    } else {
        None
    };

    Statement::IfElse(IfElse {
        condition,
        then_block,
        else_block,
    })
}

/// Parse a while loop statement
pub fn parse_while_stmt(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    let condition = parse_expr(inner.next().expect("while must have condition"));
    let block = parse_block(inner.next().expect("while must have block"));

    Statement::WhileLoop(WhileLoop {
        condition,
        block,
    })
}

/// Parse a block containing statements
pub fn parse_block(pair: Pair<Rule>) -> Block {
    let statements = pair
        .into_inner()
        .map(parse_block_statement)
        .collect::<Vec<_>>();
    Block { statements }
}

/// Parse a statement inside a block (different from top-level statements)
fn parse_block_statement(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::declaration => parse_declaration(pair),
        Rule::assignment => parse_assignment(pair),
        Rule::expr_stmt => parse_expr_stmt(pair),
        Rule::return_stmt => parse_return(pair),
        Rule::if_stmt => parse_if_stmt(pair),
        Rule::while_stmt => parse_while_stmt(pair),
        _ => panic!("unexpected statement '{:?}' inside block", pair.as_rule()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_ast;
    use amber_ast::Expression;

    #[test]
    fn test_basic_declaration() {
        let code = "comptime const baud_rate = 9600;";
        let result = crate::parse_source(code);
        assert!(result.is_ok());
    }

    #[test]
    fn test_runtime_var() {
        let code = "runtime var counter: u8 = 0;";
        let result = crate::parse_source(code);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fail_syntax() {
        // miss ";"
        let code = "const a = 10";
        let result = crate::parse_source(code);
        assert!(result.is_err());
    }

    #[test]
    fn test_ast_generation() {
        let code = "comptime const baud = 9600;";
        let program = build_ast(code).unwrap();

        if let Statement::LetBinding(binding) = &program.statements[0] {
            assert_eq!(binding.modifier, Some(Modifier::Comptime));
            assert!(!binding.is_mutable);
            assert_eq!(binding.name, "baud");

            if let Some(Expression::Literal(val)) = &binding.value {
                assert_eq!(val.to_string(), "9600");
            } else {
                panic!("Expected integer value");
            }
        } else {
            panic!("Expected LetBinding");
        }
    }

    #[test]
    fn test_while_loop_parsing() {
        let code = r#"
            fn test() {
                while true {
                    // empty block
                }
            }
        "#;
        let result = crate::build_ast(code);
        assert!(result.is_ok());
    }
}
