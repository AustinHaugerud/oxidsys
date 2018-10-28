use language::operations::Operation;

pub struct AgentIsDefenderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1708;

pub const IDENT: &str = "agent_is_defender";

impl Operation for AgentIsDefenderOp {
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
