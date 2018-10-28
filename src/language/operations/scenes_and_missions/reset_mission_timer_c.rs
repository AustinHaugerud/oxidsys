use language::operations::Operation;

pub struct ResetMissionTimerCOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2377;

pub const IDENT: &str = "reset_mission_timer_c";

impl Operation for ResetMissionTimerCOp {
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
