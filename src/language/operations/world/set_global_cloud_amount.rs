use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetGlobalCloudAmountOp;

const DOC: &str = "Sets current cloudiness (value is clamped to 0..100).";

pub const OP_CODE: u32 = 91;

pub const IDENT: &str = "set_global_cloud_amount";

impl Operation for SetGlobalCloudAmountOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
