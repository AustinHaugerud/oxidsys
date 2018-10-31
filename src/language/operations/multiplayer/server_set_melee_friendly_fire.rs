use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerSetMeleeFriendlyFireOp;

const DOC: &str = "Official docs: 0 = off, 1 = on";

pub const OP_CODE: u32 = 494;

pub const IDENT: &str = "server_set_melee_friendly_fire";

impl Operation for ServerSetMeleeFriendlyFireOp {
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
