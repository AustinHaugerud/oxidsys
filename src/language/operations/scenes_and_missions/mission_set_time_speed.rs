use language::operations::Operation;

pub struct MissionSetTimeSpeedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2003;

pub const IDENT: &str = "mission_set_time_speed";

impl Operation for MissionSetTimeSpeedOp {
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
