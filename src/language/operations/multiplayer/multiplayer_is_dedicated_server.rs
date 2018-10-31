use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerIsDedicatedServerOp;

const DOC: &str = "Checks that the code is running on dedicated multiplayer server machine.";

pub const OP_CODE: u32 = 418;

pub const IDENT: &str = "multiplayer_is_dedicated_server";

impl Operation for MultiplayerIsDedicatedServerOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
