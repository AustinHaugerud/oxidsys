use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropGetMaxHitPointsOp;

const DOC : &str = "Retrieves the maximum number of hit points that the scene prop instance has (useful to calculate the percent of damage).";

pub const OP_CODE: u32 = 1816;

pub const IDENT: &str = "scene_prop_get_max_hit_points";

impl Operation for ScenePropGetMaxHitPointsOp {
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
