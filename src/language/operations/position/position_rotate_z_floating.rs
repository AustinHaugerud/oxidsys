use language::operations::Operation;

pub struct PositionRotateZFloatingOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 734;

pub const IDENT: &str = "position_rotate_z_floating";

impl Operation for PositionRotateZFloatingOp {
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
