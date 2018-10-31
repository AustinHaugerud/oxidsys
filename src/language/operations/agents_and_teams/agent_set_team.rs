use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetTeamOp;

const DOC: &str = "Puts the agent to specified team number.";

pub const OP_CODE: u32 = 1771;

pub const IDENT: &str = "agent_set_team";

impl Operation for AgentSetTeamOp {
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
