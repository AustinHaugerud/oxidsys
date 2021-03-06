use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreServerPasswordOp;

const DOC: &str = "Stores server's password in referenced string register.";

pub const OP_CODE: u32 = 2351;

pub const IDENT: &str = "str_store_server_password";

impl Operation for StrStoreServerPasswordOp {
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
