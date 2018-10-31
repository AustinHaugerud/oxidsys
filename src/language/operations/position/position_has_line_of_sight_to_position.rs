use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionHasLineOfSightToPositionOp;

const DOC : &str = "Checks that you can see one position from another. This obviously implies that both positions must be in global space. Note this is computationally expensive, so try to keep number of these to a minimum.";

pub const OP_CODE: u32 = 707;

pub const IDENT: &str = "position_has_line_of_sight_to_position";

impl Operation for PositionHasLineOfSightToPositionOp {
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
                make_param_doc("<position_no_1>", ""),
                make_param_doc("<position_no_2>", ""),
            ],
        }
    }
}
