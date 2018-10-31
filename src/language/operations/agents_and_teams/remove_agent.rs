use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RemoveAgentOp;

const DOC: &str = "Immediately removes the agent from the scene.";

pub const OP_CODE: u32 = 1755;

pub const IDENT: &str = "remove_agent";

impl Operation for RemoveAgentOp {
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
            param_docs: vec![make_param_doc("<agent_id>", "")],
        }
    }
}
