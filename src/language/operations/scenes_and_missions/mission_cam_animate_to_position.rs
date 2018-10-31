use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamAnimateToPositionOp;

const DOC : &str = "Moves the camera to the specified position smoothly. Second parameter determines how long it will take camera to move to destination, third parameter determines whether camera velocity will be linear (value = 0) or non-linear (value = 1).";

pub const OP_CODE: u32 = 2012;

pub const IDENT: &str = "mission_cam_animate_to_position";

impl Operation for MissionCamAnimateToPositionOp {
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
                make_param_doc("<position_register_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
