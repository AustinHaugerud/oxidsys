use language::operations::Operation;

pub struct AgentIsHumanOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1704;

pub const IDENT: &str = "agent_is_human";

impl Operation for AgentIsHumanOp {
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
