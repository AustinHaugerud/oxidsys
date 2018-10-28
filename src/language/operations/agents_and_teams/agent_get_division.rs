use language::operations::Operation;

pub struct AgentGetDivisionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1773;

pub const IDENT: &str = "agent_get_division";

impl Operation for AgentGetDivisionOp {
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
