use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetScriptedDestinationOp;

const DOC : &str = "Forces the agent to travel to specified position and stay there until new behavior is set or scripted mode cleared. First optional parameter determines whether the position Z coordinate will be automatically set to ground level (value = 1) or not (value = 0). Second optional parameter added in 1.165 patch, set it to 1 to save resources.";

pub const OP_CODE: u32 = 1730;

pub const IDENT: &str = "agent_set_scripted_destination";

impl Operation for AgentSetScriptedDestinationOp {
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
            num_optional: 2,
            param_docs: vec![
                make_param_doc("<agent_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("[auto_set_z_to_ground_level]", ""),
                make_param_doc("[no_rethink]", "")
            ],
        }
    }
}
