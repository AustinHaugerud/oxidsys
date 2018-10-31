use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetNumAttachedPartiesOp;

const DOC: &str = "Retrieves total number of parties attached to referenced party.";

pub const OP_CODE: u32 = 1695;

pub const IDENT: &str = "party_get_num_attached_parties";

impl Operation for PartyGetNumAttachedPartiesOp {
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
