use language::operations::Operation;

pub struct AgentGetTroopIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1718;

pub const IDENT: &str = "agent_get_troop_id";

impl Operation for AgentGetTroopIdOp {
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
