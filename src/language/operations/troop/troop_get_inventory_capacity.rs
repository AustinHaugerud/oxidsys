use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopGetInventoryCapacityOp;

const DOC : &str = "Returns the total inventory capacity (number of inventory slots) for the specified troop. Note that this number will include equipment slots as well. Substract num_equipment_kinds (see header_items.py) to get the number of actual *inventory* slots.";

pub const OP_CODE: u32 = 1540;

pub const IDENT: &str = "troop_get_inventory_capacity";

impl Operation for TroopGetInventoryCapacityOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<troop_id>", ""),
            ],
        }
    }
}
