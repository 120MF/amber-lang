use amber_ast::{Block, Expression, Modifier, Statement, Type};
use crate::buffer::CodeBuffer;
use crate::errors::CodegenError;
use crate::expression::render_expr;
use crate::types::{binding_qualifier, type_to_c};

pub fn emit_program(buffer: &mut CodeBuffer, program: &amber_ast::Program) -> Result<(), CodegenError> {
    for statement in &program.statements {
        emit_statement(buffer, statement)?;
    }
    Ok(())
}

pub fn emit_statement(buffer: &mut CodeBuffer, statement: &Statement) -> Result<(), CodegenError> {
    match statement {
        Statement::LetBinding(lb) => emit_let_binding(
            buffer,
            lb.modifier.clone(),
            lb.is_mutable,
            &lb.name,
            lb.ty.as_ref(),
            lb.value.as_ref(),
        ),
        Statement::ExprStatement(expr) => emit_expr_statement(buffer, expr),
        Statement::Struct(def) => crate::declarations::emit_struct(buffer, def),
        Statement::Function(func) => crate::declarations::emit_function(buffer, func, None),
        Statement::Impl(block) => crate::declarations::emit_impl(buffer, block),
        Statement::IfElse(_) | Statement::WhileLoop(_) => {
            panic!("unexpected statement at top level: should be inside block")
        }
        Statement::Assignment { .. } | Statement::Return(_) => {
            panic!("unexpected statement at top level: {:?}", statement)
        }
    }
}

pub fn emit_let_binding(
    buffer: &mut CodeBuffer,
    modifier: Option<Modifier>,
    is_mutable: bool,
    name: &str,
    ty: Option<&Type>,
    value: Option<&Expression>,
) -> Result<(), CodegenError> {
    let line = render_let_binding_line(modifier, is_mutable, name, ty, value)?;
    buffer.push_line(&line);
    buffer.push_line("");
    Ok(())
}

pub fn render_let_binding_line(
    modifier: Option<Modifier>,
    is_mutable: bool,
    name: &str,
    ty: Option<&Type>,
    value: Option<&Expression>,
) -> Result<String, CodegenError> {
    let ty = ty.ok_or_else(|| CodegenError::MissingType {
        name: name.to_string(),
    })?;
    let qualifier = binding_qualifier(modifier, is_mutable);
    let mut line = format!("{}{} {}", qualifier, type_to_c(ty), name);
    if let Some(expr) = value {
        line.push_str(" = ");
        line.push_str(&render_expr(expr));
    }
    line.push(';');
    Ok(line)
}

pub fn emit_expr_statement(
    buffer: &mut CodeBuffer,
    expr: &Expression,
) -> Result<(), CodegenError> {
    let line = render_expr_statement_line(expr);
    buffer.push_line(&line);
    buffer.push_line("");
    Ok(())
}

pub fn render_expr_statement_line(expr: &Expression) -> String {
    format!("{};", render_expr(expr))
}

pub fn emit_block(buffer: &mut CodeBuffer, block: &Block, indent: usize) -> Result<(), CodegenError> {
    for statement in &block.statements {
        emit_block_statement(buffer, statement, indent)?;
    }
    Ok(())
}

pub fn emit_block_statement(
    buffer: &mut CodeBuffer,
    statement: &Statement,
    indent: usize,
) -> Result<(), CodegenError> {
    match statement {
        Statement::LetBinding(lb) => {
            let line = render_let_binding_line(
                lb.modifier.clone(),
                lb.is_mutable,
                &lb.name,
                lb.ty.as_ref(),
                lb.value.as_ref(),
            )?;
            buffer.push_indented_line(indent, &line);
            Ok(())
        }
        Statement::ExprStatement(expr) => {
            let line = render_expr_statement_line(expr);
            buffer.push_indented_line(indent, &line);
            Ok(())
        }
        Statement::Assignment { target, value } => {
            let line = format!("{} = {};", target, render_expr(value));
            buffer.push_indented_line(indent, &line);
            Ok(())
        }
        Statement::Return(expr) => {
            if let Some(e) = expr {
                let line = format!("return {};", render_expr(e));
                buffer.push_indented_line(indent, &line);
            } else {
                buffer.push_indented_line(indent, "return;");
            }
            Ok(())
        }
        Statement::IfElse(if_stmt) => {
            let cond_str = render_expr(&if_stmt.condition);
            buffer.push_indented_line(indent, &format!("if ({}) {{", cond_str));
            emit_block(buffer, &if_stmt.then_block, indent + 1)?;
            if let Some(else_block) = &if_stmt.else_block {
                buffer.push_indented_line(indent, "} else {");
                emit_block(buffer, else_block, indent + 1)?;
            }
            buffer.push_indented_line(indent, "}");
            Ok(())
        }
        Statement::WhileLoop(while_stmt) => {
            let cond_str = render_expr(&while_stmt.condition);
            buffer.push_indented_line(indent, &format!("while ({}) {{", cond_str));
            emit_block(buffer, &while_stmt.block, indent + 1)?;
            buffer.push_indented_line(indent, "}");
            Ok(())
        }
        _ => panic!("Unexpected block statement: {:?}", statement),
    }
}
