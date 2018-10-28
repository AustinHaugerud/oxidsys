use language::operations::Operation;

pub struct MultiplayerSendMessageToServerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 388;

pub const IDENT: &str = "multiplayer_send_message_to_server";

impl Operation for MultiplayerSendMessageToServerOp {
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
