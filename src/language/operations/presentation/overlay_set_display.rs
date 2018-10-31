use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetDisplayOp;

const DOC: &str = "Shows (value = 1) or hides (value = 0) the specified overlay.";

pub const OP_CODE: u32 = 947;

pub const IDENT: &str = "overlay_set_display";

impl Operation for OverlaySetDisplayOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
