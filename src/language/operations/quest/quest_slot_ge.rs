use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct QuestSlotGeOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 566;

pub const IDENT: &str = "quest_slot_ge";

impl Operation for QuestSlotGeOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<quest_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
