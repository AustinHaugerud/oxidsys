use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreMulOp;

const DOC: &str = "Assigns <destination> := <value> * <value>";

pub const OP_CODE: u32 = 2122;

pub const IDENT: &str = "store_mul";

impl Operation for StoreMulOp {
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
