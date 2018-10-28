use language::operations::Operation;

pub struct AddFactionNoteFromSregOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1118;

pub const IDENT: &str = "add_faction_note_from_sreg";

impl Operation for AddFactionNoteFromSregOp {
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
