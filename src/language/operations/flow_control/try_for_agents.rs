use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TryForAgentsOp;

const DOC: &str = "Runs a cycle, iterating all agents on the scene.";

pub const OP_CODE: u32 = 12;

pub const IDENT: &str = "try_for_agents";

impl Operation for TryForAgentsOp {
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
