use language::operations::Operation;

pub struct AgentGetCombatStateOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1739;

pub const IDENT: &str = "agent_get_combat_state";

impl Operation for AgentGetCombatStateOp {
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
