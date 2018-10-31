use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct QuestGetSlotOp;

const DOC: &str =
    "quest_slot_eq                 =  546   (quest_slot_eq, <quest_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 526;

pub const IDENT: &str = "quest_get_slot";

impl Operation for QuestGetSlotOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<quest_id>", ""),
                make_param_doc("<slot_no>", ""),
            ],
        }
    }
}
