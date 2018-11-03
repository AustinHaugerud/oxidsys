use language::operations::{Operation, ParamInfo};

pub struct ChangeScreenOptionsOp;

const DOC: &str = "Opens the standard Game Options screen, pausing the game.";

pub const OP_CODE: u32 = 2058;

pub const IDENT: &str = "change_screen_options";

impl Operation for ChangeScreenOptionsOp {
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
