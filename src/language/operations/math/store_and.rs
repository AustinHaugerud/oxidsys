use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreAndOp;

const DOC: &str = "Binary AND";

pub const OP_CODE: u32 = 2117;

pub const IDENT: &str = "store_and";

impl Operation for StoreAndOp {
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
                make_param_doc("<value>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
