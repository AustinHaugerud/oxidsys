use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopRemoveItemOp;

const DOC : &str = "Removes an item from the troop equipment or inventory. Operation will remove first matching item it finds.";

pub const OP_CODE: u32 = 1531;

pub const IDENT: &str = "troop_remove_item";

impl Operation for TroopRemoveItemOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}
