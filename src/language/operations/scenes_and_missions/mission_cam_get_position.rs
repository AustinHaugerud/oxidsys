use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamGetPositionOp;

const DOC : &str = "Retrieves the current position of camera during the mission (i.e. the point from which the player is observing the game).";

pub const OP_CODE: u32 = 2010;

pub const IDENT: &str = "mission_cam_get_position";

impl Operation for MissionCamGetPositionOp {
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
