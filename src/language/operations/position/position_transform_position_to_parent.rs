use language::operations::Operation;

pub struct PositionTransformPositionToParentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 716;

pub const IDENT: &str = "position_transform_position_to_parent";

impl Operation for PositionTransformPositionToParentOp {
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
