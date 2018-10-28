use language::operations::Operation;

pub struct AgentSetVisibilityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2096;

pub const IDENT: &str = "agent_set_visibility";

impl Operation for AgentSetVisibilityOp {
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
