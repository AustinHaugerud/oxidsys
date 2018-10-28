use language::operations::Operation;

pub struct AgentSetWieldedItemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1747;

pub const IDENT: &str = "agent_set_wielded_item";

impl Operation for AgentSetWieldedItemOp {
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
