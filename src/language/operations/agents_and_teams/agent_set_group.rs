use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetGroupOp;

const DOC: &str =
    "Puts the bot agent under command of specified player. Only works in multiplayer.";

pub const OP_CODE: u32 = 1766;

pub const IDENT: &str = "agent_set_group";

impl Operation for AgentSetGroupOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<player_leader_id>", ""),
            ],
        }
    }
}
