mod numeric;

pub use numeric::NumericLiteral;

use std::fmt;

/// Represents all literal values in the language
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// Numeric literals: integers and floating-point numbers
    Numeric(NumericLiteral),
    /// Boolean literals: true or false
    Bool(bool),
    // Future:
    // String(String),
    // Array(Vec<Literal>),
}

impl Literal {
    /// Get the inferred type of this literal
    pub fn inferred_type(&self) -> &'static str {
        match self {
            Literal::Numeric(num) => num.inferred_type(),
            Literal::Bool(_) => "bool",
        }
    }

    /// Check if this is a numeric literal
    pub fn is_numeric(&self) -> bool {
        matches!(self, Literal::Numeric(_))
    }

    /// Check if this is a boolean literal
    pub fn is_bool(&self) -> bool {
        matches!(self, Literal::Bool(_))
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Numeric(num) => write!(f, "{}", num),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_numeric() {
        let lit = Literal::Numeric(NumericLiteral::Integer(42));
        assert!(lit.is_numeric());
        assert!(!lit.is_bool());
        assert_eq!(lit.inferred_type(), "i64");
    }

    #[test]
    fn test_literal_bool() {
        let lit_true = Literal::Bool(true);
        let lit_false = Literal::Bool(false);
        
        assert!(!lit_true.is_numeric());
        assert!(lit_true.is_bool());
        assert_eq!(lit_true.inferred_type(), "bool");
        assert_eq!(lit_false.to_string(), "false");
    }

    #[test]
    fn test_literal_display() {
        let int_lit = Literal::Numeric(NumericLiteral::Integer(42));
        let bool_lit = Literal::Bool(true);
        
        assert_eq!(int_lit.to_string(), "42");
        assert_eq!(bool_lit.to_string(), "true");
    }
}
