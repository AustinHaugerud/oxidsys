use language::operations::Operation;

pub struct AgentSetIsAlarmedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1807;

pub const IDENT: &str = "agent_set_is_alarmed";

impl Operation for AgentSetIsAlarmedOp {
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
