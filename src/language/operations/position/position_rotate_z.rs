use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionRotateZOp;

const DOC : &str = "Rotates position around Z axis (rotate right/left). Pass 1 for use_global_z_axis to rotate the position around global axis instead.";

pub const OP_CODE: u32 = 725;

pub const IDENT: &str = "position_rotate_z";

impl Operation for PositionRotateZOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<position>", ""),
                make_param_doc("<angle>", ""),
                make_param_doc("[use_global_z_axis]", ""),
            ],
        }
    }
}
