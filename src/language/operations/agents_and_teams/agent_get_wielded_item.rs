use language::operations::Operation;

pub struct AgentGetWieldedItemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1726;

pub const IDENT: &str = "agent_get_wielded_item";

impl Operation for AgentGetWieldedItemOp {
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
