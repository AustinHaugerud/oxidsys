use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SceneItemGetInstanceOp;

const DOC: &str =
    "Retrieves the reference to a single instance of a scene item by it's sequential number.";

pub const OP_CODE: u32 = 1831;

pub const IDENT: &str = "scene_item_get_instance";

impl Operation for SceneItemGetInstanceOp {
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
