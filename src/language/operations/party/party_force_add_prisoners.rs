use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyForceAddPrisonersOp;

const DOC: &str =
    "Adds prisoners to party ignoring party size limits. Mostly used to add hero prisoners.";

pub const OP_CODE: u32 = 1614;

pub const IDENT: &str = "party_force_add_prisoners";

impl Operation for PartyForceAddPrisonersOp {
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
