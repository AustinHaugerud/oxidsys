use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GameInMultiplayerModeOp;

const DOC: &str = "Checks that the game is running in multiplayer mode.";

pub const OP_CODE: u32 = 419;

pub const IDENT: &str = "game_in_multiplayer_mode";

impl Operation for GameInMultiplayerModeOp {
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
