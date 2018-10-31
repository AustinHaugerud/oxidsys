use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ThisOrNextOp;

const DOC: &str = "Used in combination with conditional operations to group them into OR blocks.";

pub const OP_CODE: u32 = 1073741824;

pub const IDENT: &str = "this_or_next";

impl Operation for ThisOrNextOp {
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
            param_docs: vec![make_param_doc("<operation_name>", "")],
        }
    }
}
