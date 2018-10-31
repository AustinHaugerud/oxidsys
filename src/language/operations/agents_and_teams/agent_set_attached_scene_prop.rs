use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetAttachedScenePropOp;

const DOC : &str = "Attaches the specified prop instance to the agent. Used in multiplayer CTF missions to attach flags to players.";

pub const OP_CODE: u32 = 1757;

pub const IDENT: &str = "agent_set_attached_scene_prop";

impl Operation for AgentSetAttachedScenePropOp {
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
                make_param_doc("<scene_prop_id>", ""),
            ],
        }
    }
}
