use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerSetAntiCheatOp;

const DOC: &str = "Official docs: 0 = off, 1 = on";

pub const OP_CODE: u32 = 477;

pub const IDENT: &str = "server_set_anti_cheat";

impl Operation for ServerSetAntiCheatOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
