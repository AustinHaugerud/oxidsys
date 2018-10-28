use language::operations::Operation;

pub struct PositionGetRotationAroundXOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 742;

pub const IDENT: &str = "position_get_rotation_around_x";

impl Operation for PositionGetRotationAroundXOp {
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
