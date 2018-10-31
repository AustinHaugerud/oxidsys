use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionRotateYOp;

const DOC: &str = "Rotates position around Y axis (tilt right/left).";

pub const OP_CODE: u32 = 724;

pub const IDENT: &str = "position_rotate_y";

impl Operation for PositionRotateYOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<angle>", ""),
            ],
        }
    }
}
