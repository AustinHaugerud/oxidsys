use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropFadeInOp;

const DOC: &str = "Version 1.153+. Makes the scene prop instance reappear within specified time.";

pub const OP_CODE: u32 = 1823;

pub const IDENT: &str = "scene_prop_fade_in";

impl Operation for ScenePropFadeInOp {
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
                make_param_doc("<fade_in_time>", ""),
            ],
        }
    }
}
