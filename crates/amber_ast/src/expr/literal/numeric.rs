use std::fmt;

/// Represents numeric literals: integers and floating-point numbers
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum NumericLiteral {
    Integer(i64),
    Float(f32),
    Double(f64),
}

impl NumericLiteral {
    pub fn to_i64(&self) -> i64 {
        match self {
            NumericLiteral::Integer(i) => *i,
            NumericLiteral::Float(f) => *f as i64,
            NumericLiteral::Double(d) => *d as i64,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            NumericLiteral::Integer(i) => *i as f64,
            NumericLiteral::Float(f) => *f as f64,
            NumericLiteral::Double(d) => *d,
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, NumericLiteral::Integer(_))
    }

    /// Get the inferred C type for this literal
    pub fn inferred_type(&self) -> &'static str {
        match self {
            NumericLiteral::Integer(_) => "i64",
            NumericLiteral::Float(_) => "f32",
            NumericLiteral::Double(_) => "f64",
        }
    }
}

impl fmt::Display for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumericLiteral::Integer(i) => write!(f, "{}", i),
            NumericLiteral::Float(fl) => write!(f, "{}f", fl),
            NumericLiteral::Double(d) => write!(f, "{}d", d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_literal_conversions() {
        let int_lit = NumericLiteral::Integer(42);
        assert_eq!(int_lit.to_i64(), 42);
        assert_eq!(int_lit.to_f64(), 42.0);
        assert!(int_lit.is_integer());

        let float_lit = NumericLiteral::Float(3.14);
        assert!(!float_lit.is_integer());
        assert_eq!(float_lit.inferred_type(), "f32");

        let double_lit = NumericLiteral::Double(2.71828);
        assert_eq!(double_lit.inferred_type(), "f64");
    }

    #[test]
    fn test_numeric_literal_display() {
        assert_eq!(NumericLiteral::Integer(42).to_string(), "42");
        assert_eq!(NumericLiteral::Float(3.14).to_string(), "3.14f");
        assert_eq!(NumericLiteral::Double(2.71828).to_string(), "2.71828d");
    }
}
