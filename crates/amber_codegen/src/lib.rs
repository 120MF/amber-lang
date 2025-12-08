mod buffer;
mod declarations;
mod errors;
mod expression;
mod statements;
mod types;

pub use errors::CodegenError;

use amber_ast::Program;
use buffer::CodeBuffer;

/// Generate C code from an Amber AST program
pub fn generate_program(program: &Program) -> Result<String, CodegenError> {
    let mut buffer = CodeBuffer::default();
    statements::emit_program(&mut buffer, program)?;
    Ok(buffer.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use amber_ast::{
        Block, Function, ImplBlock, Literal, Modifier, NumericLiteral, Param, Program,
        Statement, StructDef, StructField, Type,
    };

    fn return_block(expr: amber_ast::Expression) -> Block {
        Block {
            statements: vec![Statement::Return(Some(expr))],
        }
    }

    #[test]
    fn generates_structs_functions_and_impls() {
        let program = Program {
            statements: vec![
                Statement::Struct(StructDef {
                    name: "Point".to_string(),
                    fields: vec![
                        StructField {
                            name: "x".to_string(),
                            ty: Type::I32,
                        },
                        StructField {
                            name: "y".to_string(),
                            ty: Type::I32,
                        },
                    ],
                }),
                Statement::Function(Function {
                    name: "add".to_string(),
                    params: vec![
                        Param::Typed {
                            name: "a".to_string(),
                            ty: Type::I32,
                        },
                        Param::Typed {
                            name: "b".to_string(),
                            ty: Type::I32,
                        },
                    ],
                    return_type: Some(Type::I32),
                    is_extern: false,
                    body: Some(return_block(amber_ast::Expression::BinaryExpr {
                        left: Box::new(amber_ast::Expression::Identifier("a".to_string())),
                        op: amber_ast::BinaryOp::Add,
                        right: Box::new(amber_ast::Expression::Identifier("b".to_string())),
                    })),
                }),
                Statement::Function(Function {
                    name: "HAL_Delay".to_string(),
                    params: vec![Param::Typed {
                        name: "ms".to_string(),
                        ty: Type::U32,
                    }],
                    return_type: None,
                    is_extern: true,
                    body: None,
                }),
                Statement::Impl(ImplBlock {
                    target: "Point".to_string(),
                    methods: vec![
                        Function {
                            name: "sum".to_string(),
                            params: vec![
                                Param::SelfParam,
                                Param::Typed {
                                    name: "x".to_string(),
                                    ty: Type::I32,
                                },
                                Param::Typed {
                                    name: "y".to_string(),
                                    ty: Type::I32,
                                },
                            ],
                            return_type: Some(Type::I32),
                            is_extern: false,
                            body: Some(return_block(amber_ast::Expression::BinaryExpr {
                                left: Box::new(amber_ast::Expression::Identifier("x".to_string())),
                                op: amber_ast::BinaryOp::Add,
                                right: Box::new(amber_ast::Expression::Identifier("y".to_string())),
                            })),
                        },
                        Function {
                            name: "reset".to_string(),
                            params: vec![Param::SelfParam],
                            return_type: None,
                            is_extern: false,
                            body: Some(Block {
                                statements: vec![Statement::Return(None)],
                            }),
                        },
                    ],
                }),
                Statement::Binding(amber_ast::VariableBinding {
                    modifier: Some(Modifier::Comptime),
                    is_mutable: false,
                    name: "BAUD".to_string(),
                    ty: Some(Type::I32),
                    value: Some(amber_ast::Expression::Literal(Literal::Numeric(
                        NumericLiteral::Integer(9600),
                    ))),
                }),
            ],
        };

        let output = generate_program(&program).unwrap();

        let expected = "#include <stdint.h>\n#include <stdbool.h>\n\ntypedef struct {\n    int32_t x;\n    int32_t y;\n} Point;\n\nint32_t add(int32_t a, int32_t b) {\n    return (a + b);\n}\n\nextern void HAL_Delay(uint32_t ms);\n\nint32_t Point_sum(Point* self, int32_t x, int32_t y) {\n    return (x + y);\n}\n\nvoid Point_reset(Point* self) {\n    return;\n}\n\nconst int32_t BAUD = 9600;\n\n";

        assert_eq!(output, expected);
    }
}
