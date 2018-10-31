use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ConcludeQuestOp;

const DOC : &str = "Sets quest status as concluded but keeps it in the list. Frequently used to indicate \"uncertain\" quest status, when it's neither fully successful nor a total failure.";

pub const OP_CODE: u32 = 1286;

pub const IDENT: &str = "conclude_quest";

impl Operation for ConcludeQuestOp {
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
            param_docs: vec![make_param_doc("<quest_id>", "")],
        }
    }
}
