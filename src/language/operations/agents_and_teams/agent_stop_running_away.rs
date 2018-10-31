use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentStopRunningAwayOp;

const DOC: &str = "Cancels fleeing behavior for the agent, turning him back to combat state.";

pub const OP_CODE: u32 = 1752;

pub const IDENT: &str = "agent_stop_running_away";

impl Operation for AgentStopRunningAwayOp {
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
