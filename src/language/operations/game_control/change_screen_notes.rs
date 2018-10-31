use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenNotesOp;

const DOC : &str = "Opens the Notes screen, in the selected category (note_type: 1=troops, 2=factions, 3=parties, 4=quests, 5=info_pages) and for the specified object in that category.";

pub const OP_CODE: u32 = 2053;

pub const IDENT: &str = "change_screen_notes";

impl Operation for ChangeScreenNotesOp {
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
                make_param_doc("<note_type>", ""),
                make_param_doc("<object_id>", ""),
            ],
        }
    }
}
