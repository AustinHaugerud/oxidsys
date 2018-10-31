use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreMissionTimerAMsecOp;

const DOC: &str = "Retrieves current value of first mission timer, in milliseconds.";

pub const OP_CODE: u32 = 2365;

pub const IDENT: &str = "store_mission_timer_a_msec";

impl Operation for StoreMissionTimerAMsecOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
