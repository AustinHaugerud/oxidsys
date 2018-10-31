use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StartMissionConversationOp;

const DOC: &str = "During the mission, initiates the dialog with specified troop.";

pub const OP_CODE: u32 = 1920;

pub const IDENT: &str = "start_mission_conversation";

impl Operation for StartMissionConversationOp {
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
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
