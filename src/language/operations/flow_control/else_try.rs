use language::operations::{Operation, ParamInfo};

pub struct ElseTryOp;

const DOC: &str =
    "If conditional operations in the conditional block fail, this block of code will be executed.";

pub const OP_CODE: u32 = 5;

pub const IDENT: &str = "else_try";

impl Operation for ElseTryOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
