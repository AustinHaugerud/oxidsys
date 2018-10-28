use language::operations::Operation;

pub struct MultiplayerSetMyTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 413;

pub const IDENT: &str = "multiplayer_set_my_troop";

impl Operation for MultiplayerSetMyTroopOp {
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
