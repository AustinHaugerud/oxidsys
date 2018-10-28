use language::operations::Operation;

pub struct AgentGetItemSlotOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1804;

pub const IDENT: &str = "agent_get_item_slot";

impl Operation for AgentGetItemSlotOp {
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
