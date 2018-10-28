use language::operations::Operation;

pub struct AgentGetRiderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1715;

pub const IDENT: &str = "agent_get_rider";

impl Operation for AgentGetRiderOp {
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
