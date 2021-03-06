use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StorePartnerFactionOp;

const DOC: &str = "Stores faction of the troop player is speaking to.";

pub const OP_CODE: u32 = 2201;

pub const IDENT: &str = "store_partner_faction";

impl Operation for StorePartnerFactionOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
