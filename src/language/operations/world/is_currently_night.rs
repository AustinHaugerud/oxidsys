use language::operations::{Operation, ParamInfo};

pub struct IsCurrentlyNightOp;

const DOC: &str = "Checks that it's currently night in the game.";

pub const OP_CODE: u32 = 2273;

pub const IDENT: &str = "is_currently_night";

impl Operation for IsCurrentlyNightOp {
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
