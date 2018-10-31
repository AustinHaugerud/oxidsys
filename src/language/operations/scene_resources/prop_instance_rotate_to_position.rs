use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceRotateToPositionOp;

const DOC : &str = "Specified prop instance will move to the target position within the specified duration of time, and within the same time it will rotate for the specified angle. Used in Native code to simulate behavior of belfry wheels and rotating winches.";

pub const OP_CODE: u32 = 1865;

pub const IDENT: &str = "prop_instance_rotate_to_position";

impl Operation for PropInstanceRotateToPositionOp {
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
                make_param_doc("<scene_prop_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<total_rotate_angle_fixed_point>", ""),
            ],
        }
    }
}
