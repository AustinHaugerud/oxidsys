use language::operations::Operation;

pub struct AgentSetLookTargetAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1713;

pub const IDENT: &str = "agent_set_look_target_agent";

impl Operation for AgentSetLookTargetAgentOp {
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
