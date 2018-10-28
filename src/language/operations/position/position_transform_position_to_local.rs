use language::operations::Operation;

pub struct PositionTransformPositionToLocalOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 717;

pub const IDENT: &str = "position_transform_position_to_local";

impl Operation for PositionTransformPositionToLocalOp {
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
