use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenTradeOp;

const DOC : &str = "Opens the Trade screen, using the provided troop as the trading partner. When called from module_dialogs, troop_id is optional and defaults to current dialogue partner.";

pub const OP_CODE: u32 = 2042;

pub const IDENT: &str = "change_screen_trade";

impl Operation for ChangeScreenTradeOp {
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
            num_optional: 1,
            param_docs: vec![make_param_doc(
                "[troop_id]",
                "Optional if called from dialog.",
            )],
        }
    }
}
