use language::operations::Operation;

pub struct AddQuestNoteFromSregOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1113;

pub const IDENT: &str = "add_quest_note_from_sreg";

impl Operation for AddQuestNoteFromSregOp {
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
