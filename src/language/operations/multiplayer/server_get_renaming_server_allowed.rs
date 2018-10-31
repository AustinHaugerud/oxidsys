use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerGetRenamingServerAllowedOp;

const DOC: &str = "Official docs: 0-1";

pub const OP_CODE: u32 = 475;

pub const IDENT: &str = "server_get_renaming_server_allowed";

impl Operation for ServerGetRenamingServerAllowedOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
