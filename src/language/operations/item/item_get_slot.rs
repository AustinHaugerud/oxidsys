use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemGetSlotOp;

const DOC: &str =
    "item_slot_eq                        =  547   (item_slot_eq, <item_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 527;

pub const IDENT: &str = "item_get_slot";

impl Operation for ItemGetSlotOp {
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
                make_param_doc("<item_id>", ""),
                make_param_doc("<slot_no>", ""),
            ],
        }
    }
}
