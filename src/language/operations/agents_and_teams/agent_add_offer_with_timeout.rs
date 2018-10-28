use language::operations::Operation;

pub struct AgentAddOfferWithTimeoutOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1777;

pub const IDENT: &str = "agent_add_offer_with_timeout";

impl Operation for AgentAddOfferWithTimeoutOp {
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
