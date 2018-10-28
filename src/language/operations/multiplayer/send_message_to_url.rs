use language::operations::Operation;

pub struct SendMessageToUrlOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 380;

pub const IDENT: &str = "send_message_to_url";

impl Operation for SendMessageToUrlOp {
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
