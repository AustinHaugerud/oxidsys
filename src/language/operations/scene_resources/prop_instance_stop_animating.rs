use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceStopAnimatingOp;

const DOC: &str = "Stops animating of the prop instance in the current position.";

pub const OP_CODE: u32 = 1861;

pub const IDENT: &str = "prop_instance_stop_animating";

impl Operation for PropInstanceStopAnimatingOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<scene_prop_id>", "")],
        }
    }
}
