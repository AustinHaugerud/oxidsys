use language::operations::Operation;

pub struct AgentGetLookPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1709;

pub const IDENT: &str = "agent_get_look_position";

impl Operation for AgentGetLookPositionOp {
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
