use language::operations::Operation;

pub struct AgentSetTeamOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1771;

pub const IDENT: &str = "agent_set_team";

impl Operation for AgentSetTeamOp {
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
