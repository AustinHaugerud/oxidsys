use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlayAddItemOp;

const DOC : &str = "Adds an item to the listbox or combobox. Items are indexed from 0. Note the order in which items appear in the dropdown is reverse to the order in which they're added.";

pub const OP_CODE: u32 = 931;

pub const IDENT: &str = "overlay_add_item";

impl Operation for OverlayAddItemOp {
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
                make_param_doc("<overlay_id>", ""),
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}
