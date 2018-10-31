use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTriggerParamOp;

const DOC : &str = "Version 1.153+. Retrieve the value of arbitrary trigger parameter. Parameters are enumerated starting from 1. Note that despite the introduction of this operation, there's not a single trigger with more than 3 parameters.";

pub const OP_CODE: u32 = 2070;

pub const IDENT: &str = "store_trigger_param";

impl Operation for StoreTriggerParamOp {
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
                make_param_doc("<trigger_param_no>", ""),
            ],
        }
    }
}
