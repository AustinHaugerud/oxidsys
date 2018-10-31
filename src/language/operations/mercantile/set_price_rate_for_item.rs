use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetPriceRateForItemOp;

const DOC : &str = "Sets individual price rate for a single item type. Normal price rate is 100. Deprecated, as Warband uses (game_get_item_[buy/sell]_price_factor) scripts instead.";

pub const OP_CODE: u32 = 1171;

pub const IDENT: &str = "set_price_rate_for_item";

impl Operation for SetPriceRateForItemOp {
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
                make_param_doc("<item_id>", ""),
                make_param_doc("<value_percentage>", ""),
            ],
        }
    }
}
