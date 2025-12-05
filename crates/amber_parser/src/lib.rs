use pest::Parser;
use pest_derive::Parser;

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
}
