use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropHasAgentOnItOp;

const DOC: &str = "Checks that the specified agent is standing on the scene prop instance.";

pub const OP_CODE: u32 = 1801;

pub const IDENT: &str = "scene_prop_has_agent_on_it";

impl Operation for ScenePropHasAgentOnItOp {
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
                make_param_doc("<scene_prop_instance_id>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
