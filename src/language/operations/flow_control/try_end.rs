use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TryEndOp;

const DOC: &str = "Concludes a conditional block or a cycle.";

pub const OP_CODE: u32 = 3;

pub const IDENT: &str = "try_end";

impl Operation for TryEndOp {
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
