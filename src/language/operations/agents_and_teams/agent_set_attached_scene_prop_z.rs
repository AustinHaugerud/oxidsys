use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetAttachedScenePropZOp;

const DOC : &str = "Offsets the position of the attached scene prop in relation to agent, in centimeters, along the Z axis (down/up).";

pub const OP_CODE: u32 = 1759;

pub const IDENT: &str = "agent_set_attached_scene_prop_z";

impl Operation for AgentSetAttachedScenePropZOp {
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
