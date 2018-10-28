use language::operations::Operation;

pub struct SetRiverShaderToMudOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2387;

pub const IDENT: &str = "set_river_shader_to_mud";

impl Operation for SetRiverShaderToMudOp {
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
