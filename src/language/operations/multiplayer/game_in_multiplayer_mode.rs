use language::operations::Operation;

pub struct GameInMultiplayerModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 419;

pub const IDENT: &str = "game_in_multiplayer_mode";

impl Operation for GameInMultiplayerModeOp {
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
