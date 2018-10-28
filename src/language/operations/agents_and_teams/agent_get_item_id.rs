use language::operations::Operation;

pub struct AgentGetItemIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1719;

pub const IDENT: &str = "agent_get_item_id";

impl Operation for AgentGetItemIdOp {
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
