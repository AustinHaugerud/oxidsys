use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentAiGetMoveTargetOp;

const DOC : &str = "Version 1.153+. UNTESTED. Supposedly returns the enemy agent to whom the agent is currently moving to.";

pub const OP_CODE: u32 = 2081;

pub const IDENT: &str = "agent_ai_get_move_target";

impl Operation for AgentAiGetMoveTargetOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
