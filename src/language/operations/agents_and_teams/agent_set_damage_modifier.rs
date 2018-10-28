use language::operations::Operation;

pub struct AgentSetDamageModifierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2091;

pub const IDENT: &str = "agent_set_damage_modifier";

impl Operation for AgentSetDamageModifierOp {
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
