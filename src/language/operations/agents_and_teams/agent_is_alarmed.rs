use language::operations::Operation;

pub struct AgentIsAlarmedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1806;

pub const IDENT: &str = "agent_is_alarmed";

impl Operation for AgentIsAlarmedOp {
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
