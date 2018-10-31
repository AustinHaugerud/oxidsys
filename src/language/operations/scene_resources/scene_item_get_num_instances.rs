use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SceneItemGetNumInstancesOp;

const DOC : &str = "Gets the number of specified scene items present on the scene. Scene items behave exactly like scene props (i.e. cannot be picked).";

pub const OP_CODE: u32 = 1830;

pub const IDENT: &str = "scene_item_get_num_instances";

impl Operation for SceneItemGetNumInstancesOp {
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
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}
