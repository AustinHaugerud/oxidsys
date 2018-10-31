use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamSetPositionOp;

const DOC: &str = "Moves the camera to the specified position during the mission.";

pub const OP_CODE: u32 = 2011;

pub const IDENT: &str = "mission_cam_set_position";

impl Operation for MissionCamSetPositionOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<position_register_no>", "")],
        }
    }
}
