use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionGetRotationAroundXOp;

const DOC : &str = "Returns angle (in degrees) that the position is rotated around X axis (tilt forward/backwards).";

pub const OP_CODE: u32 = 742;

pub const IDENT: &str = "position_get_rotation_around_x";

impl Operation for PositionGetRotationAroundXOp {
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
                make_param_doc("<position>", ""),
            ],
        }
    }
}
