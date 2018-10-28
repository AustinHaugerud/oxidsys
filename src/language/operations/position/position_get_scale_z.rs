use language::operations::Operation;

pub struct PositionGetScaleZOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 737;

pub const IDENT: &str = "position_get_scale_z";

impl Operation for PositionGetScaleZOp {
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
