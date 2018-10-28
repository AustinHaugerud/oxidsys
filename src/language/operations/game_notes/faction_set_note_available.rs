use language::operations::Operation;

pub struct FactionSetNoteAvailableOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1096;

pub const IDENT: &str = "faction_set_note_available";

impl Operation for FactionSetNoteAvailableOp {
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
