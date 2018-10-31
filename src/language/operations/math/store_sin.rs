use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreSinOp;

const DOC: &str = "Assigns dest := SIN (value)";

pub const OP_CODE: u32 = 2127;

pub const IDENT: &str = "store_sin";

impl Operation for StoreSinOp {
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
                make_param_doc("<destination_fixed_point>", ""),
                make_param_doc("<value_fixed_point>", ""),
            ],
        }
    }
}
