use language::operations::Operation;

pub struct StoreMissionTimerCMsecOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2367;

pub const IDENT: &str = "store_mission_timer_c_msec";

impl Operation for StoreMissionTimerCMsecOp {
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
