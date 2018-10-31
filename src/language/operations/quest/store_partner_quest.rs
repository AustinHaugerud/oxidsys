use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StorePartnerQuestOp;

const DOC : &str = "During conversation, if there's a quest given by conversation partner, the operation will return it's id.";

pub const OP_CODE: u32 = 2240;

pub const IDENT: &str = "store_partner_quest";

impl Operation for StorePartnerQuestOp {
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
