use language::operations::Operation;

pub struct MultiplayerSend2IntToServerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 390;

pub const IDENT: &str = "multiplayer_send_2_int_to_server";

impl Operation for MultiplayerSend2IntToServerOp {
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
