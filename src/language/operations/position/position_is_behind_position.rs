use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionIsBehindPositionOp;

const DOC: &str = "Checks if the second position is behind the first.";

pub const OP_CODE: u32 = 714;

pub const IDENT: &str = "position_is_behind_position";

impl Operation for PositionIsBehindPositionOp {
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
                make_param_doc("<position_base>", ""),
                make_param_doc("<position_to_check>", ""),
            ],
        }
    }
}
