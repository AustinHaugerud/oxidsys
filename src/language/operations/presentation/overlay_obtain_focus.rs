use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlayObtainFocusOp;

const DOC: &str = "Makes the specified overlay obtain input focus. Only works for text fields.";

pub const OP_CODE: u32 = 949;

pub const IDENT: &str = "overlay_obtain_focus";

impl Operation for OverlayObtainFocusOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<overlay_id>", "")],
        }
    }
}
