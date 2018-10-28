use language::operations::Operation;

pub struct PositionRotateYFloatingOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 739;

pub const IDENT: &str = "position_rotate_y_floating";

impl Operation for PositionRotateYFloatingOp {
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
