use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetJumpEntryOp;

const DOC: &str = "Defines what entry point the player will appear at when the mission starts.";

pub const OP_CODE: u32 = 1912;

pub const IDENT: &str = "set_jump_entry";

impl Operation for SetJumpEntryOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<entry_no>", "")],
        }
    }
}
