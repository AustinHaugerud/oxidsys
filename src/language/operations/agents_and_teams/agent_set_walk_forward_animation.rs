use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetWalkForwardAnimationOp;

const DOC : &str = "Defines the animation that this agent will use when walking forward. Only works for NPC agents.";

pub const OP_CODE: u32 = 1742;

pub const IDENT: &str = "agent_set_walk_forward_animation";

impl Operation for AgentSetWalkForwardAnimationOp {
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
                make_param_doc("<anim_id>", ""),
            ],
        }
    }
}
