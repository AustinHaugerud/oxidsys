use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopInventorySlotGetItemMaxAmountOp;

const DOC : &str = "Retrieves the maximum possible stack size for a specified equipment or inventory slot (if some Bread is 23/50, this operation will return 50).";

pub const OP_CODE: u32 = 1538;

pub const IDENT: &str = "troop_inventory_slot_get_item_max_amount";

impl Operation for TroopInventorySlotGetItemMaxAmountOp {
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
