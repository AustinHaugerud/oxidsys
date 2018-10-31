use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetGlobalCloudAmountOp;

const DOC: &str = "Returns current cloudiness (a value between 0..100).";

pub const OP_CODE: u32 = 90;

pub const IDENT: &str = "get_global_cloud_amount";

impl Operation for GetGlobalCloudAmountOp {
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
