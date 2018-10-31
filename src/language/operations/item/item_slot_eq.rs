use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemSlotEqOp;

const DOC: &str =
    "item_slot_ge                        =  567   (item_slot_ge, <item_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 547;

pub const IDENT: &str = "item_slot_eq";

impl Operation for ItemSlotEqOp {
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
                make_param_doc("<item_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
