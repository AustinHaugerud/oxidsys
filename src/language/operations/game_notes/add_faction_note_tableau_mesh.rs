use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddFactionNoteTableauMeshOp;

const DOC: &str =
    "Adds graphical elements to the faction's information page (usually graphical collage).";

pub const OP_CODE: u32 = 1109;

pub const IDENT: &str = "add_faction_note_tableau_mesh";

impl Operation for AddFactionNoteTableauMeshOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<faction_id>", ""),
                make_param_doc("<tableau_material_id>", ""),
            ],
        }
    }
}
