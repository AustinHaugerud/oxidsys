use language::operations::Operation;

pub struct AgentAiGetCachedEnemyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2671;

pub const IDENT: &str = "agent_ai_get_cached_enemy";

impl Operation for AgentAiGetCachedEnemyOp {
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
