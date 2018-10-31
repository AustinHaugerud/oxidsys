use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerGetAntiCheatOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 499;

pub const IDENT: &str = "server_get_anti_cheat";

impl Operation for ServerGetAntiCheatOp {
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
