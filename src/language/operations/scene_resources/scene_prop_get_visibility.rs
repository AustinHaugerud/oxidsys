use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropGetVisibilityOp;

const DOC : &str = "Retrieves the current visibility state of the scene prop instance (1 = visible, 0 = invisible).";

pub const OP_CODE: u32 = 1812;

pub const IDENT: &str = "scene_prop_get_visibility";

impl Operation for ScenePropGetVisibilityOp {
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
