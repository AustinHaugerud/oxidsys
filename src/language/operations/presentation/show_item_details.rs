use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ShowItemDetailsOp;

const DOC : &str = "Shows a popup box at the specified position, containing standard game information for the specified item. Last parameter determines price percentile multiplier. Multiplier value of 100 will display item standard price, value of 0 will display \"Default Item\" instead of price (used in multiplayer equipment selection presentation).";

pub const OP_CODE: u32 = 970;

pub const IDENT: &str = "show_item_details";

impl Operation for ShowItemDetailsOp {
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
                make_param_doc("<item_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<price_multiplier_percentile>", ""),
            ],
        }
    }
}
