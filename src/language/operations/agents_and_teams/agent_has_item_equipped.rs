use language::operations::Operation;

pub struct AgentHasItemEquippedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1729;

pub const IDENT: &str = "agent_has_item_equipped";

impl Operation for AgentHasItemEquippedOp {
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
