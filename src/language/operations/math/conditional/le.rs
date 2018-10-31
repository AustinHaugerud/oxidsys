use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct LeOp;

const DOC: &str = "Checks that value1 <= value2";

pub const OP_CODE: u32 = 2147483680;

pub const IDENT: &str = "le";

impl Operation for LeOp {
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
                make_param_doc("<value1>", ""),
                make_param_doc("<value2>", ""),
            ],
        }
    }
}
