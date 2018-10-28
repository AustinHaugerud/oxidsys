use language::operations::Operation;

pub struct TroopSetFaceKeyFromCurrentProfileOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1503;

pub const IDENT: &str = "troop_set_face_key_from_current_profile";

impl Operation for TroopSetFaceKeyFromCurrentProfileOp {
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
