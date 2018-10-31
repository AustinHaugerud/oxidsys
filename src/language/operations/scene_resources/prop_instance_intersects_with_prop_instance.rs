use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceIntersectsWithPropInstanceOp;

const DOC : &str = "Checks if two scene props are intersecting (i.e. collided). Useful when animating scene props movement. Pass -1 for second parameter to check the prop against all other props on the scene.";

pub const OP_CODE: u32 = 1880;

pub const IDENT: &str = "prop_instance_intersects_with_prop_instance";

impl Operation for PropInstanceIntersectsWithPropInstanceOp {
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
                make_param_doc("<checked_scene_prop_id>", ""),
                make_param_doc("<scene_prop_id>", ""),
            ],
        }
    }
}
