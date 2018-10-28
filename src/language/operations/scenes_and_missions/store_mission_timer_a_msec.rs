use language::operations::Operation;

pub struct StoreMissionTimerAMsecOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2365;

pub const IDENT: &str = "store_mission_timer_a_msec";

impl Operation for StoreMissionTimerAMsecOp {
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
