use language::operations::Operation;

pub struct AgentIsInLineOfSightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1826;

pub const IDENT: &str = "agent_is_in_line_of_sight";

impl Operation for AgentIsInLineOfSightOp {
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
