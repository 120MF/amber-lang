use amber_ast::{BinaryOp, Expression, Literal, NumericLiteral, Postfix, UnaryOp, Prefix};
pub fn render_expr(expr: &Expression) -> String {
    match expr {
        Expression::Literal(lit) => render_literal(lit),
        Expression::Identifier(ident) => ident.clone(),
        Expression::BinaryExpr { left, op, right } => {
            format!(
                "({} {} {})",
                render_expr(left),
                render_binary_op(op),
                render_expr(right)
            )
        }
        Expression::UnaryExpr { op, expr } => {
            format!("({})", render_unary_op(op, expr))
        }
        Expression::TernaryExpr {
            condition,
            then_expr,
            else_expr,
        } => {
            format!(
                "({} ? {} : {})",
                render_expr(condition),
                render_expr(then_expr),
                render_expr(else_expr)
            )
        }
    }
}

pub fn render_literal(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(num) => render_numeric_literal(num),
        Literal::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        &Literal::Char(c) => c.to_string(),
    }
}

pub fn render_numeric_literal(lit: &NumericLiteral) -> String {
    match lit {
        NumericLiteral::Integer(i) => i.to_string(),
        NumericLiteral::Float(f) => {
            if f.is_finite() {
                format!("{}f", f)
            } else {
                format!("{}f", f)
            }
        }
        NumericLiteral::Double(d) => d.to_string(),
    }
}

pub fn render_unary_op(op: &UnaryOp, expression: &Expression) -> String {
    match op {
        UnaryOp::PrefixOp(pre_op) => {
            let operator = match pre_op {
                Prefix::Neg => "-",
                Prefix::Pos => "+",
                Prefix::Not => "!",
                Prefix::BitNot => "~",
                Prefix::PreInc => "++",
                Prefix::PreDec => "--",
                Prefix::Deref => "*",
            };
            format!("{}{}", operator, render_expr(expression))
        }
        UnaryOp::PostfixOp(post_op) => {
            let operator = match post_op { Postfix::Index { index } => { format!("[{}]", render_expr(index)) } };
            format!("{}{}", render_expr(expression), operator)
        }
    }
}

pub fn render_binary_op(op: &BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Mod => "%",
        BinaryOp::Eq => "==",
        BinaryOp::Ne => "!=",
        BinaryOp::Lt => "<",
        BinaryOp::Le => "<=",
        BinaryOp::Gt => ">",
        BinaryOp::Ge => ">=",
        BinaryOp::BitAnd => "&",
        BinaryOp::BitOr => "|",
        BinaryOp::BitXor => "^",
        BinaryOp::Shl => "<<",
        BinaryOp::Shr => ">>",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
    }
}
