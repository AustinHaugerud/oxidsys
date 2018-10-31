use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamClearTargetAgentOp;

const DOC: &str = "Not documented.";

pub const OP_CODE: u32 = 2018;

pub const IDENT: &str = "mission_cam_clear_target_agent";

impl Operation for MissionCamClearTargetAgentOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
