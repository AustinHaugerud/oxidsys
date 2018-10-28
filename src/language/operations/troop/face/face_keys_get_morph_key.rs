use language::operations::Operation;

pub struct FaceKeysGetMorphKeyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2766;

pub const IDENT: &str = "face_keys_get_morph_key";

impl Operation for FaceKeysGetMorphKeyOp {
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
