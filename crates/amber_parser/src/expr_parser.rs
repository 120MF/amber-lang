use pest::iterators::Pair;

use amber_ast::{BinaryOp, Expression, Literal, NumericLiteral, UnaryOp};

use crate::Rule;
use crate::pratt::expr_parser;

/// Parse a primary expression (literal, identifier, or parenthesized expression)
fn parse_primary(primary: Pair<Rule>) -> Expression {
    match primary.as_rule() {
        Rule::atom => {
            let mut inner = primary.into_inner();
            let inner = inner.next().expect("atom must contain value");
            parse_primary(inner)
        }
        Rule::int_lit => {
            let val: i64 = primary.as_str().parse().unwrap();
            Expression::Literal(Literal::Numeric(NumericLiteral::Integer(val)))
        }
        Rule::float_lit => {
            let literal = primary.as_str();
            let cleaned = literal.trim_end_matches('f');
            let val: f64 = cleaned.parse().unwrap();
            if literal.ends_with('f') {
                Expression::Literal(Literal::Numeric(NumericLiteral::Float(val as f32)))
            } else {
                Expression::Literal(Literal::Numeric(NumericLiteral::Double(val)))
            }
        }
        Rule::bool_lit => {
            let b = primary.as_str() == "true";
            Expression::Literal(Literal::Bool(b))
        }
        Rule::char_lit => {
            let c = primary.as_str();
            Expression::Literal(Literal::Char(c.as_bytes()[1] as char))
        }
        Rule::ident => Expression::Identifier(primary.as_str().to_string()),
        Rule::expr | Rule::ternary_expr | Rule::math_expr | Rule::unary => parse_expr(primary),
        _ => panic!("Unknown primary: {:?}", primary.as_rule()),
    }
}

/// Parse binary operator
fn parse_binary_op(op: Pair<Rule>) -> BinaryOp {
    match op.as_rule() {
        Rule::add_op => BinaryOp::Add,
        Rule::sub_op => BinaryOp::Sub,
        Rule::mul_op => BinaryOp::Mul,
        Rule::div_op => BinaryOp::Div,
        Rule::mod_op => BinaryOp::Mod,
        Rule::eq_op => BinaryOp::Eq,
        Rule::ne_op => BinaryOp::Ne,
        Rule::lt_op => BinaryOp::Lt,
        Rule::le_op => BinaryOp::Le,
        Rule::gt_op => BinaryOp::Gt,
        Rule::ge_op => BinaryOp::Ge,
        Rule::bitwise_and => BinaryOp::BitAnd,
        Rule::bitwise_or => BinaryOp::BitOr,
        Rule::bitwise_xor => BinaryOp::BitXor,
        Rule::shl_op => BinaryOp::Shl,
        Rule::shr_op => BinaryOp::Shr,
        Rule::and_op => BinaryOp::And,
        Rule::or_op => BinaryOp::Or,
        _ => panic!("Unexpected operator: {:?}", op.as_rule()),
    }
}

/// Parse unary operator
fn parse_unary_op(op: Pair<Rule>) -> UnaryOp {
    match op.as_rule() {
        Rule::prefix_minus => UnaryOp::Neg,
        Rule::prefix_plus => UnaryOp::Pos,
        Rule::prefix_not => UnaryOp::Not,
        Rule::prefix_bitnot => UnaryOp::BitNot,
        Rule::prefix_preinc => UnaryOp::PreInc,
        Rule::prefix_predec => UnaryOp::PreDec,
        _ => panic!("Unexpected unary operator: {:?}", op.as_rule()),
    }
}

/// Parse an expression using Pratt parser with ternary operator support
pub fn parse_expr(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        // Handle ternary expression: math_expr ? expr : expr
        Rule::ternary_expr => {
            let mut inner = pair.into_inner();
            let condition = parse_math_expr(inner.next().expect("ternary: missing condition"));

            // Check if there's a question mark
            if let Some(question) = inner.next() {
                match question.as_rule() {
                    Rule::question => {
                        // This is a ternary expression
                        let then_expr =
                            parse_expr(inner.next().expect("ternary: missing then expression"));
                        let _colon = inner.next().expect("ternary: missing colon");
                        let else_expr =
                            parse_expr(inner.next().expect("ternary: missing else expression"));

                        return Expression::TernaryExpr {
                            condition: Box::new(condition),
                            then_expr: Box::new(then_expr),
                            else_expr: Box::new(else_expr),
                        };
                    }
                    _ => {
                        // No ternary operator, just return the condition
                        condition
                    }
                }
            } else {
                // No ternary, just return the condition
                condition
            }
        }
        // Handle math expression (with PrattParser)
        Rule::math_expr => parse_math_expr(pair),
        // For other rules, parse as math_expr
        _ => parse_math_expr(pair),
    }
}

/// Parse a math expression using global Pratt parser (no ternary at this level)
fn parse_math_expr(pair: Pair<Rule>) -> Expression {
    let pairs = pair.into_inner();
    expr_parser()
        .map_primary(|primary| parse_primary(primary))
        .map_prefix(|op, rhs| {
            let unary_op = parse_unary_op(op);
            Expression::UnaryExpr {
                op: unary_op,
                expr: Box::new(rhs),
            }
        })
        .map_infix(|lhs, op, rhs| {
            let binary_op = parse_binary_op(op);
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
        let code = "const a = 1 + 2 * 3;";
        let program = build_ast(code).unwrap();

        if let amber_ast::Statement::LetBinding(binding) = &program.statements[0] {
            if let Some(expr) = &binding.value {
                if let Expression::BinaryExpr { left, op, right } = expr {
                    assert_eq!(*op, BinaryOp::Add);

                    if let Expression::Literal(Literal::Numeric(num)) = &**left {
                        assert!(num.is_integer());
                        assert_eq!(num.to_i64(), 1);
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
        let code = "const a = (1 + 2) * 3;";
        let program = build_ast(code).unwrap();

        if let amber_ast::Statement::LetBinding(binding) = &program.statements[0] {
            if let Some(expr) = &binding.value {
                if let Expression::BinaryExpr { op, .. } = expr {
                    assert_eq!(*op, BinaryOp::Mul);
                } else {
                    panic!("Top level should be multiplication");
                }
            }
        }
    }

    #[test]
    fn test_bool_literal() {
        let code = "const flag: bool = true;";
        let program = build_ast(code).unwrap();

        if let amber_ast::Statement::LetBinding(binding) = &program.statements[0] {
            assert_eq!(binding.name, "flag");
            if let Some(Expression::Literal(Literal::Bool(b))) = &binding.value {
                assert!(*b);
            } else {
                panic!("Expected bool literal");
            }
        } else {
            panic!("Expected LetBinding");
        }
    }

    #[test]
    fn test_bool_literal_false() {
        let code = "const active = false;";
        let program = build_ast(code).unwrap();

        if let amber_ast::Statement::LetBinding(binding) = &program.statements[0] {
            if let Some(Expression::Literal(Literal::Bool(b))) = &binding.value {
                assert!(!*b);
            } else {
                panic!("Expected bool literal false");
            }
        } else {
            panic!("Expected LetBinding");
        }
    }

    #[test]
    fn test_char_literal() {
        let code = "const character = 'a';";
        let program = build_ast(code).unwrap();

        if let amber_ast::Statement::LetBinding(binding) = &program.statements[0] {
            if let Some(Expression::Literal(Literal::Char(c))) = &binding.value {
                assert_eq!(*c, 'a');
            } else {
                panic!("Expected char literal 'c'")
            }
        } else {
            panic!("Expected LetBinding");
        }
    }
}
