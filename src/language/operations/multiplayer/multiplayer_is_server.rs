use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerIsServerOp;

const DOC : &str = "Checks that the code is running on multiplayer server. Operation will fail on client machines or in singleplayer mode.";

pub const OP_CODE: u32 = 417;

pub const IDENT: &str = "multiplayer_is_server";

impl Operation for MultiplayerIsServerOp {
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
