use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct LeaveEncounterOp;

const DOC: &str = "Leaves encounter mode.";

pub const OP_CODE: u32 = 1301;

pub const IDENT: &str = "leave_encounter";

impl Operation for LeaveEncounterOp {
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
