use language::operations::Operation;

pub struct AgentAiSetInteractWithPlayerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2077;

pub const IDENT: &str = "agent_ai_set_interact_with_player";

impl Operation for AgentAiSetInteractWithPlayerOp {
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
