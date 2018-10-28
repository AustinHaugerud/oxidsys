use language::operations::Operation;

pub struct StrStoreAgentNameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2332;

pub const IDENT: &str = "str_store_agent_name";

impl Operation for StrStoreAgentNameOp {
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
