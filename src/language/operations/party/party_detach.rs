use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyDetachOp;

const DOC: &str = "Remove a party from attachments and place it on the world map.";

pub const OP_CODE: u32 = 1661;

pub const IDENT: &str = "party_detach";

impl Operation for PartyDetachOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<party_id>", "")],
        }
    }
}
