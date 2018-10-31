use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreStringRegOp;

const DOC: &str = "Copies the contents of one string register from another.";

pub const OP_CODE: u32 = 2321;

pub const IDENT: &str = "str_store_string_reg";

impl Operation for StrStoreStringRegOp {
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
                make_param_doc("<string_no>", ""),
            ],
        }
    }
}
