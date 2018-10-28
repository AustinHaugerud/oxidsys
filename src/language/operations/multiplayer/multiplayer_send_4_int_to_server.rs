use language::operations::Operation;

pub struct MultiplayerSend4IntToServerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 392;

pub const IDENT: &str = "multiplayer_send_4_int_to_server";

impl Operation for MultiplayerSend4IntToServerOp {
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
