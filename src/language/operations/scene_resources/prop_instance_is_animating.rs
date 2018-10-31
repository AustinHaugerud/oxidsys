use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceIsAnimatingOp;

const DOC: &str = "Checks that the scene prop instance is currently animating.";

pub const OP_CODE: u32 = 1862;

pub const IDENT: &str = "prop_instance_is_animating";

impl Operation for PropInstanceIsAnimatingOp {
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
