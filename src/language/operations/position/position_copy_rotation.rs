use language::operations::Operation;

pub struct PositionCopyRotationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 718;

pub const IDENT: &str = "position_copy_rotation";

impl Operation for PositionCopyRotationOp {
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
