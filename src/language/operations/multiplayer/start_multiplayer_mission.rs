use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StartMultiplayerMissionOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 470;

pub const IDENT: &str = "start_multiplayer_mission";

impl Operation for StartMultiplayerMissionOp {
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
                make_param_doc("<mission_template_id>", ""),
                make_param_doc("<scene_id>", ""),
                make_param_doc("<started_manually>", ""),
            ],
        }
    }
}
