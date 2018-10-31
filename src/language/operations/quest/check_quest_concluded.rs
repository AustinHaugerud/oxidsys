use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CheckQuestConcludedOp;

const DOC: &str = "Checks that the quest was concluded with any result and not taken again yet.";

pub const OP_CODE: u32 = 204;

pub const IDENT: &str = "check_quest_concluded";

impl Operation for CheckQuestConcludedOp {
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
