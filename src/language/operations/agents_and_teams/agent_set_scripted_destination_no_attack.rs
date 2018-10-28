use language::operations::Operation;

pub struct AgentSetScriptedDestinationNoAttackOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1748;

pub const IDENT: &str = "agent_set_scripted_destination_no_attack";

impl Operation for AgentSetScriptedDestinationNoAttackOp {
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
