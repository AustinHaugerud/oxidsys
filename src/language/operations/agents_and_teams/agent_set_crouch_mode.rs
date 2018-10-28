use language::operations::Operation;

pub struct AgentSetCrouchModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2098;

pub const IDENT: &str = "agent_set_crouch_mode";

impl Operation for AgentSetCrouchModeOp {
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
