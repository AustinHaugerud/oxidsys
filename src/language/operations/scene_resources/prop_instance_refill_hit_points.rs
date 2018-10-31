use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceRefillHitPointsOp;

const DOC: &str = "Restores hit points of a scene prop instance to their maximum value.";

pub const OP_CODE: u32 = 1870;

pub const IDENT: &str = "prop_instance_refill_hit_points";

impl Operation for PropInstanceRefillHitPointsOp {
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
