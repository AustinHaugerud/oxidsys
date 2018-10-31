use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct HeroCanJoinAsPrisonerOp;

const DOC: &str =
    "Checks if party can accept one hero prisoner troop. Player's party is default value.";

pub const OP_CODE: u32 = 102;

pub const IDENT: &str = "hero_can_join_as_prisoner";

impl Operation for HeroCanJoinAsPrisonerOp {
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
            num_optional: 1,
            param_docs: vec![make_param_doc("[party_id]", "")],
        }
    }
}
