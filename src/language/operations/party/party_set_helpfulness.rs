use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetHelpfulnessOp;

const DOC: &str = "Sets AI helpfulness value for the party (range 0..10000, default 100).";

pub const OP_CODE: u32 = 1647;

pub const IDENT: &str = "party_set_helpfulness";

impl Operation for PartySetHelpfulnessOp {
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
