use language::operations::Operation;

pub struct MultiplayerGetMyTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 412;

pub const IDENT: &str = "multiplayer_get_my_troop";

impl Operation for MultiplayerGetMyTroopOp {
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
