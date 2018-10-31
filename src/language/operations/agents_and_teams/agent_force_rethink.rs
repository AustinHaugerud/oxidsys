use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentForceRethinkOp;

const DOC : &str = "Forces the agent to recalculate his current actions after setting him a new scripted destination or changing other factors affecting his behavior.";

pub const OP_CODE: u32 = 1732;

pub const IDENT: &str = "agent_force_rethink";

impl Operation for AgentForceRethinkOp {
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
