use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyWoundMembersOp;

const DOC: &str = "Wounds a specified number of troops in the party.";

pub const OP_CODE: u32 = 1618;

pub const IDENT: &str = "party_wound_members";

impl Operation for PartyWoundMembersOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<party_id>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("<number>", ""),
            ],
        }
    }
}
