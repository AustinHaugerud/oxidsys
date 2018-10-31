use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropFadeOutOp;

const DOC: &str = "Version 1.153+. Makes the scene prop instance disappear within specified time.";

pub const OP_CODE: u32 = 1822;

pub const IDENT: &str = "scene_prop_fade_out";

impl Operation for ScenePropFadeOutOp {
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
                make_param_doc("<fade_out_time>", ""),
            ],
        }
    }
}
