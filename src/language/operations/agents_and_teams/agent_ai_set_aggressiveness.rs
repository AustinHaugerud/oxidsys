use language::operations::Operation;

pub struct AgentAiSetAggressivenessOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1753;

pub const IDENT: &str = "agent_ai_set_aggressiveness";

impl Operation for AgentAiSetAggressivenessOp {
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
