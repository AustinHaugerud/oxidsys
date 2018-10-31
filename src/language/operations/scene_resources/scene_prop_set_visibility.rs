use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropSetVisibilityOp;

const DOC : &str = "Shows (value = 1) or hides (value = 0) the scene prop instance. What does it do with collision? 4research.";

pub const OP_CODE: u32 = 1813;

pub const IDENT: &str = "scene_prop_set_visibility";

impl Operation for ScenePropSetVisibilityOp {
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
                make_param_doc("<scene_prop_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
