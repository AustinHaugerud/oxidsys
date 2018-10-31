use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetGlobalHazeAmountOp;

const DOC: &str = "Returns current fogginess (value between 0..100).";

pub const OP_CODE: u32 = 92;

pub const IDENT: &str = "get_global_haze_amount";

impl Operation for GetGlobalHazeAmountOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
