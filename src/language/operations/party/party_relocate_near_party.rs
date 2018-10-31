use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyRelocateNearPartyOp;

const DOC: &str = "Teleports party into vicinity of another party.";

pub const OP_CODE: u32 = 1623;

pub const IDENT: &str = "party_relocate_near_party";

impl Operation for PartyRelocateNearPartyOp {
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
                make_param_doc("<relocated_party_id>", ""),
                make_param_doc("<target_party_id>", ""),
                make_param_doc("<spawn_radius>", ""),
            ],
        }
    }
}
