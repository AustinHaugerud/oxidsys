use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreFreeInventoryCapacityOp;

const DOC: &str =
    "Calculates total number of free inventory slots that the troop has. Default troop is player.";

pub const OP_CODE: u32 = 2167;

pub const IDENT: &str = "store_free_inventory_capacity";

impl Operation for StoreFreeInventoryCapacityOp {
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
            num_required: 1,
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("[troop_id]", ""),
            ],
        }
    }
}
