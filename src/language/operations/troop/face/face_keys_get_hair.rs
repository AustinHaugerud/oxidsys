use language::operations::Operation;

pub struct FaceKeysGetHairOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2752;

pub const IDENT: &str = "face_keys_get_hair";

impl Operation for FaceKeysGetHairOp {
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
