use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CloseOrderMenuOp;

const DOC: &str = "Version 1.161+. If orders menu is currently open, it will be closed.";

pub const OP_CODE: u32 = 1789;

pub const IDENT: &str = "close_order_menu";

impl Operation for CloseOrderMenuOp {
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
