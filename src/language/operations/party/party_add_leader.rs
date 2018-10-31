use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyAddLeaderOp;

const DOC: &str = "Adds troop(s) to the party and makes it party leader.";

pub const OP_CODE: u32 = 1612;

pub const IDENT: &str = "party_add_leader";

impl Operation for PartyAddLeaderOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<party_id>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("[number]", ""),
            ],
        }
    }
}
