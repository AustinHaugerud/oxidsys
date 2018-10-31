use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetHelpfulnessOp;

const DOC: &str = "Gets party current AI helpfulness value (range 0..100).";

pub const OP_CODE: u32 = 1646;

pub const IDENT: &str = "party_get_helpfulness";

impl Operation for PartyGetHelpfulnessOp {
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
