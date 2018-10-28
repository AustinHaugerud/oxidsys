use language::operations::Operation;

pub struct FaceKeysSetMorphKeyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2767;

pub const IDENT: &str = "face_keys_set_morph_key";

impl Operation for FaceKeysSetMorphKeyOp {
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
