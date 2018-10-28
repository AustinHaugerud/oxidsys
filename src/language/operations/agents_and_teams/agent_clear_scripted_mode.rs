use language::operations::Operation;

pub struct AgentClearScriptedModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1735;

pub const IDENT: &str = "agent_clear_scripted_mode";

impl Operation for AgentClearScriptedModeOp {
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
