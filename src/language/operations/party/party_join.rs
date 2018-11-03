use language::operations::{Operation, ParamInfo};

pub struct PartyJoinOp;

const DOC: &str = "During encounter, joins encountered party to player's party";

pub const OP_CODE: u32 = 1201;

pub const IDENT: &str = "party_join";

impl Operation for PartyJoinOp {
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
