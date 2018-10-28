use language::operations::Operation;

pub struct SetShaderParamFloat4Op;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2402;

pub const IDENT: &str = "set_shader_param_float4";

impl Operation for SetShaderParamFloat4Op {
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
