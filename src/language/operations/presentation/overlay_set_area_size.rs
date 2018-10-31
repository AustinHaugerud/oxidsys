use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetAreaSizeOp;

const DOC : &str = "Defines the actual area on the screen used to display the overlay. If it's size is greater than area size, it will create a scrollable area with appropriate scrollbars. Can be used to create scrollable areas for large text, or scrollable containers with many children elements (see Host Game screen for a typical example).";

pub const OP_CODE: u32 = 929;

pub const IDENT: &str = "overlay_set_area_size";

impl Operation for OverlaySetAreaSizeOp {
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
