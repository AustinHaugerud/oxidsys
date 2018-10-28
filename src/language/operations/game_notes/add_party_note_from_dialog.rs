use language::operations::Operation;

pub struct AddPartyNoteFromDialogOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1116;

pub const IDENT: &str = "add_party_note_from_dialog";

impl Operation for AddPartyNoteFromDialogOp {
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
