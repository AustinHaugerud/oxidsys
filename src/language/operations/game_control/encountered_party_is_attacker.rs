use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct EncounteredPartyIsAttackerOp;

const DOC : &str = "Checks that the party encountered on the world map was following player (i.e. either player was trying to run away or at the very least this is a head-on clash).";

pub const OP_CODE: u32 = 39;

pub const IDENT: &str = "encountered_party_is_attacker";

impl Operation for EncounteredPartyIsAttackerOp {
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
