use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyForceAddMembersOp;

const DOC: &str =
    "Adds troops to party ignoring party size limits. Mostly used to add hero troops.";

pub const OP_CODE: u32 = 1613;

pub const IDENT: &str = "party_force_add_members";

impl Operation for PartyForceAddMembersOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("<number>", ""),
            ],
        }
    }
}
