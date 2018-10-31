use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRandomTroopToRaiseOp;

const DOC: &str = "Apparently deprecated.";

pub const OP_CODE: u32 = 2251;

pub const IDENT: &str = "store_random_troop_to_raise";

impl Operation for StoreRandomTroopToRaiseOp {
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
                make_param_doc("<lower_bound>", ""),
                make_param_doc("<upper_bound>", ""),
            ],
        }
    }
}
