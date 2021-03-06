use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionGetRotationAroundYOp;

const DOC: &str =
    "Returns angle (in degrees) that the position is rotated around Y axis (tilt right/left).";

pub const OP_CODE: u32 = 743;

pub const IDENT: &str = "position_get_rotation_around_y";

impl Operation for PositionGetRotationAroundYOp {
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
