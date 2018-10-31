use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreCurrentDayOp;

const DOC : &str = "Stores number of days that have passed since beginning of the game. Commonly used to track time when high accuracy is not required.";

pub const OP_CODE: u32 = 2272;

pub const IDENT: &str = "store_current_day";

impl Operation for StoreCurrentDayOp {
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
