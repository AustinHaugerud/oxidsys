use language::operations::Operation;

pub struct QuestSetNoteAvailableOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1098;

pub const IDENT: &str = "quest_set_note_available";

impl Operation for QuestSetNoteAvailableOp {
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
