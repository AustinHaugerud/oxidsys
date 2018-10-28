use language::operations::Operation;

pub struct FaceKeysSetBeardOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2755;

pub const IDENT: &str = "face_keys_set_beard";

impl Operation for FaceKeysSetBeardOp {
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
