use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlayGetPositionOp;

const DOC : &str = "Retrieves overlay current position to specified position trigger, using position's X and Y coordinates. Note that the screen size in Warband is (1.00,0.75), further modified by fixed point multiplier.";

pub const OP_CODE: u32 = 946;

pub const IDENT: &str = "overlay_get_position";

impl Operation for OverlayGetPositionOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<overlay_id>", ""),
            ],
        }
    }
}
