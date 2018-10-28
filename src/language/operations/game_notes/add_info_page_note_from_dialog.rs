use language::operations::Operation;

pub struct AddInfoPageNoteFromDialogOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1091;

pub const IDENT: &str = "add_info_page_note_from_dialog";

impl Operation for AddInfoPageNoteFromDialogOp {
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
