use language::operations::Operation;

pub struct TroopSetNoteAvailableOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1095;

pub const IDENT: &str = "troop_set_note_available";

impl Operation for TroopSetNoteAvailableOp {
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
