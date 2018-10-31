use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ReplaceSceneItemsWithScenePropsOp;

const DOC : &str = "Replaces all instances of specified scene item with scene props. Can only be called in ti_before_mission_start trigger in module_mission_templates.py.";

pub const OP_CODE: u32 = 1891;

pub const IDENT: &str = "replace_scene_items_with_scene_props";

impl Operation for ReplaceSceneItemsWithScenePropsOp {
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
                make_param_doc("<old_item_id>", ""),
                make_param_doc("<new_scene_prop_id>", ""),
            ],
        }
    }
}
