use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopRemoveItemsOp;

const DOC : &str = "Removes multiple items of specified type from the troop. Total price of actually removed items will be stored in reg0.";

pub const OP_CODE: u32 = 1536;

pub const IDENT: &str = "troop_remove_items";

impl Operation for TroopRemoveItemsOp {
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
                make_param_doc("<item_id>", ""),
                make_param_doc("<number>", ""),
            ],
        }
    }
}
