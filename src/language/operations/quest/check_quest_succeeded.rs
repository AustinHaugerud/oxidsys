use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CheckQuestSucceededOp;

const DOC : &str = "Checks that the quest has succeeded and not taken again yet (check will be successful even after the quest is completed).";

pub const OP_CODE: u32 = 202;

pub const IDENT: &str = "check_quest_succeeded";

impl Operation for CheckQuestSucceededOp {
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
