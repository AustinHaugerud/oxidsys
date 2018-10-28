use language::operations::Operation;

pub struct AddTroopNoteFromSregOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1117;

pub const IDENT: &str = "add_troop_note_from_sreg";

impl Operation for AddTroopNoteFromSregOp {
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
