use language::operations::Operation;

pub struct StartMapConversationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1025;

pub const IDENT: &str = "start_map_conversation";

impl Operation for StartMapConversationOp {
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
