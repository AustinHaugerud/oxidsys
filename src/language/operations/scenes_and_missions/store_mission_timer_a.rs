use language::operations::Operation;

pub struct StoreMissionTimerAOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2370;

pub const IDENT: &str = "store_mission_timer_a";

impl Operation for StoreMissionTimerAOp {
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
