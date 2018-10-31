use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTimeOfDayOp;

const DOC: &str = "Stores current day hour (value in 0..24 range).";

pub const OP_CODE: u32 = 2271;

pub const IDENT: &str = "store_time_of_day";

impl Operation for StoreTimeOfDayOp {
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
