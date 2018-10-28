use language::operations::Operation;

pub struct FaceKeysGetAgeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2762;

pub const IDENT: &str = "face_keys_get_age";

impl Operation for FaceKeysGetAgeOp {
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
