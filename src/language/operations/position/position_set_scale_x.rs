use language::operations::Operation;

pub struct PositionSetScaleXOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 744;

pub const IDENT: &str = "position_set_scale_x";

impl Operation for PositionSetScaleXOp {
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
