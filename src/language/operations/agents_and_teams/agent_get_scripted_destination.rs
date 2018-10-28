use language::operations::Operation;

pub struct AgentGetScriptedDestinationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1731;

pub const IDENT: &str = "agent_get_scripted_destination";

impl Operation for AgentGetScriptedDestinationOp {
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
