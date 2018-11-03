use language::operations::{Operation, ParamInfo};

pub struct PartyCanJoinOp;

const DOC: &str = "During encounter dialog, checks if encountered party can join player's party.";

pub const OP_CODE: u32 = 103;

pub const IDENT: &str = "party_can_join";

impl Operation for PartyCanJoinOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
