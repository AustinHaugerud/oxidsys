use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ReplacePropInstanceOp;

const DOC : &str = "Replaces a single scene prop instance with an instance of another scene prop (usually with the same dimensions, but not necessarily so). Can only be called in ti_before_mission_start trigger in module_mission_templates.py.";

pub const OP_CODE: u32 = 1889;

pub const IDENT: &str = "replace_prop_instance";

impl Operation for ReplacePropInstanceOp {
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
                make_param_doc("<new_scene_prop_id>", ""),
            ],
        }
    }
}
