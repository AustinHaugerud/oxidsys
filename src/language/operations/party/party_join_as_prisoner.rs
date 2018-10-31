use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyJoinAsPrisonerOp;

const DOC: &str = "During encounter, joins encountered party to player's party as prisoners";

pub const OP_CODE: u32 = 1202;

pub const IDENT: &str = "party_join_as_prisoner";

impl Operation for PartyJoinAsPrisonerOp {
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
