use language::operations::Operation;

pub struct AgentIsAliveOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1702;

pub const IDENT: &str = "agent_is_alive";

impl Operation for AgentIsAliveOp {
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
