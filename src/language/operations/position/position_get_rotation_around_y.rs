use language::operations::Operation;

pub struct PositionGetRotationAroundYOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 743;

pub const IDENT: &str = "position_get_rotation_around_y";

impl Operation for PositionGetRotationAroundYOp {
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
