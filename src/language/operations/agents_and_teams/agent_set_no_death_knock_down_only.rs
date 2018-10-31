use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetNoDeathKnockDownOnlyOp;

const DOC : &str = "Sets the agent as unkillable (value = 1) or normal (value = 0). Unkillable agents will drop on the ground instead of dying and will stand up afterwards.";

pub const OP_CODE: u32 = 1733;

pub const IDENT: &str = "agent_set_no_death_knock_down_only";

impl Operation for AgentSetNoDeathKnockDownOnlyOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
