use language::operations::Operation;

pub struct ConversationScreenIsActiveOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 42;

pub const IDENT: &str = "conversation_screen_is_active";

impl Operation for ConversationScreenIsActiveOp {
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
