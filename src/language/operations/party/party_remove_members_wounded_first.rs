use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyRemoveMembersWoundedFirstOp;

const DOC : &str = "Removes a certain number of troops from the party, starting with wounded. Stores total number removed in reg0.";

pub const OP_CODE: u32 = 1619;

pub const IDENT: &str = "party_remove_members_wounded_first";

impl Operation for PartyRemoveMembersWoundedFirstOp {
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
