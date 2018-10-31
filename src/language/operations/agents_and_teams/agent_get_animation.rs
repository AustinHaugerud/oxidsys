use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetAnimationOp;

const DOC: &str =
    "Retrieves current agent animation for specified body part (0 = lower, 1 = upper).";

pub const OP_CODE: u32 = 1768;

pub const IDENT: &str = "agent_get_animation";

impl Operation for AgentGetAnimationOp {
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
