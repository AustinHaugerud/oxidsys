use language::operations::Operation;

pub struct AgentSetAttackActionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1745;

pub const IDENT: &str = "agent_set_attack_action";

impl Operation for AgentSetAttackActionOp {
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
