use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetTriggerObjectPositionOp;

const DOC: &str =
    "Retrieve the position of an object which caused the trigger to fire (when appropriate).";

pub const OP_CODE: u32 = 702;

pub const IDENT: &str = "get_trigger_object_position";

impl Operation for GetTriggerObjectPositionOp {
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
            param_docs: vec![make_param_doc("<position>", "")],
        }
    }
}
