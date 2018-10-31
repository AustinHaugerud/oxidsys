use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetInventorySlotOp;

const DOC : &str = "Puts the specified item into troop's equipment or inventory slot. Be careful with setting equipment slots this way.";

pub const OP_CODE: u32 = 1543;

pub const IDENT: &str = "troop_set_inventory_slot";

impl Operation for TroopSetInventorySlotOp {
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
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}
