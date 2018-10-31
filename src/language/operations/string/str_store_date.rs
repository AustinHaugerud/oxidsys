use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreDateOp;

const DOC : &str = "Stores formatted date string, using the number of hours since start of the game (can be retrieved by a call to store_current_hours).";

pub const OP_CODE: u32 = 2340;

pub const IDENT: &str = "str_store_date";

impl Operation for StrStoreDateOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<string_register>", ""),
                make_param_doc("<number_of_hours_to_add_to_the_current_date>", ""),
            ],
        }
    }
}
