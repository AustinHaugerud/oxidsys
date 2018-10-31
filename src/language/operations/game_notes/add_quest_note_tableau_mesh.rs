use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddQuestNoteTableauMeshOp;

const DOC: &str = "Adds graphical elements to the quest's information page (not used in Native).";

pub const OP_CODE: u32 = 1111;

pub const IDENT: &str = "add_quest_note_tableau_mesh";

impl Operation for AddQuestNoteTableauMeshOp {
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
                make_param_doc("<quest_id>", ""),
                make_param_doc("<tableau_material_id>", ""),
            ],
        }
    }
}
