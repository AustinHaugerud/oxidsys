use language::operations::Operation;

pub struct PositionGetScaleXOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 735;

pub const IDENT: &str = "position_get_scale_x";

impl Operation for PositionGetScaleXOp {
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
