use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreStringOp;

const DOC : &str = "Stores a string value in the referenced string register. Only string constants and quick strings can be stored this way.";

pub const OP_CODE: u32 = 2320;

pub const IDENT: &str = "str_store_string";

impl Operation for StrStoreStringOp {
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
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}
