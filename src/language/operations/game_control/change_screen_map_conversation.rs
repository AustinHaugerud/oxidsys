use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenMapConversationOp;

const DOC : &str = "Starts the mission, same as (change_screen_mission). However once the mission starts, player will get into dialog with the specified troop, and once the dialog ends, the mission will automatically end.";

pub const OP_CODE: u32 = 2049;

pub const IDENT: &str = "change_screen_map_conversation";

impl Operation for ChangeScreenMapConversationOp {
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
