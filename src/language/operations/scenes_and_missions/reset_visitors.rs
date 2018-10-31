use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ResetVisitorsOp;

const DOC: &str = "Resets all visitors to the scene.";

pub const OP_CODE: u32 = 1262;

pub const IDENT: &str = "reset_visitors";

impl Operation for ResetVisitorsOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
