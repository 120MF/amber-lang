use pest::Parser;
use pest::iterators::Pair;
use pest::pratt_parser::{PrattParser, Assoc, Op};
use pest_derive::Parser;

use amber_ast::{Expression, Modifier, Program, Statement, Type};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AmberParser;

pub fn parse_source(input: &str) -> Result<(), String> {
    let pairs = AmberParser::parse(Rule::program, input).map_err(|e| format!("{}", e))?;
    for pair in pairs {
        println!("Rule: {:?}", pair.as_rule());
        println!("Text: {}", pair.as_str());

        for inner_pair in pair.into_inner() {
            println!("  ├─ Rule: {:?}", inner_pair.as_rule());
            println!("  └─ Text: {}", inner_pair.as_str());
        }
    }
    Ok(())
}

pub fn build_ast(input: &str) -> Result<Program, String> {
    let mut pairs = AmberParser::parse(Rule::program, input).map_err(|e| format!("{}", e))?;

    let root = pairs.next().unwrap();

    let mut statements = Vec::new();

    for pair in root.into_inner() {
        if pair.as_rule() == Rule::statement {
            statements.push(parse_statement(pair));
        }
    }
    Ok(Program { statements })
}

fn parse_statement(pair: Pair<Rule>) -> Statement {
    // statement = { declaration | assignment | expr_stmt }
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::declaration => parse_declaration(inner),
        Rule::expr_stmt => {
            let expr_pair = inner.into_inner().next().unwrap();
            Statement::ExprStatement(parse_expr(expr_pair))
        }
        _ => panic!("TODO: Implement other statements: {:?}", inner.as_rule()),
    }
}

fn parse_declaration(pair: Pair<Rule>) -> Statement {
    let inner = pair.into_inner();

    let mut modifier = None;
    let mut is_mutable = false;
    let mut name = String::new();
    let mut ty = None;
    let mut value = None;

    // iterates declaration

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
                ty = match part.as_str() {
                    "u8" => Some(Type::U8),
                    "u16" => Some(Type::U16),
                    "u32" => Some(Type::U32),
                    "u64" => Some(Type::U64),
                    "i8" => Some(Type::I8),
                    "i16" => Some(Type::I16),
                    "i32" => Some(Type::I32),
                    "i64" => Some(Type::I64),
                    "bool" => Some(Type::Bool),
                    _ => panic!("Unkown type"),
                };
            }
            Rule::expr => {
                value = Some(parse_expr(part));
            }
            _ => {}
        }
    }
    Statement::LetBinding {
        modifier,
        is_mutable,
        name,
        ty,
        value,
    }
}

fn parse_expr(pair: Pair<Rule>) -> Expression {

    let pairs = pair.into_inner();
    expr_parser()
      .map_primary(|primary|{
            match primary.as_rule(){
                Rule::int_lit =>{
                    let val: i64 = primary.as_str().parse().unwrap();
                    Expression::Integer(val)
                }
                Rule::ident => {
                    Expression::Identifier(primary.as_str().to_string())
                }
                Rule::expr => {
                    parse_expr(primary)
                }
                _ => panic!("Unknown atom: {:?}", primary.as_rule()),
            }
        })
      .map_infix(|lhs, op, rhs| {
            let binary_op = match op.as_rule(){
                Rule::add => amber_ast::BinaryOp::Add,
                Rule::sub => amber_ast::BinaryOp::Sub,
                Rule::mul => amber_ast::BinaryOp::Mul,
                Rule::div => amber_ast::BinaryOp::Div,
                _ => panic!("Unexpected operator: {:?}", op.as_rule()),
            };
            Expression::BinaryExpr { left: Box::new(lhs), op: binary_op, right: Box::new(rhs) }
            
        })
            .parse(pairs)

}

fn expr_parser() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_declaration() {
        let code = "comptime let baud_rate = 9600;";
        let result = parse_source(code);
        assert!(result.is_ok());
    }

    #[test]
    fn test_runtime_var() {
        let code = "runtime var counter: u8 = 0;";
        let result = parse_source(code);
        assert!(result.is_ok());

    }
    #[test]
    fn test_fail_syntax() {
        // miss ";"
        let code = "let a = 10";
        let result = parse_source(code);
        assert!(result.is_err());
        println!("Error msg: {}", result.err().unwrap());
    }

    #[test]
    fn test_ast_generation() {
        let code = "comptime let baud = 9600;";
        let program = build_ast(code).unwrap();

        // expected:
        if let Statement::LetBinding {
            modifier,
            is_mutable,
            name,
            value,
            ..
        } = &program.statements[0]
        {
            assert_eq!(*modifier, Some(Modifier::Comptime));
            assert!(!(*is_mutable)); // let
            assert_eq!(name, "baud");

            if let Some(Expression::Integer(val)) = value {
                assert_eq!(*val, 9600);
            } else {
                panic!("Expected integer value");
            }
        } else {
            panic!("Expected LetBinding");
        }
    }
    #[test]
fn test_expression_precedence() {
    // 测试 1 + 2 * 3
    // 应该被解析为 1 + (2 * 3)，而不是 (1 + 2) * 3
    let code = "let a = 1 + 2 * 3;";
    let program = build_ast(code).unwrap();

    if let Statement::LetBinding { value: Some(expr), .. } = &program.statements[0] {
        // 期望最外层是 Add
        if let Expression::BinaryExpr { left, op, right } = expr {
            assert_eq!(*op, amber_ast::BinaryOp::Add);
            
            // 左边应该是 1
            if let Expression::Integer(v) = **left {
                assert_eq!(v, 1);
            } else { panic!("Left should be 1"); }

            // 右边应该是一个 Mul 表达式 (2 * 3)
            if let Expression::BinaryExpr { left: r_left, op: r_op, right: r_right } = &**right {
                assert_eq!(*r_op, amber_ast::BinaryOp::Mul);
                // 检查 2 和 3...
            } else {
                panic!("Right side should be multiplication");
            }
        } else {
            panic!("Top level should be addition");
        }
    }
}

#[test]
fn test_parenthesis() {
    // 测试 (1 + 2) * 3
    // 括号应该改变优先级
    let code = "let a = (1 + 2) * 3;";
    let program = build_ast(code).unwrap();

    if let Statement::LetBinding { value: Some(expr), .. } = &program.statements[0] {
        // 期望最外层是 Mul
        if let Expression::BinaryExpr { op, .. } = expr {
            assert_eq!(*op, amber_ast::BinaryOp::Mul);
        } else {
            panic!("Top level should be multiplication");
        }
    }
}
}
