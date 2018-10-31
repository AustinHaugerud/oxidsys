use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetTimeElapsedSinceRemovedOp;

const DOC : &str = "Retrieves the number of seconds that have passed since agent's death. Native uses this only for multiplayer to track player's respawns. Can it be used in singleplayer too? 4research.";

pub const OP_CODE: u32 = 1760;

pub const IDENT: &str = "agent_get_time_elapsed_since_removed";

impl Operation for AgentGetTimeElapsedSinceRemovedOp {
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
