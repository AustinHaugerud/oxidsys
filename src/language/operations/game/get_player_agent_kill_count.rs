use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetPlayerAgentKillCountOp;

const DOC : &str = "Retrieves the total number of enemies killed by the player. Call with non-zero <get_wounded> parameter to retrieve the total number of knocked down enemies.";

pub const OP_CODE: u32 = 1701;

pub const IDENT: &str = "get_player_agent_kill_count";

impl Operation for GetPlayerAgentKillCountOp {
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
