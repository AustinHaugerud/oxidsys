use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentFadeOutOp;

const DOC : &str = "Fades out the agent from the scene (same effect as fleeing enemies when they get to the edge of map).";

pub const OP_CODE: u32 = 1749;

pub const IDENT: &str = "agent_fade_out";

impl Operation for AgentFadeOutOp {
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
