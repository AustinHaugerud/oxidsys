use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauClearOverrideItemsOp;

const DOC : &str = "Removes and previously defined equipment overrides for the troop, allowing to start from scratch.";

pub const OP_CODE: u32 = 1998;

pub const IDENT: &str = "cur_tableau_clear_override_items";

impl Operation for CurTableauClearOverrideItemsOp {
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
