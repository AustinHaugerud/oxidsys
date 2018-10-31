use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetMissionResultOp;

const DOC: &str = "Sets the result of the current mission (1 for victory, -1 for defeat).";

pub const OP_CODE: u32 = 1906;

pub const IDENT: &str = "set_mission_result";

impl Operation for SetMissionResultOp {
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
