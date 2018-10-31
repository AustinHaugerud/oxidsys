use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrClearOp;

const DOC: &str = "Clears the contents of the referenced string register.";

pub const OP_CODE: u32 = 2319;

pub const IDENT: &str = "str_clear";

impl Operation for StrClearOp {
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
            param_docs: vec![make_param_doc("<string_register>", "")],
        }
    }
}
