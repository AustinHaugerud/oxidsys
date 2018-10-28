use language::operations::Operation;

pub struct MultiplayerSend3IntToServerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 391;

pub const IDENT: &str = "multiplayer_send_3_int_to_server";

impl Operation for MultiplayerSend3IntToServerOp {
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
