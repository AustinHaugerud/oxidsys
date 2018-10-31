use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetAttachedScenePropOp;

const DOC : &str = "Retrieves the reference to scene prop instance which is attached to the agent, or -1 if there isn't any.";

pub const OP_CODE: u32 = 1756;

pub const IDENT: &str = "agent_get_attached_scene_prop";

impl Operation for AgentGetAttachedScenePropOp {
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
