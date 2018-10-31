use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TryForRangeOp;

const DOC: &str = "Runs a cycle, iterating the value in the <lower_bound>..<upper_bound>-1 range.";

pub const OP_CODE: u32 = 6;

pub const IDENT: &str = "try_for_range";

impl Operation for TryForRangeOp {
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
