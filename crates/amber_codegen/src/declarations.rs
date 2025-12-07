use amber_ast::{Function, ImplBlock, Param, StructDef, StructField};
use crate::buffer::CodeBuffer;
use crate::errors::CodegenError;
use crate::types::type_to_c;
use crate::statements::emit_block;

pub fn emit_struct(buffer: &mut CodeBuffer, def: &StructDef) -> Result<(), CodegenError> {
    buffer.push_line("typedef struct {");
    for field in &def.fields {
        emit_struct_field(buffer, field);
    }
    let line = format!("}} {};", def.name);
    buffer.push_line(&line);
    buffer.push_line("");
    Ok(())
}

pub fn emit_struct_field(buffer: &mut CodeBuffer, field: &StructField) {
    let line = format!("    {} {};", type_to_c(&field.ty), field.name);
    buffer.push_line(&line);
}

pub fn emit_function(
    buffer: &mut CodeBuffer,
    func: &Function,
    impl_target: Option<&str>,
) -> Result<(), CodegenError> {
    let signature = function_signature(func, impl_target)?;

    if func.is_extern {
        if func.body.is_some() {
            return Err(CodegenError::ExternFunctionWithBody {
                name: func.name.clone(),
            });
        }
        buffer.push_line(&format!("extern {};", signature));
        buffer.push_line("");
    } else {
        let body = func
            .body
            .as_ref()
            .ok_or_else(|| CodegenError::MissingFunctionBody {
                name: func.name.clone(),
            })?;
        buffer.push_line(&format!("{} {{", signature));
        emit_block(buffer, body, 1)?;
        buffer.push_line("}");
        buffer.push_line("");
    }
    Ok(())
}

pub fn emit_impl(buffer: &mut CodeBuffer, block: &ImplBlock) -> Result<(), CodegenError> {
    for method in &block.methods {
        if method.is_extern {
            return Err(CodegenError::ExternImplMethod {
                target: block.target.clone(),
                name: method.name.clone(),
            });
        }
        emit_function(buffer, method, Some(&block.target))?;
    }
    Ok(())
}

pub fn function_signature(
    func: &Function,
    impl_target: Option<&str>,
) -> Result<String, CodegenError> {
    let return_type = func
        .return_type
        .as_ref()
        .map(type_to_c)
        .unwrap_or_else(|| "void".to_string());
    let func_name = if let Some(target) = impl_target {
        format!("{}_{}", target, func.name)
    } else {
        func.name.clone()
    };
    let params = format_params(&func.params, impl_target)?;
    Ok(format!("{} {}({})", return_type, func_name, params))
}

pub fn format_params(
    params: &[Param],
    impl_target: Option<&str>,
) -> Result<String, CodegenError> {
    let mut parts = Vec::new();
    let mut self_count = 0;

    for param in params {
        match param {
            Param::SelfParam => {
                self_count += 1;
                if self_count > 1 {
                    return Err(CodegenError::MultipleSelfParams {
                        name: "unknown".to_string(),
                    });
                }
                if impl_target.is_none() {
                    return Err(CodegenError::SelfParamOutsideImpl {
                        name: "unknown".to_string(),
                    });
                }
                // Self param becomes Target* self
                if let Some(target) = impl_target {
                    parts.push(format!("{}* self", target));
                }
            }
            Param::Typed { name, ty } => {
                let param_type = type_to_c(ty);
                parts.push(format!("{} {}", param_type, name));
            }
        }
    }

    if parts.is_empty() {
        Ok("void".to_string())
    } else {
        Ok(parts.join(", "))
    }
}
