use language::operations::{Operation, ParamInfo};

pub struct ConversationScreenIsActiveOp;

const DOC : &str = "Checks that the player is currently in dialogue with some agent. Can only be used in triggers of module_mission_templates.py file.";

pub const OP_CODE: u32 = 42;

pub const IDENT: &str = "conversation_screen_is_active";

impl Operation for ConversationScreenIsActiveOp {
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
