use language::operations::Operation;

pub struct FaceKeysGetSkinColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2764;

pub const IDENT: &str = "face_keys_get_skin_color";

impl Operation for FaceKeysGetSkinColorOp {
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
