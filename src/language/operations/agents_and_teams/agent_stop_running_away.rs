use language::operations::Operation;

pub struct AgentStopRunningAwayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1752;

pub const IDENT: &str = "agent_stop_running_away";

impl Operation for AgentStopRunningAwayOp {
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
