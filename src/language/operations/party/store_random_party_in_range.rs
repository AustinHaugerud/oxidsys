use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRandomPartyInRangeOp;

const DOC : &str = "Retrieves one random party from the range. Generally used only for predefined parties (towns, villages etc).";

pub const OP_CODE: u32 = 2254;

pub const IDENT: &str = "store_random_party_in_range";

impl Operation for StoreRandomPartyInRangeOp {
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
                make_param_doc("<lower_bound>", ""),
                make_param_doc("<upper_bound>", ""),
            ],
        }
    }
}
