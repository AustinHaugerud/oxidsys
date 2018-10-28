use language::operations::Operation;

pub struct AgentStopSoundOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1808;

pub const IDENT: &str = "agent_stop_sound";

impl Operation for AgentStopSoundOp {
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
