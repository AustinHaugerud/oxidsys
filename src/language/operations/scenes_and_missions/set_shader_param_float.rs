use language::operations::Operation;

pub struct SetShaderParamFloatOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2401;

pub const IDENT: &str = "set_shader_param_float";

impl Operation for SetShaderParamFloatOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
