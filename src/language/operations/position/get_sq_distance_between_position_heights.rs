use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetSqDistanceBetweenPositionHeightsOp;

const DOC: &str = "Returns squared distance between position *heights* in centimeters.";

pub const OP_CODE: u32 = 715;

pub const IDENT: &str = "get_sq_distance_between_position_heights";

impl Operation for GetSqDistanceBetweenPositionHeightsOp {
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
