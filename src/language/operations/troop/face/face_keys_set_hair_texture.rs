use language::operations::Operation;

pub struct FaceKeysSetHairTextureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2759;

pub const IDENT: &str = "face_keys_set_hair_texture";

impl Operation for FaceKeysSetHairTextureOp {
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
