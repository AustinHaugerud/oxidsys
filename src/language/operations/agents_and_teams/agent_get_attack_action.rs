use language::operations::Operation;

pub struct AgentGetAttackActionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1763;

pub const IDENT: &str = "agent_get_attack_action";

impl Operation for AgentGetAttackActionOp {
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
