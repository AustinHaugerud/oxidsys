use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreSqrtOp;

const DOC: &str = "Assigns dest := SQRT (value)";

pub const OP_CODE: u32 = 2125;

pub const IDENT: &str = "store_sqrt";

impl Operation for StoreSqrtOp {
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
