use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamSetModeOp;

const DOC : &str = "Not documented. Changes main camera mode. Camera mode is 0 for automatic and 1 for manual (controlled by code). Duration parameter is used when switching from manual to auto, to determine how long will camera move to it's new position. Third parameter is not documented.";

pub const OP_CODE: u32 = 2001;

pub const IDENT: &str = "mission_cam_set_mode";

impl Operation for MissionCamSetModeOp {
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
                make_param_doc("<mission_cam_mode>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
