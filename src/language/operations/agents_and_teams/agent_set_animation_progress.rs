use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetAnimationProgressOp;

const DOC : &str = "Allows to skip the agent to a certain point in the animation cycle, as specified by the fixed point value (0..fixed_point_multiplier).";

pub const OP_CODE: u32 = 1743;

pub const IDENT: &str = "agent_set_animation_progress";

impl Operation for AgentSetAnimationProgressOp {
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
                make_param_doc("<value_fixed_point>", ""),
            ],
        }
    }
}
