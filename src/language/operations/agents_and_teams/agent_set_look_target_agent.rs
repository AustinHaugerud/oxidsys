use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetLookTargetAgentOp;

const DOC : &str = "Forces the agent to look at specified agent (track his movements). Alarmed agents will ignore this.";

pub const OP_CODE: u32 = 1713;

pub const IDENT: &str = "agent_set_look_target_agent";

impl Operation for AgentSetLookTargetAgentOp {
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
                make_param_doc("<watcher_agent_id>", ""),
                make_param_doc("<observed_agent_id>", ""),
            ],
        }
    }
}
