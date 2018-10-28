use language::operations::Operation;

pub struct AgentSetGroupOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1766;

pub const IDENT: &str = "agent_set_group";

impl Operation for AgentSetGroupOp {
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
