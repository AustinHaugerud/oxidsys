use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetTextOp;

const DOC : &str = "Changes the overlay text (if it has any). Works for labels, text fields, buttons with text labels...";

pub const OP_CODE: u32 = 920;

pub const IDENT: &str = "overlay_set_text";

impl Operation for OverlaySetTextOp {
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
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}
