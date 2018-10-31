use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenQuitOp;

const DOC: &str = "Quits the game to the main menu.";

pub const OP_CODE: u32 = 2055;

pub const IDENT: &str = "change_screen_quit";

impl Operation for ChangeScreenQuitOp {
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
