use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct IsZoomDisabledOp;

const DOC: &str = "Version 1.153+. Checks that the zoom is currently disabled in the game.";

pub const OP_CODE: u32 = 2222;

pub const IDENT: &str = "is_zoom_disabled";

impl Operation for IsZoomDisabledOp {
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
