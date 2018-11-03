use language::operations::{Operation, ParamInfo};

pub struct PresentationSetDurationOp;

const DOC : &str = "Sets presentation duration time, in 1/100th of second. Must be called when a presentation is active. If several presentations are active, duration will be set for all of them.";

pub const OP_CODE: u32 = 902;

pub const IDENT: &str = "presentation_set_duration";

impl Operation for PresentationSetDurationOp {
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
