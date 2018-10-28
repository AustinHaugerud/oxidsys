use language::operations::Operation;

pub struct AgentAiGetNumCachedEnemiesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2670;

pub const IDENT: &str = "agent_ai_get_num_cached_enemies";

impl Operation for AgentAiGetNumCachedEnemiesOp {
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
