use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyEndBattleOp;

const DOC : &str = "Version 1.153+. UNTESTED. Supposedly ends the battle in which the party is currently participating.";

pub const OP_CODE: u32 = 108;

pub const IDENT: &str = "party_end_battle";

impl Operation for PartyEndBattleOp {
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
            param_docs: vec![make_param_doc("<party_no>", "")],
        }
    }
}
