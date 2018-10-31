use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ShowItemDetailsWithModifierOp;

const DOC: &str =
    "Same as above, but displays stats and price information for an item with a modifier.";

pub const OP_CODE: u32 = 972;

pub const IDENT: &str = "show_item_details_with_modifier";

impl Operation for ShowItemDetailsWithModifierOp {
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
            num_required: 4,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<item_id>", ""),
                make_param_doc("<item_modifier>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<price_multiplier_percentile>", ""),
            ],
        }
    }
}
