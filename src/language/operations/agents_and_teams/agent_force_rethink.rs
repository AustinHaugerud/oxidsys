use language::operations::Operation;

pub struct AgentForceRethinkOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1732;

pub const IDENT: &str = "agent_force_rethink";

impl Operation for AgentForceRethinkOp {
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
