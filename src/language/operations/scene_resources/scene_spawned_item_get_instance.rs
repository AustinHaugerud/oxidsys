use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SceneSpawnedItemGetInstanceOp;

const DOC: &str =
    "Retrieves the reference to a single instance of a spawned item by it's sequential number.";

pub const OP_CODE: u32 = 1833;

pub const IDENT: &str = "scene_spawned_item_get_instance";

impl Operation for SceneSpawnedItemGetInstanceOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<item_id>", ""),
                make_param_doc("<instance_no>", ""),
            ],
        }
    }
}
