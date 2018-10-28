use language::operations::Operation;

pub struct AgentSetRangedDamageModifierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2099;

pub const IDENT: &str = "agent_set_ranged_damage_modifier";

impl Operation for AgentSetRangedDamageModifierOp {
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
