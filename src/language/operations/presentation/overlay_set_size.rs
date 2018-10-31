use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetSizeOp;

const DOC : &str = "Sets the overlay size, using position's X and Y coordinates. Note that the screen size in Warband is (1.00,0.75), further modified by fixed point multiplier. Also see (overlay_set_area_size).";

pub const OP_CODE: u32 = 925;

pub const IDENT: &str = "overlay_set_size";

impl Operation for OverlaySetSizeOp {
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
