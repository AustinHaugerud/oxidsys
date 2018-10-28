use language::operations::Operation;

pub struct AddQuestNoteFromDialogOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1112;

pub const IDENT: &str = "add_quest_note_from_dialog";

impl Operation for AddQuestNoteFromDialogOp {
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
