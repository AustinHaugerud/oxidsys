use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyIsInTownOp;

const DOC : &str = "Checks that the party has successfully reached it's destination (after being set to ai_bhvr_travel_to_party) and that it's destination is actually the referenced town_party_id.";

pub const OP_CODE: u32 = 130;

pub const IDENT: &str = "party_is_in_town";

impl Operation for PartyIsInTownOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<town_party_id>", ""),
            ],
        }
    }
}
