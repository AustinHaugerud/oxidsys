use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetAngleBetweenPositionsOp;

const DOC : &str = "Calculates angle between positions, using positions as vectors. Only rotation around Z axis is used. In other words, the function returns the difference between Z rotations of both positions.";

pub const OP_CODE: u32 = 705;

pub const IDENT: &str = "get_angle_between_positions";

impl Operation for GetAngleBetweenPositionsOp {
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
                make_param_doc("<destination_fixed_point>", ""),
                make_param_doc("<position_no_1>", ""),
                make_param_doc("<position_no_2>", ""),
            ],
        }
    }
}
