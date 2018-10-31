use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CheckQuestActiveOp;

const DOC : &str = "Checks that the quest has been started but not yet cancelled or completed. Will not fail for concluded, failed or succeeded quests for as long as they have not yet been completed.";

pub const OP_CODE: u32 = 200;

pub const IDENT: &str = "check_quest_active";

impl Operation for CheckQuestActiveOp {
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
