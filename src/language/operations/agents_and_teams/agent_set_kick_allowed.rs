use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetKickAllowedOp;

const DOC : &str = "Enables (value = 1) or disables (value = 0) kicking for the specified agent. Only makes sense for player-controlled agents as bots don't know how to kick anyway.";

pub const OP_CODE: u32 = 1754;

pub const IDENT: &str = "agent_set_kick_allowed";

impl Operation for AgentSetKickAllowedOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
