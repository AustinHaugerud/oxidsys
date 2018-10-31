use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyCanJoinPartyOp;

const DOC : &str = "Checks if first party can join second party (enough space for both troops and prisoners). If flip_prisoners flag is 1, then members and prisoners in the joinning party are flipped.";

pub const OP_CODE: u32 = 107;

pub const IDENT: &str = "party_can_join_party";

impl Operation for PartyCanJoinPartyOp {
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
                make_param_doc("<joiner_party_id>", ""),
                make_param_doc("<host_party_id>", ""),
                make_param_doc("[flip_prisoners]", ""),
            ],
        }
    }
}
