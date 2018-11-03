use language::operations::{Operation, ParamInfo};

pub struct ResetMissionTimerAOp;

const DOC: &str = "Resets the value of first mission timer and starts it from zero.";

pub const OP_CODE: u32 = 2375;

pub const IDENT: &str = "reset_mission_timer_a";

impl Operation for ResetMissionTimerAOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
