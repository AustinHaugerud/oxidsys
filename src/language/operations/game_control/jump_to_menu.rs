use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct JumpToMenuOp;

const DOC : &str = "Opens the specified game menu. Note this only happens after the current block of code completes execution.";

pub const OP_CODE: u32 = 2060;

pub const IDENT: &str = "jump_to_menu";

impl Operation for JumpToMenuOp {
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
            param_docs: vec![make_param_doc("<menu_id>", "")],
        }
    }
}
