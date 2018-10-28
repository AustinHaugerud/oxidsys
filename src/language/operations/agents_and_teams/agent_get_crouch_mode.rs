use language::operations::Operation;

pub struct AgentGetCrouchModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2097;

pub const IDENT: &str = "agent_get_crouch_mode";

impl Operation for AgentGetCrouchModeOp {
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
