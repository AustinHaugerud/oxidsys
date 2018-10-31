use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetZoomAmountOp;

const DOC: &str = "Version 1.153+. Sets new zoom rate.";

pub const OP_CODE: u32 = 2221;

pub const IDENT: &str = "set_zoom_amount";

impl Operation for SetZoomAmountOp {
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
            param_docs: vec![make_param_doc("<value_fixed_point>", "")],
        }
    }
}
