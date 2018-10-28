use language::operations::Operation;

pub struct PositionSetScaleZOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 746;

pub const IDENT: &str = "position_set_scale_z";

impl Operation for PositionSetScaleZOp {
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
