use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetNumCompanionStacksOp;

const DOC: &str =
    "Returns total number of troop stacks in the party (including player and heroes).";

pub const OP_CODE: u32 = 1650;

pub const IDENT: &str = "party_get_num_companion_stacks";

impl Operation for PartyGetNumCompanionStacksOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
