use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StartPresentationOp;

const DOC: &str = "Starts the specified presentation.";

pub const OP_CODE: u32 = 900;

pub const IDENT: &str = "start_presentation";

impl Operation for StartPresentationOp {
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
            param_docs: vec![make_param_doc("<presentation_id>", "")],
        }
    }
}
