use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerSetControlBlockDirOp;

const DOC: &str = "Official docs: 0 = automatic, 1 = by mouse movement";

pub const OP_CODE: u32 = 483;

pub const IDENT: &str = "server_set_control_block_dir";

impl Operation for ServerSetControlBlockDirOp {
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
