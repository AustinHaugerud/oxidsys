use language::operations::Operation;

pub struct FaceKeysGetFaceTextureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2756;

pub const IDENT: &str = "face_keys_get_face_texture";

impl Operation for FaceKeysGetFaceTextureOp {
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
