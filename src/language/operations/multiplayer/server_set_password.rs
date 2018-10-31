use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerSetPasswordOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 484;

pub const IDENT: &str = "server_set_password";

impl Operation for ServerSetPasswordOp {
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
            param_docs: vec![make_param_doc("<string_id>", "")],
        }
    }
}
