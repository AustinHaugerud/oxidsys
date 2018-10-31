use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetAverageGameDifficultyOp;

const DOC : &str = "Returns calculated game difficulty rating (as displayed on the Options page). Commonly used for score calculation when ending the game.";

pub const OP_CODE: u32 = 990;

pub const IDENT: &str = "get_average_game_difficulty";

impl Operation for GetAverageGameDifficultyOp {
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
