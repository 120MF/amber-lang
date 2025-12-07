use pest::iterators::Pair;

use amber_ast::{Block, Modifier, Statement, LetBinding};

use crate::expr_parser::parse_expr;
use crate::Rule;

/// Parse a declaration (let/var binding)
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
        let code = "comptime let baud_rate = 9600;";
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
        let code = "let a = 10";
        let result = crate::parse_source(code);
        assert!(result.is_err());
    }

    #[test]
    fn test_ast_generation() {
        let code = "comptime let baud = 9600;";
        let program = build_ast(code).unwrap();

        if let Statement::LetBinding(binding) = &program.statements[0]
        {
            assert_eq!(binding.modifier, Some(Modifier::Comptime));
            assert!(!binding.is_mutable);
            assert_eq!(binding.name, "baud");

            if let Some(Expression::Integer(val)) = &binding.value {
                assert_eq!(*val, 9600);
            } else {
                panic!("Expected integer value");
            }
        } else {
            panic!("Expected LetBinding");
        }
    }
}
