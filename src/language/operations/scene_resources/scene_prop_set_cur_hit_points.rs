use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropSetCurHitPointsOp;

const DOC: &str = "Version 1.153+. Sets current HP amount for scene prop.";

pub const OP_CODE: u32 = 1820;

pub const IDENT: &str = "scene_prop_set_cur_hit_points";

impl Operation for ScenePropSetCurHitPointsOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
