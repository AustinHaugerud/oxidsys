use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopAddItemsOp;

const DOC: &str = "Adds multiple items of specified type to the troop.";

pub const OP_CODE: u32 = 1535;

pub const IDENT: &str = "troop_add_items";

impl Operation for TroopAddItemsOp {
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
