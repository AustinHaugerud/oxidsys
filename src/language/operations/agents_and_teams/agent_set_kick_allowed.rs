use language::operations::Operation;

pub struct AgentSetKickAllowedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1754;

pub const IDENT: &str = "agent_set_kick_allowed";

impl Operation for AgentSetKickAllowedOp {
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
