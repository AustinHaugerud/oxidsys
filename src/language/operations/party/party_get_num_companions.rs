use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetNumCompanionsOp;

const DOC: &str = "Returns total number of party members, including leader.";

pub const OP_CODE: u32 = 1601;

pub const IDENT: &str = "party_get_num_companions";

impl Operation for PartyGetNumCompanionsOp {
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
