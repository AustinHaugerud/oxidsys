use language::operations::Operation;

pub struct ChangeScreenMapConversationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2049;

pub const IDENT: &str = "change_screen_map_conversation";

impl Operation for ChangeScreenMapConversationOp {
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
