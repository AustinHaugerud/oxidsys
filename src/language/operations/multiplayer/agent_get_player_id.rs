use language::operations::Operation;

pub struct AgentGetPlayerIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1724;

pub const IDENT: &str = "agent_get_player_id";

impl Operation for AgentGetPlayerIdOp {
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
