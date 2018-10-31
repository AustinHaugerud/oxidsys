use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetGlobalHazeAmountOp;

const DOC: &str = "Sets current fogginess (value is clamped to 0..100).";

pub const OP_CODE: u32 = 93;

pub const IDENT: &str = "set_global_haze_amount";

impl Operation for SetGlobalHazeAmountOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
