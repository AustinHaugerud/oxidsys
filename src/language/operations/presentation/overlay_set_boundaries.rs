use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetBoundariesOp;

const DOC: &str = "Changes the value boundaries for the overlays that have them.";

pub const OP_CODE: u32 = 928;

pub const IDENT: &str = "overlay_set_boundaries";

impl Operation for OverlaySetBoundariesOp {
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
                make_param_doc("<overlay_id>", ""),
                make_param_doc("<min_value>", ""),
                make_param_doc("<max_value>", ""),
            ],
        }
    }
}
