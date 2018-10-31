use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetAnimationOp;

const DOC : &str = "Forces the agent to perform the specified animation. Optional channel_no parameter determines whether upper body (value = 1) or lower body (value = 0, default) is affected by animation.";

pub const OP_CODE: u32 = 1740;

pub const IDENT: &str = "agent_set_animation";

impl Operation for AgentSetAnimationOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<agent_id>", ""),
                make_param_doc("<anim_id>", ""),
                make_param_doc("[channel_no]", ""),
            ],
        }
    }
}
