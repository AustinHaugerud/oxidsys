use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetSpawnEffectorScenePropIdOp;

const DOC : &str = "Specifies a single prop instance as a team's spawn effector. Different from (set_spawn_effector_scene_prop_kind) as other instances of the same scene prop will not affect player spawning.";

pub const OP_CODE: u32 = 427;

pub const IDENT: &str = "set_spawn_effector_scene_prop_id";

impl Operation for SetSpawnEffectorScenePropIdOp {
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
                make_param_doc("<team_no>", ""),
                make_param_doc("<scene_prop_id>", ""),
            ],
        }
    }
}
