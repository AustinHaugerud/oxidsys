use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceGetAnimationTargetPositionOp;

const DOC: &str = "Retrieves the position that the prop instance is currently animating to.";

pub const OP_CODE: u32 = 1863;

pub const IDENT: &str = "prop_instance_get_animation_target_position";

impl Operation for PropInstanceGetAnimationTargetPositionOp {
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
                make_param_doc("<pos>", ""),
                make_param_doc("<scene_prop_id>", ""),
            ],
        }
    }
}
