use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetSpawnPositionOp;

const DOC : &str = "Defines the position which will later be used by (spawn_scene_prop), (spawn_scene_item), (spawn_agent) and (spawn_horse) operations.";

pub const OP_CODE: u32 = 1970;

pub const IDENT: &str = "set_spawn_position";

impl Operation for SetSpawnPositionOp {
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
            param_docs: vec![make_param_doc("<position>", "")],
        }
    }
}
