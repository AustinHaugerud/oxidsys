use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropGetNumInstancesOp;

const DOC: &str =
    "Retrieves the total number of instances of a specified scene prop on the current scene.";

pub const OP_CODE: u32 = 1810;

pub const IDENT: &str = "scene_prop_get_num_instances";

impl Operation for ScenePropGetNumInstancesOp {
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
                make_param_doc("<scene_prop_id>", ""),
            ],
        }
    }
}
