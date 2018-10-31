use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FinishMissionOp;

const DOC: &str = "Exits the scene after the specified delay.";

pub const OP_CODE: u32 = 1907;

pub const IDENT: &str = "finish_mission";

impl Operation for FinishMissionOp {
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
            param_docs: vec![make_param_doc("<delay_in_seconds>", "")],
        }
    }
}
