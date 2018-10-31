use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FailQuestOp;

const DOC : &str = "Sets quest status as failed but keeps it in the list (player must visit quest giver to complete it before he can get another quest of the same type).";

pub const OP_CODE: u32 = 1283;

pub const IDENT: &str = "fail_quest";

impl Operation for FailQuestOp {
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
