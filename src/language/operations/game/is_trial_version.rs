use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct IsTrialVersionOp;

const DOC : &str = "Checks if the game is in trial mode (has not been purchased). Player cannot get higher than level 6 in this mode.";

pub const OP_CODE: u32 = 250;

pub const IDENT: &str = "is_trial_version";

impl Operation for IsTrialVersionOp {
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
