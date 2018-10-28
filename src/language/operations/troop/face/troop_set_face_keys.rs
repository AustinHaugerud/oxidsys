use language::operations::Operation;

pub struct TroopSetFaceKeysOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2751;

pub const IDENT: &str = "troop_set_face_keys";

impl Operation for TroopSetFaceKeysOp {
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
