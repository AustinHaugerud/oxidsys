use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemSetSlotOp;

const DOC : &str = "item_get_slot                       =  527   (item_get_slot, <destination>, <item_id>, <slot_no>),";

pub const OP_CODE: u32 = 507;

pub const IDENT: &str = "item_set_slot";

impl Operation for ItemSetSlotOp {
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
