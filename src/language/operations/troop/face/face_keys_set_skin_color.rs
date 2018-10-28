use language::operations::Operation;

pub struct FaceKeysSetSkinColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2765;

pub const IDENT: &str = "face_keys_set_skin_color";

impl Operation for FaceKeysSetSkinColorOp {
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
