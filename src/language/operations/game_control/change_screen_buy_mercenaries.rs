use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenBuyMercenariesOp;

const DOC : &str = "Opens the Buy Mercenaries interface, where player can hire troops from the party specified with (set_mercenary_source_party) operation. Only works from the dialog.";

pub const OP_CODE: u32 = 2045;

pub const IDENT: &str = "change_screen_buy_mercenaries";

impl Operation for ChangeScreenBuyMercenariesOp {
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
