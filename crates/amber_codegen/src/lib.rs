use amber_ast::{
    BinaryOp, Expression, Function, ImplBlock, Modifier, Param, Program, Statement, StructDef,
    StructField, Type,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CodegenError {
    #[error("let binding '{name}' requires an explicit type")]
    MissingType { name: String },
    #[error("function '{name}' is missing a body")]
    MissingFunctionBody { name: String },
    #[error("extern function '{name}' cannot have a body")]
    ExternFunctionWithBody { name: String },
    #[error("`self` parameter is only allowed inside impl blocks (function '{name}')")]
    SelfParamOutsideImpl { name: String },
    #[error("function '{name}' contains multiple `self` parameters")]
    MultipleSelfParams { name: String },
    #[error("impl method '{target}::{name}' cannot be declared extern")]
    ExternImplMethod { target: String, name: String },
}

pub fn generate_program(program: &Program) -> Result<String, CodegenError> {
    let mut generator = CodeGenerator::default();
    generator.emit_program(program)?;
    Ok(generator.finish())
}

#[derive(Default)]
struct CodeGenerator {
    buffer: String,
}

impl CodeGenerator {
    fn finish(self) -> String {
        format!(
            "#include <stdint.h>\n#include <stdbool.h>\n\n{}",
            self.buffer
        )
    }

    fn emit_program(&mut self, program: &Program) -> Result<(), CodegenError> {
        for statement in &program.statements {
            self.emit_statement(statement)?;
        }
        Ok(())
    }

    fn emit_statement(&mut self, statement: &Statement) -> Result<(), CodegenError> {
        match statement {
            Statement::LetBinding {
                modifier,
                is_mutable,
                name,
                ty,
                value,
            } => self.emit_let_binding(modifier.clone(), *is_mutable, name, ty.as_ref(), value.as_ref()),
            Statement::ExprStatement(expr) => self.emit_expr_statement(expr),
            Statement::Struct(def) => self.emit_struct(def),
            Statement::Function(func) => self.emit_function(func, None),
            Statement::Impl(block) => self.emit_impl(block),
        }
    }

    fn emit_let_binding(
        &mut self,
        modifier: Option<Modifier>,
        is_mutable: bool,
        name: &str,
        ty: Option<&Type>,
        value: Option<&Expression>,
    ) -> Result<(), CodegenError> {
        let ty = ty.ok_or_else(|| CodegenError::MissingType {
            name: name.to_string(),
        })?;
        let qualifier = self.binding_qualifier(modifier, is_mutable);
        let mut line = format!("{}{} {}", qualifier, self.type_to_c(ty), name);
        if let Some(expr) = value {
            line.push_str(" = ");
            line.push_str(&self.render_expr(expr));
        }
        line.push(';');
        self.push_line(&line);
        self.push_line("");
        Ok(())
    }

    fn emit_expr_statement(&mut self, expr: &Expression) -> Result<(), CodegenError> {
        let line = format!("{};", self.render_expr(expr));
        self.push_line(&line);
        self.push_line("");
        Ok(())
    }

    fn emit_struct(&mut self, def: &StructDef) -> Result<(), CodegenError> {
        self.push_line("typedef struct {");
        for field in &def.fields {
            self.emit_struct_field(field);
        }
        self.push_line(&format!("}} {};", def.name));
        self.push_line("");
        Ok(())
    }

    fn emit_struct_field(&mut self, field: &StructField) {
        let line = format!("    {} {};", self.type_to_c(&field.ty), field.name);
        self.push_line(&line);
    }

    fn emit_function(
        &mut self,
        func: &Function,
        impl_target: Option<&str>,
    ) -> Result<(), CodegenError> {
        if func.is_extern {
            if func.body.is_some() {
                return Err(CodegenError::ExternFunctionWithBody {
                    name: func.name.clone(),
                });
            }
            let signature =
                self.function_signature(&func.name, func.return_type.as_ref(), &func.params, None)?;
            self.push_line(&format!("extern {};", signature));
            self.push_line("");
            return Ok(());
        }

        let body = func
            .body
            .as_ref()
            .ok_or_else(|| CodegenError::MissingFunctionBody {
                name: func.name.clone(),
            })?;
        let signature = self.function_signature(
            &func.name,
            func.return_type.as_ref(),
            &func.params,
            impl_target,
        )?;
        self.push_line(&format!("{} {}", signature, body));
        self.push_line("");
        Ok(())
    }

    fn emit_impl(&mut self, block: &ImplBlock) -> Result<(), CodegenError> {
        for method in &block.methods {
            if method.is_extern {
                return Err(CodegenError::ExternImplMethod {
                    target: block.target.clone(),
                    name: method.name.clone(),
                });
            }
            let name = format!("{}_{}", block.target, method.name);
            let body = method
                .body
                .as_ref()
                .ok_or_else(|| CodegenError::MissingFunctionBody {
                    name: method.name.clone(),
                })?;
            let signature =
                self.function_signature(&name, method.return_type.as_ref(), &method.params, Some(&block.target))?;
            self.push_line(&format!("{} {}", signature, body));
            self.push_line("");
        }
        Ok(())
    }

    fn function_signature(
        &self,
        name: &str,
        return_type: Option<&Type>,
        params: &[Param],
        impl_target: Option<&str>,
    ) -> Result<String, CodegenError> {
        let ret = self.type_to_c_opt(return_type);
        let params = self.format_params(params, impl_target, name)?;
        Ok(format!("{} {}({})", ret, name, params))
    }

    fn format_params(
        &self,
        params: &[Param],
        impl_target: Option<&str>,
        func_name: &str,
    ) -> Result<String, CodegenError> {
        let mut parts = Vec::new();
        let mut self_seen = false;

        for param in params {
            match param {
                Param::SelfParam => {
                    let target = impl_target.ok_or_else(|| CodegenError::SelfParamOutsideImpl {
                        name: func_name.to_string(),
                    })?;
                    if self_seen {
                        return Err(CodegenError::MultipleSelfParams {
                            name: func_name.to_string(),
                        });
                    }
                    self_seen = true;
                    parts.push(format!("{}* self", target));
                }
                Param::Typed { name, ty } => {
                    parts.push(format!("{} {}", self.type_to_c(ty), name));
                }
            }
        }

        if parts.is_empty() {
            Ok("void".to_string())
        } else {
            Ok(parts.join(", "))
        }
    }

    fn binding_qualifier(&self, modifier: Option<Modifier>, is_mutable: bool) -> String {
        let mut flags = Vec::new();
        if matches!(modifier, Some(Modifier::Comptime)) {
            flags.push("const");
        }
        if !is_mutable && !flags.contains(&"const") {
            flags.push("const");
        }
        if flags.is_empty() {
            String::new()
        } else {
            format!("{} ", flags.join(" "))
        }
    }

    fn type_to_c(&self, ty: &Type) -> String {
        match ty {
            Type::U8 => "uint8_t".into(),
            Type::U16 => "uint16_t".into(),
            Type::U32 => "uint32_t".into(),
            Type::U64 => "uint64_t".into(),
            Type::I8 => "int8_t".into(),
            Type::I16 => "int16_t".into(),
            Type::I32 => "int32_t".into(),
            Type::I64 => "int64_t".into(),
            Type::Bool => "bool".into(),
            Type::Custom(name) => name.clone(),
        }
    }

    fn type_to_c_opt(&self, ty: Option<&Type>) -> String {
        ty.map(|t| self.type_to_c(t)).unwrap_or_else(|| "void".into())
    }

    fn render_expr(&self, expr: &Expression) -> String {
        match expr {
            Expression::Integer(value) => value.to_string(),
            Expression::Identifier(ident) => ident.clone(),
            Expression::BinaryExpr { left, op, right } => {
                format!(
                    "({} {} {})",
                    self.render_expr(left),
                    self.render_binary_op(op),
                    self.render_expr(right)
                )
            }
        }
    }

    fn render_binary_op(&self, op: &BinaryOp) -> &'static str {
        match op {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
        }
    }

    fn push_line(&mut self, line: &str) {
        self.buffer.push_str(line);
        self.buffer.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_structs_functions_and_impls() {
        let program = Program {
            statements: vec![
                Statement::Struct(StructDef {
                    name: "Point".into(),
                    fields: vec![
                        StructField {
                            name: "x".into(),
                            ty: Type::I32,
                        },
                        StructField {
                            name: "y".into(),
                            ty: Type::I32,
                        },
                    ],
                }),
                Statement::Function(amber_ast::Function {
                    name: "add".into(),
                    params: vec![
                        Param::Typed {
                            name: "a".into(),
                            ty: Type::I32,
                        },
                        Param::Typed {
                            name: "b".into(),
                            ty: Type::I32,
                        },
                    ],
                    return_type: Some(Type::I32),
                    body: Some("{\n    return a + b;\n}".into()),
                    is_extern: false,
                }),
                Statement::Function(amber_ast::Function {
                    name: "HAL_Delay".into(),
                    params: vec![Param::Typed {
                        name: "ms".into(),
                        ty: Type::U32,
                    }],
                    return_type: None,
                    body: None,
                    is_extern: true,
                }),
                Statement::Impl(ImplBlock {
                    target: "Point".into(),
                    methods: vec![
                        amber_ast::Function {
                            name: "new".into(),
                            params: vec![
                                Param::Typed {
                                    name: "x".into(),
                                    ty: Type::I32,
                                },
                                Param::Typed {
                                    name: "y".into(),
                                    ty: Type::I32,
                                },
                            ],
                            return_type: Some(Type::Custom("Point".into())),
                            body: Some("{\n    return Point { x: x, y: y };\n}".into()),
                            is_extern: false,
                        },
                        amber_ast::Function {
                            name: "move".into(),
                            params: vec![
                                Param::SelfParam,
                                Param::Typed {
                                    name: "dx".into(),
                                    ty: Type::I32,
                                },
                                Param::Typed {
                                    name: "dy".into(),
                                    ty: Type::I32,
                                },
                            ],
                            return_type: None,
                            body: Some(
                                "{\n    self.x = self.x + dx;\n    self.y = self.y + dy;\n}"
                                    .into(),
                            ),
                            is_extern: false,
                        },
                    ],
                }),
                Statement::LetBinding {
                    modifier: Some(Modifier::Comptime),
                    is_mutable: false,
                    name: "BAUD".into(),
                    ty: Some(Type::I32),
                    value: Some(Expression::Integer(9600)),
                },
            ],
        };

        let output = generate_program(&program).unwrap();
        let expected = r#"#include <stdint.h>
#include <stdbool.h>

typedef struct {
    int32_t x;
    int32_t y;
} Point;

int32_t add(int32_t a, int32_t b) {
    return a + b;
}

extern void HAL_Delay(uint32_t ms);

Point Point_new(int32_t x, int32_t y) {
    return Point { x: x, y: y };
}

void Point_move(Point* self, int32_t dx, int32_t dy) {
    self.x = self.x + dx;
    self.y = self.y + dy;
}

const int32_t BAUD = 9600;

"#;

        assert_eq!(output, expected);
    }
}
