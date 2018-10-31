use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenTrainingOp;

const DOC : &str = "Opens the character screen for the troop that player is currently talking to. Only works in dialogs. Deprecated, use (change_screen_view_character) instead.";

pub const OP_CODE: u32 = 2047;

pub const IDENT: &str = "change_screen_training";

impl Operation for ChangeScreenTrainingOp {
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
