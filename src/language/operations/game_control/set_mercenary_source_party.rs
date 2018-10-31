use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetMercenarySourcePartyOp;

const DOC : &str = "Defines the party from which the player will buy mercenaries with (change_screen_buy_mercenaries).";

pub const OP_CODE: u32 = 1320;

pub const IDENT: &str = "set_mercenary_source_party";

impl Operation for SetMercenarySourcePartyOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<party_id>", "")],
        }
    }
}
