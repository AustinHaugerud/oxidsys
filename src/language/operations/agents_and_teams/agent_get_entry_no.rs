use language::operations::Operation;

pub struct AgentGetEntryNoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1717;

pub const IDENT: &str = "agent_get_entry_no";

impl Operation for AgentGetEntryNoOp {
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
