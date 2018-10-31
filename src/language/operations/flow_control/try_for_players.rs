use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TryForPlayersOp;

const DOC : &str = "Version 1.165+. Iterates through all players in a multiplayer game. Set optional parameter to 1 to skip server player entry.";

pub const OP_CODE: u32 = 17;

pub const IDENT: &str = "try_for_players";

impl Operation for TryForPlayersOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("[skip_server]", ""),
            ],
        }
    }
}
