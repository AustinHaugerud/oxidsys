use language::operations::Operation;

pub struct AgentDeliverDamageToAgentAdvancedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1827;

pub const IDENT: &str = "agent_deliver_damage_to_agent_advanced";

impl Operation for AgentDeliverDamageToAgentAdvancedOp {
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
