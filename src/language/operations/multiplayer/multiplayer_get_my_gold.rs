use language::operations::Operation;

pub struct MultiplayerGetMyGoldOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 414;

pub const IDENT: &str = "multiplayer_get_my_gold";

impl Operation for MultiplayerGetMyGoldOp {
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
