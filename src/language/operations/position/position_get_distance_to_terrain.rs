use language::operations::Operation;

pub struct PositionGetDistanceToTerrainOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 792;

pub const IDENT: &str = "position_get_distance_to_terrain";

impl Operation for PositionGetDistanceToTerrainOp {
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
