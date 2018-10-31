use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSpawnNewAgentOp;

const DOC : &str = "Spawns a new agent for the specified player. Essentially a combination of (spawn_agent) and (player_control_agent) operations.";

pub const OP_CODE: u32 = 409;

pub const IDENT: &str = "player_spawn_new_agent";

impl Operation for PlayerSpawnNewAgentOp {
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
                make_param_doc("<player_id>", ""),
                make_param_doc("<entry_point>", ""),
            ],
        }
    }
}
