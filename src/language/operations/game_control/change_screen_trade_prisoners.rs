use language::operations::{Operation, ParamInfo};

pub struct ChangeScreenTradePrisonersOp;

const DOC : &str = "Opens the Sell Prisoners interface. Script \"script_game_get_prisoner_price\" will be used to determine prisoner price.";

pub const OP_CODE: u32 = 2044;

pub const IDENT: &str = "change_screen_trade_prisoners";

impl Operation for ChangeScreenTradePrisonersOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
