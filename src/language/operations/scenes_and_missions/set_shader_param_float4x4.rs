use language::operations::Operation;

pub struct SetShaderParamFloat4x4Op;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2403;

pub const IDENT: &str = "set_shader_param_float4x4";

impl Operation for SetShaderParamFloat4x4Op {
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
