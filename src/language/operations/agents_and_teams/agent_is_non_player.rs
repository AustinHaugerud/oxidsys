use language::operations::Operation;

pub struct AgentIsNonPlayerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1707;

pub const IDENT: &str = "agent_is_non_player";

impl Operation for AgentIsNonPlayerOp {
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
