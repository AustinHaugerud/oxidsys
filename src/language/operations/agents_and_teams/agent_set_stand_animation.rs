use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetStandAnimationOp;

const DOC : &str = "Defines the animation that this agent will use when standing still. Does not force the agent into actually doing this animation.";

pub const OP_CODE: u32 = 1741;

pub const IDENT: &str = "agent_set_stand_animation";

impl Operation for AgentSetStandAnimationOp {
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
