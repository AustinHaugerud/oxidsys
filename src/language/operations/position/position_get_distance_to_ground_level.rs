use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionGetDistanceToGroundLevelOp;

const DOC : &str = "This will measure the distance between position and the ground level, taking scene props and their collision meshes into account. Operation only works on the scenes and cannot be used on the global map.";

pub const OP_CODE: u32 = 793;

pub const IDENT: &str = "position_get_distance_to_ground_level";

impl Operation for PositionGetDistanceToGroundLevelOp {
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
