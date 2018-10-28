use language::operations::Operation;

pub struct FaceKeysGetBeardOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2754;

pub const IDENT: &str = "face_keys_get_beard";

impl Operation for FaceKeysGetBeardOp {
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
