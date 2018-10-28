use language::operations::Operation;

pub struct PositionGetDistanceToGroundLevelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 793;

pub const IDENT: &str = "position_get_distance_to_ground_level";

impl Operation for PositionGetDistanceToGroundLevelOp {
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
