use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreMissionTimerCOp;

const DOC: &str = "Retrieves current value of third mission timer, in seconds.";

pub const OP_CODE: u32 = 2372;

pub const IDENT: &str = "store_mission_timer_c";

impl Operation for StoreMissionTimerCOp {
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
