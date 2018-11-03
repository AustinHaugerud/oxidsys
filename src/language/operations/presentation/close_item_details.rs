use language::operations::{Operation, ParamInfo};

pub struct CloseItemDetailsOp;

const DOC: &str = "Closes the item details popup box.";

pub const OP_CODE: u32 = 971;

pub const IDENT: &str = "close_item_details";

impl Operation for CloseItemDetailsOp {
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
