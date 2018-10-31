use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StartQuestOp;

const DOC: &str = "Starts the quest and marks giver_troop as the troop who gave it.";

pub const OP_CODE: u32 = 1280;

pub const IDENT: &str = "start_quest";

impl Operation for StartQuestOp {
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
                make_param_doc("<giver_troop_id>", ""),
            ],
        }
    }
}
