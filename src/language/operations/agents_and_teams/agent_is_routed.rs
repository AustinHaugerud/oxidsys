use language::operations::Operation;

pub struct AgentIsRoutedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1699;

pub const IDENT: &str = "agent_is_routed";

impl Operation for AgentIsRoutedOp {
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
