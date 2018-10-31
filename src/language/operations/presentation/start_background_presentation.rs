use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StartBackgroundPresentationOp;

const DOC: &str =
    "Apparently allows you to start a presentation in background but stay in the menu. 4research.";

pub const OP_CODE: u32 = 901;

pub const IDENT: &str = "start_background_presentation";

impl Operation for StartBackgroundPresentationOp {
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
