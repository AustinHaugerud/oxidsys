use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetAttachedPartyWithRankOp;

const DOC: &str = "Extract party_id of a specified party among attached.";

pub const OP_CODE: u32 = 1696;

pub const IDENT: &str = "party_get_attached_party_with_rank";

impl Operation for PartyGetAttachedPartyWithRankOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
                make_param_doc("<attached_party_index>", ""),
            ],
        }
    }
}
