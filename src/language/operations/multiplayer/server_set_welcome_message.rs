use language::operations::Operation;

pub struct ServerSetWelcomeMessageOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 492;

pub const IDENT: &str = "server_set_welcome_message";

impl Operation for ServerSetWelcomeMessageOp {
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
