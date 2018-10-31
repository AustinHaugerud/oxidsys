use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AutoSaveOp;

const DOC: &str = "Version 1.161+. Saves the game to the current save slot.";

pub const OP_CODE: u32 = 985;

pub const IDENT: &str = "auto_save";

impl Operation for AutoSaveOp {
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
