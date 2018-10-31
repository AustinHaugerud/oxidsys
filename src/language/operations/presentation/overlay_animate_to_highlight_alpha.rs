use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlayAnimateToHighlightAlphaOp;

const DOC : &str = "Highlights overlay to specified alpha during a specified timeframe, specified in 1/000th of second.";

pub const OP_CODE: u32 = 935;

pub const IDENT: &str = "overlay_animate_to_highlight_alpha";

impl Operation for OverlayAnimateToHighlightAlphaOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<overlay_id>", ""),
                make_param_doc("<color>", ""),
            ],
        }
    }
}
