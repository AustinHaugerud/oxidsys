use language::operations::Operation;

pub struct AgentIsWoundedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1703;

pub const IDENT: &str = "agent_is_wounded";

impl Operation for AgentIsWoundedOp {
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
