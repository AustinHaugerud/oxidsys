use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreZoomAmountOp;

const DOC: &str = "Version 1.153+. Stores current zoom rate.";

pub const OP_CODE: u32 = 2220;

pub const IDENT: &str = "store_zoom_amount";

impl Operation for StoreZoomAmountOp {
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
            param_docs: vec![make_param_doc("<destination_fixed_point>", "")],
        }
    }
}
