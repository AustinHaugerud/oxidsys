use language::operations::Operation;

pub struct PositionRotateXFloatingOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 738;

pub const IDENT: &str = "position_rotate_x_floating";

impl Operation for PositionRotateXFloatingOp {
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
