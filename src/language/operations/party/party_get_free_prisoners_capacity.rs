use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetFreePrisonersCapacityOp;

const DOC: &str = "Calculates how many prisoners can be added to the party.";

pub const OP_CODE: u32 = 1634;

pub const IDENT: &str = "party_get_free_prisoners_capacity";

impl Operation for PartyGetFreePrisonersCapacityOp {
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
