use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreAtanOp;

const DOC: &str = "Assigns dest := ARCTAN (value)";

pub const OP_CODE: u32 = 2142;

pub const IDENT: &str = "store_atan";

impl Operation for StoreAtanOp {
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
                make_param_doc("<destination_fixed_point>", ""),
                make_param_doc("<value_fixed_point>", ""),
            ],
        }
    }
}
