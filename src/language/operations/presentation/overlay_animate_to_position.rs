use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlayAnimateToPositionOp;

const DOC : &str = "Moves overlay to specified position during a specified timeframe, specified in 1/1000th of second.";

pub const OP_CODE: u32 = 937;

pub const IDENT: &str = "overlay_animate_to_position";

impl Operation for OverlayAnimateToPositionOp {
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
                make_param_doc("<position>", ""),
            ],
        }
    }
}
