use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamSetTargetAgentOp;

const DOC: &str =
    "Not documented. if value = 0 then do not use agent's rotation, else use agent's rotation";

pub const OP_CODE: u32 = 2017;

pub const IDENT: &str = "mission_cam_set_target_agent";

impl Operation for MissionCamSetTargetAgentOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
