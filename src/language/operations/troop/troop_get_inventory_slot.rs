use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopGetInventorySlotOp;

const DOC : &str = "Retrieves the item_id of a specified equipment or inventory slot. Returns -1 when there's nothing there.";

pub const OP_CODE: u32 = 1541;

pub const IDENT: &str = "troop_get_inventory_slot";

impl Operation for TroopGetInventorySlotOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<inventory_slot_no>", ""),
            ],
        }
    }
}
