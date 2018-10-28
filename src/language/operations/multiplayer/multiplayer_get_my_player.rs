use language::operations::Operation;

pub struct MultiplayerGetMyPlayerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 415;

pub const IDENT: &str = "multiplayer_get_my_player";

impl Operation for MultiplayerGetMyPlayerOp {
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
