use language::operations::Operation;

pub struct AgentSetInvulnerableShieldOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1725;

pub const IDENT: &str = "agent_set_invulnerable_shield";

impl Operation for AgentSetInvulnerableShieldOp {
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
