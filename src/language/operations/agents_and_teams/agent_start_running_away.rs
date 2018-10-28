use language::operations::Operation;

pub struct AgentStartRunningAwayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1751;

pub const IDENT: &str = "agent_start_running_away";

impl Operation for AgentStartRunningAwayOp {
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
