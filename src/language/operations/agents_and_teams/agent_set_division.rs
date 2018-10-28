use language::operations::Operation;

pub struct AgentSetDivisionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1783;

pub const IDENT: &str = "agent_set_division";

impl Operation for AgentSetDivisionOp {
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
