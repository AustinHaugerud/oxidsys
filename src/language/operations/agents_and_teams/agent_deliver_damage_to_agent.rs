use language::operations::Operation;

pub struct AgentDeliverDamageToAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1722;

pub const IDENT: &str = "agent_deliver_damage_to_agent";

impl Operation for AgentDeliverDamageToAgentOp {
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
