use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetHilightColorOp;

const DOC: &str =
    "Highlights the overlay with specified color. May not work with some overlay types.";

pub const OP_CODE: u32 = 923;

pub const IDENT: &str = "overlay_set_hilight_color";

impl Operation for OverlaySetHilightColorOp {
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
