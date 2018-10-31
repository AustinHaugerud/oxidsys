use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateSliderOverlayOp;

const DOC : &str = "Creates horizontal slider overlay, with positions of the slider varying between min and max values. Current value of the slider can be changed with (overlay_set_val). Returns slider's overlay_id.";

pub const OP_CODE: u32 = 914;

pub const IDENT: &str = "create_slider_overlay";

impl Operation for CreateSliderOverlayOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<min_value>", ""),
                make_param_doc("<max_value>", ""),
            ],
        }
    }
}
