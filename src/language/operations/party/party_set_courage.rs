use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetCourageOp;

const DOC: &str = "Sets courage value for the party (range 4..15).";

pub const OP_CODE: u32 = 1607;

pub const IDENT: &str = "party_set_courage";

impl Operation for PartySetCourageOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<number>", ""),
            ],
        }
    }
}
