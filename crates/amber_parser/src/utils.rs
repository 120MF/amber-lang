use pest::iterators::Pair;

use amber_ast::Type;

use crate::Rule;

/// Parse a type from a grammar pair
pub fn parse_type(pair: Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::type_def => {
            let inner = pair
                .into_inner()
                .next()
                .expect("type_def must contain inner");
            parse_type(inner)
        }
        Rule::ptr_type => {
            let mutable = pair.as_str().contains("mut");
            let inner_pair = pair
                .into_inner()
                .find(|p| p.as_rule()!= Rule::kw_mut)
                .expect("ptr_type must contain inner");
            let inner_type = parse_type(inner_pair);
            Type::Pointer {
                is_mut: mutable,
                inner: Box::new(inner_type),
            }
        }
        Rule::builtin_type => match pair.as_str() {
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
            other => Type::Named(other.to_string()),
        },
        Rule::ident => Type::Named(pair.as_str().to_string()),
        _ => panic!("Unexpected type rule: {:?}", pair.as_rule()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AmberParser;
    use pest::Parser;
    #[test]
    fn test_parse_pointer_types() {
        let result = AmberParser::parse(Rule::ptr_type, "*mut u32");
        assert!(result.is_ok());
        let pair = result.unwrap().next().unwrap();
        let ty = parse_type(pair);
        assert_eq!(
            ty,
            Type::Pointer {
                is_mut: true,
                inner: Box::from(Type::U32),
            }
        )
    }
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
