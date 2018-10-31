use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenViewCharacterOp;

const DOC: &str = "Opens the character screen of another troop. Can only be used in dialogs.";

pub const OP_CODE: u32 = 2046;

pub const IDENT: &str = "change_screen_view_character";

impl Operation for ChangeScreenViewCharacterOp {
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
