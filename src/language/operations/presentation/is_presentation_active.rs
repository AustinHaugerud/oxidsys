use language::operations::{Operation, ParamInfo};

pub struct IsPresentationActiveOp;

const DOC: &str = "Checks that the specified presentation is currently running.";

pub const OP_CODE: u32 = 903;

pub const IDENT: &str = "is_presentation_active";

impl Operation for IsPresentationActiveOp {
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
