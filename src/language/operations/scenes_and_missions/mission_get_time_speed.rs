use language::operations::Operation;

pub struct MissionGetTimeSpeedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2002;

pub const IDENT: &str = "mission_get_time_speed";

impl Operation for MissionGetTimeSpeedOp {
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
