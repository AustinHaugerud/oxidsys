use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropGetInstanceOp;

const DOC: &str = "Retrieves the reference to a scene prop instance by it's number.";

pub const OP_CODE: u32 = 1811;

pub const IDENT: &str = "scene_prop_get_instance";

impl Operation for ScenePropGetInstanceOp {
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
                make_param_doc("<scene_prop_id>", ""),
                make_param_doc("<instance_no>", ""),
            ],
        }
    }
}
