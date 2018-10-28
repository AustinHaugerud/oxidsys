use language::operations::Operation;

pub struct FaceKeysSetFaceTextureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2757;

pub const IDENT: &str = "face_keys_set_face_texture";

impl Operation for FaceKeysSetFaceTextureOp {
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
