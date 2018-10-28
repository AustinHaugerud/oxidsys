use language::operations::Operation;

pub struct AgentGetTimeElapsedSinceRemovedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1760;

pub const IDENT: &str = "agent_get_time_elapsed_since_removed";

impl Operation for AgentGetTimeElapsedSinceRemovedOp {
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
