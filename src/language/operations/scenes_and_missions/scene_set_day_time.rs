use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SceneSetDayTimeOp;

const DOC : &str = "Defines the time for the scene to force the engine to select a different skybox than the one dictated by current game time. Must be called within ti_before_mission_start trigger in module_mission_templates.py. Value should be in range 0..23.";

pub const OP_CODE: u32 = 1266;

pub const IDENT: &str = "scene_set_day_time";

impl Operation for SceneSetDayTimeOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
