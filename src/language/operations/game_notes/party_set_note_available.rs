use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetNoteAvailableOp;

const DOC: &str =
    "Enables (value = 1) or disables (value = 0) party's page in the Notes / Characters section.";

pub const OP_CODE: u32 = 1097;

pub const IDENT: &str = "party_set_note_available";

impl Operation for PartySetNoteAvailableOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
