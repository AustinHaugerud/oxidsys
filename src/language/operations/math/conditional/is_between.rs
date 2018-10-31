use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct IsBetweenOp;

const DOC: &str = "Checks that lower_bound <= value < upper_bound";

pub const OP_CODE: u32 = 33;

pub const IDENT: &str = "is_between";

impl Operation for IsBetweenOp {
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
                make_param_doc("<value>", ""),
                make_param_doc("<lower_bound>", ""),
                make_param_doc("<upper_bound>", ""),
            ],
        }
    }
}
