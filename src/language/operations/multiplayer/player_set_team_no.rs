use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSetTeamNoOp;

const DOC: &str = "Assigns a player to the specified team.";

pub const OP_CODE: u32 = 403;

pub const IDENT: &str = "player_set_team_no";

impl Operation for PlayerSetTeamNoOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<player_id>", ""),
                make_param_doc("<team_id>", ""),
            ],
        }
    }
}
