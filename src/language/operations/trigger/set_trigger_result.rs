use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetTriggerResultOp;

const DOC : &str = "Sets the return value of a trigger or game_* script, when an integer value is expected by game engine.";

pub const OP_CODE: u32 = 2075;

pub const IDENT: &str = "set_trigger_result";

impl Operation for SetTriggerResultOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
