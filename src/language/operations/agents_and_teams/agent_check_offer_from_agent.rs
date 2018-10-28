use language::operations::Operation;

pub struct AgentCheckOfferFromAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1778;

pub const IDENT: &str = "agent_check_offer_from_agent";

impl Operation for AgentCheckOfferFromAgentOp {
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
