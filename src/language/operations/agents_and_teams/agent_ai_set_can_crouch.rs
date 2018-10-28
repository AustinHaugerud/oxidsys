use language::operations::Operation;

pub struct AgentAiSetCanCrouchOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2083;

pub const IDENT: &str = "agent_ai_set_can_crouch";

impl Operation for AgentAiSetCanCrouchOp {
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
