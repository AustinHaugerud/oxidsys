use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CompleteQuestOp;

const DOC: &str =
    "Successfully completes specified quest, removing it from the list of active quests.";

pub const OP_CODE: u32 = 1281;

pub const IDENT: &str = "complete_quest";

impl Operation for CompleteQuestOp {
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
