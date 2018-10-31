use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyRemoveMembersOp;

const DOC : &str = "Removes specified number of troops from a party. Stores number of actually removed troops in reg0.";

pub const OP_CODE: u32 = 1615;

pub const IDENT: &str = "party_remove_members";

impl Operation for PartyRemoveMembersOp {
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
