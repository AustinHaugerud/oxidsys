use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopAddItemOp;

const DOC : &str = "Adds an item to the troop, optionally with a modifier (see imod_* constants in header_item_modifiers.py).";

pub const OP_CODE: u32 = 1530;

pub const IDENT: &str = "troop_add_item";

impl Operation for TroopAddItemOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("<item_id>", ""),
                make_param_doc("[modifier]", ""),
            ],
        }
    }
}
