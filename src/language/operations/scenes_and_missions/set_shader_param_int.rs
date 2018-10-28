use language::operations::Operation;

pub struct SetShaderParamIntOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2400;

pub const IDENT: &str = "set_shader_param_int";

impl Operation for SetShaderParamIntOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
