use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetBanditAttractionOp;

const DOC: &str = "Sets party attractiveness to parties with bandit behavior (range 0..100).";

pub const OP_CODE: u32 = 1645;

pub const IDENT: &str = "party_set_bandit_attraction";

impl Operation for PartySetBanditAttractionOp {
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
                make_param_doc("<attaraction>", ""),
            ],
        }
    }
}
