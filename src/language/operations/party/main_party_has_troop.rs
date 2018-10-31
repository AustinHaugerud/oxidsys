use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MainPartyHasTroopOp;

const DOC: &str = "Checks if player party has specified troop.";

pub const OP_CODE: u32 = 110;

pub const IDENT: &str = "main_party_has_troop";

impl Operation for MainPartyHasTroopOp {
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
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
