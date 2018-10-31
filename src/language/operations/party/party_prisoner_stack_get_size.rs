use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyPrisonerStackGetSizeOp;

const DOC: &str = "Extracts number of troops in the specified prisoner stack.";

pub const OP_CODE: u32 = 1657;

pub const IDENT: &str = "party_prisoner_stack_get_size";

impl Operation for PartyPrisonerStackGetSizeOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
                make_param_doc("<stack_no>", ""),
            ],
        }
    }
}
