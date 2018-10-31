use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetNumPrisonersOp;

const DOC: &str = "Returns total number of party prisoners.";

pub const OP_CODE: u32 = 1602;

pub const IDENT: &str = "party_get_num_prisoners";

impl Operation for PartyGetNumPrisonersOp {
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
