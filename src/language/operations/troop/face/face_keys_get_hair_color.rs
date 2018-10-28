use language::operations::Operation;

pub struct FaceKeysGetHairColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2760;

pub const IDENT: &str = "face_keys_get_hair_color";

impl Operation for FaceKeysGetHairColorOp {
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
