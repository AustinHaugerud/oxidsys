use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopInventorySlotSetItemAmountOp;

const DOC : &str = "Sets the stack size for a specified equipment or inventory slot. Only makes sense for items like ammo or food (which show stuff like \"23/50\" in inventory). Equipment slots are in range 0..9, see ek_* constants in header_items.py for reference.";

pub const OP_CODE: u32 = 1534;

pub const IDENT: &str = "troop_inventory_slot_set_item_amount";

impl Operation for TroopInventorySlotSetItemAmountOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
