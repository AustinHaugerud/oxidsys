use language::operations::Operation;

pub struct AgentSetNoDynamicsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1762;

pub const IDENT: &str = "agent_set_no_dynamics";

impl Operation for AgentSetNoDynamicsOp {
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
