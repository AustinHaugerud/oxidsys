use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TryBeginOp;

const DOC: &str = "Opens a conditional block.";

pub const OP_CODE: u32 = 4;

pub const IDENT: &str = "try_begin";

impl Operation for TryBeginOp {
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
