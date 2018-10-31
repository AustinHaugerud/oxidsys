use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRandomOp;

const DOC : &str = "Stores a random value in the range of 0..<upper_range>-1. Deprecated, use (store_random_in_range) instead.";

pub const OP_CODE: u32 = 2135;

pub const IDENT: &str = "store_random";

impl Operation for StoreRandomOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<upper_range>", ""),
            ],
        }
    }
}
