use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetSpawnEffectorScenePropKindOp;

const DOC : &str = "Specifies some scene prop kind as one of the teams' spawn effector, making players of that team more likely to spawn closer to the specified effector prop instances. Use -1 to disable spawn effector for a team.";

pub const OP_CODE: u32 = 426;

pub const IDENT: &str = "set_spawn_effector_scene_prop_kind";

impl Operation for SetSpawnEffectorScenePropKindOp {
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
                make_param_doc("<scene_prop_kind_no>", ""),
            ],
        }
    }
}
