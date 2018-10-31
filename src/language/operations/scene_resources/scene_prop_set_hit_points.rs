use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropSetHitPointsOp;

const DOC : &str = "Sets the number of hit points that the scene prop has. Both current and max hit points are affected. Only makes sense for sokf_destructible scene props.";

pub const OP_CODE: u32 = 1814;

pub const IDENT: &str = "scene_prop_set_hit_points";

impl Operation for ScenePropSetHitPointsOp {
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
