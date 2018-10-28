use language::operations::Operation;

pub struct AgentGetNumberOfEnemiesFollowingOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1761;

pub const IDENT: &str = "agent_get_number_of_enemies_following";

impl Operation for AgentGetNumberOfEnemiesFollowingOp {
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
