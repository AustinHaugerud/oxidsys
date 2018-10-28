use language::operations::Operation;

pub struct ResetMissionTimerAOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2375;

pub const IDENT: &str = "reset_mission_timer_a";

impl Operation for ResetMissionTimerAOp {
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
