use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSortInventoryOp;

const DOC: &str = "Sorts items in troop inventory by their price (expensive first).";

pub const OP_CODE: u32 = 1511;

pub const IDENT: &str = "troop_sort_inventory";

impl Operation for TroopSortInventoryOp {
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
            num_optional: 0,
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
