use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionTransformPositionToParentOp;

const DOC : &str = "Converts position from local coordinate space to parent coordinate space. In other words, if you have some position on the scene (anchor) and a position describing some place *relative* to anchor (for example [10,20,0] means \"20 meters forward and 10 meters to the right\"), after calling this operation you will get that position coordinates on the scene in <position_dest>. Rotation and scale is also taken care of, so you can use relative angles.";

pub const OP_CODE: u32 = 716;

pub const IDENT: &str = "position_transform_position_to_parent";

impl Operation for PositionTransformPositionToParentOp {
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
                make_param_doc("<position_dest>", ""),
                make_param_doc("<position_anchor>", ""),
                make_param_doc("<position_relative_to_anchor>", ""),
            ],
        }
    }
}
