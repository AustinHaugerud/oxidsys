use language::operations::Operation;

pub struct AgentIsAllyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1706;

pub const IDENT: &str = "agent_is_ally";

impl Operation for AgentIsAllyOp {
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
