use language::operations::Operation;

pub struct AgentGetPartyIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1716;

pub const IDENT: &str = "agent_get_party_id";

impl Operation for AgentGetPartyIdOp {
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
