use language::operations::Operation;

pub struct AgentGetSimpleBehaviorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1738;

pub const IDENT: &str = "agent_get_simple_behavior";

impl Operation for AgentGetSimpleBehaviorOp {
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
