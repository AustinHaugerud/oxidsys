use language::operations::Operation;

pub struct StoreConversationTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2200;

pub const IDENT: &str = "store_conversation_troop";

impl Operation for StoreConversationTroopOp {
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
