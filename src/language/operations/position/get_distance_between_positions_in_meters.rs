use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetDistanceBetweenPositionsInMetersOp;

const DOC: &str = "Returns distance between positions in meters.";

pub const OP_CODE: u32 = 711;

pub const IDENT: &str = "get_distance_between_positions_in_meters";

impl Operation for GetDistanceBetweenPositionsInMetersOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<position_no_1>", ""),
                make_param_doc("<position_no_2>", ""),
            ],
        }
    }
}
