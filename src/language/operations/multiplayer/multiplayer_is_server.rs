use language::operations::Operation;

pub struct MultiplayerIsServerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 417;

pub const IDENT: &str = "multiplayer_is_server";

impl Operation for MultiplayerIsServerOp {
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
