use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateTextBoxOverlayOp;

const DOC: &str = "Apparently deprecated. No longer used in Native.";

pub const OP_CODE: u32 = 917;

pub const IDENT: &str = "create_text_box_overlay";

impl Operation for CreateTextBoxOverlayOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
