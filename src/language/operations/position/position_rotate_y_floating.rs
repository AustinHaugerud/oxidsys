use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionRotateYFloatingOp;

const DOC : &str = "Same as (position_rotate_y), but takes fixed point value as parameter, allowing for more precise rotation.";

pub const OP_CODE: u32 = 739;

pub const IDENT: &str = "position_rotate_y_floating";

impl Operation for PositionRotateYFloatingOp {
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
                make_param_doc("<angle_fixed_point>", ""),
            ],
        }
    }
}
