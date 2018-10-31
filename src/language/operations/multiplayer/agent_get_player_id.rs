use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetPlayerIdOp;

const DOC: &str = "Retrieves player reference that is currently controlling the specified agent.";

pub const OP_CODE: u32 = 1724;

pub const IDENT: &str = "agent_get_player_id";

impl Operation for AgentGetPlayerIdOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
