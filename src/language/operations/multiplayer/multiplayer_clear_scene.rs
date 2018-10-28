use language::operations::Operation;

pub struct MultiplayerClearSceneOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 416;

pub const IDENT: &str = "multiplayer_clear_scene";

impl Operation for MultiplayerClearSceneOp {
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
