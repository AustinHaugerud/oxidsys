use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemHasFactionOp;

const DOC : &str = "Version 1.161+. Checks that the item is available for specified faction. Note that an item with no factions set is available to all factions.";

pub const OP_CODE: u32 = 2726;

pub const IDENT: &str = "item_has_faction";

impl Operation for ItemHasFactionOp {
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
                make_param_doc("<item_kind_no>", ""),
                make_param_doc("<faction_no>", ""),
            ],
        }
    }
}
