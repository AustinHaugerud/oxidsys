use language::operations::Operation;

pub struct MissionTimeSpeedMoveToValueOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2004;

pub const IDENT: &str = "mission_time_speed_move_to_value";

impl Operation for MissionTimeSpeedMoveToValueOp {
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
