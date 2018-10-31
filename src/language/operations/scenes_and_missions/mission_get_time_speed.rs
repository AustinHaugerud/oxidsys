use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionGetTimeSpeedOp;

const DOC: &str = "Retrieves current time speed factor for the mission.";

pub const OP_CODE: u32 = 2002;

pub const IDENT: &str = "mission_get_time_speed";

impl Operation for MissionGetTimeSpeedOp {
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
            param_docs: vec![make_param_doc("<destination_fixed_point>", "")],
        }
    }
}
