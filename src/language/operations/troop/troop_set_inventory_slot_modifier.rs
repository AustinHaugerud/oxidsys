use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetInventorySlotModifierOp;

const DOC : &str = "Sets the modifier for the item in the troop's equipment or inventory slot. See imod_* constants in header_items.py for reference.";

pub const OP_CODE: u32 = 1544;

pub const IDENT: &str = "troop_set_inventory_slot_modifier";

impl Operation for TroopSetInventorySlotModifierOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<inventory_slot_no>", ""),
                make_param_doc("<imod_value>", ""),
            ],
        }
    }
}
