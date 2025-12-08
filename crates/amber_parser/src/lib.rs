pub mod pratt;
pub mod utils;
pub mod expr_parser;
pub mod stmt_parser;
pub mod decl_parser;
pub mod error;

use pest::Parser;
use pest_derive::Parser;

use amber_ast::Program;

pub use error::ParseError;

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

pub fn build_ast_with_name(input: &str, _name: String) -> Result<Program, String> {
    // For now, we ignore the name parameter
    // In the future, this could be used for better error reporting with file names
    build_ast(input)
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> amber_ast::Statement {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::declaration => stmt_parser::parse_declaration(inner),
        Rule::expr_stmt => stmt_parser::parse_expr_stmt(inner),
        Rule::assignment => stmt_parser::parse_assignment(inner),
        Rule::return_stmt => stmt_parser::parse_return(inner),
        Rule::if_stmt => stmt_parser::parse_if_stmt(inner),
        Rule::while_stmt => stmt_parser::parse_while_stmt(inner),
        Rule::struct_def => amber_ast::Statement::Struct(decl_parser::parse_struct(inner)),
        Rule::function_def => amber_ast::Statement::Function(decl_parser::parse_function(inner)),
        Rule::impl_block => amber_ast::Statement::Impl(decl_parser::parse_impl(inner)),
        _ => panic!("TODO: Implement other statements: {:?}", inner.as_rule()),
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
        let code = "let a = 10";
        let result = parse_source(code);
        assert!(result.is_err());
    }
}
