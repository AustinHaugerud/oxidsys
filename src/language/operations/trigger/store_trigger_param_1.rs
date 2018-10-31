use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTriggerParam1Op;

const DOC : &str = "Retrieve the value of the first trigger parameter. Will retrieve trigger's parameters even when called from inside a script, for as long as that script is running within trigger context.";

pub const OP_CODE: u32 = 2071;

pub const IDENT: &str = "store_trigger_param_1";

impl Operation for StoreTriggerParam1Op {
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
