use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct QuestSetSlotOp;

const DOC : &str = "quest_get_slot                =  526   (quest_get_slot, <destination>, <quest_id>, <slot_no>),";

pub const OP_CODE: u32 = 506;

pub const IDENT: &str = "quest_set_slot";

impl Operation for QuestSetSlotOp {
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
