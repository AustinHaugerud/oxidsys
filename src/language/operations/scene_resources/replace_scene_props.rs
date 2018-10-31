use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ReplaceScenePropsOp;

const DOC : &str = "Replaces all instances of specified scene prop type with another scene prop type. Commonly used to replace damaged walls with their intact versions during normal visits to castle scenes. Can only be called in ti_before_mission_start trigger in module_mission_templates.py.";

pub const OP_CODE: u32 = 1890;

pub const IDENT: &str = "replace_scene_props";

impl Operation for ReplaceScenePropsOp {
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
                make_param_doc("<old_scene_prop_id>", ""),
                make_param_doc("<new_scene_prop_id>", ""),
            ],
        }
    }
}
