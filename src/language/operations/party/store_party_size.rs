use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StorePartySizeOp;

const DOC: &str = "Stores total party size (all members and prisoners).";

pub const OP_CODE: u32 = 2156;

pub const IDENT: &str = "store_party_size";

impl Operation for StorePartySizeOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("[party_id]", ""),
            ],
        }
    }
}
