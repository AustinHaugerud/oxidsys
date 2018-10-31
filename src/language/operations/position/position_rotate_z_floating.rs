use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionRotateZFloatingOp;

const DOC : &str = "Version 1.161+. Same as (position_rotate_z), but takes fixed point value as parameter, allowing for more precise rotation.";

pub const OP_CODE: u32 = 734;

pub const IDENT: &str = "position_rotate_z_floating";

impl Operation for PositionRotateZFloatingOp {
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
                make_param_doc("<position_no>", ""),
                make_param_doc("<angle_fixed_point>", ""),
            ],
        }
    }
}
