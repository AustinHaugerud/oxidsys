use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionTimeSpeedMoveToValueOp;

const DOC : &str = "Changes the speed of time during the mission gradually, within the specified duration period. Speed of time cannot be set to zero or below. Operation only works when cheat mode is enabled.";

pub const OP_CODE: u32 = 2004;

pub const IDENT: &str = "mission_time_speed_move_to_value";

impl Operation for MissionTimeSpeedMoveToValueOp {
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
