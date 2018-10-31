use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetAttachedScenePropYOp;

const DOC : &str = "Offsets the position of the attached scene prop in relation to agent, in centimeters, along the Y axis (backwards/forward).";

pub const OP_CODE: u32 = 1809;

pub const IDENT: &str = "agent_set_attached_scene_prop_y";

impl Operation for AgentSetAttachedScenePropYOp {
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
