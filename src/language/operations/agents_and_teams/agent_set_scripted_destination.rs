use language::operations::Operation;

pub struct AgentSetScriptedDestinationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1730;

pub const IDENT: &str = "agent_set_scripted_destination";

impl Operation for AgentSetScriptedDestinationOp {
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
