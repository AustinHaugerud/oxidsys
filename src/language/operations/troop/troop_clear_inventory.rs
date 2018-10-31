use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopClearInventoryOp;

const DOC: &str = "Clears entire troop inventory. Does not affect equipped items.";

pub const OP_CODE: u32 = 1532;

pub const IDENT: &str = "troop_clear_inventory";

impl Operation for TroopClearInventoryOp {
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
