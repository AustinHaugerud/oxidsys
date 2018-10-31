use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemHasModifierOp;

const DOC : &str = "Version 1.161+. Checks that the specified modifiers is valid for the item. See the list of imod_* values in header_item_modifiers.py.";

pub const OP_CODE: u32 = 2725;

pub const IDENT: &str = "item_has_modifier";

impl Operation for ItemHasModifierOp {
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
                make_param_doc("<item_modifier_no>", ""),
            ],
        }
    }
}
