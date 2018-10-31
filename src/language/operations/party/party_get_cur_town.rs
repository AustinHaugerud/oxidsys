use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetCurTownOp;

const DOC : &str = "When a party has reached it's destination (using ai_bhvr_travel_to_party), this operation will retrieve the party_id of the destination party.";

pub const OP_CODE: u32 = 1665;

pub const IDENT: &str = "party_get_cur_town";

impl Operation for PartyGetCurTownOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
