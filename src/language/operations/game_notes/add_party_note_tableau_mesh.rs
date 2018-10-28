use language::operations::Operation;

pub struct AddPartyNoteTableauMeshOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1110;

pub const IDENT: &str = "add_party_note_tableau_mesh";

impl Operation for AddPartyNoteTableauMeshOp {
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
