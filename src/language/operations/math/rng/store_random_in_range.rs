use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRandomInRangeOp;

const DOC: &str = "Stores a random value in the range of <range_low>..<range_high>-1.";

pub const OP_CODE: u32 = 2136;

pub const IDENT: &str = "store_random_in_range";

impl Operation for StoreRandomInRangeOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<range_low>", ""),
                make_param_doc("<range_high>", ""),
            ],
        }
    }
}
