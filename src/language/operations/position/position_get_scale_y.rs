use language::operations::Operation;

pub struct PositionGetScaleYOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 736;

pub const IDENT: &str = "position_get_scale_y";

impl Operation for PositionGetScaleYOp {
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
