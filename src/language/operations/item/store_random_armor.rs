use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRandomArmorOp;

const DOC: &str = "Deprecated since early M&B days.";

pub const OP_CODE: u32 = 2259;

pub const IDENT: &str = "store_random_armor";

impl Operation for StoreRandomArmorOp {
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
