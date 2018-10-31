use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RemoveMemberFromPartyOp;

const DOC : &str = "Removes hero member from party. Player party is default value. Will display a message about companion leaving the party. Should not be used with regular troops (it will successfully remove one of them, but will produce some meaningless spam).";

pub const OP_CODE: u32 = 1210;

pub const IDENT: &str = "remove_member_from_party";

impl Operation for RemoveMemberFromPartyOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("[party_id]", ""),
            ],
        }
    }
}
