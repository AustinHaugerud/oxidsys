use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentStopSoundOp;

const DOC: &str = "Stops whatever sound agent is currently performing.";

pub const OP_CODE: u32 = 1808;

pub const IDENT: &str = "agent_stop_sound";

impl Operation for AgentStopSoundOp {
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
