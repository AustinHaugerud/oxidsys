use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionSetTimeSpeedOp;

const DOC : &str = "Instantly changes the speed of time during the mission. Speed of time cannot be set to zero or below. Operation only works when cheat mode is enabled.";

pub const OP_CODE: u32 = 2003;

pub const IDENT: &str = "mission_set_time_speed";

impl Operation for MissionSetTimeSpeedOp {
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
            param_docs: vec![make_param_doc("<value_fixed_point>", "")],
        }
    }
}
