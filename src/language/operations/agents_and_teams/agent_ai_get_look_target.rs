use language::operations::Operation;

pub struct AgentAiGetLookTargetOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2080;

pub const IDENT: &str = "agent_ai_get_look_target";

impl Operation for AgentAiGetLookTargetOp {
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
