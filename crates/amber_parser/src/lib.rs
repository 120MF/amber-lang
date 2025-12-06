use pest::Parser;
use pest::iterators::Pair;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest_derive::Parser;

use amber_ast::{
    Expression, Function, ImplBlock, Modifier, Param, Program, Statement, StructDef, StructField,
    Type,
};

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
        Rule::struct_def => Statement::Struct(parse_struct(inner)),
        Rule::function_def => Statement::Function(parse_function(inner)),
        Rule::impl_block => Statement::Impl(parse_impl(inner)),
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
                ty = Some(parse_type(part));
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
        .map_primary(|primary| parse_primary(primary))
        .map_infix(|lhs, op, rhs| {
            let binary_op = match op.as_rule() {
                Rule::add => amber_ast::BinaryOp::Add,
                Rule::sub => amber_ast::BinaryOp::Sub,
                Rule::mul => amber_ast::BinaryOp::Mul,
                Rule::div => amber_ast::BinaryOp::Div,
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

fn expr_parser() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
}

fn parse_struct(pair: Pair<Rule>) -> StructDef {
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

fn parse_function(pair: Pair<Rule>) -> Function {
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
                        body = Some(block_pair.as_str().to_string());
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

fn parse_impl(pair: Pair<Rule>) -> ImplBlock {
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

fn parse_type(pair: Pair<Rule>) -> Type {
    match pair.as_str() {
        "u8" => Type::U8,
        "u16" => Type::U16,
        "u32" => Type::U32,
        "u64" => Type::U64,
        "i8" => Type::I8,
        "i16" => Type::I16,
        "i32" => Type::I32,
        "i64" => Type::I64,
        "bool" => Type::Bool,
        other => Type::Custom(other.to_string()),
    }
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
        let code = "let a = 1 + 2 * 3;";
        let program = build_ast(code).unwrap();

        if let Statement::LetBinding {
            value: Some(expr), ..
        } = &program.statements[0]
        {
            // out: Add
            if let Expression::BinaryExpr { left, op, right } = expr {
                assert_eq!(*op, amber_ast::BinaryOp::Add);

                // left: 1
                if let Expression::Integer(v) = **left {
                    assert_eq!(v, 1);
                } else {
                    panic!("Left should be 1");
                }

                // right: (2 * 3)
                if let Expression::BinaryExpr {
                    left: r_left,
                    op: r_op,
                    right: r_right,
                } = &**right
                {
                    assert_eq!(*r_op, amber_ast::BinaryOp::Mul);
                    //2 and 3...
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
        let code = "let a = (1 + 2) * 3;";
        let program = build_ast(code).unwrap();

        if let Statement::LetBinding {
            value: Some(expr), ..
        } = &program.statements[0]
        {
            if let Expression::BinaryExpr { op, .. } = expr {
                assert_eq!(*op, amber_ast::BinaryOp::Mul);
            } else {
                panic!("Top level should be multiplication");
            }
        }
    }

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
                assert!(func.body.is_some());
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
                fn new(x: i32, y: i32) -> Point {
                    return Point { x: x, y: y };
                }

                fn move(self, dx: i32, dy: i32) {
                    self.x = self.x + dx;
                    self.y = self.y + dy;
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
                assert_eq!(first.return_type, Some(Type::Custom("Point".into())));

                let second = &block.methods[1];
                assert_eq!(second.name, "move");
                assert!(matches!(second.params.first(), Some(Param::SelfParam)));
            }
            _ => panic!("Expected impl block"),
        }
    }
}
