use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetLevelBoundaryOp;

const DOC : &str = "Returns the amount of experience points required to reach the specified level (will return 0 for 1st level). Maximum possible level in the game is 63.";

pub const OP_CODE: u32 = 991;

pub const IDENT: &str = "get_level_boundary";

impl Operation for GetLevelBoundaryOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<level_no>", ""),
            ],
        }
    }
}
