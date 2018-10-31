use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyCanJoinAsPrisonerOp;

const DOC: &str =
    "During encounter dialog, checks if encountered party can join player's party as prisoners.";

pub const OP_CODE: u32 = 104;

pub const IDENT: &str = "party_can_join_as_prisoner";

impl Operation for PartyCanJoinAsPrisonerOp {
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
