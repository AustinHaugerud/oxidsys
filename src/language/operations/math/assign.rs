use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AssignOp;

const DOC: &str = "Directly assigns a value to a variable or register.";

pub const OP_CODE: u32 = 2133;

pub const IDENT: &str = "assign";

impl Operation for AssignOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
