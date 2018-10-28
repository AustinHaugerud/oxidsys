use language::operations::Operation;

pub struct PlayerSetFaceKeysOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2748;

pub const IDENT: &str = "player_set_face_keys";

impl Operation for PlayerSetFaceKeysOp {
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
