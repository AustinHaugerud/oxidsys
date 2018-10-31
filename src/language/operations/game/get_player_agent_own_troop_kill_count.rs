use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetPlayerAgentOwnTroopKillCountOp;

const DOC : &str = "Retrieves the total number of allies killed by the player. Call with non-zero <get_wounded> parameter to retrieve the total number of knocked down allies.";

pub const OP_CODE: u32 = 1705;

pub const IDENT: &str = "get_player_agent_own_troop_kill_count";

impl Operation for GetPlayerAgentOwnTroopKillCountOp {
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
                make_param_doc("[get_wounded]", ""),
            ],
        }
    }
}
