use language::operations::Operation;

pub struct AddTroopNoteTableauMeshOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1108;

pub const IDENT: &str = "add_troop_note_tableau_mesh";

impl Operation for AddTroopNoteTableauMeshOp {
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
