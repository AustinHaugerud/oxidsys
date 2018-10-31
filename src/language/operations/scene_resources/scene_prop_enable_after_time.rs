use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropEnableAfterTimeOp;

const DOC : &str = "Prevents usable scene prop from being used for the specified time period in 1/100th of second. Commonly used to implement \"cooldown\" periods.";

pub const OP_CODE: u32 = 1800;

pub const IDENT: &str = "scene_prop_enable_after_time";

impl Operation for ScenePropEnableAfterTimeOp {
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
                make_param_doc("<time_period>", ""),
            ],
        }
    }
}
