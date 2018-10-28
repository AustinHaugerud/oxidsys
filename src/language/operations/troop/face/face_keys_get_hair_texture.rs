use language::operations::Operation;

pub struct FaceKeysGetHairTextureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2758;

pub const IDENT: &str = "face_keys_get_hair_texture";

impl Operation for FaceKeysGetHairTextureOp {
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
