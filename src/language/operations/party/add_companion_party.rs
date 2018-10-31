use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddCompanionPartyOp;

const DOC : &str = "Creates a new empty party with specified hero as party leader and the only member. Party is spawned at the position of player's party.";

pub const OP_CODE: u32 = 1233;

pub const IDENT: &str = "add_companion_party";

impl Operation for AddCompanionPartyOp {
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
            num_optional: 0,
            param_docs: vec![make_param_doc("<troop_id_hero>", "")],
        }
    }
}
