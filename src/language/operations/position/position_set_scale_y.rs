use language::operations::Operation;

pub struct PositionSetScaleYOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 745;

pub const IDENT: &str = "position_set_scale_y";

impl Operation for PositionSetScaleYOp {
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
