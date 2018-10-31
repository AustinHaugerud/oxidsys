use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ResetMissionTimerBOp;

const DOC: &str = "Resets the value of second mission timer and starts it from zero.";

pub const OP_CODE: u32 = 2376;

pub const IDENT: &str = "reset_mission_timer_b";

impl Operation for ResetMissionTimerBOp {
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
