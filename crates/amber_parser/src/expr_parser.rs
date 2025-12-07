use pest::iterators::Pair;

use amber_ast::{BinaryOp, Expression};

use crate::pratt::create_expr_parser;
use crate::Rule;

/// Parse a primary expression (atom, integer, identifier, or parenthesized expression)
fn parse_primary(primary: Pair<Rule>) -> Expression {
    match primary.as_rule() {
        Rule::atom => {
            let mut inner = primary.into_inner();
            let inner = inner.next().expect("atom must contain value");
            parse_primary(inner)
        }
        Rule::int_lit => {
            let val: i64 = primary.as_str().parse().unwrap();
            Expression::Integer(val)
        }
        Rule::ident => Expression::Identifier(primary.as_str().to_string()),
        Rule::expr => parse_expr(primary),
        _ => panic!("Unknown primary: {:?}", primary.as_rule()),
    }
}

/// Parse an expression with operator precedence using Pratt parser
pub fn parse_expr(pair: Pair<Rule>) -> Expression {
    let pairs = pair.into_inner();
    create_expr_parser()
        .map_primary(|primary| parse_primary(primary))
        .map_infix(|lhs, op, rhs| {
            let binary_op = match op.as_rule() {
                Rule::add => BinaryOp::Add,
                Rule::sub => BinaryOp::Sub,
                Rule::mul => BinaryOp::Mul,
                Rule::div => BinaryOp::Div,
                _ => panic!("Unexpected operator: {:?}", op.as_rule()),
            };
            Expression::BinaryExpr {
                left: Box::new(lhs),
                op: binary_op,
                right: Box::new(rhs),
            }
        })
        .parse(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_ast;

    #[test]
    fn test_expression_precedence() {
        let code = "let a = 1 + 2 * 3;";
        let program = build_ast(code).unwrap();

        if let amber_ast::Statement::LetBinding(binding) = &program.statements[0]
        {
            if let Some(expr) = &binding.value {
                if let Expression::BinaryExpr { left, op, right } = expr {
                    assert_eq!(*op, BinaryOp::Add);

                    if let Expression::Integer(v) = **left {
                        assert_eq!(v, 1);
                    } else {
                        panic!("Left should be 1");
                    }

                    if let Expression::BinaryExpr {
                        left: _r_left,
                        op: r_op,
                        right: _r_right,
                    } = &**right
                    {
                        assert_eq!(*r_op, BinaryOp::Mul);
                    } else {
                        panic!("Right side should be multiplication");
                    }
                } else {
                    panic!("Top level should be addition");
                }
            }
        }
    }

    #[test]
    fn test_parenthesis() {
        let code = "let a = (1 + 2) * 3;";
        let program = build_ast(code).unwrap();

        if let amber_ast::Statement::LetBinding(binding) = &program.statements[0]
        {
            if let Some(expr) = &binding.value {
                if let Expression::BinaryExpr { op, .. } = expr {
                    assert_eq!(*op, BinaryOp::Mul);
                } else {
                    panic!("Top level should be multiplication");
                }
            }
        }
    }
}
