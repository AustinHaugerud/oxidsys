use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct NegOp;

const DOC: &str = "Used in combination with conditional operations to invert their results.";

pub const OP_CODE: u32 = 2147483648;

pub const IDENT: &str = "neg";

impl Operation for NegOp {
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
            param_docs: vec![make_param_doc("<operation_name>", "")],
        }
    }
}
