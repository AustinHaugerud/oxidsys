use language::operations::Operation;

pub struct ServerAddMessageToLogOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 473;

pub const IDENT: &str = "server_add_message_to_log";

impl Operation for ServerAddMessageToLogOp {
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
