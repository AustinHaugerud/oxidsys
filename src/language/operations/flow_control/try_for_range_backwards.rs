use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TryForRangeBackwardsOp;

const DOC : &str = "Same as above, but iterates the value in the opposite direction (from higher values to lower).";

pub const OP_CODE: u32 = 7;

pub const IDENT: &str = "try_for_range_backwards";

impl Operation for TryForRangeBackwardsOp {
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
