use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenControlsOp;

const DOC: &str = "Opens the standard Configure Controls screen, pausing the game.";

pub const OP_CODE: u32 = 2057;

pub const IDENT: &str = "change_screen_controls";

impl Operation for ChangeScreenControlsOp {
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
