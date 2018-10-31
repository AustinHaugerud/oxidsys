use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TryForPartiesOp;

const DOC: &str = "Runs a cycle, iterating all parties on the map.";

pub const OP_CODE: u32 = 11;

pub const IDENT: &str = "try_for_parties";

impl Operation for TryForPartiesOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
