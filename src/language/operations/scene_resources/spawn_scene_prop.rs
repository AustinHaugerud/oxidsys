use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SpawnScenePropOp;

const DOC : &str = "Spawns a new scene prop instance of the specified type at the position defined by the last call to (set_spawn_position). Operation was supposed to store the prop_instance_id of the spawned position in reg0, but does not do this at the moment.";

pub const OP_CODE: u32 = 1974;

pub const IDENT: &str = "spawn_scene_prop";

impl Operation for SpawnScenePropOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<scene_prop_id>", "")],
        }
    }
}
