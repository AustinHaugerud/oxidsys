use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreNumRegularPrisonersOp;

const DOC: &str = "Deprecated and does not work. Do not use.";

pub const OP_CODE: u32 = 2159;

pub const IDENT: &str = "store_num_regular_prisoners";

impl Operation for StoreNumRegularPrisonersOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
