use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerGetMyPlayerOp;

const DOC: &str = "Client operation. Retrieves current player's player_id reference.";

pub const OP_CODE: u32 = 415;

pub const IDENT: &str = "multiplayer_get_my_player";

impl Operation for MultiplayerGetMyPlayerOp {
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
