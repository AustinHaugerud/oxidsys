use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamAnimateToPositionAndApertureOp;

const DOC: &str =
    "Not documented. if value = 0, then camera velocity will be linear. else it will be non-linear";

pub const OP_CODE: u32 = 2016;

pub const IDENT: &str = "mission_cam_animate_to_position_and_aperture";

impl Operation for MissionCamAnimateToPositionAndApertureOp {
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
                make_param_doc("<position_register_no>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
