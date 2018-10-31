use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTriggerParam3Op;

const DOC : &str = "Retrieve the value of the third trigger parameter. Will retrieve trigger's parameters even when called from inside a script, for as long as that script is running within trigger context.";

pub const OP_CODE: u32 = 2073;

pub const IDENT: &str = "store_trigger_param_3";

impl Operation for StoreTriggerParam3Op {
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
