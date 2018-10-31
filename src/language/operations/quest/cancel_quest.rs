use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CancelQuestOp;

const DOC: &str =
    "Cancels specified quest without completing it, removing it from the list of active quests.";

pub const OP_CODE: u32 = 1284;

pub const IDENT: &str = "cancel_quest";

impl Operation for CancelQuestOp {
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
