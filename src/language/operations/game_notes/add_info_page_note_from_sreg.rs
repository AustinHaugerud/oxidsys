use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddInfoPageNoteFromSregOp;

const DOC : &str = "Adds any text stored in string register to info page notes. Each info page has 16 note slots. Last parameter is used to mark the note as time-dependent, if it's value is 1, then the note will be marked (\"Report is current\") and will be updated appropriately as the game progresses (\"Report is X days old\").";

pub const OP_CODE: u32 = 1092;

pub const IDENT: &str = "add_info_page_note_from_sreg";

impl Operation for AddInfoPageNoteFromSregOp {
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
            num_required: 4,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<info_page_id>", ""),
                make_param_doc("<note_slot_no>", ""),
                make_param_doc("<string_id>", ""),
                make_param_doc("<expires_with_time>", ""),
            ],
        }
    }
}
