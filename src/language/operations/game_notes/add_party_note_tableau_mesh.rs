use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddPartyNoteTableauMeshOp;

const DOC: &str = "Adds graphical elements to the party's information page (usually map icon).";

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

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<party_id>", ""),
                make_param_doc("<tableau_material_id>", ""),
            ],
        }
    }
}
