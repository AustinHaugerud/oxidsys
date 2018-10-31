use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct Store01RandomPartiesInRangeOp;

const DOC : &str = "Stores two random, different parties in a range to reg0 and reg1. Generally used only for predefined parties (towns, villages etc).";

pub const OP_CODE: u32 = 2255;

pub const IDENT: &str = "store01_random_parties_in_range";

impl Operation for Store01RandomPartiesInRangeOp {
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
                make_param_doc("<lower_bound>", ""),
                make_param_doc("<upper_bound>", ""),
            ],
        }
    }
}
