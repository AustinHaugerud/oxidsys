use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopGetInventorySlotModifierOp;

const DOC : &str = "Retrieves the modifier value (see imod_* constants in header_items.py) for an item in the specified equipment or inventory slot. Returns 0 when there's nothing there, or if item does not have any modifiers.";

pub const OP_CODE: u32 = 1542;

pub const IDENT: &str = "troop_get_inventory_slot_modifier";

impl Operation for TroopGetInventorySlotModifierOp {
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
