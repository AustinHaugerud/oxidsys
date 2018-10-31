use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetAttachedToOp;

const DOC: &str = "Retrieves the party that the referenced party is attached to, if any.";

pub const OP_CODE: u32 = 1694;

pub const IDENT: &str = "party_get_attached_to";

impl Operation for PartyGetAttachedToOp {
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
