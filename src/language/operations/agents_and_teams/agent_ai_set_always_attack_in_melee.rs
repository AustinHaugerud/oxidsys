use language::operations::Operation;

pub struct AgentAiSetAlwaysAttackInMeleeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1737;

pub const IDENT: &str = "agent_ai_set_always_attack_in_melee";

impl Operation for AgentAiSetAlwaysAttackInMeleeOp {
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
