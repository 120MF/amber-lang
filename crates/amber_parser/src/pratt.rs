use lazy_static::lazy_static;
use pest::pratt_parser::{Assoc, Op, PrattParser};

use crate::Rule;

lazy_static! {
    /// Global PrattParser instance for expression parsing with all operators
    /// This avoids recreating the parser on every expression parse, improving performance
    pub static ref EXPR_PARSER: PrattParser<Rule> = {
        PrattParser::new()
            // Logical OR (lowest precedence)
            .op(Op::infix(Rule::or_op, Assoc::Left))
            // Logical AND
            .op(Op::infix(Rule::and_op, Assoc::Left))
            // Bitwise OR
            .op(Op::infix(Rule::bitwise_or, Assoc::Left))
            // Bitwise XOR
            .op(Op::infix(Rule::bitwise_xor, Assoc::Left))
            // Bitwise AND
            .op(Op::infix(Rule::bitwise_and, Assoc::Left))
            // Comparison operators
            .op(Op::infix(Rule::eq_op, Assoc::Left)
                | Op::infix(Rule::ne_op, Assoc::Left)
                | Op::infix(Rule::lt_op, Assoc::Left)
                | Op::infix(Rule::le_op, Assoc::Left)
                | Op::infix(Rule::gt_op, Assoc::Left)
                | Op::infix(Rule::ge_op, Assoc::Left))
            // Shift operators
            .op(Op::infix(Rule::shl_op, Assoc::Left)
                | Op::infix(Rule::shr_op, Assoc::Left))
            // Additive operators
            .op(Op::infix(Rule::add_op, Assoc::Left)
                | Op::infix(Rule::sub_op, Assoc::Left))
            // Multiplicative operators
            .op(Op::infix(Rule::mul_op, Assoc::Left)
                | Op::infix(Rule::div_op, Assoc::Left)
                | Op::infix(Rule::mod_op, Assoc::Left))
            // Unary prefix operators (highest precedence)
            .op(Op::prefix(Rule::prefix_minus)
                | Op::prefix(Rule::prefix_plus)
                | Op::prefix(Rule::prefix_not)
                | Op::prefix(Rule::prefix_bitnot)
                | Op::prefix(Rule::prefix_preinc)
                | Op::prefix(Rule::prefix_predec))
    };
}

/// Convenience function to get a reference to the global EXPR_PARSER
/// Use this instead of creating a new parser each time
pub fn expr_parser() -> &'static PrattParser<Rule> {
    &EXPR_PARSER
}
