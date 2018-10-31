use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentIsAllyOp;

const DOC : &str = "Checks that the agent is allied to the player (belongs to player's party or allied party in current encounter).";

pub const OP_CODE: u32 = 1706;

pub const IDENT: &str = "agent_is_ally";

impl Operation for AgentIsAllyOp {
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
            param_docs: vec![make_param_doc("<agent_id>", "")],
        }
    }
}
