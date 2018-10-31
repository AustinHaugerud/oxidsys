use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreCurrentHoursOp;

const DOC : &str = "Stores number of hours that have passed since beginning of the game. Commonly used to track time when accuracy up to hours is required.";

pub const OP_CODE: u32 = 2270;

pub const IDENT: &str = "store_current_hours";

impl Operation for StoreCurrentHoursOp {
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
