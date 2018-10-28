use language::operations::Operation;

pub struct StrStorePlayerFaceKeysOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2747;

pub const IDENT: &str = "str_store_player_face_keys";

impl Operation for StrStorePlayerFaceKeysOp {
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
