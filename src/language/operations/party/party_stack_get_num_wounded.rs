use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyStackGetNumWoundedOp;

const DOC: &str = "Extracts number of wounded troops in the specified troop stack.";

pub const OP_CODE: u32 = 1654;

pub const IDENT: &str = "party_stack_get_num_wounded";

impl Operation for PartyStackGetNumWoundedOp {
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
