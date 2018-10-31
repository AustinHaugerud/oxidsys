use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetupQuestGiverOp;

const DOC: &str =
    "Apparently deprecated, as quest giver troop is now defined as a parameter of (start_quest).";

pub const OP_CODE: u32 = 1291;

pub const IDENT: &str = "setup_quest_giver";

impl Operation for SetupQuestGiverOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<quest_id>", ""),
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}
