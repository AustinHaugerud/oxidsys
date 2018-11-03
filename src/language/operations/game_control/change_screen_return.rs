use language::operations::{Operation, ParamInfo};

pub struct ChangeScreenReturnOp;

const DOC : &str = "Closes any current screen and returns the player to worldmap (to scene?). 4research how it behaves in missions.";

pub const OP_CODE: u32 = 2040;

pub const IDENT: &str = "change_screen_return";

impl Operation for ChangeScreenReturnOp {
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
