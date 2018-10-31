use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct IsEditModeEnabledOp;

const DOC: &str = "Version 1.153+. Checks that Edit Mode is currently enabled in the game.";

pub const OP_CODE: u32 = 255;

pub const IDENT: &str = "is_edit_mode_enabled";

impl Operation for IsEditModeEnabledOp {
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
