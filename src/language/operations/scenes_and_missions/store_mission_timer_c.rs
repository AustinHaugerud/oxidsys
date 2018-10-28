use language::operations::Operation;

pub struct StoreMissionTimerCOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2372;

pub const IDENT: &str = "store_mission_timer_c";

impl Operation for StoreMissionTimerCOp {
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
