use pest::iterators::Pair;

use amber_ast::Type;

use crate::Rule;

/// Parse a type from a grammar pair
pub fn parse_type(pair: Pair<Rule>) -> Type {
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
        "char" => Type::Char,
        "void" => Type::Void,
        other => Type::Custom(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_builtin_types() {
        use crate::AmberParser;
        use pest::Parser;

        let result = AmberParser::parse(Rule::builtin_type, "u32");
        assert!(result.is_ok());

        let pair = result.unwrap().next().unwrap();
        let ty = parse_type(pair);
        assert_eq!(ty, Type::U32);
    }
}
