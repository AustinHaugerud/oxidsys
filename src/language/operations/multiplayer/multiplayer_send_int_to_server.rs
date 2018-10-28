use language::operations::Operation;

pub struct MultiplayerSendIntToServerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 389;

pub const IDENT: &str = "multiplayer_send_int_to_server";

impl Operation for MultiplayerSendIntToServerOp {
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
