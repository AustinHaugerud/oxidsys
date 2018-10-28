use language::operations::Operation;

pub struct PositionGetScreenProjectionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 750;

pub const IDENT: &str = "position_get_screen_projection";

impl Operation for PositionGetScreenProjectionOp {
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
