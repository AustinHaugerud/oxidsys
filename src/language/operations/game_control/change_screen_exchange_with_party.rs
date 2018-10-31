use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenExchangeWithPartyOp;

const DOC : &str = "Effectively duplicates (change_screen_exchange_members), but party_id parameter is obligatory and the operation doesn't have an option to prevent party leader from being exchanged.";

pub const OP_CODE: u32 = 2050;

pub const IDENT: &str = "change_screen_exchange_with_party";

impl Operation for ChangeScreenExchangeWithPartyOp {
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
